# QAROS C API Documentation (qaros repo)

Status: In progress (first full pass implemented 2026-07-05)

Path conventions in this document: paths starting with `qaros/` (or plain
`docs/`, `qar-streaming-c/` where obvious) are inside THIS repo. Paths starting
with `qar-streaming/`, `qar-runtime-launcher/`, `qar-streaming-viz/`,
`qar-common-lib/`, `docs/website/` (Quaternar-internal site) or `.claude/`
refer to the private Quaternar monorepo, which consumes this repo as the
`qaros` submodule at its root. Tasks touching those paths must be executed
from a Quaternar monorepo checkout.

## Goal

Comprehensive, user-facing documentation for the qar-streaming-c API, published
from the public distribution repo `qaros` (github.com/Quaternar/qaros, deployed
to https://docs.quaternar.com/qaros/ via the repo's GitHub Pages workflow).
Audiences: (a) operators / project managers of integrating software (no code),
(b) engineers integrating the C API. Every code sample must come from compiled
example programs so samples can never silently go stale.

## Architecture decisions

1. **qaros is a git submodule of Quaternar** at `<repo-root>/qaros`. DONE.
2. **Unified header mirror on every build.** The existing single-header
   generation (`qar-streaming/libs/qar-streaming-c/CMakeLists.txt`, custom
   command around `cmake/generate_single_header.py`) was extended: when
   `${QAR_QAROS_DIR}` (cache var, default `${CMAKE_SOURCE_DIR}/qaros`) contains
   `qar-streaming-c/include`, the generated `qar_streaming.h` is
   `copy_if_different`-ed there as part of the same custom command. So any
   build of the `qar-streaming-c_v0_single_header` target (a dependency of the
   main lib) refreshes the published header. DONE.
3. **Docs live in `qaros/docs/website`** (Docusaurus v3, already deployed by
   `.github/workflows/docs-website.yml`). Hand-written pages in `docs/`,
   generated API reference via doxygen + doxygen2docusaurus (`npm run build`
   regenerates). The Quaternar-internal `docs/website` is a separate site and
   was not touched.
4. **Two-audience split**: `docs/user-guide/` (position 2) and
   `docs/developer-guide/` (position 3), each with `_category_.json` and a
   generated index. `docs/intro.md` is the landing page with a doc map.
5. **Compiled tutorials**: examples in `qaros/qar-streaming-c/examples/*.c`
   carry Doxygen `\page` + `\snippet` (`//! [region]` markers). The Doxyfile
   already has the examples dir on INPUT + EXAMPLE_PATH, so tutorials appear
   under `/api/qar-streaming-c/pages/...`. Developer-guide pages link to those
   pages for the authoritative, compiling versions of each flow.
6. **Multi-language code tabs**: developer-guide pages that show code are
   `.mdx` and use `@theme/Tabs` with `groupId="language"` (C now, C# tab is a
   TODO placeholder until qar-streaming-sharp bindings are published in qaros).
7. **Diagrams**: hand-authored SVGs in
   `qaros/docs/website/static/img/diagrams/` (neutral light-card palette that
   reads in both themes). No mermaid dependency added yet.
8. **TODO convention**: any section whose facts could not be verified in the
   codebase gets an explicit `:::note TODO` admonition describing exactly what
   belongs there — never invented content.

## Page inventory and content plan

### Landing
- `intro.md` — what QarOS is, audience map, one overview diagram, start table. DONE.

### User Guide (`docs/user-guide/`)
- `qaros-world.md` (1) — entity model: Hub, devices/peers, sessions, app
  volumes, GUI panels, streams, mixer, identity; lifecycle table. DONE.
- `coordinate-systems.md` (2) — WORLD(ECEF, via `QarGeoAnchorFrame`) → ROOM
  (meters, OpenXR right-handed Y-up −Z-forward — inherited convention, marked
  as such) → APP VOLUME (pose = cuboid **center** + size) → APP CONTENT
  (`app_pose` in unscaled volume meters, `app_scale` = room-m per app-m);
  panels = **top-left corner** pose directly in room space; gesture spaces;
  cheat-sheet table. TODO in page: co-location/room-origin workflow per device;
  normative handedness statement in headers. DONE (with TODOs).
- `security-and-onboarding.md` (3) — 4-layer model (L0 discovery 7445 / L1
  EC-JPAKE 8-char 10s single-use codes / L2 AES-256-GCM + CSR enrollment via
  CA gateway / L3 mTLS 7447), per-hub step-ca CA, renewal, revocation
  (blacklist), hub federation = merged trust bundles, honest limitations list.
  TODO in page: separate auditor-level threat-model page. DONE.
- `using-the-hub.md` (4) — three processes (qaros-hub-service tray /
  qar-runtime-launcher / qar-streaming-viz), startup chain, tray menu,
  launcher config groups (sourceApps[], targetApps[], networking, flags).
  TODOs in page: visualizer UI walkthrough with screenshots (hub-side "show my
  pairing code" screen needs verification — only HubConnectSplashView (enter
  remote code) was found in viz); full annotated config-schema reference
  generated from LauncherConfig.cpp. DONE (with TODOs).
- `onboarding-devices.md` (5) — push (targetApps auto-invite, today's path) vs
  pull (code entry via C API); per-device: HoloLens (remoting player +
  `hololens:<host>`), Quest (`quest:`), Android (`android:<host>`; NOTE: no
  `qar-android-player` exists — it is qar-openxr-app + qar-android-streamer),
  visualizer, ZED; forget/blacklist. TODOs in page: on-device code-entry/QR UI
  for Quest/Android (planned, not shipped), ZED end-to-end setup. DONE.
- `the-mixer.md` (6) — per-user compositor: warp to latest pose → mask to
  volume OBB → depth-sort + blend; one mixer per user; **8 = max source
  volumes per view** (not max users); frame pacing behavior; depth/alpha
  semantics; runs on hub via pipeline planner. DONE.
- `deployment-and-installation.md` (7) — artifact matrix (ZIP recommended
  today / NuGet / MSIX / APK), ZIP install steps, firewall 7440–7460 +
  multicast 239.77.77.77:7445 + mTLS 7447, data under %LOCALAPPDATA%,
  package/ drop-in layout for the SDK repo. TODOs in page: MSIX promotion
  blocked on config-seeding-to-LocalState + startup-task toggle; formal
  compatibility policy; system requirements. DONE (with TODOs).
- `glossary.md` (8). DONE.

### Developer Guide (`docs/developer-guide/`)
- `getting-started.md` (1) — SDK contents, single header (auto-generated,
  never edit), static vs dynamic loading table + skeleton, canonical program
  shape, building examples, bindings overview. DONE.
- `api-conventions.md` (2) — the six patterns: opaque handles vs value IDs
  (+leave/destroy distinction), QarResult by value + expected-control-flow
  statuses, `QarStructureHeader{type,next}` + `qar_*_default()` + extension
  chaining, blocking/async pairs + exactly-once result callbacks + cancel
  tokens + progress callback, count-then-fetch, ownership/threading rules;
  device kinds appear only in invite connection strings. DONE.
- `onboarding-and-sessions.mdx` (3) — canonical rejoin→onboard flow (C tab
  verified against header; C# tab TODO), sibling invites
  (serialize/deserialize), inviting devices, leave vs forget table. Links to
  compiled tutorial. DONE.
- `peers.md` (4) — presentation, enumeration, app_state/room_tag, subscribe. DONE.
- `gui-panels.mdx` (5) — idempotent get_or_create by common_name, mutations,
  per-peer visibility, subscriptions, integration patterns (web UI in a panel,
  panel-driven app state, docking to a volume). DONE.
- `app-volumes.mdx` (6) — two-transform model, latest-getters, world anchors,
  editing lock / used-by, full gesture system (rules, precision, sensitivity
  modes, event consumption). TODOs in page: worked room→app hit-testing
  example; verify QarAppVolumeGestureEvent field names when writing it. DONE.
- `rendering-streams.mdx` (7) — sender init (verified struct), frame loop
  contract (render with given pose/FOV, report true near/far), layouts, D3D11
  extension + keyed mutex, hand read-back, auto-reconnect. DONE.
- `language-bindings.md` (8) — C++ guidance; C#/Unity section is one big TODO
  spec (binding mapping rules, Unity lifecycle, compiled C# examples). DONE.
- `troubleshooting.md` (9) — status-code table, frequent mistakes, where logs
  live. TODO in page: grow from support cases. DONE.

### Diagrams (static/img/diagrams/)
- `qaros-world-overview.svg` — hub internals + devices + arrows. DONE.
- `coordinate-spaces.svg` — nested WORLD/ROOM/VOLUME/CONTENT + panel. DONE.
- `security-layers.svg` — L0→L3 stack. DONE.
- `mixer-flow.svg` — volumes → warp/mask/blend → per-user frame. DONE.

## Examples work (compiled tutorial sources)

The old examples used the removed `qar_runtime_create_session`/
`qar_session_join` API and stopped compiling against the mirrored new header.
Reworked (all now pass `clang -fsyntax-only -std=c11` against the new
`qar_streaming.h`):

- `common.h` — added `//! [example_onboarding_persistence]` (uuid-text
  load/save of `QarOnboardingId`) and `//! [example_obtain_session]`
  (rejoin → SESSION_NOT_FOUND → onboard_with_code → persist id) shared helper.
- `session_create_and_join.c` → **renamed** `onboarding_and_rejoin.c`; new
  `\page qar_c_tutorial_onboarding` covering args, persistence, rejoin/onboard,
  sibling-invite mint+serialize, leave/forget (`--forget` flag).
- `app_volume_management.c`, `gui_panel_operations.c`,
  `cpu_rendering_visualizer.c` — ported to the helper; `add_*` →
  `get_or_create` with required `common_name`; `qar_session_destroy` →
  `qar_session_leave` + `qar_session_handle_destroy`; progress-callback
  signature fixed; optional `[pairing-code]` argv added.
- `examples/CMakeLists.txt` target list updated.

Bug found & fixed in the main repo while verifying: `v0/detail/default_inits.h`
included C++ `<cstddef>` → the generated single header did not compile as C.
Changed to `<stddef.h>`.

## Task board

Statuses: DONE · IN-PROGRESS · TODO · BLOCKED(reason). Each task lists enough
context for an independent agent to execute it without re-deriving the plan.
Ground rules for all tasks: verify every API name against the *generated*
`qaros/qar-streaming-c/include/qar_streaming.h` before writing it into docs or
examples; mark anything unverifiable with a `:::note TODO` admonition instead
of guessing; clang-format every touched C/C++ file with
`./build/tools/llvm/bin/clang-format.exe -i <file>`; never edit generated files
(`qar_streaming.h`, `docs/website/api/**`).

### T1 — Infrastructure: submodule + header mirror — DONE
- `qaros` submodule added at repo root (`.gitmodules`).
- `qar-streaming/libs/qar-streaming-c/CMakeLists.txt`: `QAR_QAROS_DIR` cache
  var + `copy_if_different` mirror appended to the single-header custom
  command. Verified: building target `qar-streaming-c_v0_single_header`
  regenerates and mirrors the header.
- Bug fixed en route: `include/qar_streaming_c/v0/detail/default_inits.h`
  included C++ `<cstddef>`; now `<stddef.h>` (header was un-compilable as C).

### T2 — User Guide pages — DONE (8 pages)
All under `qaros/docs/website/docs/user-guide/` — see page inventory above.
In-page TODO admonitions intentionally remain (they are tasks T7–T12 below).

### T3 — Developer Guide pages — DONE (9 pages)
All under `qaros/docs/website/docs/developer-guide/`. Code-bearing pages are
`.mdx` with `<Tabs groupId="language">`; C tabs verified against the header;
C# tabs are placeholders (task T8).

### T4 — Diagrams — DONE (4 SVGs)
`qaros/docs/website/static/img/diagrams/{qaros-world-overview,
coordinate-spaces,security-layers,mixer-flow}.svg`. Hand-authored, no external
assets. If regenerating: keep light card background (#f6f8fb), 960px viewBox.

### T5 — Example ports to onboarding API — DONE
See "Examples work" above. All 5 examples pass
`clang -fsyntax-only -std=c11 -Iinclude -Iexamples -DQAR_ENABLE_DYNAMIC_LOADING
--target=x86_64-pc-windows-msvc` from `qaros/qar-streaming-c/`.
Note: they are syntax-verified, NOT yet link/run-verified against a real
runtime DLL + hub (needs a binary package in `qaros/package/` — task T14).

### T6 — Site build verification — DONE (verified 2026-07-05)
`cd qaros/docs/website && npm install && npm run build` exits 0 **with no
PATH setup and no dependency on the Quaternar monorepo checkout** — verified
from a clean `node_modules`/`doxygen/.tools`/`build` state with only stock
Node.js + the OS's `tar` on PATH (no `build/tools/doxygen`, no system
Doxygen). Verified `build/` contains all 8 user-guide pages, all 9
developer-guide pages, all 4 diagrams under `build/img/diagrams/`, and all 5
tutorial pages under `build/api/qar-streaming-c/pages/` including
`qar-c-tutorial-onboarding` (from `onboarding_and_rejoin.c`).
`onBrokenLinks: "throw"` is the link check and it passed clean — no link
fixes were needed this pass.
- `npm ci` may fail on lockfile mismatch — use `npm install`.
- **Doxygen is now self-provisioned inside the repo, not read from PATH.**
  The original setup ran a bare `doxygen` from `PATH` and downloaded a
  fallback via the npm `doxygen` wrapper package — that fallback's download
  landed in npm's ephemeral `npx` cache (never wired onto `PATH`, so it
  didn't help), was pinned to 1.9.1 (predates Doxygen emitting
  `Doxyfile.xml` in XML output, added in 1.9.3, required by
  `doxygen2docusaurus`), and the wrapper package itself (sourceforge-mirror
  redirects, bundled `dmg`/`ftp` deps) is known to be unreliable/hang. Also,
  the very first fix attempt here leaned on
  `<Quaternar-repo-root>/build/tools/doxygen` — wrong, since `qaros` must
  build standalone outside the monorepo. Replaced with
  `qaros/docs/website/scripts/{doxygen-version,doxygen-release,
  doxygen-paths,download-doxygen,run-doxygen}.js`: `postinstall` downloads a
  pinned, SHA256-verified Doxygen 1.9.8 archive directly from doxygen.nl
  (same source and version as `qar-streaming/cmake/HostToolsWindows.cmake`'s
  bundled Doxygen) into `doxygen/.tools/1.9.8/`, extracted via the system
  `tar` (present on Windows 10+/Linux/macOS; note GNU tar on PATH — e.g. Git
  Bash — cannot extract `.zip`, only bsdtar/libarchive can, so this must run
  under a shell where a zip-capable `tar` resolves first, which is the case
  for PowerShell/cmd.exe on Windows), and `docs:xml:qar-streaming` runs that
  exact binary by explicit path instead of a bare `doxygen`. Override with
  `QAROS_DOXYGEN_BIN` env var if needed. CI's separate `apt-get install
  doxygen` step is now redundant but harmless. Documented in
  `qaros/docs/website/README.md` under "Prerequisites".
- **Live regen while editing.** Added `npm run dev` (runs `docusaurus start`
  + a `nodemon` watcher side by side via `concurrently`) and `npm run
  docs:watch` (the watcher alone), driven by `nodemon-doxygen.json`, which
  watches `qar-streaming-c/include`, `qar-streaming-c/examples`, the
  Doxyfile, and `doxygen2docusaurus.json` and re-runs `docs:generate` on
  change; Docusaurus's own dev server then hot-reloads the regenerated
  `api/` markdown. Verified end-to-end: editing `qar_streaming.h` while
  `docs:watch` was running triggered a full doxygen + doxygen2docusaurus
  regen. Plain `npm run start`/`npm run build` are unchanged (generate
  once, no watching).

### T7 — Visualizer/Hub UI walkthrough with screenshots — TODO
File: `user-guide/using-the-hub.md` (TODO admonition marks the spot).
- First VERIFY: does any qar-streaming-viz view display the *local* hub's
  current pairing code/QR? Research found only `HubConnectSplashView`
  (qar-streaming-viz/ui/views/HubConnectSplashView.hpp — form for entering a
  *remote* hub's host+code). The backend mints/publishes codes every 8 s
  (LauncherOnboarding.hpp:123-165, OnboardingPakeServer.hpp:169-208) and logs
  them, but a "show my code" screen was not found. If it doesn't exist, this
  is a product gap — file it, and document the log-based workaround honestly.
- Then: screenshots of tray menu, visualizer window, app-volumes panel,
  warping timing view (`ui/views/WarpingTimingView.hpp`), hub-connect screen.
  Store under `qaros/docs/website/static/img/screenshots/`.

### T8 — C# bindings docs + examples — BLOCKED(bindings not published in qaros)
Files: `developer-guide/language-bindings.md` (spec of what to write is in the
page's TODO), every `C#` tab in `onboarding-and-sessions.mdx`,
`gui-panels.mdx`, `app-volumes.mdx`, `rendering-streams.mdx`.
- Prereq: publish the qar-streaming-sharp v0 onboarding surface into
  `qaros/qar-streaming-sharp/` with a compilable example project (mirror the C
  examples 1:1 so tabs stay parallel).
- Keep `groupId="language"` so tab choice syncs site-wide.
- Consider a second doxygen project (doxygen supports C#) or csharp snippets
  via the same `//! [region]` mechanism if the C# sources are doxygen inputs.

### T9 — Launcher configuration schema reference — TODO
New page `user-guide/launcher-config-reference.md` (or grow the section in
using-the-hub.md). Source of truth: `qar-runtime-launcher/src/LauncherConfig.cpp`
(parsing) + `qar-runtime-launcher/qar-runtime-launcher.jsonc` (annotated
defaults; note the working tree also has a `.json`→`.jsonc` rename in flight).
For every key: type, default, effect, example. Key groups already sketched in
using-the-hub.md: sourceApps[], targetApps[], networking
(discoveryPort/discoveryHost/onboardingPakePort/sessionEndpoints/
onboardingInviteIntervalSec), behavior flags (sendDefaultInvite, enableMixer,
unifyTargetAppWithMixer, applyDeterministicIds, handleInvites,
detectConnectedPeers, replyToServiceExists, handleSourceAppLaunchRequests,
targetAppOverride), paths (caDir, runtimeStorageRoot, logFolder,
startupTargetPath), startupTarget{type, hubConnectionId, trustedHubCaBundles[]}.

### T10 — Advanced security page (auditor level) — TODO
New page `user-guide/security-deep-dive.md` (link from
security-and-onboarding.md's TODO). Port + de-internalize from the main repo's
`docs/Onboarding and Certificates.md` (exhaustive: layer protocols, EC-JPAKE
transcript binding, SecureChannel nonce scheme, CA gateway policy inventory,
URI SANs `qar://peer/<id>` + `qar://session/<id>`, storage layout, renewal,
revocation/blacklist, hub federation via CaTrustStoreService, assumptions
A1–A7, limitations L1–L11, roadmap). Keep the public page free of source paths.

### T11 — More compiled tutorials + link the "Compiled tutorial" TODOs — TODO
- Extend `qaros/qar-streaming-c/examples/app_volume_management.c` with gesture
  configuration (`qar_app_volume_gesture_configuration_default()` then
  customize rules) + world-anchor set/clear + a room→app hit-testing helper
  (needs the volume's latest pose/app_pose/app_scale; verify
  `QarAppVolumeGestureEvent` field names in the generated header first).
- Add a D3D11 render-sender example (guard with `#ifdef QAR_ENABLE_D3D11`,
  chain `QarStreamParamsD3D11`).
- Then replace each developer-guide page's "Compiled tutorial" TODO with links
  to the generated pages: URL pattern
  `/api/qar-streaming-c/pages/qar-c-tutorial-<page-id-with-dashes>/` (doxygen
  `\page qar_c_tutorial_onboarding` → `qar-c-tutorial-onboarding`).
- Sidebar: `qaros/docs/website/sidebar_api.ts` may pin an Examples category —
  check it lists the new tutorial pages.

### T12 — System requirements + compatibility policy — TODO
Fill the two TODOs in `user-guide/deployment-and-installation.md`:
- Hardware/OS/network requirements: needs input from the team (no single
  source in-repo; check demo/packaging feature docs and marketing material).
- Compatibility policy: derive from the X-macro/deprecation rules in
  AGENTS.md ("QAR C API Dynamic Loading Rules") — v0 exports never renamed,
  deprecated entries keep exporting and return deprecation errors, loaded
  structs are app-owned. Write it as a user-facing promise.

### T13 — MSIX promotion — BLOCKED(two items in windows-msix-packaging feature)
When config-seed-to-LocalState and the startup-task tray toggle land, rewrite
the MSIX section of deployment-and-installation.md as the primary install path
and demote ZIP.

### T14 — Run-verify examples against a real runtime — TODO
Syntax passes; do an end-to-end run: drop a binary package into
`qaros/package/`, `cmake --preset x64-windows && cmake --build --preset
x64-windows-debug`, start a hub, run `onboarding_and_rejoin.exe
<dll> <bin-dir> <code>` twice (first onboards, second must rejoin silently),
then with `--forget`. Fix anything the compiler didn't catch (arg order,
runtime behavior). Update the tutorial prose if flows differ.

### T15 — Retire stale internal docs — TODO
`qar-streaming/libs/qar-streaming-c/docs/API_Reference.md` still teaches the
removed `qar_runtime_create_session`/`qar_session_join` flow and wrong
signatures (`qar_result_is_error(&r)` — actual takes QarResult by value).
Either delete it pointing to docs.quaternar.com, or regenerate from the new
docs. `Getting_Started.md` there is current-ish but duplicates the qaros site;
decide one source of truth.

### T16 — qaros README refresh — TODO (small)
`qaros/README.md`: add a "Documentation" map pointing at the new
user-guide/developer-guide split and mention the examples are the compiled
sources for all tutorials; mention the `[pairing-code]` argv the examples now
take.

### T17 — qaros repo hygiene found during work — TODO (small)
- `qaros/.clangd` forces `-std=c++17` and an absent compile_commands.json, so
  the IDE shows false errors in the C examples (missing include path). Add a
  `qaros/qar-streaming-c/examples/.clangd` with
  `CompileFlags: {Add: [-std=c11, -I../include, -I., -DQAR_ENABLE_DYNAMIC_LOADING], Remove: [-std=c++17]}`
  or generate compile_commands via the qaros CMake preset.
- `docusaurus.config.ts` `editUrl` still points at the facebook template —
  point it at `https://github.com/Quaternar/qaros/tree/main/docs/website/`.

### T18 — Commit & submodule pinning — TODO (needs human decision)
Work spans two repos. Suggested order: (1) commit inside `qaros` (new docs,
diagrams, examples, mirrored header), push a branch; (2) commit in Quaternar:
`.gitmodules` + `qaros` gitlink, the CMake mirror change, the
`default_inits.h` C-compatibility fix, this feature file. The Quaternar
working tree has many unrelated modified files from other work — stage
selectively, do not commit unrelated changes.

## How to build / verify

- Regenerate + mirror header: `cmake --build --preset windows-x64-debug
  --target qar-streaming-c_v0_single_header` (mirror requires the qaros
  submodule checked out; override dir with `-DQAR_QAROS_DIR=...`).
- Syntax-check examples without a full build:
  `clang -fsyntax-only -std=c11 -Iinclude -Iexamples -DQAR_ENABLE_DYNAMIC_LOADING
  --target=x86_64-pc-windows-msvc examples/<file>.c` (from `qaros/qar-streaming-c`).
- Site: `cd qaros/docs/website && npm ci && npm run build` (runs doxygen →
  doxygen2docusaurus → docusaurus; `onBrokenLinks: throw` validates links).
