/** \file cpu_rendering_visualizer.c
 *  \brief Picks a target app from the peer list and streams CPU-rendered
 *  frames to it.
 *  \example cpu_rendering_visualizer.c
 */

/** \page qar_c_tutorial_cpu_rendering CPU Rendering with a Visualizer
 * \tableofcontents
 *
 * \section cpu_rendering_overview Overview
 * - Join the QAROS on this PC as a local app, as in the
 *   \ref qar_c_tutorial_app_volumes tutorial
 * - Create the app volume the stream is displayed in
 * - Pick a target app (a headset, or a receiver in the Visualizer) from the
 *   session's peer list
 * - Create a CPU render sender for that target and submit a short gradient
 *   animation
 *
 * \section cpu_rendering_prereq Prerequisites
 * - Complete the \ref qar_c_tutorial_gui_panels tutorial
 * - QAROS installed and running on this PC, with a target app in its session:
 *   a connected headset, or a receiver opened in the QAROS Visualizer
 *
 * \section cpu_rendering_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target cpu_rendering_visualizer
 * ./build/x64-windows/Debug/cpu_rendering_visualizer.exe
 * \endcode
 * The first run asks once in QAROS whether the app may join.
 *
 * \section cpu_rendering_setup Library, Runtime, and Session Setup
 * The installed runtime is loaded and the app joins as a local app with the
 * shared helper of the \ref qar_c_tutorial_app_volumes tutorial.
 * \snippet cpu_rendering_visualizer.c cpu_setup
 *
 * \section cpu_rendering_volume Create the App Volume
 * A render sender is displayed in an app volume, and the sender is what
 * attaches its target to that volume, so the volume comes first.
 * \snippet cpu_rendering_visualizer.c cpu_volume
 *
 * \section cpu_rendering_target Pick a Target from the Peer List
 * Your application decides which targets it renders for; QAROS does not choose
 * for it. A target is a peer whose role is QAR_PEER_ROLE_TARGET and whose
 * presence is QAR_PEER_PRESENCE_ONLINE (or QAR_PEER_PRESENCE_UNKNOWN, a peer
 * the hub has not reconciled yet). Do not wait for a render stream request to
 * find one: a target requests an app's content only after a sender has
 * attached it to the app's volume, so for a new app no request ever comes.
 * \snippet cpu_rendering_visualizer.c cpu_target
 * The shared helper subscribes to peer updates first and then lists the peers
 * already there, so a target joining in between is not missed:
 * \snippet common.h example_find_target
 *
 * \section cpu_rendering_frames Render and Submit Frames
 * The sender's peer_id is the target, never this app's own peer id. Each
 * begin-frame opens a frame cycle that show-frame closes.
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
render_gradient(QarVideoFrameCpu* frame, size_t frame_index)
{
	if(!frame)
	{
		return;
	}

	for(size_t tex_idx = 0; tex_idx < frame->textures_count; ++tex_idx)
	{
		QarVideoTextureCpu* texture = &frame->textures[tex_idx];
		if(!texture->texture_data || texture->size.width == 0 || texture->size.height == 0)
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

/** \brief Leave the session (when there is one), release the runtime and
 *  unload the library. */
static void
teardown(QarRuntime* runtime, QarSession* session)
{
	if(session != NULL)
	{
		qar_session_handle_destroy(session);
	}
	qar_runtime_handle_destroy(runtime);
	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
}

int
main(void)
{
	//! [cpu_setup]
	if(!qar_library_load(NULL)) /* NULL: the default lookup, normally the installed QAROS */
	{
		fprintf(
			stderr,
			"Could not load the QAROS runtime (reason above): install QAROS, or fix "
			"QAR_INSTANCE.\n"
		);
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
	QarResult runtime_result = qar_runtime_create(&runtime_init, &runtime);
	if(qar_result_is_error(runtime_result) || runtime == NULL)
	{
		log_result("qar_runtime_create", runtime_result);
		log_result("qar_library_destroy", qar_library_destroy());
		qar_library_unload();
		return 4;
	}

	QarSession* session = NULL;
	if(example_join_as_local_app(
		   runtime, "CPU Rendering Example", "main.cpu-rendering.examples.quaternar", &session
	   ) != 0
	   || session == NULL)
	{
		teardown(runtime, NULL);
		return 5;
	}
	//! [cpu_setup]

	//! [cpu_volume]
	/* A render sender requires the app volume its stream is shown in. */
	QarAppVolumeInit volume_init = qar_app_volume_init_default();
	volume_init.common_name = "cpu-volume.examples.qaros";
	volume_init.display_name = "CPU Tutorial Volume";
	volume_init.pose = qar_pose_default();
	volume_init.pose.position.z = -1.0f;
	volume_init.size = qar_app_volume_size_default();

	QarAppVolumeId volume_id = qar_app_volume_id_default();
	QarResult volume_result = qar_app_volumes_get_or_create(session, &volume_init, &volume_id);
	if(qar_result_is_error(volume_result))
	{
		log_result("qar_app_volumes_get_or_create", volume_result);
		teardown(runtime, session);
		return 6;
	}
	//! [cpu_volume]

	//! [cpu_target]
	/* Two minutes for someone to connect a headset or open a receiver. */
	QarPeerId target_peer_id = qar_peer_id_default();
	if(!example_wait_for_target(session, 120000, &target_peer_id))
	{
		teardown(runtime, session);
		return 7;
	}
	//! [cpu_target]

	//! [cpu_frames]
	QarRenderSenderInit sender_init = qar_render_sender_init_default();
	sender_init.graphics_api = QAR_GRAPHICS_API_CPU;
	sender_init.peer_id = target_peer_id; /* who receives the frames */
	sender_init.app_volume_id = &volume_id;

	QarRenderSender* sender = NULL;
	QarResult sender_result = qar_render_sender_create(session, &sender_init, NULL, &sender);
	if(qar_result_is_error(sender_result) || sender == NULL)
	{
		log_result("qar_render_sender_create", sender_result);
		teardown(runtime, session);
		return 8;
	}

	/* The layout the sender negotiated: the views laid out on its textures. */
	QarVideoFrameLayout layout = qar_video_frame_layout_default();
	QarResult layout_result = qar_render_sender_layout(sender, &layout);
	if(qar_result_is_error(layout_result))
	{
		log_result("qar_render_sender_layout", layout_result);
	}

	const size_t frame_count = 3;
	for(size_t frame_index = 0; frame_index < frame_count; ++frame_index)
	{
		/* Waits for the receiver and hands out the poses to render with. */
		QarRenderFrameInfo* frame_info = NULL;
		QarResult begin_result = qar_render_sender_begin_frame(sender, NULL, &frame_info);
		if(qar_result_is_error(begin_result) || frame_info == NULL)
		{
			log_result("qar_render_sender_begin_frame", begin_result);
			break;
		}

		QarVideoFrameCpu frame = qar_video_frame_cpu_default();
		QarResult frame_result = qar_render_sender_frame_cpu(sender, &frame);
		if(qar_result_is_success(frame_result))
		{
			render_gradient(&frame, frame_index);
		}
		else
		{
			log_result("qar_render_sender_frame_cpu", frame_result);
		}

		/* Show closes the frame cycle that begin-frame opened, also when the
		 * frame could not be filled. */
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
	//! [cpu_frames]

	teardown(runtime, session);
	return 0;
}
