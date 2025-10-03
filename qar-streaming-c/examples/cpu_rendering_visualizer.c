/** \file cpu_rendering_visualizer.c
 *  \brief Invites a Visualizer peer and submits CPU-rendered frames.
 *  \example cpu_rendering_visualizer.c
 */

/** \page qar_c_tutorial_cpu_rendering CPU Rendering with a Visualizer
 * \tableofcontents
 *
 * \section cpu_rendering_overview Overview
 * - Host and join a session as in earlier tutorials
 * - Invite a Visualizer peer using a connection string
 * - Create a CPU render sender and submit a short gradient animation
 *
 * \section cpu_rendering_prereq Prerequisites
 * - Complete the \ref qar_c_tutorial_gui_panels tutorial
 *
 * \section cpu_rendering_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target cpu_rendering_visualizer
 * ./build/x64-windows/Debug/cpu_rendering_visualizer.exe <path-to-qar-streaming-c.dll> [runtime-dir]
 * \endcode
 *
 * \section cpu_rendering_args Parse Arguments
 * \snippet cpu_rendering_visualizer.c cpu_args
 *
 * \section cpu_rendering_setup Library, Runtime, and Session Setup
 * \snippet cpu_rendering_visualizer.c cpu_setup
 *
 * \section cpu_rendering_invite Invite the Visualizer Peer
 * \snippet cpu_rendering_visualizer.c cpu_invite
 *
 * \section cpu_rendering_frames Render and Submit Frames
 * \snippet cpu_rendering_visualizer.c cpu_frames
 */

#include "common.h"

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
        "Usage: %s <path-to-qar-streaming-c-library> [runtime-binaries-dir]\n",
        name
    );
}

static char g_visualizer_connection[QAR_MAX_STRING_LENGTH] = { 0 };

static void
on_invite_result(QarResult status, const QarPeerId* peer_id, void* user_state)
{
    (void)user_state;
    if(qar_result_is_success(status) && peer_id)
    {
        printf("Visualizer invite accepted by peer ");
        print_hex_id(peer_id->data, QAR_MAX_ID_LENGTH);
        printf("\n");
    }
    else
    {
        log_result("qar_session_invite_peer_async(result)", status);
    }
}

static void
on_invite_update(const char* message, void* user_state)
{
    (void)user_state;
    if(message)
    {
        printf("Invite update: %s\n", message);
    }
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
    else if(get_dir_from_path(library_path, runtime_dir_buffer, sizeof(runtime_dir_buffer)))
    {
        runtime_dir = runtime_dir_buffer;
    }
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

    QarSessionCreateInit create_init = qar_session_create_init_default();
    QarSessionInvite* invite = NULL;
    QarResult invite_result = qar_runtime_create_session(runtime, &create_init, &invite);
    if(qar_result_is_error(invite_result) || invite == NULL)
    {
        log_result("qar_runtime_create_session", invite_result);
        qar_runtime_destroy(runtime);
        qar_library_destroy();
        qar_library_unload();
        return 5;
    }

    QarSession* session = NULL;
    QarSessionJoinInit join_init = qar_session_join_init_default();
    join_init.invite_data = invite->data;
    join_init.invite_data_size = invite->data_size;
    join_init.peer_spec_init.display_name = "CPU Renderer";

    QarResult join_result = qar_session_join(&join_init, &session);
    log_result("qar_session_join", join_result);
    qar_session_invite_destroy(invite);
    if(qar_result_is_error(join_result) || session == NULL)
    {
        qar_runtime_destroy(runtime);
        qar_library_destroy();
        qar_library_unload();
        return 6;
    }
    //! [cpu_setup]

    //! [cpu_invite]
    QarResult connection_result = qar_session_invite_connection_string_visualizer(
        g_visualizer_connection,
        sizeof(g_visualizer_connection)
    );
    log_result("qar_session_invite_connection_string_visualizer", connection_result);
    if(qar_result_is_success(connection_result))
    {
        printf("Visualizer connection string: %s\n", g_visualizer_connection);
        QarSessionInvitePeerInit invite_peer = qar_session_invite_peer_init_default();
        invite_peer.connection_string = g_visualizer_connection;

        QarResult invite_peer_result = qar_session_invite_peer_async(
            session,
            &invite_peer,
            on_invite_result,
            on_invite_update,
            NULL
        );
        log_result("qar_session_invite_peer_async", invite_peer_result);
    }
    //! [cpu_invite]

    //! [cpu_frames]
    QarRenderSenderInit sender_init = qar_render_sender_init_default();
    sender_init.graphics_api = QAR_GRAPHICS_API_CPU;

    QarRenderSender* sender = NULL;
    QarResult sender_result = qar_render_sender_create(session, &sender_init, NULL, &sender);
    if(qar_result_is_error(sender_result) || sender == NULL)
    {
        log_result("qar_render_sender_create", sender_result);
        qar_session_destroy(session);
        qar_runtime_destroy(runtime);
        qar_library_destroy();
        qar_library_unload();
        return 7;
    }
    log_result("qar_render_sender_create", sender_result);

    QarVideoFrameLayout layout = qar_video_frame_layout_default();
    QarResult layout_result = qar_render_sender_layout(sender, &layout);
    log_result("qar_render_sender_layout", layout_result);

    const size_t frame_count = 3;
    for(size_t frame_index = 0; frame_index < frame_count; ++frame_index)
    {
        QarRenderFrameInfo* frame_info = NULL;
        QarResult begin_result = qar_render_sender_begin_frame(sender, NULL, &frame_info);
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
            log_result("qar_render_sender_show_frame", qar_render_sender_show_frame(sender, &show));
        }

        qar_render_frame_info_handle_destroy(frame_info);
    }

    qar_render_stream_handle_destroy(sender);
    //! [cpu_frames]

    qar_session_destroy(session);
    qar_runtime_destroy(runtime);
    QarResult destroy_result = qar_library_destroy();
    log_result("qar_library_destroy", destroy_result);
    qar_library_unload();
    return 0;
}