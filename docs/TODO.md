# QAROS Documentation — Deferred Work (TODO)

This file tracks documentation work that is **known but not yet doable** — content
that could not be verified in the codebase, depends on unshipped product features,
or requires assets/input that do not exist in-repo yet. It is deliberately kept
**out of the published documentation** so the docs read as finished prose; add
items here instead of leaving `:::note TODO` admonitions in the pages.

Status legend: TODO · BLOCKED(reason) · NEEDS-INPUT(who).

Last swept: 2026-07-08 (three-audience Knowledge Base restructure).

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

- **MSIX as the primary install path** — BLOCKED(two items) / T13. Promotion needs
  (1) seeding the launcher config into the package `LocalState` (packaged install
  root is read-only) and (2) a tray toggle for the startup task. Tracked in the
  `windows-msix-packaging` feature. When landed, rewrite the MSIX section of
  `operator-guide/installation` as the primary path and demote ZIP, and revisit
  `operator-guide/maintenance-and-updates`.

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
  - Publish real NuGet package metadata; confirm the package id used in
    `developer-guide/getting-started` (currently `QarStreamingSharp`).
  - Verify the C# member names the docs currently infer (not all were in the
    binding fact-check): `session.GuiPanels.GetOrCreate` / `NavigateToUri`,
    `session.AppVolumes.GetOrCreate`, `session.RenderSenders.Create`,
    `session.Peers.UpdateDisplayName`, `runtime.OnboardWithCode` / `Rejoin` /
    `Forget`, and options/DTO field names. Confirmed members: `Result<T>`,
    `.IsSuccess`/`.Info`/`.Value`/`.EnsureSuccess()`/`.ValueOrThrow()`,
    `NativeCallException`, `session.Peers.GetAll()` → `Result<PeerSpec[]>`,
    `session.Peers.Updated` event, `SubscribeUpdates` → `Result<IDisposable>`,
    `IDisposable` handles, the `Qar` namespace, target frameworks.

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

## Content to add when the source work lands

- **Room co-location / room-origin workflow** — TODO. Document precisely how
  multiple headsets co-locate to one room origin (relocalization / QR-anchor
  workflow per device type), and consider stating the room-space handedness/axis
  convention normatively in the C headers rather than by OpenXR inheritance.
  `developer-guide/coordinate-systems` currently frames both as inherited platform
  behavior.
- **Worked room→app hit-testing example** — TODO / T11. Add a room-space point →
  app-space point helper (respecting the center-of-cuboid convention) to
  `qar-streaming-c/examples/app_volume_management.c` with a snippet section, then
  reference it from `developer-guide/app-volumes`.
- **D3D11 render-sender example** — TODO / T11. Add a D3D11 sender example
  (guard `#ifdef QAR_ENABLE_D3D11`, chain `QarStreamParamsD3D11`) and extend
  `developer-guide/rendering-streams` / the `cpu-rendering` tutorial.
- **Gesture-config + world-anchor tutorial sections** — TODO / T11. Extend the
  `app_volume_management.c` example (gesture rules, world-anchor set/clear) and the
  `tutorials/c/app-volume-management` page. Verify `QarAppVolumeGestureEvent` field
  names against the generated header first.
- **Visualizer UI walkthrough with screenshots** — TODO / T7. Tray menu, visualizer
  window, app-volumes panel, warping/timing view, hub-connect screen. Store under
  `docs/website/static/img/screenshots/`. Referenced honestly (no tour) in
  `operator-guide/running-the-hub`.
- **Developer troubleshooting growth** — TODO. Grow `developer-guide/troubleshooting`
  from real support cases: dynamic-loading failures (missing dependent DLLs,
  architecture mismatch), firewall/multicast checklists per network type,
  certificate-expiry recovery, and a "collect diagnostics" script.

## Security documentation — track findings to closure

`operator-guide/security-deep-dive` presents the intended design and folds these
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

## Repo hygiene / housekeeping

- **Retire stale internal C API docs** — TODO / T15.
  `qar-streaming/libs/qar-streaming-c/docs/API_Reference.md` still teaches the removed
  `qar_runtime_create_session` / `qar_session_join` flow and wrong signatures. Delete
  it (pointing at docs.quaternar.com) or regenerate. Decide one source of truth vs.
  its `Getting_Started.md`.
- **Run-verify examples against a real runtime** — TODO / T14. The 5 C examples pass
  `clang -fsyntax-only` but are not yet link/run-verified end-to-end against a real
  runtime DLL + Hub. Drop a binary package into `qaros/package/`, build, and run
  `onboarding_and_rejoin.exe` twice (onboard, then silent rejoin) then `--forget`.
- **Commit & submodule pinning** — NEEDS-INPUT(human) / T18. Work spans two repos.
  Commit inside `qaros` first (docs, components, config, examples), push a branch;
  then in Quaternar commit `.gitmodules` + the `qaros` gitlink + the CMake mirror
  change + the `default_inits.h` C-compat fix. Stage selectively — the Quaternar
  working tree has unrelated changes.
