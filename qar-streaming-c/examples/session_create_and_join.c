/** \file session_create_and_join.c
 *  \brief Creates a runtime, hosts a session invite, and joins it from multiple peers.
 *  \example session_create_and_join.c
 */

/** \page qar_c_tutorial_session_create Session Creation and Joining
 * \tableofcontents
 *
 * \section session_create_overview Overview
 * - Reuse dynamic loading from the previous tutorial
 * - Create a runtime bound to a runtime-binaries directory
 * - Produce a session invite, host the session, and join it twice
 *
 * \section session_create_prereq Prerequisites
 * - Complete the \ref qar_c_tutorial_dynamic_loading tutorial
 * - Have the runtime binaries available next to the shared library (or pass an explicit path)
 *
 * \section session_create_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target session_create_and_join
 * ./build/x64-windows/Debug/session_create_and_join.exe <path-to-qar-streaming-c.dll> [runtime-dir]
 * \endcode
 *
 * \section session_create_args Parse Arguments
 * \snippet session_create_and_join.c session_args
 *
 * \section session_create_load Load and Initialize
 * \snippet session_create_and_join.c session_load
 *
 * \section session_create_runtime Create the Runtime
 * \snippet session_create_and_join.c session_runtime
 *
 * \section session_create_invite Host and Join the Session
 * \snippet session_create_and_join.c session_invite
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
    const char* name = program_name ? program_name : "session_create_and_join";
    printf(
        "Usage: %s <path-to-qar-streaming-c-library> [runtime-binaries-dir]\n",
        name
    );
}

static void
print_session_id(const char* label, const QarSession* session)
{
    if(!session)
    {
        return;
    }

    QarSessionId session_id = qar_session_id_default();
    if(qar_result_is_success(qar_session_get_id(session, &session_id)))
    {
        printf("%s session id: ", label);
        print_hex_id(session_id.data, QAR_MAX_ID_LENGTH);
        printf("\n");
    }
}

int
main(int argc, char** argv)
{
    //! [session_args]
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
    //! [session_args]

    //! [session_load]
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
    //! [session_load]

    //! [session_runtime]
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
    //! [session_runtime]

    //! [session_invite]
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

    printf("Host invite size: %zu bytes\n", invite->data_size);
    print_hex_id(invite->session_id.data, QAR_MAX_ID_LENGTH);
    printf(" <= Session ID in invite\n");

    QarSession* host_session = NULL;
    QarSessionJoinInit host_join = qar_session_join_init_default();
    host_join.invite_data = invite->data;
    host_join.invite_data_size = invite->data_size;
    host_join.peer_spec_init.display_name = "Session Host";

    QarResult host_join_result = qar_session_join(&host_join, &host_session);
    log_result("qar_session_join(host)", host_join_result);
    if(qar_result_is_error(host_join_result) || host_session == NULL)
    {
        qar_session_invite_destroy(invite);
        qar_runtime_destroy(runtime);
        qar_library_destroy();
        qar_library_unload();
        return 6;
    }
    print_session_id("Host peer", host_session);

    uint8_t* invite_copy = NULL;
    size_t invite_copy_size = invite->data_size;
    if(invite_copy_size > 0)
    {
        invite_copy = (uint8_t*)malloc(invite_copy_size);
        if(invite_copy)
        {
            memcpy(invite_copy, invite->data, invite_copy_size);
        }
    }

    qar_session_invite_destroy(invite);

    QarSession* guest_session = NULL;
    if(invite_copy)
    {
        QarSessionJoinInit guest_join = qar_session_join_init_default();
        guest_join.invite_data = invite_copy;
        guest_join.invite_data_size = invite_copy_size;
        guest_join.peer_spec_init.display_name = "Guest Peer";

        QarResult guest_join_result = qar_session_join(&guest_join, &guest_session);
        log_result("qar_session_join(guest)", guest_join_result);
        if(qar_result_is_success(guest_join_result) && guest_session)
        {
            print_session_id("Guest peer", guest_session);
        }
    }

    free(invite_copy);
    //! [session_invite]

    if(guest_session)
    {
        qar_session_destroy(guest_session);
    }

    if(host_session)
    {
        qar_session_destroy(host_session);
    }

    qar_runtime_destroy(runtime);
    QarResult destroy_result = qar_library_destroy();
    log_result("qar_library_destroy", destroy_result);
    qar_library_unload();
    return 0;
}