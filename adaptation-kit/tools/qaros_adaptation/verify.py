"""`verify`: checks a running QAROS web app source through the Visualizer's MCP endpoint
(`POST http://<host>:<port>/mcp`, JSON-RPC `tools/call`). It needs the MCP endpoint enabled and the
Visualizer in Developer UI mode, and at least one streamed user (a headset or a streamed test
receiver) so the page draws users' views.

Checks:

1. **Depth coverage:** every 2x2 block of a colour view that holds colour has depth in its packed
   depth sample (not the clear value). Colour without depth is dropped by the mixer.
2. **Thin-line coverage:** the same, only over thin features (a coloured block with at most three
   coloured neighbours of eight: lines and edges one or two pixels wide), which a single-texel pack
   misses in about half their blocks.
3. **Alpha coverage** (only with `TransportAlpha`): every coloured block has a non-zero alpha sample.
4. **Depth pack readback** (profiles using webgl-atlas-pack@1, such as three.js and raw WebGL): the
   pack shader, compiled in the page's own WebGL2, writes near/mid/far depth as the expected bytes
   (`verify-depth.py`).

The layout comes from the page's next `qaros-render-frame` request, the pixels from `capture_surface`
(the atlas the page painted). "Coloured" means farther than `--threshold` from `--background` in
any channel: users' views have no page background, so their empty pixels are the clear colour.
"""

from __future__ import annotations

import base64
import json
import os
import re
import struct
import urllib.request
import zlib
from dataclasses import dataclass, field
from pathlib import Path

DEFAULT_PORT = 17872


class VerifyError(RuntimeError):
    """The Visualizer could not be reached, or a check failed to run."""


def default_port() -> int:
    """The Visualizer binds QAR_MCP_SERVER_PORT, else QAR_VIZ_AUTOMATION_PORT, else 17872."""
    value = os.environ.get("QAR_MCP_SERVER_PORT") or os.environ.get("QAR_VIZ_AUTOMATION_PORT", "")
    return int(value) if value.isdigit() else DEFAULT_PORT


# --------------------------------------------------------------------------------------------------
# MCP
# --------------------------------------------------------------------------------------------------


@dataclass(frozen=True)
class Mcp:
    host: str = "127.0.0.1"
    port: int = DEFAULT_PORT
    timeout: float = 30.0

    def call(self, name: str, arguments: dict) -> dict:
        """The tool's full `result` (structuredContent and content items)."""
        body = json.dumps({
            "jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": {"name": name, "arguments": arguments},
        }).encode()
        request = urllib.request.Request(
            f"http://{self.host}:{self.port}/mcp", data=body, headers={"Content-Type": "application/json"})
        try:
            with urllib.request.urlopen(request, timeout=self.timeout) as response:
                answer = json.load(response)
        except OSError as error:
            raise VerifyError(f"cannot reach the Visualizer MCP endpoint at {self.host}:{self.port}: {error}") from error
        if "error" in answer:
            raise VerifyError(f"MCP {name} failed: {answer['error']}")
        result = answer["result"]
        if result.get("isError"):
            texts = [item.get("text", "") for item in result.get("content", [])]
            raise VerifyError(f"MCP {name} failed: {' '.join(texts)}")
        return result

    def structured(self, name: str, arguments: dict) -> dict:
        result = self.call(name, arguments)
        if result.get("structuredContent") is None:
            texts = [item.get("text", "") for item in result.get("content", [])]
            raise VerifyError(f"MCP {name} answered no structured content: {' '.join(texts)}")
        return result["structuredContent"]

    def image(self, name: str, arguments: dict) -> bytes:
        for item in self.call(name, arguments).get("content", []):
            if item.get("type") == "image" and item.get("data"):
                return base64.b64decode(item["data"])
        raise VerifyError(f"MCP {name} answered no image")

    def evaluate(self, instance: str, expression: str, await_promise: bool = False):
        result = self.structured("devtools_send", {
            "instance_id": instance,
            "method": "Runtime.evaluate",
            "params": {"expression": expression, "returnByValue": True, "awaitPromise": await_promise},
        })
        payload = result.get("payload", {})
        if not result.get("success") or "exceptionDetails" in payload:
            raise VerifyError(f"page evaluation failed: {json.dumps(payload.get('exceptionDetails') or result)[:400]}")
        return payload.get("result", {}).get("value")


def _address_of(source: dict) -> str:
    for key in ("address", "url", "current_address", "start_address"):
        if isinstance(source.get(key), str):
            return source[key]
    return ""


def find_instance(mcp: Mcp, address: str | None = None, pattern: str | None = None) -> str:
    """The web app source to verify: the one at `address`, else the one whose address matches
    `pattern` (the profile's AddressPattern), else the only one."""
    from .profile import matches_address  # noqa: PLC0415

    listing = mcp.structured("viz_webapp_list", {})
    sources = listing.get("sources") or listing.get("instances") or []
    if not sources:
        raise VerifyError("the Visualizer has no web app source; open the page first")
    if address:
        chosen = [source for source in sources if _address_of(source) == address]
    elif pattern:
        chosen = [source for source in sources if matches_address(pattern, _address_of(source))]
    else:
        chosen = sources
    if len(chosen) != 1:
        found = ", ".join(f"{source.get('instance_id')} ({_address_of(source)})" for source in sources)
        raise VerifyError(f"{len(chosen)} web app sources match; pass --instance. Sources: {found}")
    return str(chosen[0]["instance_id"])


# --------------------------------------------------------------------------------------------------
# PNG
# --------------------------------------------------------------------------------------------------


@dataclass(frozen=True)
class Image:
    width: int
    height: int
    channels: int
    pixels: bytes  # rows top-down, `channels` bytes per pixel

    def pixel(self, x: int, y: int) -> tuple[int, ...]:
        offset = (y * self.width + x) * self.channels
        return tuple(self.pixels[offset : offset + self.channels])


def decode_png(data: bytes) -> Image:
    """8-bit greyscale, RGB or RGBA, non-interlaced: what capture_surface writes."""
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise VerifyError("not a PNG image")
    offset = 8
    header = None
    compressed = bytearray()
    while offset < len(data):
        length, kind = struct.unpack_from(">I4s", data, offset)
        chunk = data[offset + 8 : offset + 8 + length]
        offset += 12 + length
        if kind == b"IHDR":
            header = struct.unpack(">IIBBBBB", chunk)
        elif kind == b"IDAT":
            compressed += chunk
        elif kind == b"IEND":
            break
    if header is None:
        raise VerifyError("the PNG has no header")
    width, height, depth, colour, _, _, interlace = header
    channels = {0: 1, 2: 3, 6: 4, 4: 2}.get(colour)
    if depth != 8 or channels is None or interlace:
        raise VerifyError(f"unsupported PNG (bit depth {depth}, colour type {colour}, interlace {interlace})")
    raw = zlib.decompress(bytes(compressed))
    stride = width * channels
    out = bytearray(height * stride)
    previous = bytearray(stride)
    position = 0
    for row in range(height):
        kind = raw[position]
        line = bytearray(raw[position + 1 : position + 1 + stride])
        position += 1 + stride
        for index in range(stride):
            left = line[index - channels] if index >= channels else 0
            up = previous[index]
            up_left = previous[index - channels] if index >= channels else 0
            if kind == 1:
                line[index] = (line[index] + left) & 0xFF
            elif kind == 2:
                line[index] = (line[index] + up) & 0xFF
            elif kind == 3:
                line[index] = (line[index] + ((left + up) >> 1)) & 0xFF
            elif kind == 4:
                estimate = left + up - up_left
                distances = (abs(estimate - left), abs(estimate - up), abs(estimate - up_left))
                predictor = left if distances[0] <= distances[1] and distances[0] <= distances[2] else up if distances[1] <= distances[2] else up_left
                line[index] = (line[index] + predictor) & 0xFF
        out[row * stride : (row + 1) * stride] = line
        previous = line
    return Image(width, height, channels, bytes(out))


# --------------------------------------------------------------------------------------------------
# Coverage
# --------------------------------------------------------------------------------------------------


@dataclass
class Coverage:
    """One colour view's blocks and how many of them miss depth or alpha."""

    label: str
    blocks: int = 0
    depth_missing: int = 0
    thin_blocks: int = 0
    thin_missing: int = 0
    alpha_blocks: int = 0
    alpha_missing: int = 0
    notes: list[str] = field(default_factory=list)

    @staticmethod
    def ratio(missing: int, total: int) -> float:
        return 1.0 if total == 0 else 1.0 - missing / total

    @property
    def depth(self) -> float:
        return self.ratio(self.depth_missing, self.blocks)

    @property
    def thin(self) -> float:
        return self.ratio(self.thin_missing, self.thin_blocks)

    @property
    def alpha(self) -> float:
        return self.ratio(self.alpha_missing, self.alpha_blocks)


def _views_by_eye(target: dict, data_type: str) -> dict[str, dict]:
    views = {}
    for view in target.get("views", []):
        if view.get("dataType") == data_type:
            eye = view.get("eye", "left")
            views["left" if eye == "none" else eye] = view
    return views


def _depth_sample(image: Image, view: dict, column: int, row: int) -> int | None:
    rect = view["rect"]
    half_width = int(view.get("logicalWidth", rect["width"])) // 2
    tile_height = int(rect["height"])
    if column >= half_width or row >= 2 * tile_height:
        return None
    x = rect["x"] + column + (half_width if row >= tile_height else 0)
    y = rect["y"] + (row - tile_height if row >= tile_height else row)
    if not (0 <= x < image.width and 0 <= y < image.height):
        return None
    r, g, b = image.pixel(x, y)[:3]
    return r | (g << 8) | (b << 16)


def _alpha_sample(image: Image, view: dict, column: int, row: int) -> int | None:
    rect = view["rect"]
    x, y = rect["x"] + column, rect["y"] + row // 3
    if column >= rect["width"] or row // 3 >= rect["height"] or not (0 <= x < image.width and 0 <= y < image.height):
        return None
    return image.pixel(x, y)[row % 3]


def analyse(
    image: Image,
    detail: dict,
    background: tuple[int, int, int] = (0, 0, 0),
    threshold: int = 16,
) -> list[Coverage]:
    """Depth, thin-line and alpha coverage of every target's colour views in `image` (the atlas)."""
    results = []
    for target_index, target in enumerate(detail.get("targets", [])):
        colours = _views_by_eye(target, "color")
        depths = _views_by_eye(target, "depth")
        alphas = _views_by_eye(target, "alpha")
        for eye, colour in sorted(colours.items()):
            coverage = Coverage(f"target {target.get('id', target_index)} {eye}")
            camera = (target.get("cameras") or {}).get(eye) or {}
            empty_depth = 0 if camera.get("depthConvention") == "inverted" else 0xFFFFFF
            depth = depths.get(eye) or depths.get("left")
            alpha = alphas.get(eye)
            if depth is None:
                coverage.notes.append("no depth view")
            rect = colour["rect"]
            columns, rows = (rect["width"] + 1) // 2, (rect["height"] + 1) // 2
            coloured = [[False] * columns for _ in range(rows)]
            for row in range(rows):
                for column in range(columns):
                    for dy in (0, 1):
                        for dx in (0, 1):
                            x, y = rect["x"] + 2 * column + dx, rect["y"] + 2 * row + dy
                            if x >= rect["x"] + rect["width"] or y >= rect["y"] + rect["height"]:
                                continue
                            if not (0 <= x < image.width and 0 <= y < image.height):
                                continue
                            pixel = image.pixel(x, y)
                            if max(abs(pixel[channel] - background[channel]) for channel in range(3)) > threshold:
                                coloured[row][column] = True
            for row in range(rows):
                for column in range(columns):
                    if not coloured[row][column]:
                        continue
                    neighbours = sum(
                        coloured[row + dy][column + dx]
                        for dy in (-1, 0, 1)
                        for dx in (-1, 0, 1)
                        if (dx or dy) and 0 <= row + dy < rows and 0 <= column + dx < columns
                    )
                    thin = neighbours <= 3
                    coverage.blocks += 1
                    coverage.thin_blocks += thin
                    if depth is not None:
                        sample = _depth_sample(image, depth, column, row)
                        if sample is None or sample == empty_depth:
                            coverage.depth_missing += 1
                            coverage.thin_missing += thin
                    if alpha is not None:
                        coverage.alpha_blocks += 1
                        sample = _alpha_sample(image, alpha, column, row)
                        if not sample:
                            coverage.alpha_missing += 1
            results.append(coverage)
    return results


# --------------------------------------------------------------------------------------------------
# The live checks
# --------------------------------------------------------------------------------------------------

_NEXT_REQUEST = r"""new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error('no qaros-render-frame request within 10 s: is a user streaming?')), 10000);
    window.addEventListener('qaros-render-frame', ({ detail }) => {
        clearTimeout(timer);
        resolve(JSON.parse(JSON.stringify(detail)));
    }, { once: true });
})"""


def next_request(mcp: Mcp, instance: str) -> dict:
    detail = mcp.evaluate(instance, _NEXT_REQUEST, await_promise=True)
    if not isinstance(detail, dict) or not detail.get("targets"):
        raise VerifyError("the render-frame request names no target: connect a user (headset or streamed receiver)")
    return detail


def capture_atlas(mcp: Mcp, instance: str) -> Image:
    return decode_png(mcp.image("capture_surface", {"instance_id": instance}))


_PACK_READBACK = r"""(() => {
    const canvas = document.createElement('canvas');
    canvas.width = canvas.height = 1;
    const gl = canvas.getContext('webgl2');
    if (!gl) throw Error('WebGL2 unavailable');
    const shader = (type, source) => {
        const object = gl.createShader(type);
        gl.shaderSource(object, source); gl.compileShader(object);
        if (!gl.getShaderParameter(object, gl.COMPILE_STATUS))
            throw Error(gl.getShaderInfoLog(object));
        return object;
    };
    try {
        const program = gl.createProgram();
        gl.attachShader(program, shader(gl.VERTEX_SHADER, __VERTEX__));
        gl.attachShader(program, shader(gl.FRAGMENT_SHADER, __FRAGMENT__));
        gl.linkProgram(program);
        if (!gl.getProgramParameter(program, gl.LINK_STATUS))
            throw Error(gl.getProgramInfoLog(program));
        gl.useProgram(program);
        const texture = gl.createTexture();
        gl.bindTexture(gl.TEXTURE_2D, texture);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
        gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
        const at = (name) => gl.getUniformLocation(program, name);
        gl.uniform1i(at('depthTexture'), 0);
        gl.uniform2i(at('eyeOrigin'), 0, 0);
        gl.uniform2i(at('eyeSize'), 1, 1);
        gl.uniform2i(at('destinationOrigin'), 0, 0);
        gl.uniform1i(at('halfWidth'), 1);
        gl.uniform1i(at('tileHeight'), 1);
        return [0, 0.5, 1].map(depth => {
            gl.texImage2D(gl.TEXTURE_2D, 0, gl.R32F, 1, 1, 0, gl.RED, gl.FLOAT, new Float32Array([depth]));
            gl.drawArrays(gl.TRIANGLES, 0, 3);
            const bytes = new Uint8Array(4);
            gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, bytes);
            return Array.from(bytes);
        });
    } finally {
        gl.getExtension('WEBGL_lose_context')?.loseContext();
    }
})()"""

PACK_EXPECTED = [[0, 0, 0, 255], [0, 0, 128, 255], [255, 255, 255, 255]]


def pack_shaders(library: Path) -> tuple[str, str]:
    """The full-screen vertex shader and the depth pack fragment shader of a WebGL pack library
    (webgl-atlas-pack@1), with its `${blockFunctions}` expanded."""
    source = library.read_text(encoding="utf-8")
    vertex = re.search(r"const fullScreenVertex = `([^`]+)`;", source)
    fragment = re.search(r"const packFragment = `([^`]+)`;", source)
    block = re.search(r"const blockFunctions = `([^`]+)`;", source)
    if vertex is None or fragment is None:
        raise VerifyError(f"{library.name} defines no fullScreenVertex / packFragment shader")
    text = fragment.group(1)
    if "${blockFunctions}" in text:
        if block is None:
            raise VerifyError(f"{library.name} uses blockFunctions but does not define it")
        text = text.replace("${blockFunctions}", block.group(1))
    return vertex.group(1), text


def pack_readback(mcp: Mcp, instance: str, library: Path) -> list:
    """Near, mid and far depth through the library's pack shader in the page's WebGL2."""
    vertex, fragment = pack_shaders(library)
    expression = _PACK_READBACK.replace("__VERTEX__", json.dumps(vertex)).replace("__FRAGMENT__", json.dumps(fragment))
    actual = mcp.evaluate(instance, expression)
    if actual != PACK_EXPECTED:
        raise VerifyError(f"near/mid/far depth bytes: expected {PACK_EXPECTED}, got {actual}")
    return actual
