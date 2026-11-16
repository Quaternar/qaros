# QAROS adaptation kit

Everything to adapt a website's 3D canvas (three.js, raw WebGL, WebGPU) so QAROS streams it to AR
and VR users: the shared adaptation libraries, starter templates, reference examples and a command
line tool. It ships with QAROS in `<install>/share/qaros/adaptation-kit/` and is versioned with it.

Guide: https://docs.quaternar.com/docs/developer-guide/web-app-adaptations/

## Layout

| Folder | What |
|---|---|
| `lib/` | the libraries, one file per `name@major` (`threejs-render-frame@1.js`), each with a header naming its version, stage and needs; `lib/tests/` their Node tests. An installed kit also has `lib/index.json` (names, versions, stages, SHA-256), which QAROS resolves `Library` references with |
| `templates/` | starter adaptations: `threejs`, `webgl`, `webgpu`, `webgpu-redirect`, each with a `profile.json`, its own scripts, a README and a test |
| `examples/` | the reference apps (three.js, WebGL, WebGPU) and the three.js reference panels |
| `tools/` | `qaros-adaptation.py` (the CLI) and `verify-depth.py` |

## Libraries by name

A profile script names a kit library instead of bundling a copy:

```json
{ "Name": "render-frame-core", "Stage": "afterLoad", "Library": "render-frame-core@1" }
```

- QAROS injects the installed kit's file at load time, so a library fix reaches every adaptation
  that uses it. A minor version (a fix, a new option) keeps the major; a breaking change is a new
  major, and the old one keeps shipping.
- `Library` and `File` are mutually exclusive in one script entry. The earlier `"Library":
  "name.js"` means `name@1`.
- An adaptation that needs a major this QAROS does not have fails to load with "requires
  `name@2`, this QAROS provides @1: update QAROS".
- `pin` bundles a copy (`File`) to freeze an exact version; its header shows which.

## The CLI

Python 3, standard library only. From an install: `python -I <kit>/tools/qaros-adaptation.py`; from
the repository: `python -I qaros/adaptation-kit/tools/qaros-adaptation.py`. `--kit <dir>` (before or
after the command) picks another kit.

| Command | Does |
|---|---|
| `init <template> <dir> [--name --address --engine --display-name --id]` | a new adaptation from a template; asks for missing values in a terminal |
| `check <dir> [--ignore CODE] [--strict]` | `profile.json` schema, every `Library` in the kit, `@needs` order, script files; warns on known pitfalls: single-texel depth pack, mean alpha pack, iframe pages, page-region tokens left in own scripts |
| `test <dir>` | the adaptation's `test/*.test.mjs` and the kit tests of the libraries it uses (`node --test`) |
| `package <dir> [--out --unpacked --version --sign key.pem --key-vault --key-name --public-key --signer]` | the `.qarapp` (or its unpacked folder): file manifest, optional signature, verified as QAROS imports it |
| `pin <dir> <library>` | replaces a `Library` reference with a copy at `scripts/<name>@<major>.js` |
| `verify <dir> [--instance --address --host --port]` | against a running web app source (Visualizer MCP endpoint, Developer mode): depth coverage of the coloured pixels, thin-line coverage, alpha coverage, depth pack readback |
| `index <kit> [--version --out --check]` | writes or checks `lib/index.json` |

A typical loop:

```
python -I tools/qaros-adaptation.py init threejs my-viewer --address "https://example.com/viewer/*"
python -I tools/qaros-adaptation.py check my-viewer
python -I tools/qaros-adaptation.py test my-viewer
python -I tools/qaros-adaptation.py package my-viewer --out dist
```

The tool's own tests: `python -I -m unittest discover -s tools/tests -t tools`.
