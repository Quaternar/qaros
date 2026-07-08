/** \file cpu_rendering_visualizer.c
 *  \brief Waits for a render stream request and submits CPU-rendered frames.
 *  \example cpu_rendering_visualizer.c
 */

/** \page qar_c_tutorial_cpu_rendering CPU Rendering with a Visualizer
 * \tableofcontents
 *
 * \section cpu_rendering_overview Overview
 * - Obtain a session as in earlier tutorials (rejoin or onboard)
 * - Subscribe to incoming render stream requests and wait for a peer (e.g. a
 *   Visualizer) to request one
 * - Create a CPU render sender for the requesting peer and submit a short
 *   gradient animation
 *
 * \section cpu_rendering_prereq Prerequisites
 * - Complete the \ref qar_c_tutorial_gui_panels tutorial
 * - A running QarOS hub (for the first run, read the pairing code off its
 *   onboarding screen), with a peer that requests a render stream from this
 *   application once it is running
 *
 * \section cpu_rendering_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target cpu_rendering_visualizer
 * ./build/x64-windows/Debug/cpu_rendering_visualizer.exe
 * <path-to-qar-streaming-c.dll> [runtime-dir] [pairing-code]
 * \endcode
 *
 * \section cpu_rendering_args Parse Arguments
 * \snippet cpu_rendering_visualizer.c cpu_args
 *
 * \section cpu_rendering_setup Library, Runtime, and Session Setup
 * The session is obtained with the shared rejoin-or-onboard helper described
 * in the \ref qar_c_tutorial_onboarding tutorial.
 * \snippet cpu_rendering_visualizer.c cpu_setup
 *
 * \section cpu_rendering_request Wait for a Render Stream Request
 * \snippet cpu_rendering_visualizer.c cpu_request
 *
 * \section cpu_rendering_frames Render and Submit Frames
 * \snippet cpu_rendering_visualizer.c cpu_frames
 */

#include "common.h"

#include <windows.h>

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

static void
print_usage(const char* program_name)
{
	const char* name = program_name ? program_name : "cpu_rendering_visualizer";
	printf(
		"Usage: %s <path-to-qar-streaming-c-library> [runtime-binaries-dir] "
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
	bool has_request;
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

	QarStreamId stream_id = qar_stream_id_default();
	log_result(
		"qar_render_request_get_stream_id",
		qar_render_request_get_stream_id(request, &stream_id)
	);

	if(qar_result_is_success(peer_result) && !state->has_request)
	{
		printf("Render stream requested by peer ");
		print_hex_id(target_peer_id.data, QAR_MAX_ID_LENGTH);
		printf("\n");
		state->target_peer_id = target_peer_id;
		state->has_request = true;
	}

	qar_render_request_handle_destroy(request);
}

static void
render_gradient(QarVideoFrameCpu* frame, size_t frame_index)
{
	if(!frame)
	{
		return;
	}

	for(size_t tex_idx = 0; tex_idx < frame->textures_count; ++tex_idx)
	{
		QarVideoTextureCpu* texture = &frame->textures[tex_idx];
		if(!texture->texture_data || texture->size.width == 0
		   || texture->size.height == 0)
		{
			continue;
		}

		const uint32_t width = texture->size.width;
		const uint32_t height = texture->size.height;
		const uint32_t pitch = texture->pitch;

		for(uint32_t y = 0; y < height; ++y)
		{
			uint8_t* row = texture->texture_data + y * pitch;
			for(uint32_t x = 0; x < width; ++x)
			{
				const size_t pixel_index = x * 4;
				row[pixel_index + 0] = (uint8_t)((x + frame_index * 8) % 256);
				row[pixel_index + 1] = (uint8_t)((y + frame_index * 16) % 256);
				row[pixel_index + 2] = (uint8_t)((frame_index * 32) % 256);
				row[pixel_index + 3] = 255;
			}
		}
	}
}

int
main(int argc, char** argv)
{
	//! [cpu_args]
	if(argc < 2)
	{
		print_usage(argv[0]);
		return 1;
	}

	const char* library_path = argv[1];
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
	//! [cpu_args]

	//! [cpu_setup]
	if(!qar_library_load(library_path))
	{
		fprintf(stderr, "Failed to load '%s'.\n", library_path);
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
		   "cpu_rendering_visualizer.onboarding-id.txt",
		   "CPU Renderer",
		   &onboarding_id,
		   &session
	   ) != 0
	   || session == NULL)
	{
		qar_runtime_destroy(runtime);
		qar_library_destroy();
		qar_library_unload();
		return 5;
	}
	//! [cpu_setup]

	//! [cpu_request]
	RenderRequestState request_state = { false, qar_peer_id_default() };
	QarResult subscribe_result = qar_render_sender_subscribe_requests(
		session, on_render_request, &request_state, NULL
	);
	log_result("qar_render_sender_subscribe_requests", subscribe_result);

	printf(
		"Waiting for a peer to request a render stream (e.g. open a "
		"Visualizer on the hub) ...\n"
	);
	while(!request_state.has_request)
	{
		Sleep(50);
	}
	//! [cpu_request]

	//! [cpu_frames]
	QarRenderSenderInit sender_init = qar_render_sender_init_default();
	sender_init.graphics_api = QAR_GRAPHICS_API_CPU;
	sender_init.peer_id = request_state.target_peer_id;

	QarRenderSender* sender = NULL;
	QarResult sender_result =
		qar_render_sender_create(session, &sender_init, NULL, &sender);
	if(qar_result_is_error(sender_result) || sender == NULL)
	{
		log_result("qar_render_sender_create", sender_result);
		log_result("qar_session_leave", qar_session_leave(session));
		qar_session_handle_destroy(session);
		qar_runtime_destroy(runtime);
		qar_library_destroy();
		qar_library_unload();
		return 6;
	}
	log_result("qar_render_sender_create", sender_result);

	QarVideoFrameLayout layout = qar_video_frame_layout_default();
	QarResult layout_result = qar_render_sender_layout(sender, &layout);
	log_result("qar_render_sender_layout", layout_result);

	const size_t frame_count = 3;
	for(size_t frame_index = 0; frame_index < frame_count; ++frame_index)
	{
		QarRenderFrameInfo* frame_info = NULL;
		QarResult begin_result =
			qar_render_sender_begin_frame(sender, NULL, &frame_info);
		log_result("qar_render_sender_begin_frame", begin_result);
		if(qar_result_is_error(begin_result) || frame_info == NULL)
		{
			break;
		}

		QarVideoFrameCpu frame = qar_video_frame_cpu_default();
		QarResult frame_result = qar_render_sender_frame_cpu(sender, &frame);
		log_result("qar_render_sender_frame_cpu", frame_result);
		if(qar_result_is_success(frame_result))
		{
			render_gradient(&frame, frame_index);

			QarRenderFrameShow show = qar_render_frame_show_default();
			show.rendered_near_far.near_plane = 0.1f;
			show.rendered_near_far.far_plane = 10.0f;
			log_result(
				"qar_render_sender_show_frame",
				qar_render_sender_show_frame(sender, &show)
			);
		}

		qar_render_frame_info_handle_destroy(frame_info);
	}

	qar_render_stream_handle_destroy(sender);
	//! [cpu_frames]

	log_result("qar_session_leave", qar_session_leave(session));
	qar_session_handle_destroy(session);
	qar_runtime_destroy(runtime);
	QarResult destroy_result = qar_library_destroy();
	log_result("qar_library_destroy", destroy_result);
	qar_library_unload();
	return 0;
}
