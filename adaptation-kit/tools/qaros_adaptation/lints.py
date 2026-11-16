"""The known-pitfall lints of `check`. All are warnings, heuristics over the adaptation's own
scripts (a `File`, a pinned copy, or inline `JavaScript`); kit libraries referenced by `Library` are
tested by the kit itself. Each has a code that `check --ignore <code>` skips.

| Code | Finds | Heuristic |
|---|---|---|
| `depth-single-texel` | a depth packer that keeps one texel of each 2x2 block | the script (comments removed) writes 24-bit depth (`16777215`) and has no reduction over the block: no accumulator `d = min(d, ...)` / `max`, no nested `min(min(`, no `blockDepth` |
| `alpha-mean` | an alpha packer that averages its 2x2 block | the script packs alpha (`alphaEncoding`, an `"alpha"` view, `unorm8-rgb-three`, and samples a texture) and either scales an alpha sum (`* 0.25`, `/ 4.0`) or has no max reduction (`a = max(a, ...)`, `max(max(`, `blockAlpha`) |
| `iframe` | an address pattern that matches pages which frame their content | the pattern names an index or gallery page (`index`, `gallery`, `examples`, `samples`, `demos`, or ends in `/`) and no own script handles frames (`iframe`, `window.top`, `frameElement`, `contentWindow`, `frames`) |
| `page-region-token` | `__PAGE_REGION_WIDTH__` / `__PAGE_REGION_HEIGHT__` left in an own script | `package` replaces them from `PageRegion`; a folder loaded without packaging keeps them, which is a syntax error or a wrong size. Pinned kit copies (with a `@qaros-library` header) are exempt: they are written for that substitution. |

Why each matters: a single texel of a 2x2 block misses a 1-px line or thin edge in about half its
blocks, so that colour reaches the mixer with no depth and is dropped (dashed lines). A mean dims a
1-px line's alpha to a quarter or a half. A gallery page that shows the 3D content in an iframe has
no renderer at top level, so the source never starts.
"""

from __future__ import annotations

import re
from pathlib import Path

from .profile import PAGE_REGION_TOKENS, Report, ScriptInfo

_DEPTH_PACK = re.compile(r"16777215")
# A reduction over the block: an accumulator (`d = min(d, ...)`), nested min/max, or the kit's helper.
_DEPTH_REDUCTION = re.compile(r"\b(\w+)\s*=\s*(?:min|max)\s*\(\s*\1\b|\b(?:min|max)\s*\(\s*(?:min|max)\s*\(|\bblockDepth\b")
_ALPHA_PACK = re.compile(r"alphaEncoding|unorm8-rgb-three|dataType\s*===?\s*[\"']alpha[\"']|alphaPack")
_ALPHA_MEAN = re.compile(r"(?:\.a\b|[Aa]lpha|coverage)[^;\n]*(?:\*\s*0?\.25\b|0?\.25\s*\*|/\s*4(?:\.0*)?\b)")
_ALPHA_REDUCTION = re.compile(r"\b(\w+)\s*=\s*max\s*\(\s*\1\b|\bmax\s*\(\s*max\s*\(|\bblockAlpha\b")
_ALPHA_SAMPLE = re.compile(r"\b(?:texelFetch|textureLoad|texture2D|texture)\s*\(")
_COMMENT = re.compile(r"/\*.*?\*/|(?<![:\w\\])//[^\n]*", re.S)
_FRAME_HANDLING = re.compile(r"\biframe\b|window\.top\b|\bframeElement\b|\bcontentWindow\b|window\.frames\b|\bwindow\s*!==?\s*top\b")
_INDEX_PAGE = re.compile(r"(?:/|index[^/]*|gallery[^/]*|examples?[^/]*|samples?[^/]*|demos?[^/]*)[*?]*$", re.I)
_INDEX_WORD = re.compile(r"index|gallery|examples|samples|demos", re.I)


def _label(info: ScriptInfo) -> str:
    return f"'{info.file}'" if info.file else f"inline script '{info.name}'"


def _code(text: str) -> str:
    """The script without comments, so a comment ("nearest", "iframe") never satisfies a lint."""
    return _COMMENT.sub(" ", text)


def _own_scripts(infos: list[ScriptInfo]) -> list[ScriptInfo]:
    return [info for info in infos if info.kind in ("file", "inline") and info.text]


def lint_depth(info: ScriptInfo, report: Report) -> None:
    code = _code(info.text)
    if _DEPTH_PACK.search(code) and not _DEPTH_REDUCTION.search(code):
        report.warning(
            "depth-single-texel",
            f"{_label(info)} packs 24-bit depth but keeps no nearest depth of each 2x2 block; one texel per block "
            "misses thin lines and edges (they stream with no depth and are dropped). Take the min (max for "
            "reversed Z) of the block's four depths, as the kit's render-frame libraries do",
        )


def lint_alpha(info: ScriptInfo, report: Report) -> None:
    code = _code(info.text)
    if not (_ALPHA_PACK.search(code) and _ALPHA_SAMPLE.search(code)):
        return
    if _ALPHA_MEAN.search(code) or not _ALPHA_REDUCTION.search(code):
        report.warning(
            "alpha-mean",
            f"{_label(info)} packs alpha without the largest coverage of each 2x2 block; a mean dims a 1-px "
            "line to a quarter or a half. Take the max of the block's four alphas",
        )


def lint_iframe(profile: dict, infos: list[ScriptInfo], report: Report) -> None:
    pattern = profile.get("AddressPattern")
    if not isinstance(pattern, str) or "://" not in pattern:
        return
    path = pattern.split("://", 1)[1]
    path = path[path.find("/"):] if "/" in path else "/"
    if not (_INDEX_PAGE.search(path) or _INDEX_WORD.search(path)):
        return
    if any(_FRAME_HANDLING.search(_code(info.text)) for info in _own_scripts(infos)):
        return
    report.warning(
        "iframe",
        f"AddressPattern '{pattern}' matches index or gallery pages; if one shows its 3D content in an iframe, "
        "the top-level page has no renderer and the source never starts. Open the framed page at top level "
        "from a script (see the iframes recipe), or narrow the pattern",
    )


def lint_page_region_tokens(info: ScriptInfo, report: Report) -> None:
    if info.kind != "file" or info.provides:
        return
    tokens = [token for token in PAGE_REGION_TOKENS if token in info.text]
    if tokens:
        report.warning(
            "page-region-token",
            f"{_label(info)} holds {' and '.join(tokens)}: `package` replaces them from PageRegion, but the folder "
            "loaded as it is keeps them. Read the region from the page instead (window.__qarAdaptationPageRegion "
            "or the render-frame request's pageRegion)",
        )


def run(folder: Path, profile: dict, infos: list[ScriptInfo], report: Report) -> None:
    del folder  # every lint works on the scripts already read
    for info in _own_scripts(infos):
        lint_depth(info, report)
        lint_alpha(info, report)
        lint_page_region_tokens(info, report)
    lint_iframe(profile, infos, report)
