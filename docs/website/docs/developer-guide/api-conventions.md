---
sidebar_position: 2
title: API Conventions
description: The patterns every QarOS C API module follows — handles, results, extensible init structs, async pairs, and enumeration.
---

# API Conventions

The C API is deliberately uniform: once you know these six patterns, every module (onboarding, panels, volumes, streams, peers) reads the same way.

## 1. Opaque handles and value IDs

Two kinds of identifiers exist:

- **Opaque handles** (`QarRuntime*`, `QarSession*`, `QarGuiPanel*`, `QarAppVolume*`, `QarRenderSender*`, `QarPeerSpec*`, `QarCancelToken*`, `QarOnboardingInvite*`) — pointers to library-owned objects. You never inspect them; you pass them back to API functions and release them with the matching `qar_*_handle_destroy()`. Validity can be checked with `qar_*_handle_is_valid()`.
- **Value IDs** (`QarPeerId`, `QarSessionId`, `QarGuiPanelId`, `QarAppVolumeId`, `QarStreamId`, `QarOnboardingId`) — small copyable structs (UUIDs). They can be stored, compared (`qar_*_id_equals`), converted to text (`qar_uuid_to_string`), and persisted.

Destroying a *handle* never destroys the *thing*: `qar_session_handle_destroy` releases your reference, while `qar_session_leave` actually leaves the session. Similarly `qar_runtime_handle_destroy` vs `qar_runtime_destroy`.

## 2. Results and status codes

Every fallible function returns a **`QarResult` by value**. Inspect it with:

```c
QarResult r = qar_some_call(...);
if (qar_result_is_error(r)) { /* handle */ }
if (qar_result_has_code(r, QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND)) { /* specific case */ }
qar_result_log_if_error(r);   // or just log it
```

Some statuses are *expected control flow*, not failures — for example `QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND` from a rejoin means "onboard instead", and `QAR_STATUS_PAKE_ERROR` means "re-prompt the user for the code". Tutorials call these out where they matter.

## 3. Extensible init structs (`QarStructureHeader`)

Every `...Init` struct begins with a `QarStructureHeader { type, next }`:

- **`type`** identifies the struct for ABI-safe evolution — always create structs with their `qar_*_default()` helper, which zero-fills the struct and stamps the correct type.
- **`next`** chains optional **extension structs**, exactly like OpenXR/Vulkan. For example, onboarding targets the local Hub by default; chaining a `QarOnboardHostExt` reaches a remote one:

```c
QarOnboardWithCodeInit init = qar_onboard_with_code_init_default();
init.code = "ABCD-EFGH";

QarOnboardHostExt host = qar_onboard_host_ext_default();
host.hostname = "192.168.1.50";
init.header.next = &host.header;   // chain the extension
```

This is how the API grows without breaking compiled applications: new capabilities appear as new extension structs, never as changed signatures.

## 4. Blocking / async pairs

Every long-running operation exists twice:

- **Blocking**: `qar_runtime_onboard_with_code(...)` — blocks until done; optional *progress callback* fires from an internal thread while you wait (the last progress callback returns before the call does).
- **Async**: `qar_runtime_onboard_with_code_async(...)` — returns immediately; a *result callback* fires **exactly once** on a background thread (on success, error, *and* cancellation). After it returns, the library never touches your `user_state` again.

Both variants accept an optional **`QarCancelToken`** — create it with `qar_cancel_token_create` (or `_with_timeout`), cancel from any thread, and keep it alive until the operation finished.

Progress callbacks share one signature (`qar_progress_callback_t`) delivering a severity, a percentage, and a message — suitable for driving a progress UI directly.

:::warning Callback rules
Result and progress callbacks arrive on **library threads**. Do not call blocking QarOS API functions from inside them, and marshal to your main thread before touching your own single-threaded state.
:::

## 5. Count-then-fetch enumeration and string getters

Enumerations and variable-size getters use the standard two-call C idiom:

```c
size_t count = 0;
qar_query_peer_specs_count(session, &count);          // 1: ask for the size
QarPeerSpec** specs = malloc(count * sizeof(*specs));
qar_query_peer_specs(session, specs, count, &count);  // 2: fetch
// ... use ...
for (size_t i = 0; i < count; ++i) qar_peer_spec_handle_destroy(specs[i]);
free(specs);
```

String getters likewise take a caller-provided buffer plus size (e.g. `qar_peer_spec_get_display_name(spec, buf, sizeof(buf))`); serialization uses `*_serialized_size` followed by `*_serialize`.

## 6. Ownership and lifetime rules

- Init structs and the strings they point to are **copied before the call returns** — stack-allocate freely.
- Out-handles are **owned by you**: release every handle you receive (from calls *and* from result callbacks).
- Handles delivered *inside* callbacks (e.g. the session in an onboard result callback) are yours only on success.
- A session handle keeps the runtime alive internally, so destruction order mistakes don't dangle — but the intended order is still: leave session → destroy session handle → destroy runtime → destroy library.
- The API is designed for **single-threaded use** per runtime; callbacks are the only cross-thread surface.

## Where device kinds appear

Deliberately, almost nowhere. The API has no "device type" parameter — a peer is a peer. The single exception is inviting a device into the session, where a **connection string** selects the transport: helpers build them per device (`qar_session_invite_connection_string_hololens("192.168.1.7", ...)` → `"hololens:192.168.1.7"`, plus `_quest`, `_visualizer`, `_zed`). See [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions).
