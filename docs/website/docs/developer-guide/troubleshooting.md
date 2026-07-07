---
sidebar_position: 9
title: Troubleshooting
description: Common failures during integration — what they mean and what to do.
---

# Troubleshooting

## Status codes that are part of normal flow

| Status | Meaning | What to do |
|---|---|---|
| `QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND` | The onboarding ID has no persisted slot on this machine | Fall back to `qar_runtime_onboard_with_code` — this is the expected first-run path |
| `QAR_STATUS_PAKE_ERROR` | Pairing code wrong or expired (codes live ~10 s) | Re-prompt the user for the current code and retry |
| `QAR_STATUS_ONBOARDING_HUB_UNREACHABLE` | Discovery couldn't reach a Hub | Verify the Hub is running, host/port (remote Hubs need a `QarOnboardHostExt`), and firewall/multicast on the network |
| `QAR_STATUS_ONBOARDING_SESSION_STILL_ACTIVE` | `forget` called while the session is active | Call `qar_session_leave` first, then `forget` |

## Frequent integration mistakes

- **Forgetting to persist the onboarding ID.** The ID returned by onboarding is the *only* key to the persisted identity. Losing it means pairing again.
- **Calling API functions from inside callbacks.** Result/progress/update callbacks run on library threads; blocking API calls from there can deadlock. Queue to your own thread.
- **Zero-initializing init structs by hand.** Always use `qar_*_default()` — it stamps `header.type`, without which calls fail.
- **Mismatched projection metadata.** If streamed content "swims" or sits at the wrong depth, your `show_frame` near/far or per-view poses don't match what you actually rendered with. This is a metadata bug, not a network problem — see [Rendering Streams](/docs/developer-guide/rendering-streams#the-frame-loop).
- **Confusing panel and volume pose conventions.** Panel pose = top-left corner; volume pose = cuboid center.
- **Destroying the runtime while async operations run.** Cancel (and wait for the exactly-once result callback) before teardown.

## Where to look

- **Library logs** — enable with `QarLibraryInit.enable_console_logging` and/or `log_folder_path`.
- **Hub-side logs** — the runtime launcher's log folder (`logFolder` in the launcher config).
- **Warping monitor** — the visualizer's timing view shows per-volume stream latency/FPS/jitter, which quickly separates "my app renders slowly" from "the network is dropping frames".

:::note TODO
Extend this page as support cases accumulate: dynamic-loading failures (missing dependent DLLs, architecture mismatch), firewall/multicast checklists per network type, certificate-expiry recovery, and a "collect diagnostics" script for support tickets.
:::

## Getting help

- GitHub Issues on this repository for bugs and feature requests.
- GitHub Discussions for integration questions.
