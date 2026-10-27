/** \file dynamic_loading.c
 *  \brief Finds and loads the QAROS runtime, prints which one loaded, and runs
 *  the library lifecycle.
 *  \example dynamic_loading.c
 */

/* The page holds tables and command lines wider than the column limit. */
// clang-format off
/** \page qar_c_tutorial_dynamic_loading Dynamic Loading
 * \tableofcontents
 *
 * \section dynamic_loading_build What you build
 * A program that finds `qar-streaming-c.dll`, the QAROS runtime library,
 * loads it, prints which file it loaded and which runtime that is, then
 * initializes the library and tears it down again. Every other QAROS program
 * in C starts with these lines.
 *
 * What you will learn:
 * - how `qar_library_load(NULL)` finds the runtime, and when to pass a path
 *   instead
 * - why loading is all or nothing
 * - what to print so a user can tell which runtime your program runs against
 * - the order of init, destroy and unload
 *
 * \section dynamic_loading_prereq Before you start
 * - **Windows**, with QAROS installed. No Hub needs to run: this program only
 *   loads the library.
 * - **This repository built** with CMake: `cmake --preset x64-windows`, then
 *   `cmake --build --preset x64-windows-debug`. The programs land in
 *   `build/x64-windows/qar-streaming-c/examples/Debug/`.
 * - **A runtime at least as new as the header.** The loader needs every
 *   function `qar_streaming.h` declares, so the installed QAROS must be at
 *   least as new as this repository's header.
 *
 * \section dynamic_loading_run Run it
 * With QAROS installed, run it without arguments. The default lookup finds
 * the installed runtime:
 * \code{.bash}
 * .\build\x64-windows\qar-streaming-c\examples\Debug\dynamic_loading.exe
 * \endcode
 *
 * It prints this, with your runtime's version in place of `<version>`:
 * \code{.txt}
 * Loading the QAROS runtime (default lookup)
 * Loaded C:\Program Files\Quaternar\QAROS\bin\qar-streaming-c.dll (the installed QAROS)
 * Runtime <version> (release, C API v0)
 * Library initialized.
 * qar_library_destroy: SUCCESS (code=0)
 * Library unloaded. Dynamic loading tutorial complete.
 * \endcode
 *
 * The `Loaded` line names the file and, in brackets, the lookup step that
 * found it. The `Runtime` line shows the flavor, `release` or `demo`. Started
 * by a QAROS development hub, or with `QAR_INSTANCE` set to an instance's name,
 * the same run loads that instance's runtime and prints `(QAR_INSTANCE)`.
 *
 * To load one library file and nothing else, name it:
 * \code{.bash}
 * .\build\x64-windows\qar-streaming-c\examples\Debug\dynamic_loading.exe --qaros-library=C:\path\to\qar-streaming-c.dll
 * \endcode
 *
 * Exit codes:
 * | Code | Meaning |
 * |---|---|
 * | 0 | loaded, initialized and torn down |
 * | 1 | an argument other than `--qaros-library=PATH`, or `qar_library_init` failed |
 * | 2 | no runtime loaded |
 *
 * \section dynamic_loading_walkthrough Walkthrough
 *
 * \subsection dynamic_loading_args 1. Decide where the runtime comes from
 * A distributed application never names a file. It runs against the QAROS
 * installed on the machine, so the Hub and the application use the same
 * runtime. A path is for one specific file, such as a development build of
 * QAROS.
 *
 * The program takes the path from `--qaros-library=PATH` and refuses any
 * other argument:
 * \snippet dynamic_loading.c dynamic_args
 *
 * The helper it calls returns NULL when the flag is absent:
 * \snippet common.h example_library_path
 *
 * \subsection dynamic_loading_load 2. Load the library
 * \snippet dynamic_loading.c dynamic_load
 *
 * \ref qar_library_load opens the library and resolves every function the
 * header declares. If one is missing, it prints the missing name, unloads the
 * file and returns `false`. You never hold a half-bound API that fails at its
 * first call. A second call while the library is loaded returns `true` and
 * loads nothing.
 *
 * With a path, it tries that one file and nothing else. With NULL, it runs
 * the default lookup that every QAROS binding follows. The first match wins:
 * | Step | Where it looks | Printed as |
 * |---|---|---|
 * | 0 | a `qar-streaming-c.dll` already loaded in this process | |
 * | 1 | the folder of the executable (or of the DLL that holds this code) | `beside the executable` |
 * | 2 | when `QAR_INSTANCE` names a QAROS instance, its runtime folder: `default` is the installed QAROS, any other name the `RuntimeFolder` recorded under `HKCU\SOFTWARE\Quaternar\QAROS\Instances\<name>` | `QAR_INSTANCE` |
 * | 3 | the installed QAROS: the `bin` folder of the `InstallFolder` the installer records in the registry (HKLM) | `the installed QAROS` |
 * | 4 | each folder on `PATH`, in order | `PATH` |
 *
 * Two steps end the lookup when they fail. When steps 0 and 1 find nothing
 * and `QAR_INSTANCE` is set, the lookup ends at step 2: an unknown instance or
 * a runtime missing there is an error, and the installed QAROS and `PATH` are
 * not tried, even when they hold a good one. An installed QAROS whose runtime fails to load is
 * an error too. The current directory is never searched. Each file loads from
 * its absolute path, and its dependencies come from its own folder and
 * System32, never from your executable's folder.
 *
 * Do not ship `qar-streaming-c.dll` with your application. A copy beside the
 * executable wins over the installation (step 1), and the Hub refuses a local
 * app whose runtime is another version or flavor than its own.
 *
 * \subsection dynamic_loading_which 3. Print what you loaded
 * \snippet dynamic_loading.c dynamic_which
 *
 * The C API does not report which step found the runtime. Since the lookup
 * stops at the first step that has one, and steps 2 and 3 stop it even when
 * they fail, the helper reads the step back from what is set on this machine,
 * in the lookup's own order:
 * \snippet common.h example_print_loaded_runtime
 *
 * `GetModuleFileNameW` on `g_qar_dynamic_library_handle`, the handle
 * `QAR_IMPLEMENT_DYNAMIC_LOADING()` defines, gives the file.
 * \ref qar_runtime_info gives the runtime's version, its flavor and the C API
 * generation; it works before \ref qar_library_init.
 * `qar_requested_instance_w` and \ref qar_installed_bin_folder read the same
 * environment variable and registry value the lookup reads.
 *
 * Print this in your own application too. A copy of the runtime beside the
 * executable wins over the installation, and this line is how you notice. A
 * bug report with the file and the version saves a round of questions.
 *
 * \subsection dynamic_loading_init 4. Initialize the library
 * \snippet dynamic_loading.c dynamic_init
 *
 * \ref qar_library_init sets up the process-wide state, once per process,
 * before any runtime is created. Console logging prints the library's log
 * lines to the console; `log_folder_path` names a folder for a log file,
 * `qar_streaming.log`.
 * When it fails, unload the library before you stop.
 *
 * \subsection dynamic_loading_teardown 5. Tear down
 * \snippet dynamic_loading.c dynamic_teardown
 *
 * \section dynamic_loading_ownership Ownership and teardown in C
 * - **One library per process.** \ref qar_library_init and
 *   \ref qar_library_destroy are process-wide. Call each once.
 * - **Reverse order.** Destroy every session, then every runtime, then call
 *   \ref qar_library_destroy, then \ref qar_library_unload. Nothing is
 *   destroyed for you.
 * - **Unload last.** \ref qar_library_unload frees the file and clears every
 *   function pointer. Call nothing of the API after it.
 * - **The strings of QarRuntimeInfo** are static and stay valid until the
 *   library is unloaded. Copy them if you keep them longer.
 * - **Leave QarRuntimeInit::runtime_binaries_folder_path empty** when you
 *   create a runtime. The runtime then uses the folder the library came from.
 *
 * \section dynamic_loading_troubleshooting Troubleshooting
 * | What you see | Cause | What to do |
 * |---|---|---|
 * | `qar-streaming-c.dll is not beside this module, QAR_INSTANCE is not set, QAROS is not installed ...`, exit 2 | no runtime in any step of the lookup | install QAROS |
 * | ``QAR_INSTANCE is '...', but no runtime folder is recorded for that instance ...`` or ``... but no loadable qar-streaming-c.dll is in its runtime folder ...``, exit 2 | the variable names an instance without a loadable runtime; the lookup does not go on | start that instance's hub, or fix or unset the variable |
 * | ``Failed to load symbol '...' for module '...'.``, exit 2 | the runtime is older than the header: it lacks a function the header declares | install a QAROS at least as new as this repository's header |
 * | `No QAROS runtime loaded.` alone, after `Loading qar-streaming-c from:` | the file you named does not exist or does not load | name the `qar-streaming-c.dll` inside the runtime's folder, where its dependencies are |
 * | `Loaded ... (beside the executable)` when you expected the installation | a copy of the runtime sits beside your executable and wins | remove it; do not ship the runtime with your application |
 *
 * The <a href="/docs/developer-guide/troubleshooting">Troubleshooting</a> page
 * covers what comes after loading.
 *
 * \section dynamic_loading_next Next
 * - \ref qar_c_tutorial_local_app_onboarding "Local App Onboarding":
 *   how an app on the Hub machine joins, with nothing to type.
 * - \ref qar_c_tutorial_onboarding "Code Pairing and Rejoin":
 *   how an app joins with a pairing code and rejoins later.
 * - <a href="/docs/developer-guide/authoring-a-binding#the-default-lookup">The default lookup</a>,
 *   the rules every QAROS binding follows.
 * - <a href="/docs/developer-guide/tutorials/rust/dynamic-loading">Dynamic Loading in Rust</a>:
 *   the same program with the Rust binding.
 */
// clang-format on

#include "common.h"

QAR_IMPLEMENT_DYNAMIC_LOADING()

static void
print_usage(const char* program_name)
{
	printf("Usage: %s [--qaros-library=PATH]\n", program_name);
	printf(
		"Without a path the default lookup finds the runtime; with one, only that file is "
		"loaded.\n"
	);
}

int
main(int argc, char** argv)
{
	example_print_as_it_happens();
	//! [dynamic_args]
	const char* library_path = example_library_path(argc, argv);
	for(int i = 1; i < argc; ++i)
	{
		if(strncmp(argv[i], "--qaros-library=", strlen("--qaros-library=")) != 0)
		{
			print_usage(argv[0]);
			return 1;
		}
	}
	//! [dynamic_args]

	//! [dynamic_load]
	if(library_path != NULL)
	{
		printf("Loading qar-streaming-c from: %s\n", library_path);
	}
	else
	{
		printf("Loading the QAROS runtime (default lookup)\n");
	}
	/* NULL runs the default lookup. On failure it has already printed why. */
	if(!qar_library_load(library_path))
	{
		fprintf(
			stderr,
			"No QAROS runtime loaded. Install QAROS, fix or unset QAR_INSTANCE, or pass "
			"--qaros-library=PATH.\n"
		);
		return 2;
	}
	//! [dynamic_load]

	//! [dynamic_which]
	example_print_loaded_runtime(library_path);
	//! [dynamic_which]

	//! [dynamic_init]
	QarLibraryInit library_init = qar_library_init_default();
	library_init.enable_console_logging = true;
	library_init.log_folder_path = NULL; /* no log file: the console is enough here */

	QarResult init_result = qar_library_init(&library_init);
	if(qar_result_is_error(init_result))
	{
		log_result("qar_library_init", init_result);
		qar_library_unload();
		return 1;
	}
	printf("Library initialized.\n");
	//! [dynamic_init]

	//! [dynamic_teardown]
	/* Destroy first: it needs the functions that unloading removes. */
	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	printf("Library unloaded. Dynamic loading tutorial complete.\n");
	//! [dynamic_teardown]

	return 0;
}
