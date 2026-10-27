//! QuaternAR's `d3d11_render_sender.c` sample in Rust, on the safe D3D11 render sender of
//! `qaros` (feature `d3d11`, module `qaros::d3d11`). D3D11 comes from the `windows` crate.
//! Windows only; elsewhere `main` says so and exits 0, like the C sample's non-D3D11 build.
//!
//! ```text
//! cargo run -p qaros-examples --bin d3d11_render_sender -- [pairing-code] [--frames=300]
//! ```
//!
//! The C sample's flow: a session (rejoin with the persisted onboarding id, else onboard with
//! the pairing code; with neither, unlike the C sample, join the QAROS Hub on this machine as a
//! local app), the "D3D11 Tutorial Volume" app volume, a D3D11.1 device, one render
//! sender with the device chained in `QarStreamParamsD3D11` (bind flags, the library acquiring
//! and releasing each texture's keyed mutex), then frames in
//! which every color view of the shared textures is cleared on the GPU to an animated color
//! with `ID3D11DeviceContext1::ClearView`: the left eye red-ish, the right eye blue-ish, both
//! pulsing. Depth views are left as the sender hands them out, as in the C sample.
//!
//! Where it follows this repository's conventions instead of the C sample:
//!
//! - The target is the first running target app in the peer list (the header's "Basic flow"),
//!   not the first render request.
//! - The device is created on the session's adapter (`Session::graphics_adapter`, matched by
//!   LUID), the header's step 4; the default hardware adapter when the session names none.
//! - The frame views are declared: color of both eyes side by side in texture 0, depth in
//!   texture 1.
//! - The depth textures are bound as depth-stencil only. The C sample adds shader resource,
//!   which QAROS's typed `DXGI_FORMAT_D32_FLOAT` depth textures cannot take.
//! - A paused, reconnecting or transfer-less stream, or an app volume that does not resolve
//!   yet, keeps the sender instead of ending the run.
//!
//! Against `qaros-stub`, whose D3D11 emulation lends placeholder addresses instead of textures,
//! the frames are submitted without drawing.

// ANCHOR: main
#[cfg(windows)]
fn main() -> std::process::ExitCode {
    qaros_examples::report(d3d11::run())
}

#[cfg(not(windows))]
fn main() {
    println!("d3d11_render_sender requires Windows (QAR_ENABLE_D3D11).");
}
// ANCHOR_END: main

#[cfg(windows)]
mod d3d11 {
    use std::path::Path;
    use std::time::Duration;

    use qaros::d3d11::{D3D11Frame, D3D11StreamParams};
    use qaros::{
        AppVolumeConfig, BlockingOptions, Eye, FrameLayout, FrameViewSpec, GraphicsAdapter,
        PeerInfo, Pose, RenderSender, RenderSenderConfig, Session, ShowParams, Vec3, ViewKind,
    };
    use qaros_examples::{
        Arguments, ExampleResult, create_runtime, frame_limit, is_running_target, is_waiting,
        join_session, load_library,
    };
    use windows::Win32::Foundation::{HMODULE, RECT};
    use windows::Win32::Graphics::Direct3D::{
        D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_1,
    };
    use windows::Win32::Graphics::Direct3D11::{
        D3D11_BIND_DEPTH_STENCIL, D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE,
        D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device,
        ID3D11Device1, ID3D11DeviceContext, ID3D11DeviceContext1, ID3D11RenderTargetView,
    };
    use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIAdapter, IDXGIFactory1};
    use windows::core::Interface as _;

    const DEFAULT_FRAME_COUNT: usize = 300;
    const ID_FILE: &str = "d3d11_render_sender.onboarding-id.txt";
    const DISPLAY_NAME: &str = "D3D11 Renderer";
    /// The C sample polls for its render request every 50 ms; the peer list is polled likewise.
    const TARGET_POLL_INTERVAL: Duration = Duration::from_millis(50);
    /// The C sample's animation period, in frames.
    const PULSE_FRAMES: usize = 120;
    const NEAR_PLANE: f32 = 0.1;
    const FAR_PLANE: f32 = 10.0;

    // ANCHOR: run
    pub fn run() -> ExampleResult {
        let arguments = Arguments::parse();
        let frame_limit = frame_limit(&arguments, DEFAULT_FRAME_COUNT)?;

        let library = load_library()?;
        let runtime = create_runtime(&library, None)?;
        let session = join_session(
            &library,
            &runtime,
            arguments.pairing_code.as_deref(),
            Path::new(ID_FILE),
            DISPLAY_NAME,
        )?;

        let mut volume_config = AppVolumeConfig::new("d3d11-volume.examples.qaros");
        volume_config.display_name = Some("D3D11 Tutorial Volume".to_string());
        volume_config.pose = Pose::from_position(Vec3::new(0.0, 0.0, -1.0));
        let volume = session.app_volumes().get_or_create(&volume_config)?;

        println!("Waiting for a target app in the peer list ...");
        let target = wait_for_target(&session)?;

        let (device, context) = create_d3d11(session.graphics_adapter()?)?;
        let mut config = RenderSenderConfig::new(target.id);
        config.layout = FrameLayout::SideBySide;
        config.views = FRAME_VIEWS.to_vec();
        config.app_volume = Some(volume);
        let mut sender = create_sender(&session, &config, &device, &context)?;
        println!(
            "Rendering for target {} (peer {})",
            target.display_name, target.id
        );
        // qaros-stub lends placeholder addresses instead of D3D11 textures; Direct3D crashes on
        // them, so against the stub the frames go out undrawn.
        let draws = library.runtime_info()?.flavor != "stub";
        if !draws {
            println!("qaros-stub lends no D3D11 textures: submitting the frames without drawing.");
        }

        let mut frames_shown = 0_usize;
        for frame_index in 0.. {
            if frames_shown >= frame_limit {
                break;
            }
            match render_frame(&mut sender, &device, &context, frame_index, draws) {
                Ok(shown) => frames_shown += usize::from(shown),
                Err(error) => {
                    println!("{error}");
                    break;
                }
            }
        }
        println!("Submitted {frames_shown} frames.");
        // The sender goes first and releases its own references on the device and context;
        // then this function's, the session, the runtime and the library: the C sample's
        // teardown order.
        drop(sender);
        drop(context);
        drop(device);
        drop(session);
        drop(runtime);
        drop(library);
        Ok(())
    }
    // ANCHOR_END: run

    // ANCHOR: targets
    /// The first running target app the session lists, polled until one is there.
    fn wait_for_target(session: &Session) -> qaros::Result<PeerInfo> {
        loop {
            if let Some(target) = session.peers()?.into_iter().find(is_running_target) {
                return Ok(target);
            }
            std::thread::sleep(TARGET_POLL_INTERVAL);
        }
    }
    // ANCHOR_END: targets

    // ANCHOR: device
    /// The DXGI adapter whose LUID is the session's, compared byte for byte in memory order (the
    /// order `GraphicsAdapter::Luid` holds).
    fn adapter_with_luid(luid: [u8; 8]) -> ExampleResult<IDXGIAdapter> {
        // SAFETY: plain factory creation.
        let factory: IDXGIFactory1 = unsafe { CreateDXGIFactory1() }?;
        for index in 0.. {
            // SAFETY: live factory; enumeration ends with DXGI_ERROR_NOT_FOUND.
            let Ok(adapter) = (unsafe { factory.EnumAdapters1(index) }) else {
                break;
            };
            // SAFETY: live adapter.
            let adapter_luid = unsafe { adapter.GetDesc1() }?.AdapterLuid;
            let mut adapter_bytes = [0_u8; 8];
            adapter_bytes[..4].copy_from_slice(&adapter_luid.LowPart.to_le_bytes());
            adapter_bytes[4..].copy_from_slice(&adapter_luid.HighPart.to_le_bytes());
            if adapter_bytes == luid {
                return Ok(adapter.cast()?);
            }
        }
        Err(format!(
            "the session's adapter (LUID {}) is not among this machine's DXGI adapters",
            GraphicsAdapter::Luid(luid)
        )
        .into())
    }

    /// A D3D11.1 device and immediate context on the session's adapter when it names one by
    /// LUID, else on the default hardware adapter (the C sample's `create_d3d11`).
    fn create_d3d11(
        session_adapter: Option<GraphicsAdapter>,
    ) -> ExampleResult<(ID3D11Device1, ID3D11DeviceContext1)> {
        let adapter = match session_adapter {
            Some(GraphicsAdapter::Luid(luid)) => Some(adapter_with_luid(luid)?),
            Some(GraphicsAdapter::Uuid(_)) | None => None,
        };
        let driver_type: D3D_DRIVER_TYPE = if adapter.is_some() {
            D3D_DRIVER_TYPE_UNKNOWN
        } else {
            D3D_DRIVER_TYPE_HARDWARE
        };
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;
        // SAFETY: valid out-pointers; the adapter, when given, is live.
        unsafe {
            D3D11CreateDevice(
                adapter.as_ref(),
                driver_type,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&[D3D_FEATURE_LEVEL_11_1]),
                D3D11_SDK_VERSION,
                Some(&raw mut device),
                None,
                Some(&raw mut context),
            )
        }
        .map_err(|error| format!("D3D11CreateDevice failed ({error})"))?;
        let (Some(device), Some(context)) = (device, context) else {
            return Err("D3D11CreateDevice returned no device or context".into());
        };
        let device1 = device
            .cast::<ID3D11Device1>()
            .map_err(|_| "D3D11.1 interfaces not available")?;
        let context1 = context
            .cast::<ID3D11DeviceContext1>()
            .map_err(|_| "D3D11.1 interfaces not available")?;
        Ok((device1, context1))
    }
    // ANCHOR_END: device

    // ANCHOR: create_sender
    /// Color of both eyes side by side in texture 0, depth side by side in texture 1.
    const FRAME_VIEWS: [FrameViewSpec; 4] = [
        FrameViewSpec {
            kind: ViewKind::Color,
            eye: Eye::Left,
            texture_index: 0,
        },
        FrameViewSpec {
            kind: ViewKind::Color,
            eye: Eye::Right,
            texture_index: 0,
        },
        FrameViewSpec {
            kind: ViewKind::Depth,
            eye: Eye::Left,
            texture_index: 1,
        },
        FrameViewSpec {
            kind: ViewKind::Depth,
            eye: Eye::Right,
            texture_index: 1,
        },
    ];

    fn create_sender(
        session: &Session,
        config: &RenderSenderConfig,
        device: &ID3D11Device1,
        context: &ID3D11DeviceContext1,
    ) -> qaros::Result<RenderSender> {
        let mut d3d11 = D3D11StreamParams::new(device, context);
        d3d11.color_bind_flags = (D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE)
            .0
            .cast_unsigned();
        // QAROS creates the depth textures as DXGI_FORMAT_D32_FLOAT, a typed depth format that
        // D3D11 cannot bind as a shader resource: with D3D11_BIND_SHADER_RESOURCE added,
        // CreateTexture2D, and with it sender creation, fails with E_INVALIDARG.
        d3d11.depth_bind_flags = D3D11_BIND_DEPTH_STENCIL.0.cast_unsigned();
        d3d11.acquire_keyed_mutex_sync = true;
        // SAFETY: this example uses the immediate context on this thread only, the thread that
        // drives the session and the sender.
        let sender = unsafe {
            session.create_d3d11_render_sender(config, d3d11, &BlockingOptions::default())
        }?;
        println!("qar_render_sender_create: SUCCESS");
        Ok(sender)
    }
    // ANCHOR_END: create_sender

    // ANCHOR: render_frame
    /// One frame: begin, clear the color views when `draws`, show. False when the stream is
    /// waiting and no frame was shown.
    fn render_frame(
        sender: &mut RenderSender,
        device: &ID3D11Device1,
        context: &ID3D11DeviceContext1,
        frame_index: usize,
        draws: bool,
    ) -> qaros::Result<bool> {
        let frame = match sender.begin_frame(&BlockingOptions::default()) {
            Ok(frame) => frame,
            Err(error) if is_waiting(&error) => return Ok(false),
            Err(error) => return Err(error),
        };
        match frame.d3d11_frame() {
            Ok(d3d11_frame) if draws => {
                clear_color_views(device, context, &d3d11_frame, frame_index);
            }
            Ok(_) => {}
            Err(error) => println!("{error}"),
        }
        // Show closes the frame cycle that begin-frame opened.
        let shown = frame.show(&ShowParams {
            near: NEAR_PLANE,
            far: FAR_PLANE,
            depth_scale: 1.0,
        });
        match shown {
            Ok(()) => Ok(true),
            Err(error) if is_waiting(&error) => Ok(false),
            Err(error) => Err(error),
        }
    }
    // ANCHOR_END: render_frame

    // ANCHOR: clear_color_views
    /// The C sample's `clear_color_views`: each color view cleared to an animated color, the
    /// left eye red-ish, the right eye blue-ish, both pulsing.
    #[allow(clippy::cast_precision_loss)]
    fn clear_color_views(
        device: &ID3D11Device1,
        context: &ID3D11DeviceContext1,
        frame: &D3D11Frame<'_>,
        frame_index: usize,
    ) {
        let pulse = (frame_index % PULSE_FRAMES) as f32 / PULSE_FRAMES as f32;
        for view in frame.views() {
            if view.kind != ViewKind::Color {
                continue;
            }
            let Some(shared_texture) = frame.texture(view.texture_index as usize) else {
                continue;
            };
            // Lent by the sender for this frame: no AddRef, no Release.
            let Some(texture) = shared_texture.texture() else {
                continue;
            };
            let mut render_target: Option<ID3D11RenderTargetView> = None;
            // SAFETY: live device and texture; valid out-pointer.
            if let Err(error) = unsafe {
                device.CreateRenderTargetView(texture, None, Some(&raw mut render_target))
            } {
                eprintln!("CreateRenderTargetView failed ({error})");
                continue;
            }
            let Some(render_target) = render_target else {
                continue;
            };
            let right_eye = view.eye == Eye::Right;
            let color = [
                if right_eye { 0.1 } else { pulse },
                0.2,
                if right_eye { pulse } else { 0.1 },
                1.0,
            ];
            let (Ok(left), Ok(top), Ok(right), Ok(bottom)) = (
                i32::try_from(view.start_x),
                i32::try_from(view.start_y),
                i32::try_from(view.end_x),
                i32::try_from(view.end_y),
            ) else {
                continue;
            };
            let rectangle = RECT {
                left,
                top,
                right,
                bottom,
            };
            // SAFETY: live context and render-target view; the rectangle lies on the texture.
            unsafe { context.ClearView(&render_target, &color, Some(&[rectangle])) };
        }
    }
    // ANCHOR_END: clear_color_views
}
