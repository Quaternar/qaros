/** \file dynamic_loading.c
 *  \brief Demonstrates dynamic loading and the basic library lifecycle.
 *  \example dynamic_loading.c
 */

/** \page qar_c_tutorial_dynamic_loading Dynamic Loading Overview
 * \tableofcontents
 *
 * \section dynamic_loading_intro What You Will Learn
 * - Parse command-line arguments to locate the shared library
 * - Dynamically load \c qar-streaming-c at runtime
 * - Initialize and tear down the global library state
 *
 * \section dynamic_loading_prereq Prerequisites
 * - Build the SDK and locate the produced \c qar-streaming-c shared library
 * - Ensure runtime binaries sit next to the library (default assumption)
 *
 * \section dynamic_loading_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target dynamic_loading
 * ./build/x64-windows/Debug/dynamic_loading.exe <path-to-qar-streaming-c.dll> [runtime-dir]
 * \endcode
 *
 * \section dynamic_loading_args Parse Arguments
 * Obtain the shared-library path and optional runtime directory.
 * \snippet dynamic_loading.c dynamic_args
 *
 * \section dynamic_loading_load Load the Shared Library
 * Use \ref qar_library_load to dynamically load the DLL/SO/Dylib.
 * \snippet dynamic_loading.c dynamic_load
 *
 * \section dynamic_loading_init Initialize and Shutdown
 * Initialize global state with \ref qar_library_init, then destroy and unload when done.
 * \snippet dynamic_loading.c dynamic_init
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
    const char* name = program_name ? program_name : "dynamic_loading";
    printf(
        "Usage: %s <path-to-qar-streaming-c-library> [runtime-binaries-dir]\n",
        name
    );
    printf(
        "Example: %s build/Debug/bin/qar-streaming-c.dll build/Debug/bin\n",
        name
    );
}

int
main(int argc, char** argv)
{
    //! [dynamic_args]
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
    //! [dynamic_args]

    //! [dynamic_load]
    printf("Loading qar-streaming-c from: %s\n", library_path);
    if(!qar_library_load(library_path))
    {
        fprintf(stderr, "Failed to load '%s'. Ensure the path is correct.\n", library_path);
        return 2;
    }
    //! [dynamic_load]

    //! [dynamic_init]
    QarLibraryInit library_init = qar_library_init_default();
    library_init.enable_console_logging = true;
    library_init.log_folder_path = NULL; // Console logging is sufficient for this tutorial

    QarResult init_result = qar_library_init(&library_init);
    if(qar_result_is_error(init_result))
    {
        log_result("qar_library_init", init_result);
        qar_library_unload();
        return 3;
    }

    printf("Library initialized. Runtime binaries directory: %s\n", runtime_dir ? runtime_dir : "(derived from library path)");

    QarResult destroy_result = qar_library_destroy();
    log_result("qar_library_destroy", destroy_result);

    qar_library_unload();
    printf("Library unloaded. Dynamic loading tutorial complete.\n");
    //! [dynamic_init]

    return 0;
}