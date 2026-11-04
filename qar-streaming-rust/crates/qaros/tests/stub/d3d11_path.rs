//! The D3D11 render sender against `qaros-stub`, which emulates it without Direct3D: it checks
//! the chained `QarStreamParamsD3D11`, reports what it was given in its call log, and stands
//! in addresses for the shared textures and keyed mutexes. These tests therefore prove the
//! wrapper's side (the chain, the frame record it reads, the one-open-frame and graphics-API
//! refusals, the teardown order, the device references) and nothing of what QAROS does with a
//! real device. On Windows one test also runs the reference keeping on a real WARP device.

use qaros::d3d11::{D3D11Frame, D3D11StreamParams};
use qaros::qaros_sys as sys;
use qaros::{PixelFormat, RenderSender, RenderSenderConfig};

use super::*;

const COLOR_BIND_FLAGS: u32 = 0x28;
const DEPTH_BIND_FLAGS: u32 = 0x48;
const CALLBACK_DEADLINE: Duration = Duration::from_secs(5);

/// Stand-in device and context: addresses the stub records and never dereferences.
struct FakeDevice {
    device: Box<u64>,
    context: Box<u64>,
}

impl FakeDevice {
    fn new() -> Self {
        Self {
            device: Box::new(0),
            context: Box::new(0),
        }
    }

    fn device_address(&self) -> usize {
        std::ptr::from_ref(self.device.as_ref()) as usize
    }

    fn context_address(&self) -> usize {
        std::ptr::from_ref(self.context.as_ref()) as usize
    }

    fn stream_params(&self, acquire_keyed_mutex_sync: bool) -> D3D11StreamParams {
        // SAFETY: qaros-stub only records the pointers; the boxes outlive every sender of
        // these tests.
        let created = unsafe {
            D3D11StreamParams::from_raw(
                self.device_address() as *mut sys::ID3D11Device1,
                self.context_address() as *mut sys::ID3D11DeviceContext1,
            )
        };
        let mut params = created.expect("non-null stand-ins");
        params.color_bind_flags = COLOR_BIND_FLAGS;
        params.depth_bind_flags = DEPTH_BIND_FLAGS;
        params.acquire_keyed_mutex_sync = acquire_keyed_mutex_sync;
        params
    }

    fn creation_report(&self, acquire_keyed_mutex_sync: bool) -> String {
        format!(
            "d3d11 sender created textures=2 device={:#x} context={:#x} color_bind={COLOR_BIND_FLAGS:#x} depth_bind={DEPTH_BIND_FLAGS:#x} acquire_keyed_mutex_sync={acquire_keyed_mutex_sync}",
            self.device_address(),
            self.context_address()
        )
    }
}

fn create_d3d11_sender(
    session: &qaros::Session,
    config: &RenderSenderConfig,
    params: D3D11StreamParams,
) -> qaros::Result<RenderSender> {
    // SAFETY: the stub uses no context; the stand-ins outlive the sender (FakeDevice).
    unsafe { session.create_d3d11_render_sender(config, params, &BlockingOptions::default()) }
}

fn texture_addresses(frame: &D3D11Frame<'_>) -> Vec<usize> {
    frame
        .textures()
        .iter()
        .map(|texture| texture.as_raw() as usize)
        .collect()
}

fn keys(frame: &D3D11Frame<'_>) -> Vec<(u32, u32)> {
    frame
        .keyed_mutexes()
        .iter()
        .map(|keyed_mutex| (keyed_mutex.acquire_key, keyed_mutex.release_key))
        .collect()
}

#[test]
fn a_d3d11_sender_chains_its_parameters_and_lends_the_shared_textures() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let fake_device = FakeDevice::new();
    let mut sender = create_d3d11_sender(
        &session,
        &stereo_color_and_depth(VISUALIZER_PEER),
        fake_device.stream_params(true),
    )
    .expect("sender");
    assert!(
        fixture
            .control
            .calls()
            .contains(&fake_device.creation_report(true)),
        "{:?}",
        fixture.control.calls()
    );

    let before_first_frame = sender.d3d11_frame_state().expect("record before a frame");
    assert_eq!(before_first_frame.views().len(), 4);
    let shared_textures = texture_addresses(&before_first_frame);
    assert_eq!(shared_textures.len(), 2);
    assert!(shared_textures.iter().all(|address| *address != 0));
    assert_ne!(shared_textures[0], shared_textures[1]);
    let sizes: Vec<(u32, u32, PixelFormat)> = before_first_frame
        .textures()
        .iter()
        .map(|texture| (texture.size.width, texture.size.height, texture.size.format))
        .collect();
    assert_eq!(
        sizes,
        [
            (128, 64, PixelFormat::B8G8R8A8),
            (128, 64, PixelFormat::D32Float)
        ]
    );
    assert!(
        before_first_frame.keyed_mutexes().is_empty(),
        "the library drives the keyed mutexes"
    );

    let frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("begin");
    {
        let d3d11_frame = frame.d3d11_frame().expect("frame record");
        assert_eq!(texture_addresses(&d3d11_frame), shared_textures);
        let right_color = d3d11_frame
            .view_index(Eye::Right, ViewKind::Color)
            .expect("right color view");
        assert_eq!(right_color, 1);
        let right_view = d3d11_frame.views()[right_color];
        assert_eq!(
            d3d11_frame
                .texture(right_view.texture_index as usize)
                .map(|texture| texture.as_raw() as usize),
            Some(shared_textures[0])
        );
        let right_pose = frame.view(right_color).expect("right pose");
        assert!(right_pose.pose.position.x > 0.0);
    }
    let refused = sender.d3d11_frame_state();
    assert!(
        matches!(&refused, Err(Error::InvalidArgument(message)) if message.contains("read its D3D11 frame through the frame")),
        "{refused:?}"
    );
    assert!(matches!(
        sender.begin_owned_frame(&BlockingOptions::default()),
        Err(Error::InvalidArgument(_))
    ));
    frame
        .show(&ShowParams {
            near: 0.1,
            far: 10.0,
            depth_scale: 1.0,
        })
        .expect("show");

    drop(sender);
    drop(session);
    drop(runtime);
    drop(library);
    let calls = fixture.control.calls();
    assert!(
        calls.contains(&"d3d11 frame 0 shown".to_string()),
        "{calls:?}"
    );
    let shown = position_of(&calls, "render_sender_show_frame");
    let frame_info_destroyed = position_of(&calls, "render_frame_info_handle_destroy");
    let sender_destroyed = position_of(&calls, "render_stream_handle_destroy");
    let session_destroyed = position_of(&calls, "session_handle_destroy");
    let runtime_destroyed = position_of(&calls, RUNTIME_RELEASE_CALL);
    let library_destroyed = position_of(&calls, "library_destroy");
    assert!(shown < frame_info_destroyed);
    assert!(frame_info_destroyed < sender_destroyed);
    assert!(sender_destroyed < session_destroyed);
    assert!(session_destroyed < runtime_destroyed);
    assert!(runtime_destroyed < library_destroyed);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

#[test]
fn the_application_gets_the_keyed_mutexes_and_keys_of_each_frame_when_it_drives_them() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let fake_device = FakeDevice::new();
    let mut sender = create_d3d11_sender(
        &session,
        &stereo_color_and_depth(VISUALIZER_PEER),
        fake_device.stream_params(false),
    )
    .expect("sender");
    assert!(
        fixture
            .control
            .calls()
            .contains(&fake_device.creation_report(false))
    );

    let mut keys_per_frame = Vec::new();
    for _ in 0..2 {
        let frame = sender
            .begin_frame(&BlockingOptions::default())
            .expect("begin");
        let d3d11_frame = frame.d3d11_frame().expect("frame record");
        let keyed_mutexes = d3d11_frame.keyed_mutexes();
        assert_eq!(keyed_mutexes.len(), d3d11_frame.texture_count());
        let mutex_addresses: Vec<usize> = keyed_mutexes
            .iter()
            .map(|keyed_mutex| keyed_mutex.as_raw() as usize)
            .collect();
        assert!(mutex_addresses.iter().all(|address| *address != 0));
        assert!(
            mutex_addresses
                .iter()
                .all(|address| !texture_addresses(&d3d11_frame).contains(address))
        );
        keys_per_frame.push(keys(&d3d11_frame));
        frame.show(&ShowParams::default()).expect("show");
    }
    assert_eq!(
        keys_per_frame,
        [vec![(0, 1), (0, 1)], vec![(1, 2), (1, 2)]],
        "the stub's keys, read again every frame"
    );
    let next_frame = sender.d3d11_frame_state().expect("record between frames");
    assert_eq!(keys(&next_frame), [(2, 3), (2, 3)]);
    drop(sender);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn d3d11_access_is_refused_on_a_cpu_sender_without_calling_the_library() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let mut cpu_sender = session
        .create_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            &BlockingOptions::default(),
        )
        .expect("cpu sender");
    let refused = cpu_sender.d3d11_frame_state();
    assert!(
        matches!(&refused, Err(Error::InvalidArgument(message)) if message.contains("not created for D3D11")),
        "{refused:?}"
    );
    let frame = cpu_sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("begin");
    assert!(matches!(
        frame.d3d11_frame(),
        Err(Error::InvalidArgument(_))
    ));
    drop(frame);
    assert!(
        !fixture
            .control
            .calls()
            .iter()
            .any(|call| call == "render_sender_frame_d3d11"),
        "the wrapper refuses before the library is called"
    );

    #[cfg(feature = "vulkan")]
    {
        let fake_device = FakeDevice::new();
        let mut d3d11_sender = create_d3d11_sender(
            &session,
            &stereo_color_and_depth(VISUALIZER_PEER),
            fake_device.stream_params(true),
        )
        .expect("d3d11 sender");
        let refused = d3d11_sender.vulkan_frame_state();
        assert!(
            matches!(&refused, Err(Error::InvalidArgument(message)) if message.contains("not created for Vulkan")),
            "{refused:?}"
        );
    }
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn library_failures_of_the_d3d11_calls_carry_the_function_and_status() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (_runtime, session) = onboarded_session(&fixture, &library);
    let fake_device = FakeDevice::new();

    fixture.control.fail_next("render_sender_create", 7);
    let refused = create_d3d11_sender(
        &session,
        &stereo_color_and_depth(VISUALIZER_PEER),
        fake_device.stream_params(true),
    )
    .expect_err("injected creation failure");
    assert_eq!(
        refused.to_string(),
        "qar_render_sender_create failed with QAR_STATUS_LOGIC_ERROR (7): injected failure"
    );

    let mut sender = create_d3d11_sender(
        &session,
        &stereo_color_and_depth(VISUALIZER_PEER),
        fake_device.stream_params(true),
    )
    .expect("sender");
    fixture.control.fail_next("render_sender_frame_d3d11", 7);
    let refused = sender
        .d3d11_frame_state()
        .expect_err("injected frame failure");
    let Error::Api { function, .. } = &refused else {
        panic!("expected an API error, got {refused}");
    };
    assert_eq!(*function, "qar_render_sender_frame_d3d11");
    assert!(refused.has_code(StatusCode::LOGIC_ERROR));

    let frame = sender
        .begin_owned_frame(&BlockingOptions::default())
        .expect("the sender works after the failed read");
    assert_eq!(frame.d3d11_frame().expect("record").texture_count(), 2);
    drop(frame);
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
}

#[test]
fn a_d3d11_creation_started_without_waiting_delivers_and_a_dropped_one_is_cancelled() {
    let fixture = Fixture::new();
    fixture.control.set_render_sender_create_delay(50);
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let fake_device = FakeDevice::new();

    // SAFETY: as create_d3d11_sender; the stand-ins outlive the session.
    let mut pending = unsafe {
        session.start_d3d11_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            fake_device.stream_params(true),
            None,
            || {},
        )
    }
    .expect("start");
    assert!(pending.poll().is_none(), "the stub delays the callback");
    let deadline = Instant::now() + CALLBACK_DEADLINE;
    let sender = loop {
        if let Some(outcome) = pending.poll() {
            break outcome.expect("sender");
        }
        assert!(Instant::now() < deadline, "no callback");
        std::thread::sleep(Duration::from_millis(1));
    };
    assert_eq!(
        sender.d3d11_frame_state().expect("record").texture_count(),
        2
    );
    drop(sender);
    assert!(
        fixture
            .control
            .calls()
            .contains(&fake_device.creation_report(true))
    );

    // Long enough that the drop cancels the creation before the stub would create the sender.
    fixture.control.set_render_sender_create_delay(5_000);
    // SAFETY: as above.
    let abandoned = unsafe {
        session.start_d3d11_render_sender(
            &stereo_color_and_depth(VISUALIZER_PEER),
            fake_device.stream_params(true),
            None,
            || {},
        )
    }
    .expect("second start");
    drop(abandoned);
    // Waiting for the cancelled creation's callback here makes the log order below fixed: the
    // session's destroy would end it otherwise.
    super::async_create::wait_for_log_entry(
        &fixture.control,
        "render_sender_create_async cancelled",
    );
    drop(session);
    drop(runtime);
    drop(library);
    let calls = fixture.control.calls();
    let count_of = |entry: &str| calls.iter().filter(|call| *call == entry).count();
    assert_eq!(
        count_of("render_sender_create_async delivered"),
        1,
        "{calls:?}"
    );
    assert_eq!(count_of("render_stream_handle_destroy"), 1, "{calls:?}");
    assert_eq!(count_of(&fake_device.creation_report(true)), 1, "{calls:?}");
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}

/// The references `D3D11StreamParams::new` takes, counted on a real device: Direct3D's WARP
/// rasterizer, which every Windows has. The stub still emulates the sender; only the device
/// is real.
#[cfg(windows)]
mod real_device {
    use windows::Win32::Foundation::HMODULE;
    use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_WARP;
    use windows::Win32::Graphics::Direct3D11::{
        D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device,
        ID3D11Device1, ID3D11DeviceContext, ID3D11DeviceContext1,
    };
    use windows::core::{IUnknown, Interface};

    use super::*;

    fn warp_device() -> (ID3D11Device1, ID3D11DeviceContext1) {
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;
        // SAFETY: valid out-pointers; no adapter.
        unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_WARP,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                None,
                D3D11_SDK_VERSION,
                Some(&raw mut device),
                None,
                Some(&raw mut context),
            )
        }
        .expect("WARP device");
        (
            device.expect("device").cast().expect("ID3D11Device1"),
            context
                .expect("context")
                .cast()
                .expect("ID3D11DeviceContext1"),
        )
    }

    /// The object's reference count, left as it was: `Release` returns the count after it.
    fn reference_count<Object: Interface>(object: &Object) -> u32 {
        let raw = object.as_raw();
        // SAFETY: every COM interface pointer is also an IUnknown pointer.
        let unknown = unsafe { IUnknown::from_raw_borrowed(&raw) }.expect("a live object");
        // SAFETY: a live COM object; AddRef and Release are balanced.
        unsafe {
            (unknown.vtable().AddRef)(raw);
            (unknown.vtable().Release)(raw)
        }
    }

    #[test]
    fn the_sender_holds_the_device_and_context_until_it_and_its_session_are_gone() {
        let fixture = Fixture::new();
        let library = load_stub();
        let (_runtime, session) = onboarded_session(&fixture, &library);
        let (device, context) = warp_device();
        let device_references = reference_count(&device);
        let context_references = reference_count(&context);

        let params = D3D11StreamParams::new(&device, &context);
        assert_eq!(params.device_raw() as usize, device.as_raw() as usize);
        assert_eq!(reference_count(&device), device_references + 1);
        // SAFETY: the WARP context is used on this thread only.
        let sender = unsafe {
            session.create_d3d11_render_sender(
                &stereo_color_and_depth(VISUALIZER_PEER),
                params,
                &BlockingOptions::default(),
            )
        }
        .expect("sender");
        assert_eq!(reference_count(&device), device_references + 1);
        assert_eq!(reference_count(&context), context_references + 1);
        let record = sender.d3d11_frame_state().expect("record");
        assert!(
            record
                .textures()
                .iter()
                .all(|texture| texture.texture().is_some())
        );

        drop(sender);
        assert_eq!(reference_count(&device), device_references);
        assert_eq!(reference_count(&context), context_references);
        let report = format!(
            "d3d11 sender created textures=2 device={:#x} context={:#x} color_bind=0x0 depth_bind=0x0 acquire_keyed_mutex_sync=true",
            device.as_raw() as usize,
            context.as_raw() as usize
        );
        assert!(fixture.control.calls().contains(&report));
        assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    }
}

/// The async begin-frame hands out the same D3D11 frame record as the blocking one.
#[test]
fn an_async_begin_frame_lends_the_shared_textures() {
    let fixture = Fixture::new();
    let library = load_stub();
    let (runtime, session) = onboarded_session(&fixture, &library);
    let fake_device = FakeDevice::new();
    let mut sender = create_d3d11_sender(
        &session,
        &stereo_color_and_depth(VISUALIZER_PEER),
        fake_device.stream_params(true),
    )
    .expect("sender");
    let shared_textures = texture_addresses(&sender.d3d11_frame_state().expect("record"));

    let pending = sender.start_owned_frame(None, || {}).expect("start");
    let frame = pending.wait().expect("frame");
    assert_eq!(
        texture_addresses(&frame.d3d11_frame().expect("frame record")),
        shared_textures
    );
    frame
        .show(&ShowParams {
            near: 0.1,
            far: 10.0,
            depth_scale: 1.0,
        })
        .expect("show");
    drop(sender);
    drop(session);
    drop(runtime);
    drop(library);
    let calls = fixture.control.calls();
    assert!(
        calls.contains(&"d3d11 frame 0 shown".to_string()),
        "{calls:?}"
    );
    assert_eq!(fixture.errors_in_log(), Vec::<String>::new());
    assert_eq!(fixture.control.live_handle_count(), 0);
}
