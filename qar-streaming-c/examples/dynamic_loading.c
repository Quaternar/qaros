/** \file dynamic_loading.c
 *  \brief Demonstrates dynamic loading and the basic library lifecycle.
 *  \example dynamic_loading.c
 */

/** \page qar_c_tutorial_dynamic_loading Dynamic Loading Overview
 * \tableofcontents
 *
 * \section dynamic_loading_intro What You Will Learn
 * - Load the \c qar-streaming-c runtime of the QAROS installed on this PC at
 *   run time, with no link-time dependency on it
 * - Initialize and tear down the global library state
 *
 * \section dynamic_loading_prereq Prerequisites
 * - QAROS installed on this PC. The application ships no copy of
 *   \c qar-streaming-c: a copy beside the executable would be loaded first.
 *
 * \section dynamic_loading_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target dynamic_loading
 * ./build/x64-windows/Debug/dynamic_loading.exe [installed|path-to-qar-streaming-c.dll]
 * \endcode
 * With no argument, or `installed`, it loads the installed QAROS. A path loads
 * that library file only, for development builds of QAROS itself.
 *
 * \section dynamic_loading_setup Enable Dynamic Loading
 * Define `QAR_ENABLE_DYNAMIC_LOADING` before including the header, and put
 * `QAR_IMPLEMENT_DYNAMIC_LOADING()` in exactly one translation unit. The API
 * functions are then resolved by symbol name into tables the application
 * owns, so a newer runtime keeps working with an application built against an
 * older header.
 *
 * \section dynamic_loading_load Load the Runtime
 * \ref qar_library_load with NULL takes the first of: a \c qar-streaming-c
 * already loaded in the process, one beside the calling module, the runtime of
 * the QAROS instance the `QAR_INSTANCE` environment variable names, the
 * installed QAROS (`<InstallFolder>\bin` under `HKLM\SOFTWARE\Quaternar\QAROS`),
 * and `PATH`. The current directory is never searched. With nothing found it
 * prints where it looked and returns false, so the application can run without
 * AR/VR. A QAROS development hub sets `QAR_INSTANCE` for the apps it starts, so
 * they run its runtime; set the same value to start one by hand against it. A
 * set `QAR_INSTANCE` naming no known instance is an error, never a fall-back to
 * the installed runtime.
 * \snippet dynamic_loading.c dynamic_load
 *
 * \section dynamic_loading_init Initialize and Shutdown
 * Initialize global state with \ref qar_library_init, then destroy and unload
 * when done. Leave \ref QarRuntimeInit::runtime_binaries_folder_path empty
 * when you create a runtime: it defaults to the folder the library was loaded
 * from.
 * \snippet dynamic_loading.c dynamic_init
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

int
main(int argc, char** argv)
{
	//! [dynamic_load]
	/* NULL (no argument, or "installed"): the installed QAROS runtime. */
	const char* library_path = argc >= 2 ? library_path_argument(argv[1]) : NULL;
	printf("Loading qar-streaming-c from: %s\n", describe_library(library_path));
	if(!qar_library_load(library_path))
	{
		fprintf(
			stderr,
			"Failed to load %s. Ensure QAROS is installed or the path is correct.\n",
			describe_library(library_path)
		);
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

	QarRuntimeInfo info;
	memset(&info, 0, sizeof(info));
	QarResult info_result = qar_runtime_info(&info);
	if(qar_result_is_success(info_result))
	{
		printf("Loaded QAROS runtime %s (%s).\n", info.version, info.flavor);
	}
	else
	{
		log_result("qar_runtime_info", info_result);
	}

	QarResult destroy_result = qar_library_destroy();
	log_result("qar_library_destroy", destroy_result);

	qar_library_unload();
	printf("Library unloaded. Dynamic loading tutorial complete.\n");
	//! [dynamic_init]

	return 0;
}
