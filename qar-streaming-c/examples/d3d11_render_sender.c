/** \file d3d11_render_sender.c
 *  \brief Streams D3D11-rendered frames to a peer that requests them.
 *  \example d3d11_render_sender.c
 */

/** \page qar_c_tutorial_d3d11_rendering D3D11 Rendering
 * \tableofcontents
 *
 * \section d3d11_rendering_overview Overview
 * - Obtain a session as in earlier tutorials (rejoin or onboard)
 * - Create the app volume the stream is displayed in
 * - Wait for a peer (e.g. a Visualizer) to request a render stream
 * - Create a D3D11 device, chain it into the render sender with
 *   QarStreamParamsD3D11, and clear each colour view of the sender's shared
 *   textures directly on the GPU - no CPU copy
 *
 * \section d3d11_rendering_prereq Prerequisites
 * - Complete the \ref qar_c_tutorial_cpu_rendering tutorial; this one differs
 *   only in the graphics backend
 * - Windows with a D3D11.1-capable GPU
 * - A running QarOS hub (for the first run, read the pairing code off its
 *   onboarding screen), with a peer that requests a render stream from this
 *   application once it is running
 *
 * \section d3d11_rendering_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target d3d11_render_sender
 * ./build/x64-windows/Debug/d3d11_render_sender.exe
 * <installed|path-to-qar-streaming-c.dll> [runtime-dir] [pairing-code]
 * \endcode
 *
 * \section d3d11_rendering_setup Library, Runtime, Session, and App Volume
 * Argument parsing and session setup match the \ref
 * qar_c_tutorial_cpu_rendering tutorial. A render sender is displayed in an
 * app volume, so one is created before the sender.
 * \snippet d3d11_render_sender.c d3d11_volume
 *
 * \section d3d11_rendering_device Create the D3D11 Device
 * The sender needs the ID3D11Device1 / ID3D11DeviceContext1 interfaces, so the
 * device is created as usual and queried for them.
 * \snippet d3d11_render_sender.c d3d11_device
 *
 * \section d3d11_rendering_sender Create the D3D11 Render Sender
 * Select QAR_GRAPHICS_API_D3D11 and chain QarStreamParamsD3D11 into
 * QarRenderSenderInit::header.next. The bind flags decide how the shared
 * textures can be used; keep acquire_keyed_mutex_sync true so the library
 * acquires and releases each texture's keyed mutex around the frame cycle.
 * \snippet d3d11_render_sender.c d3d11_sender
 *
 * \section d3d11_rendering_frames Render and Submit Frames
 * Inside each begin-frame / show-frame cycle, qar_render_sender_frame_d3d11()
 * hands out the shared textures and the views laid out on them. Each colour
 * view is a pixel rectangle on one texture; here it is cleared to an animated
 * colour with ID3D11DeviceContext1::ClearView. A real renderer draws its scene
 * into the same rectangle, using the view poses from the frame info.
 * \snippet d3d11_render_sender.c d3d11_frames
 * The clear itself:
 * \snippet d3d11_render_sender.c d3d11_clear
 */

#define COBJMACROS
#include "common.h"

#include <windows.h>

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

#ifdef QAR_ENABLE_D3D11

QAR_IMPLEMENT_DYNAMIC_LOADING()

static void
print_usage(const char* program_name)
{
	const char* name = program_name ? program_name : "d3d11_render_sender";
	printf(
		"Usage: %s <installed|path-to-qar-streaming-c-library> "
		"[runtime-binaries-dir] "
		"[pairing-code]\n",
		name
	);
	printf(
		"The pairing code is required on the first run only; later runs "
		"rejoin with the persisted onboarding id.\n"
	);
}

typedef struct RenderRequestState
{
	volatile bool has_request;
	QarPeerId target_peer_id;
} RenderRequestState;

static void
on_render_request(QarRenderStreamRequest* request, void* user_state)
{
	RenderRequestState* state = (RenderRequestState*)user_state;

	QarPeerId target_peer_id = qar_peer_id_default();
	QarResult peer_result =
		qar_render_request_get_target_peer_id(request, &target_peer_id);
	log_result("qar_render_request_get_target_peer_id", peer_result);
	if(qar_result_is_success(peer_result) && !state->has_request)
	{
		state->target_peer_id = target_peer_id;
		state->has_request = true;
	}

	qar_render_request_handle_destroy(request);
}

typedef struct D3D11Objects
{
	ID3D11Device1* device;
	ID3D11DeviceContext1* context;
} D3D11Objects;

static void
release_d3d11(D3D11Objects* d3d)
{
	if(d3d->context)
	{
		ID3D11DeviceContext1_Release(d3d->context);
	}
	if(d3d->device)
	{
		ID3D11Device1_Release(d3d->device);
	}
	d3d->context = NULL;
	d3d->device = NULL;
}

//! [d3d11_device]
static bool
create_d3d11(D3D11Objects* out_d3d)
{
	const D3D_FEATURE_LEVEL levels[] = { D3D_FEATURE_LEVEL_11_1 };
	ID3D11Device* device = NULL;
	ID3D11DeviceContext* context = NULL;
	HRESULT hr = D3D11CreateDevice(
		NULL,
		D3D_DRIVER_TYPE_HARDWARE,
		NULL,
		D3D11_CREATE_DEVICE_BGRA_SUPPORT,
		levels,
		1,
		D3D11_SDK_VERSION,
		&device,
		NULL,
		&context
	);
	if(FAILED(hr))
	{
		fprintf(
			stderr, "D3D11CreateDevice failed (0x%08lX)\n", (unsigned long)hr
		);
		return false;
	}

	out_d3d->device = NULL;
	out_d3d->context = NULL;
	HRESULT device_hr = ID3D11Device_QueryInterface(
		device, &IID_ID3D11Device1, (void**)&out_d3d->device
	);
	HRESULT context_hr = ID3D11DeviceContext_QueryInterface(
		context, &IID_ID3D11DeviceContext1, (void**)&out_d3d->context
	);
	ID3D11DeviceContext_Release(context);
	ID3D11Device_Release(device);
	if(FAILED(device_hr) || FAILED(context_hr))
	{
		fprintf(stderr, "D3D11.1 interfaces not available\n");
		release_d3d11(out_d3d);
		return false;
	}
	return true;
}
//! [d3d11_device]

//! [d3d11_clear]
/** \brief Clear every colour view of the frame to an animated colour. */
static void
clear_color_views(
	ID3D11Device1* device,
	ID3D11DeviceContext1* context,
	const QarVideoFrameD3D11* frame,
	size_t frame_index
)
{
	const float t = (float)(frame_index % 120) / 120.0f;
	for(size_t i = 0; i < frame->texture_views_count; ++i)
	{
		const QarVideoFrameView* view = &frame->texture_views[i];
		if(view->data_type != QAR_VIDEO_FRAME_VIEW_TYPE_COLOR
		   || view->texture_index >= frame->textures_count)
		{
			continue;
		}

		ID3D11Texture2D* texture = frame->textures[view->texture_index].texture;
		ID3D11RenderTargetView* rtv = NULL;
		HRESULT hr = ID3D11Device1_CreateRenderTargetView(
			device, (ID3D11Resource*)texture, NULL, &rtv
		);
		if(FAILED(hr))
		{
			fprintf(
				stderr,
				"CreateRenderTargetView failed (0x%08lX)\n",
				(unsigned long)hr
			);
			continue;
		}

		/* Left eye red-ish, right eye blue-ish, both pulsing. */
		const bool right = view->eye == QAR_VIDEO_FRAME_VIEW_EYE_RIGHT;
		const float color[4] = {
			right ? 0.1f : t, 0.2f, right ? t : 0.1f, 1.f
		};
		const D3D11_RECT rect = { (LONG)view->start_x,
								  (LONG)view->start_y,
								  (LONG)view->end_x,
								  (LONG)view->end_y };
		ID3D11DeviceContext1_ClearView(
			context, (ID3D11View*)rtv, color, &rect, 1
		);
		ID3D11RenderTargetView_Release(rtv);
	}
}
//! [d3d11_clear]

static void
teardown(QarRuntime* runtime, QarSession* session)
{
	if(session)
	{
		/* Destroying the handle leaves the session. */
		qar_session_handle_destroy(session);
	}
	qar_runtime_destroy(runtime);
	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
}

int
main(int argc, char** argv)
{
	if(argc < 2)
	{
		print_usage(argv[0]);
		return 1;
	}

	const char* library_path = library_path_argument(argv[1]);
	const char* runtime_dir = NULL;
	char runtime_dir_buffer[1024] = { 0 };
	if(argc >= 3)
	{
		runtime_dir = argv[2];
	}
	else if(get_dir_from_path(
				library_path, runtime_dir_buffer, sizeof(runtime_dir_buffer)
			))
	{
		runtime_dir = runtime_dir_buffer;
	}
	const char* pairing_code = (argc >= 4) ? argv[3] : NULL;

	if(!qar_library_load(library_path))
	{
		fprintf(stderr, "Failed to load %s.\n", describe_library(library_path));
		return 2;
	}

	QarLibraryInit library_init = qar_library_init_default();
	library_init.enable_console_logging = true;
	QarResult library_result = qar_library_init(&library_init);
	if(qar_result_is_error(library_result))
	{
		log_result("qar_library_init", library_result);
		qar_library_unload();
		return 3;
	}

	QarRuntime* runtime = NULL;
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	runtime_init.runtime_binaries_folder_path = runtime_dir;
	QarResult runtime_result = qar_runtime_create(&runtime_init, &runtime);
	if(qar_result_is_error(runtime_result) || runtime == NULL)
	{
		log_result("qar_runtime_create", runtime_result);
		qar_library_destroy();
		qar_library_unload();
		return 4;
	}

	QarOnboardingId onboarding_id = qar_onboarding_id_default();
	QarSession* session = NULL;
	if(example_obtain_session(
		   runtime,
		   pairing_code,
		   "d3d11_render_sender.onboarding-id.txt",
		   "D3D11 Renderer",
		   &onboarding_id,
		   &session
	   ) != 0
	   || session == NULL)
	{
		teardown(runtime, NULL);
		return 5;
	}

	//! [d3d11_volume]
	QarAppVolumeInit volume_init = qar_app_volume_init_default();
	volume_init.common_name = "d3d11-volume.examples.qaros";
	volume_init.display_name = "D3D11 Tutorial Volume";
	volume_init.pose = qar_pose_default();
	volume_init.pose.position.z = -1.0f;
	volume_init.size = qar_app_volume_size_default();

	QarAppVolumeId volume_id = qar_app_volume_id_default();
	QarResult volume_result =
		qar_app_volumes_get_or_create(session, &volume_init, &volume_id);
	log_result("qar_app_volumes_get_or_create", volume_result);
	if(qar_result_is_error(volume_result))
	{
		teardown(runtime, session);
		return 6;
	}
	//! [d3d11_volume]

	RenderRequestState request_state = { false, qar_peer_id_default() };
	QarResult subscribe_result = qar_render_sender_subscribe_requests(
		session, on_render_request, &request_state, NULL
	);
	log_result("qar_render_sender_subscribe_requests", subscribe_result);
	if(qar_result_is_error(subscribe_result))
	{
		teardown(runtime, session);
		return 7;
	}
	printf("Waiting for a peer to request a render stream ...\n");
	while(!request_state.has_request)
	{
		Sleep(50);
	}

	D3D11Objects d3d = { NULL, NULL };
	if(!create_d3d11(&d3d))
	{
		teardown(runtime, session);
		return 8;
	}

	//! [d3d11_sender]
	QarStreamParamsD3D11 d3d11_params = qar_stream_params_d3d11_default();
	d3d11_params.d3d11_device = d3d.device;
	d3d11_params.d3d11_context = d3d.context;
	d3d11_params.color_bind_flags =
		D3D11_BIND_RENDER_TARGET | D3D11_BIND_SHADER_RESOURCE;
	d3d11_params.depth_bind_flags =
		D3D11_BIND_DEPTH_STENCIL | D3D11_BIND_SHADER_RESOURCE;
	d3d11_params.acquire_keyed_mutex_sync = true;

	QarRenderSenderInit sender_init = qar_render_sender_init_default();
	sender_init.graphics_api = QAR_GRAPHICS_API_D3D11;
	sender_init.peer_id = request_state.target_peer_id;
	sender_init.app_volume_id = &volume_id;
	sender_init.header.next = &d3d11_params.header;

	QarRenderSender* sender = NULL;
	QarResult sender_result =
		qar_render_sender_create(session, &sender_init, NULL, &sender);
	log_result("qar_render_sender_create", sender_result);
	if(qar_result_is_error(sender_result) || sender == NULL)
	{
		release_d3d11(&d3d);
		teardown(runtime, session);
		return 9;
	}
	//! [d3d11_sender]

	//! [d3d11_frames]
	const size_t frame_count = 300;
	for(size_t frame_index = 0; frame_index < frame_count; ++frame_index)
	{
		QarRenderFrameInfo* frame_info = NULL;
		QarResult begin_result =
			qar_render_sender_begin_frame(sender, NULL, &frame_info);
		if(qar_result_is_error(begin_result) || frame_info == NULL)
		{
			log_result("qar_render_sender_begin_frame", begin_result);
			break;
		}

		QarVideoFrameD3D11 frame = qar_video_frame_d3d11_default();
		QarResult frame_result = qar_render_sender_frame_d3d11(sender, &frame);
		if(qar_result_is_success(frame_result))
		{
			clear_color_views(d3d.device, d3d.context, &frame, frame_index);
		}
		else
		{
			log_result("qar_render_sender_frame_d3d11", frame_result);
		}

		/* Show closes the frame cycle that begin-frame opened. */
		QarRenderFrameShow show = qar_render_frame_show_default();
		show.rendered_near_far.near_plane = 0.1f;
		show.rendered_near_far.far_plane = 10.0f;
		QarResult show_result = qar_render_sender_show_frame(sender, &show);
		qar_render_frame_info_handle_destroy(frame_info);
		if(qar_result_is_error(show_result))
		{
			log_result("qar_render_sender_show_frame", show_result);
			break;
		}
	}

	qar_render_stream_handle_destroy(sender);
	//! [d3d11_frames]

	release_d3d11(&d3d);
	teardown(runtime, session);
	return 0;
}

#else // QAR_ENABLE_D3D11

int
main(void)
{
	printf("d3d11_render_sender requires Windows (QAR_ENABLE_D3D11).\n");
	return 0;
}

#endif // QAR_ENABLE_D3D11
