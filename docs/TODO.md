# QAROS Documentation — Deferred Work (TODO)

This file tracks documentation work that is **known but not yet doable** — content
that could not be verified in the codebase, depends on unshipped product features,
or requires assets/input that do not exist in-repo yet. It is deliberately kept
**out of the published documentation** so the docs read as finished prose; add
items here instead of leaving `:::note TODO` admonitions in the pages.

Status legend: TODO · BLOCKED(reason) · NEEDS-INPUT(who).

Last swept: 2026-10-06 (synced with internal docs; done items removed).

---

## Blocked on unshipped product features

- **On-device onboarding UI for Quest / Android** — BLOCKED(product). Code-based
  onboarding is supported by the API, but current Quest/Android player builds join
  through the operator-configured launcher invite (push path). An on-device
  code-entry screen and QR room-tag scanning are planned, not shipped. Pages that
  state this honestly today: `user-guide/getting-started`, `user-guide/onboarding-devices`.
  When it ships, update those two pages plus `user-guide/troubleshooting` ("get a
  new code") and `operator-guide/managing-devices`.

- **"Show this Hub's pairing code" visualizer screen** — BLOCKED(product) / T7.
  Only a hub-connect screen for entering a *remote* Hub's code was found in
  `qar-streaming-viz` (`HubConnectSplashView`). The backend mints a fresh code
  roughly every 8 s (10 s TTL) and writes it to the launcher log, so operators
  read the current code from the log today. `operator-guide/managing-devices` and
  `operator-guide/running-the-hub` document this workaround. When a local
  "show my code" screen exists, update both and add the click path.

- **Per-machine installer** (Quaternar #223) — documented as `QAROS-Setup-<version>.exe`.
  Verify against the first release-signed RC: setup file name and download location, tray
  autostart default, the uninstaller's user-data prompt, the demo setup's name, and whether the
  NuGet and Unity ZIP still bundle the runtime. Add Visualizer screenshots of the approval prompt
  and the Approved apps list.

## Blocked on the C# binding not being published in `qaros`

- **Compiled C# examples project** — BLOCKED(binding not in qaros) / R12 / old T8.
  The `qar-streaming-sharp` binding lives only in the Quaternar monorepo
  (`qar-streaming/libs/qar-streaming-sharp/`); the `qaros` submodule ships only
  `qar-streaming-c`. There is no C# examples project and no NuGet packaging metadata
  (`PackageId`/`Version`) in the csproj. Until it is published into
  `qaros/qar-streaming-sharp/` with a compiled `QarStreaming.Examples` project:
  - The C# code tabs across the Developer Guide are **illustrative** — accurate to
    the verified binding surface but not compiled from a sample. Each such page
    carries a one-line `:::info` caveat. Replace with compiled, snippet-extracted
    C# once the project exists (region markers `//! [region]`, or Docusaurus code
    references).
  - Add a `Tutorials > C#` subfolder (`developer-guide/tutorials/csharp/*.mdx`)
    mirroring the 5 C tutorials 1:1.
  - The NuGet ids are `Quaternar.Qaros.Streaming[.Demo]` (now in
    `developer-guide/getting-started`); publish them where integrators can reach them.
  - C# members the docs use are confirmed against the binding: `session.AppVolumes.GetOrCreate`
    / `session.GuiPanels.GetOrCreate` (options take the required common name in their
    constructor), `GuiPanels.NavigateToUri`, `session.RenderSenders.Create`,
    `session.Peers.UpdateDisplayName`, `runtime.Onboard` / `Rejoin` / `Forget`,
    `session.InviteTargetApp`. Remaining risk is in options/DTO field names inside the snippets.

## Needs team input (no single in-repo source)

- **Hub hardware minimums** — NEEDS-INPUT(team) / T12. Exact CPU/GPU/RAM/NIC
  minimum and recommended specs. `operator-guide/system-requirements` states
  honestly that sizing scales with user/stream count and should be confirmed with
  Quaternar rather than inventing numbers.
- **Per-device OS build minimums** — NEEDS-INPUT(team). HoloLens 2 OS build, Quest
  OS version, Android version. Deferred to the shipped package notes.
- **Formal ABI / deprecation timeline + runtime↔SDK compatibility matrix** —
  NEEDS-INPUT(team) / T12. Only the qualitative v0 promise is documented in
  `operator-guide/maintenance-and-updates`; a formal timeline/matrix does not
  exist to cite yet.
- **Cloud-relay setup steps** — NEEDS-INPUT(team). The relay deployment model is
  acknowledged in `operator-guide/networking-and-federation`, but concrete relay
  configuration steps are not in-repo.
- **ZED camera end-to-end setup** — NEEDS-INPUT(team). Drivers, which machine runs
  the ZED source, and volume placement. `user-guide/onboarding-devices` states it
  depends on the deployment.
- **Launcher config keys left out of the reference** — NEEDS-INPUT(team) / T9.
  `operator-guide/launcher-config-reference` omits: `hardwareUniqueName` (read and
  passed to the Hub, effect unclear); source-app `type: "ApiApp"` (how the launcher
  locates the API application is not configurable from the entry); the exact
  semantic difference of `startupTarget.type` `"local"` vs `"auto"`; behaviour when
  `schema_version` is omitted.

## Content to add when the source work lands

- **Room co-location / room-origin workflow** — TODO. Document precisely how
  multiple headsets co-locate to one room origin (relocalization / QR-anchor
  workflow per device type), and consider stating the room-space handedness/axis
  convention normatively in the C headers rather than by OpenXR inheritance.
  `developer-guide/coordinate-systems` currently frames both as inherited platform
  behavior.
- **Visualizer UI walkthrough with screenshots** — TODO / T7. Tray menu, visualizer
  window, Source Applications panel (tiles, add flow, Running list), Logs panel, app-volumes panel, warping/timing view, hub-connect screen. Store under
  `docs/website/static/img/screenshots/`. Referenced honestly (no tour) in
  `operator-guide/running-the-hub`.
- **Developer troubleshooting growth** — TODO. Grow `developer-guide/troubleshooting`
  from real support cases: dynamic-loading failures (missing dependent DLLs,
  architecture mismatch), firewall/multicast checklists per network type,
  certificate-expiry recovery, and a "collect diagnostics" script.

## Security documentation — track findings to closure

`operator-guide/security-deep-dive` (T10 — DONE) presents the intended design and folds these
**current, known** gaps into its limitations/roadmap honestly. They are engineering
items, tracked here so the docs can be tightened as each is fixed (do not overstate
guarantees in the meantime):

- Client private keys and CA secrets stored in plaintext (OS-ACL protected only);
  encrypted-at-rest (DPAPI / Android Keystore) planned.
- Network onboarding path: enrolled peer identity is currently self-asserted
  (claimed) rather than fully bound to the authenticated channel.
- Hub-to-Hub trust-bundle federation messages are not yet signed/authenticated.
- Device blacklist is keyed on an attacker-choosable device id (a revoked device can
  re-pair under a new id).
- No CA-signing rate limiter; no SNI hostname verification; admin/revocation control
  surface is in-process only (no authenticated remote control plane yet).
- T10 left out / to verify before documenting: (a) whether a Hub leaving a federation
  actually triggers trust withdrawal in production (a hub-leave handler exists, no
  publisher found) - the page says there is no operator withdraw action; (b) that Hub B
  also merges Hub A's root (page only describes A merging B's); (c) how a push-invited
  remote device (HoloLens/Android) receives its provisioned key - page only covers
  Hub-launched processes; (d) whether step-ca rejects or clamps a `certLifetimeHours`
  above the 24 h provisioner cap.
- T10 blacklist drift: resolved by ADR-0254 (hub publishes its revocation list; peers that read
  it refuse the device, others accept it until expiry); `managing-devices` and
  `security-deep-dive` rewritten to match.
- There is no production online renewal (gateway renewal binding unwired), so
  "silent rejoin" lasts only until 75 % of the cert lifetime (24 h cap).

## Repo hygiene / housekeeping

- **Run-verify examples against a real runtime** — TODO / T14. The 5 C examples pass
  `clang -fsyntax-only` but are not yet link/run-verified end-to-end against a real
  runtime DLL + Hub. Drop a binary package into `qaros/package/`, build, and run
  `onboarding_and_rejoin.exe` twice (onboard, then silent rejoin) then `--forget`.
- **Commit & submodule pinning** — NEEDS-INPUT(human) / T18. Work spans two repos.
  Commit inside `qaros` first (docs, components, config, examples), push a branch;
  then in Quaternar commit `.gitmodules` + the `qaros` gitlink + the CMake mirror
  change + the `default_inits.h` C-compat fix. Stage selectively — the Quaternar
  working tree has unrelated changes.

## To verify (added 2026-10-06 sync with internal docs)

- **Launcher config `sourceApps[].type: "ApiApp"`** — NEEDS-INPUT(team). QAROS API apps are added
  from the visualizer; whether the file-based `sourceApps[]` accepts them, and with which keys, is
  not verified. `operator-guide/launcher-config-reference` keeps listing only the older kinds.
- **Reconnect in the visualizer UI** — TODO. What a viewer/operator sees while a stream reconnects
  (status text, frozen frame) was not traced. `operator-guide/running-the-hub` gives timings only.
- **HoloLens player name on device** — TODO. `player-installation-to-device/hololens-2` still says
  the app appears as *QuaternAR Player* / *Skyline Player*; the shipped package is
  `QAROS Player HoloLens <v>.msix`. Confirm the tile name and whether `Dependencies/` and the `.cer`
  must be selected in Device Portal.
- **App-facing log streaming** — NEEDS-INPUT(team). Best-effort log streaming with lost-message
  markers is visible in the visualizer's Logs panel only; the C API has no log subscription. Document
  an app API if one is planned.
- **API app launch policy wording** — TODO(engineering). The registry enum comment says "one process
  for every target", the behaviour is one shared process for all targets (what the docs say).
