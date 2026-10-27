# QAROS Rust binding

## Goal

Map the QAROS streaming C API (`qar_streaming.h`) into Rust in two layers:

1. **`qaros-sys`, the raw layer**: bindgen output for every exported `qar_impl_*` function and
   every type, generated per target and committed; the `QarApi` function table; the runtime
   lookup; hand ports of the header's `static inline` `qar_*_default()` initializers. No
   business logic.
2. **`qaros`, the safe layer**: types that own the C handles, enforce the teardown order, turn
   callbacks into owned snapshots and return `qaros::Result`. Whatever it does not wrap stays
   reachable through `Library::raw_api()`.

Around them: `qaros-stub` (the in-memory stand-in for the runtime that the safe layer's tests load),
`qaros-conformance` (the initializer ports against the C originals), `examples` (ports of the C
examples, tutorial sources, the developer guide's Rust tabs), `xtask` and
`tests/qar-streaming-rust-ext` (integration tests against `qar-streaming-c-ext`; test-only, not
a default workspace member, nothing depends on it).

## Naming

- Crates carry the product name: `qaros-sys` and `qaros`.
- Raw types keep the C names (`QarSession`, `QarRenderSenderInit`). Safe types drop the `Qar`
  prefix (`Session`, `RenderSender`, `PeerId`, `StatusCode`).
- Bind the exported `qar_impl_*` symbols, never the header's `static inline` `qar_*` wrappers,
  which are not exports. The `function` of `Error::Api` is the public C name
  (`"qar_runtime_create"`), as in the header and the guide.
- C enums are bindgen newtypes in `qaros-sys`; the safe layer maps the ones it uses to Rust
  enums with `from_raw` and `to_raw` (`crates/qaros/src/types.rs`). `StatusCode` constants
  drop the `QAR_STATUS_` prefix, and the rendering producer codes also drop `PRODUCER_` and
  `DO_` (`RENDERING_UNABLE_TO_BEGIN_FRAME`); `StatusCode::name` gives the header's name.
- Ids are 16-byte newtypes from `define_id!` (`PeerId(pub [u8; 16])`); onboarding ids are
  persisted with `Library::uuid_to_string`.
- Strings go to C as UTF-8 `CString`s through `crates/qaros/src/strings.rs`; an interior NUL is
  `Error::InvalidArgument`. Strings from C are read lossily.

## Required workflow

1. Read the header or its diff: constants, enums, structures, callbacks, handles, functions,
   `qar_*_default()` initializers, destroy functions, async and subscription calls.
2. Raw layer first: regenerate, port the initializers, then the stub (exports follow by
   themselves, behaviour by hand).
3. Safe layer second. Ownership comes from the create, destroy, query and getter functions and
   the header's text, never from what seems natural.
4. Classify every callback API (see [Callbacks](#callbacks)).
5. Test against the stub, and against `qar-streaming-c-ext` where the stub cannot show real
   behaviour.
6. Write down uncertain ABI details, missing destroy functions or undocumented semantics
   instead of guessing: in a doc comment where the code assumes something, and in the report.
7. Work through the [change checklist](#change-checklist).

## Formatting and checks

rustfmt's defaults (4 spaces, 100 columns); `.editorconfig` matches them for other editors. The
workspace lints in `Cargo.toml`: clippy pedantic, `unwrap_used` and `expect_used` warned (test
files allow them explicitly), `unsafe_op_in_unsafe_fn` denied, `missing_debug_implementations`
warned. Every `unsafe` block has a `// SAFETY:` comment, every public `unsafe fn` a `# Safety`
section.

After every change, from the workspace root:

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --features qaros/vulkan,qaros-examples/vulkan,qar-streaming-rust-ext/e2e -- -D warnings
cargo test --workspace
cargo xtask tutorials --check
cargo xtask guide --check
```

As the change needs: on a host without Windows the same clippy with
`--target x86_64-pc-windows-msvc`; for Vulkan code
`QAROS_REQUIRE_VULKAN=1 cargo test -p qaros --features vulkan` (without the variable, a host
without a Vulkan driver prints `skipped` and passes); for doc comments
`RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace`.

## Loading the runtime

`QarApi::load_default` (and `Library::load_default`) follows the default lookup of "Authoring a
Language Binding": already loaded, beside the binding, `QAROS_RUNTIME_DIR`, the installed QAROS
(`HKLM` only), `PATH`. A set `QAROS_RUNTIME_DIR` or an installation whose runtime does not load
is reported, never skipped. Decisions live in `crates/qaros-sys/src/lookup.rs`, tested against a
simulated system; system calls in `crates/qaros-sys/src/platform.rs`.

Loading resolves every symbol of the table or none (`LoadError::MissingSymbols` names all that
are missing), so bindings from a newer header do not load an older runtime.

## Cancellation

- A blocking call takes `&BlockingOptions`: an optional progress `Sender` and an optional
  `&CancelToken`, whose borrow keeps the token alive for the call; `None` passes NULL.
- `Library::create_cancel_token(timeout)` makes a token. It is not `Send`;
  `CancelToken::cancel_handle()` cancels from any thread while the token lives.
- A subscription always owns a token of its own (`SessionInner::subscribe`): ending it is
  cancelling that token.
- An async call keeps its token until the callback comes. A token the library may still use
  after its last owner is gone goes to `CancelTokenInner::destroy_at_library_teardown`.
- Add a cancel parameter only where the C function takes a `QarCancelToken*`; where it takes
  none, write the gap down.

## The raw layer

- `crates/qaros-sys/src/generated/` is written by `cargo xtask bindgen`, never by hand. Per
  target: `types.rs` (with bindgen's layout tests) and `functions.rs`, the declarations inside
  `qar_impl_function_list!`, which `crates/qaros-sys/src/loader.rs` expands into `QarApi` and
  `SYMBOL_NAMES` and `qaros-stub` into its exports.
- Targets: `x86_64-pc-windows-msvc`, `x86_64-unknown-linux-gnu`, and `aarch64-apple-darwin` for
  development against the stub. A new one is `cargo xtask bindgen <target>` plus a `cfg` entry
  in `crates/qaros-sys/src/lib.rs`.
- Every target binds Vulkan (`-DQAR_ENABLE_VULKAN`); macOS also gets `-DQAR_ENABLE_D3D11` so
  the stub can emulate the D3D11 sender; Windows and Linux keep the header's choice (no D3D11 on
  Linux). OpenGL is not bound. `xtask/shim/`, `xtask/shim-d3d11/` and `xtask/shim-vulkan/`
  stand in for the Windows SDK, D3D11 and Vulkan headers.
- C enum width differs per compiler (`int` under MSVC, `unsigned int` under clang), so newtype
  inner types differ per target; cast where it matters (`axis_mask` in
  `crates/qaros-sys/src/defaults.rs`).
- The table owns the opened library. Function pointers copied out of it (destroy functions
  handed to `OwnedHandle`, the cancel function behind `CancelHandle`) stay valid only because
  the DLL is never unloaded (see "The library stays mapped"); do not change one without the
  other.
- `QarApi::release_runtime` calls `qar_impl_runtime_destroy` when the bindings declare it,
  else `qar_impl_runtime_handle_destroy`; `RUNTIME_RELEASE_SYMBOL` names the one in use.
- A raw-only call takes the C handle of a safe value: `as_raw()` on `Runtime`, `Session`,
  `RenderSender`, `CancelToken` and `OnboardingInvite`. Taking the pointer is safe and using it
  `unsafe`; the safe value keeps owning it, so raw code never destroys it. A new type that owns
  a handle a raw-only function takes gets an `as_raw()` with the contract of `Session::as_raw`
  and a test in `crates/qaros/tests/stub/raw_handles.rs`.

The same sources build against two binding sets: those of the header of runtime
0.9.0-dev.2916, which declares `qar_runtime_destroy` and lacks the peer presence, role,
category and device identity functions, the member-hub functions and
`qar_platform_set_android_java_vm`, and those of the public header in the QAROS
repository (`qar-streaming-c/include/qar_streaming.h`), which has them and drops
`qar_runtime_destroy`. The README says which header this checkout's bindings come from. A
function in only one of the two sets stays raw only;
its stub behaviour, if any, goes into the `default_behaviour!` arm in
`crates/qaros-stub/src/exports.rs`, as `qar_impl_runtime_destroy`'s does. A status code only
the public header declares gets its `StatusCode` constant by value
(`StatusCode::ONBOARDING_RE_PAIR_REQUIRED`), and so does a value of an enum both sets have
(`PixelFormat::D16Unorm`, the `by value:` part of `c_enum!`). Types only the public header
declares, the render sender's colour extensions, are used under the cfg
`qaros_color_extensions`: `qaros-sys`'s build script sets it from the target's generated
bindings and passes it on as `DEP_QAR_STREAMING_C_COLOR_EXTENSIONS`, which the build scripts of
`qaros`, `qaros-stub` and `qaros-conformance` turn into the same cfg.

## Init structures, defaults and `header.next` chains

- Every init or extension structure starts from its ported default in
  `crates/qaros-sys/src/defaults.rs`, so its `QarStructureHeader` is stamped; never from a
  zeroed or hand-built value. Members the C initializer zero-fills are zero in the port (other
  structure types through the `zero_initializable!` list).
- `qaros-conformance` checks every port: an `EXPORT_DEFAULT` line in
  `crates/qaros-conformance/src/defaults.c` and a `Type => function` pair in `conformance!` in
  `crates/qaros-conformance/tests/defaults.rs` (D3D11 ones inside `#ifdef QAR_ENABLE_D3D11` and
  the test's `d3d11` module).
- Chain an extension by pointing `header.next` at its `header`, in the order the header
  documents. For a blocking call, the init, its extensions and their `CString`s are locals that
  outlive the call (`Runtime::rejoin_on_adapter`, `crates/qaros/src/runtime.rs`):

```rust
let mut init = defaults::qar_rejoin_init_default();
init.onboarding_id = onboarding_id.to_raw();
init.presentation = presentation_strings.to_raw();
let mut adapter_extension = adapter.map(GraphicsAdapter::to_raw);
if let Some(extension) = adapter_extension.as_mut() {
    init.header.next = (&raw mut extension.header).cast();
}
```

- For an async call the library may read the init until it calls back: the init and all it
  points to are one heap allocation owned by the callback state, freed after the callback or
  leaked when it never comes (`RenderSenderInitStorage`, `InitAllocation`).
- The stub checks every header it receives (`header_mismatch`), so an unstamped structure fails
  the tests.

## Errors

- Every fallible call returns `qaros::Result`. Map each `QarResult` right after its call with
  `check(api, "qar_public_name", result)` (`crates/qaros/src/error.rs`): `Ok(())` or
  `Error::Api` with the function, the `StatusCode`, the `qar_result_message` text and the
  `StatusCode::WRAPPED_CAUSES` that `qar_result_has_code` finds. (`CancelHandle::cancel`, which
  has no table at hand, builds its error without the message and causes.)
- Status-only C functions become `Result<()>` (`Session::set_display_name`); a value written
  through an out-pointer becomes `Result<T>`. A null handle after success is an error from
  `require_handle`, not a panic.
- Expected outcomes are errors the caller tests: `has_code` for the code itself
  (`ONBOARDING_SESSION_NOT_FOUND` from a rejoin means onboard instead), `has_cause` for a wrapped
  one (begin-frame wraps a paused, reconnecting or transfer-less stream and an unresolved app
  volume in `RENDERING_UNABLE_TO_BEGIN_FRAME`; `is_waiting` in `examples/src/lib.rs`). Only a C
  out-flag becomes a non-error result: `try_begin_owned_frame` returns `Ok(None)` while `ready`
  is false.
- A new code that someone tests gets a `StatusCode` constant and a `name()` arm; a code a
  result can wrap also goes into `WRAPPED_CAUSES`.
- `Error::InvalidArgument` is what the wrapper refuses before calling C (an interior NUL, more
  than `QAR_MAX_FRAME_VIEWS` views, a second open frame, the zero onboarding id).
- No panics in library code; a panicking user callback is stopped at the FFI boundary
  (`deliver` in `crates/qaros/src/callbacks.rs`).

## Ownership and lifetime

- Every handle releases itself on drop and holds an `Rc` to its parent: frame info, render
  sender, session, runtime, library. The C teardown order therefore holds in any Rust drop
  order (`teardown_follows_sender_session_runtime_library_whatever_the_drop_order` in
  `crates/qaros/tests/stub.rs`). Where fields must release in a given order, declaration order
  is that order and a comment says so (`RenderSenderInner`).
- Handles are not `Send`: the SDK is single-threaded per runtime. `qar_library_init` is
  process-wide, so a second live `Library` is `Error::LibraryAlreadyLoaded`.
- A handle from a create, onboard, rejoin, begin-frame, query or getter is owned and released
  once by its matching destroy (`qar_render_stream_handle_destroy` for a sender); short-lived
  ones use `OwnedHandle`. A handle passed into a callback is borrowed: read it there, never
  destroy or keep it. The exception the header states is a render request, which its callback
  releases (`render_request_trampoline`).
- **Bounded wait, then a deliberate leak.** A session dropped with cancelled sender creations
  that have not called back waits `CANCELLED_CREATION_WAIT` (500 ms) for all of them, destroying
  any sender delivered meanwhile. If one is still pending, the session, its callback state, its
  runtime and the library are leaked, because destroying the session could block for good
  (`stub_cancelled_creation_teardown.rs`, `stub_session_leak.rs`).
- **The library stays mapped.** `LibraryInner` holds `ManuallyDrop<QarApi>` and never frees the
  DLL: unloading it after `qar_library_destroy` has crashed processes at exit. After a clean
  destroy a later load initializes it again; after a failed one every load is
  `Error::LibraryAlreadyLoaded` (`stub_library_stays_loaded.rs`).
- Do not wrap a call whose cancellation never calls back: `qar_render_sender_begin_frame_async`
  stays raw; `try_begin_owned_frame` is the non-blocking begin.

## Callbacks

- **One-shot async** (`_async`, a final `QarResult` once): the callback runs on a library
  thread, during or after the call. It records the outcome under a mutex, notifies waiters and
  runs the caller's `wake` closure there (panics caught), nothing more; the owner's thread
  builds the non-`Send` wrapper when it polls (`PendingRenderSender::poll`). Dropping the
  pending value cancels and returns at once; the session keeps the creation until its callback
  and applies the bounded wait at teardown. The library holds one `Arc` reference to the state
  until the callback reclaims it; a callback that never comes leaks it.
- **Continuous subscription** (`subscribe`, `updates`): callbacks run on library threads,
  possibly concurrently, until the subscription ends. Wrap it as a method taking
  `impl Fn(Event) + Send + Sync + 'static` and returning a `Subscription` guard
  (`Session::subscribe_peer_updates`, `GuiPanels::subscribe`, `AppVolumes::subscribe`); the
  trampoline delivers an owned snapshot (`PeerInfo`, `GuiPanelInfo`, `AppVolumeInfo`).
  `qaros::channel()` forwards events into an `mpsc` receiver.
- **Progress** of a blocking call goes to `BlockingOptions::progress` and ends with the call.

Ending a subscription: dropping the `Subscription` (or `cancel()`) cancels its token, which
waits for a running callback, and none starts afterwards. The callback state is freed only after
the session handle is destroyed, so a channel stays connected until then; `detach()` keeps
delivering for the session's lifetime. A callback is `Send + Sync`, so it cannot hold a
`Session`, `Library` or `Subscription`: the header's rules for calls made from inside a
callback need no code here. One rule is left: never drop a
`Subscription`, `Session` or `Library` while holding a lock a callback takes, since the drop
waits for that callback.

## Mapping examples by API category

A C shape from the header, then the Rust shape in this crate. `// ...` marks left-out lines.

### Value returned directly

The header returns no handle directly (every handle comes through an out-pointer with a
`QarResult`); plain values may be. `CancelToken::is_cancelled`, `crates/qaros/src/cancel.rs`:

```c
static inline bool qar_cancel_token_is_cancelled(const QarCancelToken* token);
```

```rust
pub fn is_cancelled(&self) -> bool {
    let api = &self.inner.library.api;
    // SAFETY: the handle is live until CancelTokenInner drops.
    unsafe { (api.qar_impl_cancel_token_is_cancelled)(self.inner.handle.as_ptr()) }
}
```

### Handle through an out-pointer plus `QarResult`

`Runtime::create`, `crates/qaros/src/runtime.rs`. The public type wraps an `Rc` of an inner type
that owns the handle and holds its parent:

```c
static inline QarResult
qar_runtime_create(const QarRuntimeInit* init, QarRuntime** out_runtime);
static inline void qar_runtime_handle_destroy(QarRuntime* handle);
```

```rust
let mut handle: *mut sys::QarRuntime = ptr::null_mut();
// SAFETY: init is stamped and its strings outlive the call; handle is
// a valid out-pointer.
let result = unsafe { (api.qar_impl_runtime_create)(&raw const init, &raw mut handle) };
check(api, "qar_runtime_create", result)?;
let handle = require_handle(handle, "qar_runtime_create")?;
Ok(Self {
    inner: Rc::new(RuntimeInner {
        handle,
        library: Rc::clone(library),
    }),
})
// ...
impl Drop for RuntimeInner {
    fn drop(&mut self) {
        // SAFETY: every session holds an Rc to this value, so all session
        // handles are already destroyed; the handle is not used again.
        unsafe { self.library.api.release_runtime(self.handle.as_ptr()) };
    }
}
```

### Value getter through a pointer

`Session::id`, `crates/qaros/src/session.rs`; the out value starts from its default:

```c
static inline QarResult
qar_session_get_id(const QarSession* session, QarSessionId* out_session_id);
```

```rust
pub fn id(&self) -> Result<SessionId> {
    let api = self.inner.api();
    let mut session_id = defaults::qar_session_id_default();
    // SAFETY: live session handle; valid out-pointer.
    let result = unsafe {
        (api.qar_impl_session_get_id)(self.inner.handle.as_ptr(), &raw mut session_id)
    };
    check(api, "qar_session_get_id", result)?;
    Ok(SessionId::from_raw(session_id))
}
```

### String getter through a buffer

`read_gui_panel_info`, `crates/qaros/src/panels.rs`. The getters report neither the size they
need nor truncation, so `read_string` passes a `GETTER_BUFFER_SIZE` (4096) buffer:

```c
static inline QarResult qar_gui_panel_get_display_name(
    QarGuiPanel* handle, char* out_buffer, size_t buffer_size
);
```

```rust
let display_name = strings::read_string(|buffer: *mut c_char, buffer_size: usize| {
    // SAFETY: handle is live; the buffer is writable for buffer_size bytes.
    let result =
        unsafe { (api.qar_impl_gui_panel_get_display_name)(handle, buffer, buffer_size) };
    check(api, "qar_gui_panel_get_display_name", result)
})?;
```

### Borrowed object in a callback

`peer_update_trampoline`, `crates/qaros/src/session.rs`. The snapshot reader is the one the
query uses, so a callback and a query read the same fields:

```c
typedef void (*qar_peer_update_callback_t)(
    QarPeerSpec* handle, void* user_state
);
```

```rust
unsafe extern "C" fn peer_update_trampoline(
    handle: *mut sys::QarPeerSpec,
    user_state: *mut c_void,
) {
    // SAFETY: registered by subscribe_peer_updates with this event type.
    let state = unsafe { CallbackState::<PeerInfo>::from_user_state(user_state) };
    if handle.is_null() {
        return;
    }
    // The handle is borrowed for the duration of the callback: read, never destroy.
    match read_peer_info(state.api(), handle) {
        Ok(peer) => state.deliver(peer, "peer update"),
        Err(error) => log::warn!("reading a peer update: {error}"),
    }
}
```

### Count, then fill

`Session::peers`, `crates/qaros/src/session.rs`. `query_owned_handles` owns every handle the
fill wrote, even when it fails, snapshots each and destroys them all; every count-then-fill
query of handles uses it (`GuiPanels::list` too):

```c
static inline QarResult
qar_query_peer_specs_count(QarSession* session, size_t* out_count);
static inline QarResult qar_query_peer_specs(
    QarSession* session,
    QarPeerSpec** out_handles,
    size_t handles_buffer_size,
    size_t* out_handles_written
);
```

```rust
pub fn peers(&self) -> Result<Vec<PeerInfo>> {
    let api = self.inner.api();
    let session = self.inner.handle.as_ptr();
    query_owned_handles(
        api,
        // SAFETY: live session handle; valid out-pointer.
        |count| unsafe { (api.qar_impl_query_peer_specs_count)(session, count) },
        // SAFETY: the buffer holds `capacity` handle slots.
        |handles, capacity, written| unsafe {
            (api.qar_impl_query_peer_specs)(session, handles, capacity, written)
        },
        api.qar_impl_peer_spec_handle_destroy,
        "qar_query_peer_specs",
        |handle| read_peer_info(api, handle),
    )
}
```

### One-shot async

`PendingRenderSender::start` and `render_sender_created_trampoline`,
`crates/qaros/src/pending_sender.rs`. When the start call itself fails, the state's library
reference is leaked and the token kept until the library goes: the header does not say whether
a callback can still follow.

```c
typedef void (*qar_render_sender_create_callback_t)(
    QarResult status, QarRenderSender* out_stream, void* user_state
);
static inline QarResult qar_render_sender_create_async(
    QarSession* session,
    QarRenderSenderInit* init,
    qar_render_sender_create_callback_t callback,
    void* user_state,
    QarCancelToken* token
);
```

```rust
let user_state = Arc::into_raw(Arc::clone(&shared))
    .cast_mut()
    .cast::<c_void>();
let api = session.api();
// SAFETY: live session; init and what it points to live in the
// callback state, which the library's reference keeps alive until the
// callback; the token outlives the operation (see AbandonedCreation).
let result = unsafe {
    (api.qar_impl_render_sender_create_async)(
        session.handle.as_ptr(),
        init,
        Some(render_sender_created_trampoline),
        user_state,
        token.handle.as_ptr(),
    )
};
// ...
// SAFETY: user_state is the Arc::into_raw reference handed over in
// PendingRenderSender::start, reclaimed here once (module docs).
let shared = unsafe { Arc::from_raw(user_state.cast_const().cast::<CreationShared>()) };
let mut state = shared.lock();
// ...
*state = CreationState::Delivered(CreationOutcome {
    result: status,
    sender: sender as usize,
});
drop(state);
shared.called_back.notify_all();
```

### Continuous subscription

`Session::subscribe_peer_updates`, `crates/qaros/src/session.rs`. `SessionInner::subscribe`
creates the token, hands the callback state to C through `Box::into_raw`, keeps both in the
session and returns the guard; every subscription goes through it:

```c
static inline QarResult qar_peer_subscribe_updates(
    QarSession* session,
    qar_peer_update_callback_t callback,
    void* user_state,
    QarCancelToken* token
);
```

```rust
pub fn subscribe_peer_updates(
    &self,
    callback: impl Fn(PeerInfo) + Send + Sync + 'static,
) -> Result<Subscription> {
    let session = self.inner.handle.as_ptr();
    self.inner.subscribe(
        Box::new(callback),
        "qar_peer_subscribe_updates",
        |api, user_state, token| {
            // SAFETY: live session; user_state stays valid until the
            // session handle is destroyed.
            unsafe {
                (api.qar_impl_peer_subscribe_updates)(
                    session,
                    Some(peer_update_trampoline),
                    user_state,
                    token,
                )
            }
        },
    )
}
```

## Rendering

- One render sender per target peer; every graphics API uses the same `RenderSender`, `Frame`
  and `OwnedFrame`.
- **One open frame per sender.** `begin_frame` returns a `Frame` borrowing the sender;
  `begin_owned_frame` and `try_begin_owned_frame` return an `OwnedFrame` that keeps it alive. A
  second begin, or a frame record read outside a cycle, while a frame is open is
  `Error::InvalidArgument` without a library call (`refuse_if_frame_open`). A frame dropped
  without `show` releases its frame info unpresented.
- **CPU:** each texture is a `&mut [u8]` borrowing the frame, unusable after `show`;
  overlapping textures are refused (two mutable slices over the same bytes).
- **Vulkan** (feature `vulkan`, `ash` 0.38): creation is `unsafe` (the caller vouches for the
  device, its extensions and its lifetime); QAROS owns the images; the frame record is borrowed
  mutably because the library's pointer lives only until the next `frame_vulkan` call.
  `docs/vulkan-api.md`.
- **D3D11** (feature `d3d11`, `windows` crate 0.62): compiled where the bindings carry D3D11
  (`cfg(qaros_d3d11)` from the build script, Windows and macOS) and to nothing elsewhere, since
  features are additive. `D3D11StreamParams::new` takes references the sender keeps until it is
  destroyed; creation is `unsafe` because QAROS uses the immediate context at times and on
  threads the header does not document; textures and keyed mutexes are lent for the record's
  borrow. `docs/d3d11-api.md`.
- Vulkan or D3D11 frame access on a sender of another API fails without a library call.
- A rule the wrapper cannot enforce is written, in agreement, in the `# Safety` section of the
  `unsafe fn`, the module's "Safety rules" (`vulkan.rs`, `d3d11.rs`) and the docs page's
  "Safety rules", each saying which rules are enforced and which are the caller's.

## The stub

`qaros-stub` is a cdylib the tests and examples load like the runtime, on every target the
bindings cover. It logs every call, checks structure headers, and records double destroys,
wrong destroy functions and out-of-order teardown. It proves the wrapper's side only: a stub pass
is never evidence of what QAROS does where the header is silent.

- **Symbol set.** It exports exactly the binding set it is built against:
  `crates/qaros-stub/src/exports.rs` turns the generated list into the `StubApi` trait, whose
  defaults return `QAR_STATUS_NOT_IMPLEMENTED` (or false, a zero id, nothing), and one
  `#[no_mangle]` export per function. A new header function needs no edit to be exported.
- **Behaviour** goes into `impl StubApi for Stub` in `crates/qaros-stub/src/behaviour.rs` when
  the safe layer or a test calls the function (Vulkan in `vulkan.rs`, D3D11 in `d3d11.rs`,
  state in `state.rs`): check headers with `header_mismatch`, create handles with `new_handle`,
  release them with `destroy_handle`, run callbacks on threads of their own as the runtime does.
- **Fault controls** are the `qar_stub_*` exports in `behaviour.rs` (`qar_stub_fail_next`;
  peers that join, leave, retire, pause, stop answering or restart; delayed, unanswered or
  cancel-ignoring creations; begin-frame delay; reported adapter; render requests; call log)
  and `QAROS_STUB_SCENARIO`, `QAROS_STUB_REPORT`, `QAROS_STUB_VULKAN_LIBRARY`,
  `QAROS_STUB_VULKAN_QUEUE_FAMILY`. Tests reach them through `StubControl` in
  `crates/qaros/tests/support/mod.rs`. A failure mode the safe layer handles gets a control and
  a test.
- Stub tests: `crates/qaros/tests/stub.rs` with its modules in `crates/qaros/tests/stub/`; a test
  that needs its own process (a leak, the library staying loaded) is its own test binary in
  `crates/qaros/tests/`.

## Tutorials and guide tabs

Both pull code from anchored regions (`// ANCHOR: <name>` to `// ANCHOR_END: <name>`, no
nesting or overlap) with `{{#include <path>:<name>}}` on its own line inside a fenced block, the
path relative to the source. Edit the sources and the code, never the output.

- **Tutorials:** `examples/tutorials/*.md` and `_category_.json`, regions in
  `examples/src/bin/*.rs` and `examples/src/lib.rs`. `cargo xtask tutorials` writes the pages to
  `OUTPUT_DIRECTORY` (`xtask/src/tutorials.rs`); `--check` fails on a missing, stale or
  left-over page, a broken include or anchor, and `<` or `{` in prose outside code (MDX).
- **Guide tabs:** `examples/guide/<page>.md`, one `## ` heading per C tab of the page in page
  order, regions in `examples/tests/guide/` (compiled with the tests, never run; a raw-only call
  goes through `library.raw_api()` with an `as_raw()` handle). A snippet of a function only one
  binding set has goes into a module that `examples/tests/guide/main.rs` declares from the
  generated function list, as `peers_identity.rs` is.
  `cargo xtask guide` puts each tab last in its `<CodeTabs>`, replacing a Rust tab already
  there, in the pages under `PAGES_DIRECTORY` (`xtask/src/guide.rs`); where that is `None` it
  only expands the sources. `--check` fails on a stale page; both modes fail when a page's C
  tabs and source headings differ in number, on a source without its page and on a page with a
  C tab but no source.
- Code changed inside a region changes the pages: rerun both. Prose that describes a changed
  call changes too, and each tutorial's account of what has run against QAROS stays true.

## The coverage table

The README's "Coverage" lists every area of the header (sections, and function groups inside
the larger ones): safe in Rust, raw only, and whether the C# binding (`qar-streaming-sharp/src`
in the QAROS repository) has it. Update it in the same change when a raw-only function gets a
wrapper, when the header adds or removes a function or area, or when a function moves between
the two header sets. Check Rust cells against `crates/qaros/src`, raw cells against the
generated function list, C# cells against the C# sources.

## Change checklist

### A header change

- [ ] Change `qar-streaming-c/include/qar_streaming.h`, then run `cargo xtask bindgen`, which
      regenerates every committed target. The header change and the regenerated bindings
      land in one commit.
- [ ] Read the diff of `crates/qaros-sys/src/generated/` for every target. A removed or renamed
      function breaks the build wherever the safe layer or the stub names it.
- [ ] New or changed `qar_*_default()`: port, `EXPORT_DEFAULT` line, `conformance!` pair
      (see [Init structures](#init-structures-defaults-and-headernext-chains)).
- [ ] Stub: behaviour for new functions the safe layer or a test calls; `behaviour.rs` follows
      changed signatures.
- [ ] Safe layer: enums and status codes (`types.rs`, `error.rs`), the wrappers that fill
      changed structures, and every behaviour change the header's text states.
- [ ] Each new function: wrapped (next list) or raw only and in the coverage table.
- [ ] Everything under [Formatting and checks](#formatting-and-checks).
- [ ] Docs: README (Use, Coverage, Loading the runtime, Tests), `docs/vulkan-api.md`,
      `docs/d3d11-api.md`, tutorials and guide tabs that show a changed call.
- [ ] Changelog: a reader-facing change to the developer guide (the Rust tutorials or tabs)
      gets a `- **Docs:**` entry under `## [Unreleased]` in the repository's root
      `CHANGELOG.md`; a change to the binding's public API goes to the Quaternar
      superproject's changelogs, from which that file is generated (the pull request
      template's "Changelog").

### Wrapping a raw-only function

- [ ] Its category (above) and the matching pattern.
- [ ] Ownership of every handle it takes or returns, from the header's text, and what releases
      it.
- [ ] Inits from their ported default; chained extensions alive while the library reads them.
- [ ] Present in both binding sets, else it stays raw (see [The raw layer](#the-raw-layer)).
- [ ] A method on the owning type (or a module in `crates/qaros/src`, re-exported from
      `lib.rs`) returning `qaros::Result`, its doc comment naming the C function and what the
      header leaves unsaid.
- [ ] Stub behaviour faithful to the header, with a fault control if the safe layer handles
      its failures.
- [ ] Stub tests: success, the C error path (`qar_stub_fail_next`), the teardown order of new
      handles, callback lifetime for async calls and subscriptions; an integration test in
      `tests/qar-streaming-rust-ext` where only the runtime can show it.
- [ ] Tutorials and guide tabs that call it raw only: the example or snippet and its source,
      then both generators.
- [ ] The coverage table row moves from raw only to safe.
- [ ] Docs and the changelog, as for a header change.

## Output expectations

- Code goes where its kind lives: raw items in `qaros-sys` (`defaults.rs`, `loader.rs`), one
  module per API area in `crates/qaros/src`, stub behaviour in `crates/qaros-stub/src`, stub
  tests in `crates/qaros/tests`, examples in `examples/src/bin`.
- Public types implement `Debug` and have a doc comment naming the C type or function they wrap,
  if any.
- Report what was verified and how (commands, tests, platforms), what ran only against the
  stub, and what is unknown or assumed about the C API. Never present a stub result as runtime
  behaviour.
