"""`verify`: the PNG reader, the coverage analysis of an atlas, and the command against a stand-in
Visualizer MCP endpoint."""

from __future__ import annotations

import base64
import json
import struct
import threading
import zlib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

from .helpers import KIT, Scratch, adaptation_profile, make_adaptation, make_kit, run_cli

from qaros_adaptation import verify  # noqa: E402

FAR = 0xFFFFFF


def encode_png(width: int, height: int, rows: list[bytes], channels: int = 3) -> bytes:
    """A PNG whose rows use every filter type in turn, so the reader's unfiltering is exercised."""
    colour = {3: 2, 4: 6}[channels]
    raw = bytearray()
    previous = bytes(width * channels)
    for index, row in enumerate(rows):
        kind = index % 5
        out = bytearray()
        for i, value in enumerate(row):
            left = row[i - channels] if i >= channels else 0
            up = previous[i]
            up_left = previous[i - channels] if i >= channels else 0
            if kind == 0:
                predictor = 0
            elif kind == 1:
                predictor = left
            elif kind == 2:
                predictor = up
            elif kind == 3:
                predictor = (left + up) >> 1
            else:
                estimate = left + up - up_left
                distances = (abs(estimate - left), abs(estimate - up), abs(estimate - up_left))
                predictor = left if distances[0] <= distances[1] and distances[0] <= distances[2] else up if distances[1] <= distances[2] else up_left
            out.append((value - predictor) & 0xFF)
        raw += bytes([kind]) + out
        previous = row

    def chunk(kind: bytes, data: bytes) -> bytes:
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)

    header = struct.pack(">IIBBBBB", width, height, 8, colour, 0, 0, 0)
    return b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header) + chunk(b"IDAT", zlib.compress(bytes(raw))) + chunk(b"IEND", b"")


class Atlas:
    """A 64x16 RGB atlas with one user's left eye: colour 16x8 at (0, 0), depth at (16, 0), alpha at
    (40, 0), laid out as the render-frame protocol packs them."""

    WIDTH, HEIGHT = 64, 16
    COLOUR = {"x": 0, "y": 0, "width": 16, "height": 8}
    DEPTH = {"x": 16, "y": 0, "width": 16, "height": 2}
    ALPHA = {"x": 40, "y": 0, "width": 8, "height": 2}

    def __init__(self) -> None:
        self.pixels = [[(0, 0, 0) for _ in range(self.WIDTH)] for _ in range(self.HEIGHT)]
        for row in range(4):
            for column in range(8):
                self.depth(column, row, FAR)

    def colour(self, x: int, y: int, value=(255, 255, 255)) -> None:
        self.pixels[y][x] = value

    def depth(self, column: int, row: int, value: int) -> None:
        half, tile = 8, self.DEPTH["height"]
        x = self.DEPTH["x"] + column + (half if row >= tile else 0)
        y = self.DEPTH["y"] + (row - tile if row >= tile else row)
        self.pixels[y][x] = (value & 255, (value >> 8) & 255, (value >> 16) & 255)

    def alpha(self, column: int, row: int, value: int) -> None:
        x, y = self.ALPHA["x"] + column, self.ALPHA["y"] + row // 3
        pixel = list(self.pixels[y][x])
        pixel[row % 3] = value
        self.pixels[y][x] = tuple(pixel)

    def png(self) -> bytes:
        return encode_png(self.WIDTH, self.HEIGHT, [bytes(channel for pixel in row for channel in pixel) for row in self.pixels])

    @classmethod
    def detail(cls, alpha: bool = False) -> dict:
        views = [
            {"dataType": "color", "eye": "left", "logicalWidth": 16, "logicalHeight": 8, "rect": cls.COLOUR},
            {"dataType": "depth", "eye": "none", "logicalWidth": 16, "logicalHeight": 8, "rect": cls.DEPTH},
        ]
        if alpha:
            views.append({"dataType": "alpha", "eye": "left", "logicalWidth": 16, "logicalHeight": 8, "rect": cls.ALPHA})
        return {"requestId": 1, "atlas": {"width": cls.WIDTH, "height": cls.HEIGHT},
                "targets": [{"id": "u1", "cameras": {"left": {"depthConvention": "regular"}}, "views": views}]}


def thin_line(atlas: Atlas, nearest: bool) -> None:
    """A 1-px vertical line at x = 3 over the whole view: column 1 of the half-resolution samples. A
    nearest-of-block pack gives every block depth; a single-texel pack (the even texel) none."""
    for y in range(8):
        atlas.colour(3, y)
    for row in range(4):
        atlas.depth(1, row, 0x400000 if nearest else FAR)


class PngTests(Scratch):
    def test_decode_every_filter(self) -> None:
        rows = [bytes((x * 7 + y * 13 + channel * 31) % 256 for x in range(9) for channel in range(4)) for y in range(11)]
        image = verify.decode_png(encode_png(9, 11, rows, channels=4))
        self.assertEqual((image.width, image.height, image.channels), (9, 11, 4))
        self.assertEqual(image.pixels, b"".join(rows))
        with self.assertRaises(verify.VerifyError):
            verify.decode_png(b"not a png")


class CoverageTests(Scratch):
    def analyse(self, atlas: Atlas, alpha: bool = False):
        [coverage] = verify.analyse(verify.decode_png(atlas.png()), Atlas.detail(alpha))
        return coverage

    def test_nearest_pack_covers_a_thin_line(self) -> None:
        atlas = Atlas()
        thin_line(atlas, nearest=True)
        coverage = self.analyse(atlas)
        self.assertEqual((coverage.blocks, coverage.thin_blocks), (4, 4))
        self.assertEqual((coverage.depth, coverage.thin), (1.0, 1.0))

    def test_single_texel_pack_misses_a_thin_line(self) -> None:
        atlas = Atlas()
        thin_line(atlas, nearest=False)
        coverage = self.analyse(atlas)
        self.assertEqual(coverage.depth_missing, 4)
        self.assertEqual(coverage.thin, 0.0)

    def test_a_solid_area_is_not_thin(self) -> None:
        atlas = Atlas()
        for y in range(8):
            for x in range(8):
                atlas.colour(x, y)
        for row in range(4):
            for column in range(4):
                atlas.depth(column, row, 0x100000)
        coverage = self.analyse(atlas)
        self.assertEqual(coverage.blocks, 16)
        self.assertEqual(coverage.thin_blocks, 4, "only the corners of a 4x4 area have 3 neighbours; edges have 5")
        self.assertEqual(coverage.depth, 1.0)

    def test_alpha_coverage(self) -> None:
        atlas = Atlas()
        thin_line(atlas, nearest=True)
        for row in range(4):
            atlas.alpha(1, row, 255 if row != 2 else 0)
        coverage = self.analyse(atlas, alpha=True)
        self.assertEqual((coverage.alpha_blocks, coverage.alpha_missing), (4, 1))

    def test_inverted_depth_clears_to_zero(self) -> None:
        atlas = Atlas()
        thin_line(atlas, nearest=False)  # the line's samples hold the regular far value
        detail = Atlas.detail()
        detail["targets"][0]["cameras"]["left"]["depthConvention"] = "inverted"
        [coverage] = verify.analyse(verify.decode_png(atlas.png()), detail)
        self.assertEqual(coverage.depth, 1.0, "with inverted depth 0xFFFFFF is the nearest, not empty")


class ShaderTests(Scratch):
    def test_pack_shaders_expand_the_block_functions(self) -> None:
        library = KIT / "lib" / "webgl-atlas-pack@1.js"
        vertex, fragment = verify.pack_shaders(library)
        self.assertIn("#version 300 es", vertex)
        self.assertIn("float blockDepth(", fragment)
        self.assertNotIn("${", fragment)


class FakeVisualizer:
    """A stand-in for the Visualizer's MCP endpoint: one source, one request, one atlas."""

    def __init__(self, atlas: Atlas, readback=verify.PACK_EXPECTED, empty_polls: int = 0, timeouts: int = 0) -> None:
        self.atlas, self.readback, self.calls = atlas, readback, []
        # The first `empty_polls` layout reads find no request yet (None: never one); the first
        # `timeouts` DevTools calls time out the way the Visualizer's gateway reports it.
        self.empty_polls, self.timeouts = empty_polls, timeouts
        visualizer = self

        class Handler(BaseHTTPRequestHandler):
            def do_POST(self) -> None:  # noqa: N802
                request = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
                name, arguments = request["params"]["name"], request["params"]["arguments"]
                visualizer.calls.append(name)
                result = visualizer.answer(name, arguments)
                body = json.dumps({"jsonrpc": "2.0", "id": request["id"], "result": result}).encode()
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)

            def log_message(self, *args) -> None:
                pass

        self.server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.port = self.server.server_address[1]
        threading.Thread(target=self.server.serve_forever, daemon=True).start()

    def answer(self, name: str, arguments: dict) -> dict:
        if name == "viz_webapp_list":
            return {"structuredContent": {"sources": [{"instance_id": "web-1", "address": "https://example.com/app.html"}]}}
        if name == "devtools_send":
            assert not arguments["params"].get("awaitPromise"), "verify never waits in the page"
            if self.timeouts:
                self.timeouts -= 1
                return {"isError": True, "content": [{"type": "text", "text": "Code: 6 Desc: Timeout Msg: DevTools call timed out"}]}
            expression = arguments["params"]["expression"]
            if "__qarVerifyRequest" in expression:
                if self.empty_polls is None or self.empty_polls > 0:
                    self.empty_polls = None if self.empty_polls is None else self.empty_polls - 1
                    value = None
                else:
                    value = Atlas.detail()
            else:
                value = self.readback
            return {"structuredContent": {"success": True, "payload": {"result": {"value": value}}}}
        if name == "capture_surface":
            return {"content": [{"type": "image", "mimeType": "image/png", "data": base64.b64encode(self.atlas.png()).decode()}]}
        return {"isError": True, "content": [{"type": "text", "text": f"unknown tool {name}"}]}

    def close(self) -> None:
        self.server.shutdown()
        self.server.server_close()


class VerifyCommandTests(Scratch):
    def run_verify(self, nearest: bool, *extra: str, empty_polls=0, timeouts=0):
        atlas = Atlas()
        thin_line(atlas, nearest)
        visualizer = FakeVisualizer(atlas, empty_polls=empty_polls, timeouts=timeouts)
        self.addCleanup(visualizer.close)
        kit = make_kit(self.dir)
        folder = make_adaptation(self.dir, "a", adaptation_profile())
        return run_cli("verify", folder, "--kit", kit, "--port", visualizer.port, *extra), visualizer

    def test_layout_polled_until_a_request_arrives(self) -> None:
        completed, visualizer = self.run_verify(True, empty_polls=2)
        self.assertRuns(completed)
        self.assertEqual(visualizer.calls.count("devtools_send"), 3)
        self.assertIn("depth 100.00%", completed.stdout)

    def test_devtools_timeout_is_retried(self) -> None:
        completed, visualizer = self.run_verify(True, timeouts=1)
        self.assertRuns(completed)
        self.assertEqual(visualizer.calls.count("devtools_send"), 2)

    def test_no_request_is_reported_as_not_checked(self) -> None:
        completed, visualizer = self.run_verify(True, "--layout-wait", "1", empty_polls=None)
        self.assertRuns(completed, 2)
        self.assertIn("coverage not checked: no render-frame request reached the page within 1 s", completed.stdout)
        self.assertEqual(completed.stderr, "")
        self.assertNotIn("capture_surface", visualizer.calls)

    def test_good_atlas_passes(self) -> None:
        completed, visualizer = self.run_verify(nearest=True)
        self.assertRuns(completed)
        self.assertIn("target u1 left: 4 coloured blocks, depth 100.00%, thin 100.00% of 4", completed.stdout)
        self.assertEqual(visualizer.calls, ["viz_webapp_list", "devtools_send", "capture_surface"])

    def test_single_texel_atlas_fails(self) -> None:
        completed, _ = self.run_verify(nearest=False)
        self.assertRuns(completed, 1)
        self.assertIn("thin-line depth coverage 0.00%", completed.stderr)

    def test_pack_shader_readback(self) -> None:
        """A profile with the WebGL pack shaders (here a pinned webgl-atlas-pack@1) gets the readback."""
        profile = adaptation_profile()
        profile["Scripts"].append({"Name": "atlas-pack", "Stage": "afterLoad", "File": "scripts/webgl-atlas-pack@1.js"})
        pack = (KIT / "lib" / "webgl-atlas-pack@1.js").read_text(encoding="utf-8")
        atlas = Atlas()
        thin_line(atlas, True)
        visualizer = FakeVisualizer(atlas, readback=[[0, 0, 0, 255], [0, 0, 0, 255], [0, 0, 0, 255]])
        self.addCleanup(visualizer.close)
        kit = make_kit(self.dir)
        folder = make_adaptation(self.dir, "b", profile, {"scripts/site.js": "x", "scripts/webgl-atlas-pack@1.js": pack})
        completed = run_cli("verify", folder, "--kit", kit, "--port", visualizer.port)
        self.assertRuns(completed, 1)
        self.assertIn("depth pack readback: near/mid/far depth bytes", completed.stderr)
        self.assertRuns(run_cli("verify", folder, "--kit", kit, "--port", visualizer.port, "--no-readback"))
        visualizer.readback = verify.PACK_EXPECTED
        completed = run_cli("verify", folder, "--kit", kit, "--port", visualizer.port)
        self.assertRuns(completed)
        self.assertIn("depth pack readback OK", completed.stdout)

    def test_unreachable_visualizer(self) -> None:
        kit = make_kit(self.dir)
        folder = make_adaptation(self.dir, "c")
        completed = run_cli("verify", folder, "--kit", kit, "--port", "1", "--instance", "x")
        self.assertRuns(completed, 1)
        self.assertIn("cannot reach the Visualizer MCP endpoint", completed.stderr)
