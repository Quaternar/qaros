---
sidebar_position: 2
title: Onboarding and Rejoin
description: Obtain a session from Rust by rejoining with a saved onboarding id or onboarding with a pairing code, hand an invite to a second runtime, and leave with destroy or forget.
---

## What you build

A program, run on the Hub machine, that gets a QAROS session with a pairing code. On its first run it pairs with the code the Hub shows and saves the onboarding id it gets back. On every later run it rejoins with that id, silently. It also mints an invite and uses it to onboard a second runtime in the same process, then leaves, keeping both identities, or erasing them with `--forget`.

The example passes no Hub address, so the code finds only your own Hub, on the machine it runs on. An app on another machine pairs the same way and names the Hub's host and port, the port shown beside the Hub's code:

```rust
OnboardMethod::Code { code, hub: Some(HubAddress { hostname: "192.0.2.10", port: hub_port }) }
```

What you will learn:

- the difference between a session handle, which lives for one run, and the onboarding id, which you persist
- the rejoin-or-onboard startup, and which status codes are normal control flow
- how a second API instance joins with an invite instead of a second pairing code
- what dropping a session does, and when to call `Runtime::forget`

A source app on the Hub machine needs no code: it joins as a local app with nothing to persist ([Local App Onboarding](/docs/developer-guide/tutorials/rust/local-app-onboarding)). This example pairs with a code anyway, as its C original does, to show the path of an app on another machine.

## Before you start

- Work through [Dynamic Loading](/docs/developer-guide/tutorials/rust/dynamic-loading) first: this example loads the runtime the same way.
- **Windows, on the Hub machine**, with the QAROS runtime (installed, or `QAROS_RUNTIME_DIR` naming its `bin` folder) and a Hub running under your Windows user. The example passes no Hub address, so it finds only that Hub, as step 3 explains.
- **A pairing code** for the first run. The Visualizer on the Hub machine shows the current one: open the Connection panel, then **Hubs** > **Pair another Hub** > **Let another hub join us** ([Reading this Hub's code](/docs/operator-guide/managing-devices#reading-this-hubs-code)). It lives about ten seconds, so type it quickly.
- **macOS or Linux:** the example runs against `qaros-stub`. The stub accepts any code except `WRONG` (a pairing error) and `UNREACHABLE` (no Hub).
- **The latest stable Rust.** Run the commands from `qar-streaming-rust/`.

## Run it

The example writes its state into the working directory: a storage folder per runtime (`onboarding_and_rejoin.primary-state`, `onboarding_and_rejoin.sibling-state`) and the two saved ids (`onboarding_and_rejoin.primary-onboarding-id.txt`, `onboarding_and_rejoin.sibling-onboarding-id.txt`).

```powershell
# First run: pair with the code the Hub shows.
cargo run -p qaros-examples --bin onboarding_and_rejoin -- <PAIRING-CODE>

# Later runs: rejoin with the saved id. No code.
cargo run -p qaros-examples --bin onboarding_and_rejoin

# Leave and erase both identities. The next run needs a code again.
cargo run -p qaros-examples --bin onboarding_and_rejoin -- --forget
```

Against the stub, name it with `QAROS_DLL`. The stub run writes the same storage folders and id files, which a later run against QAROS from that folder would try to rejoin with, so run the stub from a folder of its own, as [On macOS, against the stub](/docs/developer-guide/tutorials/rust/#on-macos-against-the-stub) shows: on macOS, from `target/stub-run`, `QAROS_DLL="$PWD/../debug/libqaros_stub.dylib"`. The first run onboards with a code and prints (with your own paths and ids):

```text
Loading qar-streaming-c from: .../libqaros_stub.dylib
Loaded .../libqaros_stub.dylib
qar_runtime_create: SUCCESS
qar_runtime_onboard: SUCCESS
Primary session obtained with onboarding id b066743e-5786-e018-0100-000000000000
Minted sibling invite (26 bytes, expires at unix 1792580420).
qar_runtime_create: SUCCESS
Sibling runtime onboarded with onboarding id 78d47f3e-5786-e018-0200-000000000000
Destroying sibling session handle keeps its persisted onboarding state for rejoin.
Destroying primary session handle keeps its persisted onboarding state for rejoin.
```

The second run, without a code, rejoins: the same primary id, and `qar_runtime_rejoin: SUCCESS` where the first run onboarded. The run with `--forget` ends with:

```text
Destroying primary session handle keeps its persisted onboarding state for rejoin.
qar_runtime_forget(sibling): SUCCESS
qar_runtime_forget(primary): SUCCESS
```

A run without a code and without a saved id stops before it reaches the Hub:

```text
error: No persisted onboarding id in 'onboarding_and_rejoin.primary-onboarding-id.txt'. Pass the pairing code shown on the hub's onboarding screen.
```

## Walkthrough

### 1. Read the arguments

```rust
{{#include ../src/bin/onboarding_and_rejoin.rs:arguments}}
```

The first argument that does not start with `--` is the pairing code; `--forget` asks for the full un-pair at the end. A code on the command line is fine for a tutorial. In an application, ask for it in your UI and never log it.

### 2. Create a runtime with its own storage, and obtain a session

```rust
{{#include ../src/bin/onboarding_and_rejoin.rs:primary}}
```

A runtime keeps its identities (certificates and session state) in a storage folder. Without one, the library uses its default (on Windows `%LOCALAPPDATA%\Quaternar\QarOS`). Give every runtime in one application a folder of its own, as this example does for both of its runtimes, so their identities never mix.

`obtain_session` returns two things with different lifetimes. The `Session` is a handle for this run only. The `OnboardingId` is a 16-byte key to the identity stored in the runtime's storage folder, and it outlives the process: it is the one thing your app must save.

### 3. Rejoin, or onboard with the code

This is the canonical startup ([Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions#onboarding-code-a-hub-on-another-pc)):

```rust
{{#include ../src/lib.rs:obtain_session}}
```

- **Rejoin first.** `Runtime::rejoin` shows no UI and needs no code. Call it on every start that has a saved id.
- **`ONBOARDING_SESSION_NOT_FOUND` is normal flow.** It means the id has no stored identity in this runtime's storage folder, for example after a forget or with a fresh folder. Onboard instead.
- **Any other rejoin error is real.** `ONBOARDING_HUB_UNREACHABLE`, for example, means no Hub answered. Report it and stop; asking for a pairing code would not help.
- **Onboard with the code.** `OnboardMethod::Code { code, hub: None }` finds your own Hub, the one running under your Windows user: first through the `hub-endpoint.json` file the Hub writes into the QAROS data folder on this machine, then through a discovery beacon whose owner is your user, listening up to 6 seconds. To reach any other Hub, pass `hub: Some(HubAddress { hostname, port })` with the port shown beside the Hub's code.
- **`PAKE_ERROR` is normal flow too.** The code was wrong or has expired. Read the current code off the Hub and try again.
- **Save the id at once.** `Runtime::onboard` returns an `Onboarded` with the new `onboarding_id` and the `session`. Without the saved id the next start must pair again.

Both calls block until the Hub answers. `BlockingOptions::default()` passes no progress channel and no cancel token; an application with a UI passes a `CancelToken` so the user can abort.

The CPU, D3D11, GUI panel and app volume examples start with `join_session` instead, which takes the same two steps and then, with neither an id nor a code, joins the Hub as a local app on the Hub machine ([CPU Rendering](/docs/developer-guide/tutorials/rust/cpu-rendering) shows it). This example always onboards with a code, as its C original does.

### 4. Save the onboarding id

```rust
{{#include ../src/lib.rs:onboarding_id_file}}
```

The id is written in the library's UUID text form (`Library::uuid_to_string`) and read back with `Library::uuid_from_string`. An unreadable file is reported and treated as no id, so the program asks for a code instead of failing.

### 5. Mint an invite for a second instance

```rust
{{#include ../src/bin/onboarding_and_rejoin.rs:mint_invite}}
```

A second QAROS API instance of the same application, such as a helper process or a plug-in host, must not pair with a code again ([Sibling onboarding](/docs/developer-guide/onboarding-and-sessions#sibling-onboarding-one-app-several-api-instances)). The onboarded instance asks its session for an invite and serializes it to bytes. Your application moves those bytes to the other instance over whatever channel it has. Invites expire: mint one when it is needed and hand it over at once.

### 6. Onboard the second runtime from the invite

```rust
{{#include ../src/bin/onboarding_and_rejoin.rs:sibling}}
```

Here both instances live in one process, so the "transfer" is a `Vec<u8>`. The sibling gets its own runtime and storage folder, turns the bytes back into an invite with `Library::deserialize_invite` and onboards with `OnboardMethod::Invite`. It gets an onboarding id of its own and saves it.

This example onboards the sibling from a fresh invite on every run and overwrites the sibling's id file. Each run therefore leaves one more stored identity behind in the sibling's storage folder; every run prints a new sibling id, and `--forget` erases only the last one. A real second instance rejoins with its saved id like the primary does, and asks for an invite only when that rejoin returns `ONBOARDING_SESSION_NOT_FOUND`.

### 7. Leave the sessions, keeping the identities

```rust
{{#include ../src/bin/onboarding_and_rejoin.rs:destroy}}
```

Dropping a `Session` calls `qar_session_handle_destroy`. The session ends for this process, and the stored identity stays, so the next run rejoins silently. This is every normal shutdown ([Teardown: destroy vs. forget](/docs/developer-guide/onboarding-and-sessions#teardown-destroy-vs-forget)).

### 8. Forget, only when the user un-pairs

```rust
{{#include ../src/bin/onboarding_and_rejoin.rs:forget}}
```

`Runtime::forget` erases the stored identity: its certificate and state. The next start needs a new pairing code. Call it only for an explicit "sign out" or "unpair this device", never on a normal exit.

Forget only after the session of that identity is gone. While it is alive, the call fails with `ONBOARDING_SESSION_STILL_ACTIVE`; that is why the drops come first. Delete your saved id together with the identity, as the example does with its files.

## Ownership and teardown

- **Each handle keeps its parent alive.** A `Session` keeps its `Runtime` alive, and a `Runtime` its `Library`. The C order (session, then runtime, then library) holds whatever order your values drop in. A runtime is destroyed after its last session, even if you drop the `Runtime` value first.
- **`Onboarded` is two values.** `onboarding_id` is a plain `Copy` value; `session` is the handle. The example copies the id out and drops `sibling.session` on its own, which is why `Runtime::forget` can still name the id afterwards.
- **An invite does not hold the session.** `OnboardingInvite` keeps only the library alive, so its bytes and the handle stay valid after the session that minted them is gone. Dropping it calls `qar_onboarding_invite_handle_destroy`. Onboarding borrows the invite for the call only, so you may drop it right after, as the example does.
- **Drop order matters for forget, not for memory.** The wrapper makes any drop order safe. It cannot know when you want to forget, so it is up to you to end the session first.
- **What the wrapper does not do for you:** save the onboarding id, pick a storage folder per runtime, or tell a rejoin that should fall back from one that should not. Those decisions are in `obtain_session`, which is yours to copy.
- **Not `Send`.** Runtimes, sessions and invites stay on the thread that created them. To hand an invite to another thread or process, send its serialized bytes.

## Troubleshooting

| What you see | Cause | What to do |
|---|---|---|
| `No persisted onboarding id in '...'. Pass the pairing code ...` | first run, or the id file was removed | pass the code the Hub shows |
| `qar_runtime_onboard failed with QAR_STATUS_PAKE_ERROR (1600)`, then `Pairing code rejected or expired` | the code was wrong or older than about ten seconds | read the current code and run again |
| `qar_runtime_onboard failed with QAR_STATUS_ONBOARDING_HUB_UNREACHABLE (1802)` | no Hub of your Windows user was found | start QAROS; for any other Hub, pass its address with `HubAddress` |
| `qar_runtime_rejoin failed with QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND (1800) ...; onboarding instead`, then `No persisted onboarding id` | the saved id has no identity in this storage folder: it was forgotten, or the folder was deleted or changed. The second message says "no id" although the id file exists | pass a new code; keep the id file and the storage folder together |
| `qar_runtime_forget failed with QAR_STATUS_ONBOARDING_SESSION_STILL_ACTIVE` | a session of that identity is still alive | drop the session first |
| the program loads the wrong runtime or none | see [Dynamic Loading](/docs/developer-guide/tutorials/rust/dynamic-loading#troubleshooting) | |

[Status codes that are part of normal flow](/docs/developer-guide/troubleshooting#status-codes-that-are-part-of-normal-flow) lists the others.

## From C

The C tutorial [Onboarding and Rejoin](/docs/developer-guide/tutorials/c/onboarding-and-rejoin) is the same program.

| C | Rust |
|---|---|
| `example_obtain_session` in `common.h` | `obtain_session` in the examples' `lib.rs` |
| `qar_runtime_create` with `QarRuntimeInit::storage_folder_path` | `Library::create_runtime(&RuntimeConfig { storage_directory, .. })` |
| `qar_runtime_rejoin` with `QarRejoinInit` | `Runtime::rejoin(id, &presentation, &options)`, returns a `Session` |
| `qar_runtime_onboard` with `QarOnboardCodeExt`, optionally `QarOnboardHostExt` | `Runtime::onboard(&OnboardMethod::Code { code, hub }, ...)`, returns `Onboarded` |
| `qar_runtime_onboard` with `QarOnboardInviteExt` | `Runtime::onboard(&OnboardMethod::Invite(&invite), ...)` |
| `qar_session_request_onboarding_invite` | `Session::request_onboarding_invite(&options)` |
| `qar_onboarding_invite_serialized_size` and `qar_onboarding_invite_serialize` | `OnboardingInvite::serialize()`, returns `Vec<u8>` |
| `qar_onboarding_invite_deserialize` | `Library::deserialize_invite(&bytes)` |
| `qar_onboarding_invite_get_expires_unix` | `OnboardingInvite::expires_unix()` |
| `qar_onboarding_invite_handle_destroy`, `qar_session_handle_destroy`, `qar_runtime_handle_destroy` | drop the value |
| `qar_runtime_forget` with `QarForgetInit` | `Runtime::forget(id)` |
| `qar_result_has_code(r, QAR_STATUS_PAKE_ERROR)` | `error.has_code(StatusCode::PAKE_ERROR)` |
| `qar_uuid_to_string`, `qar_uuid_from_string` | `Library::uuid_to_string`, `Library::uuid_from_string` |
| `--hub=HOST:PORT`, chained as `QarOnboardHostExt` | no Hub address: `OnboardMethod::Code` with `hub: None` |
| exit codes 1 and 2 | exit code 1 on any error, with the message on stderr |

One difference in behaviour: the Rust `rejoin` and `forget` refuse the zero onboarding id with `Error::InvalidArgument` before calling C.

## Next

- [Local App Onboarding](/docs/developer-guide/tutorials/rust/local-app-onboarding): how an app on the Hub machine joins.
- [Onboarding and Sessions](/docs/developer-guide/onboarding-and-sessions): every onboarding mode, one-time codes for several devices, choosing the GPU.
- [Peers](/docs/developer-guide/peers): what the session shows you once you are in.
