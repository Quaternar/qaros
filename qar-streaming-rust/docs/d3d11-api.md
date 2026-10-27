# The D3D11 path in `qaros`

The Rust API over QAROS's D3D11 render sender (header `qar_streaming.h`, `QarStreamParamsD3D11` and `qar_render_sender_frame_d3d11`; example `qar-streaming-c/examples/d3d11_render_sender.c`). Enable it with the `d3d11` feature:

```toml
qaros = { path = "<path to the binding>/crates/qaros", features = ["d3d11"] }
```

Handles are `windows` crate 0.62 types (`ID3D11Device1`, `ID3D11DeviceContext1`, `ID3D11Texture2D`, `IDXGIKeyedMutex`), the version the examples use.

## Where it compiles

The header enables its D3D11 types and functions on Windows only (`QAR_ENABLE_D3D11` for every `_WIN32` build). The feature follows the bindings:

- **Windows:** the whole module, with the `windows` crate types.
- **macOS:** the module without the items that take or lend `windows` crate types, which does not build there. macOS has no QAROS runtime; its bindings exist for `qaros-stub`, and `cargo xtask bindgen` generates them with `QAR_ENABLE_D3D11` so that the stub can emulate the sender and the tests below run without Windows.
- **Linux:** nothing. The Linux runtime's header has no D3D11, so neither have its bindings.

The feature compiles to nothing rather than failing to build: Cargo features are additive and unified across a workspace, so a crate that enables `qaros/d3d11` for a Windows-only path (as `qaros-examples` does) still builds on Linux, and `--all-features` works everywhere. Code that names `qaros::d3d11` belongs behind `#[cfg(windows)]`.

## Types and functions

| Item | What it does |
| --- | --- |
| `d3d11::D3D11StreamParams::new(&ID3D11Device1, &ID3D11DeviceContext1)` (Windows) | `QarStreamParamsD3D11` with the header's defaults (`qar_stream_params_d3d11_default`: no bind flags, `acquire_keyed_mutex_sync` true). Takes a reference (`AddRef`) on the device and the context. |
| `unsafe d3d11::D3D11StreamParams::from_raw(*mut ID3D11Device1, *mut ID3D11DeviceContext1) -> Option<Self>` | The same from raw pointers (another D3D11 binding, a test). `None` for a null pointer. No reference taken. |
| `D3D11StreamParams { color_bind_flags: u32, depth_bind_flags: u32, acquire_keyed_mutex_sync: bool }` | The header's fields: `D3D11_BIND_FLAG` bits of the color and depth textures, and who drives the keyed mutexes. `device_raw()` and `context_raw()` return the pointers. |
| `unsafe Session::create_d3d11_render_sender(&RenderSenderConfig, D3D11StreamParams, &BlockingOptions) -> Result<RenderSender>` | `qar_render_sender_create` with `QAR_GRAPHICS_API_D3D11` and the parameters chained. Same `RenderSender` type as the CPU and Vulkan paths. |
| `unsafe Session::start_d3d11_render_sender(&RenderSenderConfig, D3D11StreamParams, Option<Duration>, wake) -> Result<PendingRenderSender>` | `qar_render_sender_create_async`, polled like the other senders. |
| `RenderSender::d3d11_frame_state(&self) -> Result<D3D11Frame<'_>>` | The frame record outside a frame cycle, readable from creation (create views up front). Fails while a frame is open, and on a sender of another API. |
| `OwnedFrame::d3d11_frame(&self)`, `Frame::d3d11_frame(&self) -> Result<D3D11Frame<'_>>` | The frame record inside a cycle (`qar_render_sender_frame_d3d11`). Borrows the frame, so it ends before `show`. |
| `D3D11Frame::views() -> Vec<VideoFrameView>`, `view_index(Eye, ViewKind) -> Option<usize>` | Views per eye and color or depth; the index also selects the pose and field of view (`frame.view(index)`). |
| `D3D11Frame::textures() -> Vec<D3D11Texture<'_>>`, `texture(index)`, `texture_count()` | The shared textures (`QarVideoTextureD3D11`), in `texture_index` order. |
| `D3D11Texture { size: TextureSize }`, `as_raw() -> *mut ID3D11Texture2D`, `texture() -> Option<&ID3D11Texture2D>` (Windows) | One texture, lent for the borrow of the frame: no `AddRef`, no `Release`. |
| `D3D11Frame::keyed_mutexes() -> Vec<D3D11KeyedMutex<'_>>` | `QarSyncFrameD3D11`: one entry per texture when the application drives the keyed mutexes; empty when no external synchronization is needed. |
| `D3D11KeyedMutex { acquire_key: u32, release_key: u32 }`, `as_raw()`, `mutex() -> Option<&IDXGIKeyedMutex>` (Windows) | The keyed mutex of one texture and the keys for `AcquireSync` and `ReleaseSync`. |
| `D3D11Frame::as_raw() -> &QarVideoFrameD3D11` | The record as the library wrote it, for C helpers. |

The raw `qar_render_sender_frame_d3d11` and `qar_video_sender_frame_d3d11` stay reachable through `library.raw_api()`.

## Safety rules

The wrapper enforces what a borrow or a reference count can express and documents the rest. Sources: the header (`QarStreamParamsD3D11`, `QarSyncFrameD3D11`, `qar_render_sender_frame_d3d11`, `QarGraphicsDeviceId`) and the C sample.

- **Device lifetime (enforced).** Parameters from `D3D11StreamParams::new` hold a reference on the device and the context. The sender keeps its own until it is destroyed; when it held the last reference to its session, the session is destroyed first, then the references are released. A creation started with `start_d3d11_render_sender` keeps them until its sender is taken; if its `PendingRenderSender` is dropped first, the session keeps them until the callback comes, and when it never comes they are never released. Parameters from `from_raw` carry no reference, so the caller keeps the pointers valid that long; that is part of `from_raw`'s contract.
- **The immediate context (the caller's).** QAROS uses the context it is given. When, and on which thread, the header does not say, and a D3D11 immediate context is not thread-safe. This is why creating the sender is `unsafe`: use the context only on the thread that drives the session and its senders (every `qaros` handle stays on one thread anyway), or turn on the device's multithread protection (`ID3D11Multithread::SetMultithreadProtected`).
- **Ownership of the textures (enforced).** The shared textures and keyed mutexes are the sender's. `D3D11Texture::texture` and `D3D11KeyedMutex::mutex` lend them for the borrow of the `D3D11Frame`, which borrows the sender or the open frame, so they cannot be used after `show` or the sender's destruction through the lent reference. Clone the interface (`AddRef`) to keep one longer; the object then lives as long as that clone, as COM objects do.
- **Read the record every cycle (the caller's).** The header gives the record per call and does not say the textures stay the same between cycles; `qar_render_sender_change_layout` changes the layout of later frames. Views and textures created up front from `d3d11_frame_state` are valid only while the record names the same textures.
- **Writes only inside a frame cycle (the caller's).** The header: "Readable before the first begin-frame; writable only inside a frame cycle." `d3d11_frame_state` is for reading and creating views; render between a begin and its `show`. The wrapper cannot see GPU work.
- **Keyed mutexes (the caller's).** With `acquire_keyed_mutex_sync` true (the header's default) the library acquires and releases each texture's keyed mutex around the frame cycle; leave them alone. With it false the application drives the chain: `AcquireSync(acquire_key)` before rendering and `ReleaseSync(release_key)` after, for each entry, inside the cycle. The header says the entries match the textures and an empty list means no external synchronization; it does not say which keys to expect.
- **One open frame (enforced).** As on every path: one frame per sender between begin and show or drop; `d3d11_frame_state` fails while a frame is open.
- **The device (documented).** A D3D11.1 device (`ID3D11Device1`, `ID3D11DeviceContext1`) on the session's adapter (`Session::graphics_adapter`): the header creates the session's devices, and with them every sender's frame textures, on that adapter. The wrapper does not check the adapter.

## One frame

```rust
use qaros::d3d11::D3D11StreamParams;
use qaros::{BlockingOptions, Eye, RenderSenderConfig, ShowParams, ViewKind};
use windows::Win32::Graphics::Direct3D11::{D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE};

// Session onboarded with runtime.onboard(..); device and context created on
// session.graphics_adapter()?, the LUID matched against the DXGI adapters.
let mut d3d11 = D3D11StreamParams::new(&device, &context);
d3d11.color_bind_flags = (D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE).0.cast_unsigned();
let mut config = RenderSenderConfig::new(target_peer);
config.app_volume = Some(volume_id);
// SAFETY: the immediate context is used on this thread only.
let mut sender = unsafe { session.create_d3d11_render_sender(&config, d3d11, &BlockingOptions::default())? };

let frame = sender.begin_frame(&BlockingOptions::default())?;
let d3d11_frame = frame.d3d11_frame()?;
if let Some(left) = d3d11_frame.view_index(Eye::Left, ViewKind::Color) {
    let view = d3d11_frame.views()[left];
    let pose = frame.view(left)?;
    if let Some(texture) = d3d11_frame.texture(view.texture_index as usize).and_then(|shared| shared.texture()) {
        // Create a render-target view on `texture` and draw the left eye with `pose` into
        // view.start_x..view.end_x, view.start_y..view.end_y.
    }
}
frame.show(&ShowParams { near: 0.1, far: 10.0, depth_scale: 1.0 })?;
```

`examples/src/bin/d3d11_render_sender.rs` is the complete program: the device on the session's adapter, the sender, and a `ClearView` of every color view per frame.

## Against `qaros-stub`

The stub emulates the D3D11 sender without Direct3D, on Windows and macOS alike:

- At sender creation it checks the chained extension: `QAR_STRUCTURE_TYPE_STREAM_D3D11_PARAMS_EXT`, its `header.next` null, device and context non-null. It logs `d3d11 sender created textures=N device=0x... context=0x... color_bind=0x... depth_bind=0x... acquire_keyed_mutex_sync=...` and creates one stand-in per layout texture for the texture and one for its keyed mutex: plain heap allocations, not COM objects.
- `qar_render_sender_frame_d3d11` fills the record with the sender's views and texture sizes (as on the CPU path) and the stand-ins' addresses. With `acquire_keyed_mutex_sync` true it reports no synchronization entry; with it false one per texture, with keys of the stub's own (acquire = the frame's index, release = the index plus one), so that a test sees the record read again each frame. It refuses the call on a sender of another API.
- `show_frame` logs `d3d11 frame N shown`; there are no pixels to check.

What it proves: the wrapper chains a stamped `QarStreamParamsD3D11` with the pointers, flags and keyed-mutex choice it was given and selects the D3D11 API; blocking and polled creation; reading the record into views, textures and keyed mutexes, frame by frame; the refusals (a second open frame, the record outside a cycle while a frame is open, D3D11 access on a CPU sender without a library call, Vulkan access on a D3D11 sender); errors carrying the function and status; the teardown order (frame info after show, then sender, session, runtime, library). On Windows one test runs the reference keeping on a real WARP device: the parameters take one reference on the device and one on the context, the sender holds them while it lives, and they are released when it is dropped.

What it does not prove: that QAROS accepts the device or creates shared textures on it, that the textures are usable as render targets or reach a receiver, which bind flags QAROS needs, the keyed-mutex keys QAROS uses and whether the list is empty when the library drives them, when and on which threads QAROS uses the context, and whether textures change between frames. A stand-in texture or keyed mutex must never be called through: `texture()` and `mutex()` lend them as `windows` types on Windows, and any method call would crash. The stub tests only compare addresses.
