/** \file local_app_onboarding.c
 *  \brief Smallest source app that joins the QAROS installed on this machine
 *  as a local app: no code, the user approves it once in QAROS.
 *  \example local_app_onboarding.c
 */

/* The page holds tables and command lines wider than the column limit. */
// clang-format off
/** \page qar_c_tutorial_local_app_onboarding Local App Onboarding
 * \tableofcontents
 *
 * \section local_app_build What you build
 * The smallest source app that joins QAROS on the Hub machine. It loads the
 * installed runtime, asks the Hub to let it in, and exits with a code that
 * says what happened. The first time, QAROS asks the user to allow the app;
 * later starts of the same executable join silently.
 *
 * This is how a source app on the Hub machine joins QAROS. There is nothing
 * to type and no onboarding id to store. An app on another machine pairs with
 * a code instead
 * (\ref qar_c_tutorial_onboarding "Onboarding and Rejoin").
 *
 * What you will learn:
 * - how QarOnboardLocalAppExt joins, and what the user sees during the
 *   approval
 * - why a local app always loads the installed runtime
 * - how to follow the approval wait through progress messages
 * - which status codes a refusal comes with, and what to tell the user for
 *   each
 *
 * \section local_app_prereq Before you start
 * - Work through
 *   \ref qar_c_tutorial_dynamic_loading "Dynamic Loading"
 *   first: this program loads the runtime the same way.
 * - **Windows, with QAROS installed** on the machine the app runs on. When the
 *   Hub is not running, joining as a local app starts it, and the Hub keeps
 *   running after the app exits. Elsewhere the call fails with
 *   `QAR_STATUS_NOT_IMPLEMENTED`.
 * - **Someone at the Hub machine** to click **Allow** on the first run. The
 *   prompt waits up to 120 seconds by default.
 *
 * \section local_app_run Run it
 * \code{.bash}
 * .\build\x64-windows\qar-streaming-c\examples\Debug\local_app_onboarding.exe
 *
 * # Wait up to 90 s for the approval instead of the Hub's default.
 * .\build\x64-windows\qar-streaming-c\examples\Debug\local_app_onboarding.exe 90000
 * \endcode
 *
 * The only argument is the approval timeout in milliseconds. Absent or `0`
 * means the Hub's default of 120 s; the Hub caps longer waits at 10 minutes.
 * The program takes no library path: like a shipped app, it always runs the
 * default lookup.
 *
 * On the first run, the expected output while the approval prompt is open
 * and after the user clicks **Allow** is this. The lines before `Joined
 * QAROS.` are the runtime's own progress messages:
 * \code{.txt}
 * Connecting to the local QAROS hub.
 * Waiting for approval in QAROS hub
 * Certificate signed.
 * Onboarding complete.
 * Joined QAROS.
 * qar_library_destroy: SUCCESS (code=0)
 * \endcode
 *
 * Later starts of the same executable print the same lines without
 * `Waiting for approval in QAROS hub`.
 *
 * Exit codes, for scripts:
 * | Code | Status | Meaning |
 * |---|---|---|
 * | 0 | | joined |
 * | 1 | any other | other error, or an approval timeout that is not a number |
 * | 2 | | no runtime loaded: QAROS is not installed, or its runtime could not be loaded |
 * | 3 | `QAR_STATUS_ONBOARDING_HUB_UNREACHABLE` (1802) | no Hub answers on this machine, and none could be started |
 * | 4 | `QAR_STATUS_ONBOARDING_APPROVAL_DENIED` (1805) | Deny, a closed prompt, or a revoked approval |
 * | 5 | `QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT` (1806) | nobody answered in time |
 * | 6 | `QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC` (1807) | the program answering on this machine is not the installed QAROS |
 * | 7 | `QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH` (1808) | the executable no longer matches its approval |
 *
 * \section local_app_walkthrough Walkthrough
 *
 * \subsection local_app_timeout_step 1. Read the approval timeout
 * \snippet local_app_onboarding.c local_app_timeout
 *
 * The API reads a zero timeout as "the Hub's default", so a mistyped argument
 * that turned into 0 would silently become 120 s. The program refuses text
 * that is not a whole number of milliseconds and exits with 1:
 * \code{.txt}
 * approval timeout 'soon' is not a number of milliseconds
 * \endcode
 *
 * \subsection local_app_load_step 2. Load the installed runtime
 * \snippet local_app_onboarding.c local_app_load
 *
 * A local app always calls \ref qar_library_load with NULL, never with a path
 * of its own
 * (<a href="/docs/developer-guide/getting-started#load-the-installed-runtime">Load the installed runtime</a>).
 * The Hub refuses a runtime of another version or flavor than its own, such
 * as a private copy shipped with the app, with
 * `QAR_STATUS_ONBOARDING_FAILED` and the reason in the message.
 *
 * Console logging is off: this app talks to a user, not a developer. When no
 * runtime loads, the header's loader has already printed where it looked;
 * the program adds one line for the user and exits with 2.
 *
 * \subsection local_app_runtime_step 3. Create a runtime
 * \snippet local_app_onboarding.c local_app_runtime
 *
 * `qar_runtime_init_default()` is all a local app needs. With
 * QarRuntimeInit::runtime_binaries_folder_path empty, the runtime uses the
 * folder the library was loaded from, the installed `bin` folder.
 *
 * \subsection local_app_onboard_step 4. Join, and follow the approval
 * \snippet local_app_onboarding.c local_app_onboard
 *
 * QarOnboardLocalAppExt, chained as the only mode extension, asks the Hub on
 * this machine to let the process in. The Hub identifies the process from
 * Windows: its path, file hash, signature and user. Your `display_name` is
 * shown to the user separately, labelled "App says:".
 * - **Approved before and unchanged:** the session comes back at once, with
 *   no prompt.
 * - **Otherwise** QAROS shows "Allow local_app_onboarding.exe to join QAROS?"
 *   in the Visualizer and as a tray window, in front of every other window.
 *   **Deny** is the default button, and closing the prompt counts as Deny.
 * - **Allow** saves a trust record on the Hub. For an unsigned executable it
 *   holds the exact path and file hash, so every rebuild asks again. That is
 *   expected during development; sign release builds and users approve them
 *   once
 *   (<a href="/docs/developer-guide/onboarding-and-sessions#the-trust-record">The trust record</a>).
 *
 * \ref qar_runtime_onboard blocks until the Hub answers, which can take
 * minutes. The progress callback shows what it waits for. The library calls
 * it on its own thread while your thread is blocked, and the last call
 * returns before \ref qar_runtime_onboard does. The callback only prints, so
 * it needs no state:
 * \snippet common.h example_local_app_result
 *
 * The program passes no cancel token, so it can only wait. An application
 * passes a QarCancelToken and cancels it when the user quits.
 *
 * A local app is **stateless**. The onboarding id comes back zero, and there
 * is nothing to save: every start calls \ref qar_runtime_onboard again, and
 * the Hub remembers the approval. \ref qar_runtime_rejoin and
 * \ref qar_runtime_forget do not apply.
 *
 * \subsection local_app_explain_step 5. Tell the user, and leave
 * \snippet local_app_onboarding.c local_app_teardown
 *
 * Each refusal comes with its own status code, and each needs a different
 * answer from the user. `example_local_app_exit_code`, shown in step 4, maps
 * them to the exit codes above and prints what to do. None of them is worth
 * retrying silently. A timeout may be offered again; a denial is retried only
 * when the user asks; a Hub that is not authentic is never retried
 * (<a href="/docs/developer-guide/onboarding-and-sessions#status-codes-and-what-to-tell-the-user">Status codes and what to tell the user</a>).
 *
 * The sample programs that use a session join the same way when they are
 * given neither a stored id nor a code (`example_join_session` in
 * `common.h`).
 *
 * \section local_app_ownership Ownership and teardown in C
 * - **The session is yours.** On success \ref qar_runtime_onboard writes the
 *   only handle to it. \ref qar_session_handle_destroy ends it; the Hub keeps
 *   the approval, not the session.
 * - **Reverse order.** The session, then the runtime
 *   (\ref qar_runtime_handle_destroy), then the library.
 * - **The init structures are copied.** `init` and `local_app` live on the
 *   stack and are only read during the call.
 * - **The progress callback** runs only during the blocking call. Anything it
 *   reaches through its state pointer must live that long.
 * - **Nothing is written to disk** by the app.
 *
 * \section local_app_troubleshooting Troubleshooting
 * | What you see | Cause | What to do |
 * |---|---|---|
 * | exit 2, `Could not load QAROS: install or repair it; the line above says why.` after `qar-streaming-c.dll is not beside this module, ...` | no runtime in any step of the lookup | install QAROS (<a href="/docs/operator-guide/installation">Installation</a>) |
 * | exit 2, the same line after ``QAROS_RUNTIME_DIR is '...', but no loadable qar-streaming-c.dll is there.`` | the variable names a folder without a loadable runtime; the lookup stops there | fix or unset `QAROS_RUNTIME_DIR` |
 * | exit 2, the same line after `` The installed QAROS runtime in '...' could not be loaded. `` | a broken installation | repair or reinstall QAROS |
 * | exit 2, the same line after ``Failed to load symbol '...'`` | the runtime is older than this repository's header | install a QAROS at least as new as the header |
 * | exit 3, `Could not start or reach QAROS on this machine. Start QAROS from the Start menu, then start this app again.` | QAROS is installed but its Hub does not answer, and the runtime did not start it | start **QAROS** from the Start menu, then the app again |
 * | exit 4 | someone clicked **Deny**, closed the prompt, or an operator revoked the app | start the app again and click **Allow** |
 * | exit 5 | nobody answered within the timeout | answer the prompt in the Visualizer or the tray window; pass a longer timeout if users need it |
 * | exit 6 | the program serving the local enrollment is not the installed, signed QAROS | do not retry; reinstall QAROS from the official installer |
 * | exit 7 | the executable was moved, copied, re-signed or changed since its approval | revoke the old approval in the Visualizer (<a href="/docs/operator-guide/source-applications#approved-local-apps">Approved local apps</a>) and start again |
 * | exit 1, `QAR_STATUS_ONBOARDING_FAILED` with a runtime version or flavor in the message | the app loaded a runtime other than the installed one, through `QAROS_RUNTIME_DIR` or a copy beside the executable | unset the variable, remove the copy |
 * | exit 1, ``approval timeout '...' is not a number of milliseconds`` | the argument is not a number | pass milliseconds, or nothing |
 * | the prompt comes back after every build | the executable is unsigned, so its approval is for one exact file | expected during development |
 *
 * More in
 * <a href="/docs/developer-guide/troubleshooting#joining-qaros-as-a-local-app">Troubleshooting: Joining QAROS as a local app</a>.
 *
 * \section local_app_next Next
 * - \ref qar_c_tutorial_cpu_rendering "CPU Rendering":
 *   join the same way, then stream frames to the target apps.
 * - <a href="/docs/developer-guide/onboarding-and-sessions">Onboarding and Sessions</a>:
 *   every onboarding mode, and what changes when QAROS itself starts your app.
 * - <a href="/docs/developer-guide/tutorials/rust/local-app-onboarding">Local App Onboarding in Rust</a>:
 *   the same program with the Rust binding.
 */
// clang-format on

#include "common.h"

#include <errno.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

//! [local_app_timeout]
/** \brief Parse the optional approval timeout in milliseconds. Absent or 0
 *  asks for the Hub's default. Returns false for text that is not a whole
 *  number of milliseconds that fits in 32 bits. */
static bool
parse_approval_timeout(const char* text, uint32_t* out_timeout_ms)
{
	*out_timeout_ms = 0;
	if(text == NULL)
	{
		return true;
	}
	/* strtoul alone reads "soon" as 0 (the Hub's default) and "-1" as a huge
	 * number, so check that the whole text is digits and in range. */
	if(text[0] < '0' || text[0] > '9')
	{
		return false;
	}
	char* end = NULL;
	errno = 0;
	const unsigned long long value = strtoull(text, &end, 10);
	if(errno != 0 || *end != '\0' || value > UINT32_MAX)
	{
		return false;
	}
	*out_timeout_ms = (uint32_t)value;
	return true;
}
//! [local_app_timeout]

int
main(int argc, char** argv)
{
	example_print_as_it_happens();
	uint32_t approval_timeout_ms = 0;
	if(!parse_approval_timeout(argc >= 2 ? argv[1] : NULL, &approval_timeout_ms))
	{
		fprintf(stderr, "approval timeout '%s' is not a number of milliseconds\n", argv[1]);
		return 1;
	}

	//! [local_app_load]
	/* A local app always runs the default lookup: the Hub refuses a runtime of
	 * another version or flavor than its own. */
	if(!qar_library_load(NULL))
	{
		/* The lookup has printed why: nothing found, a QAROS_RUNTIME_DIR without
		 * a runtime, a broken install, or a runtime older than the header. */
		fprintf(stderr, "Could not load QAROS: install or repair it; the line above says why.\n");
		return 2;
	}
	QarLibraryInit library_init = qar_library_init_default();
	library_init.enable_console_logging = false; /* this app talks to a user */
	if(qar_result_is_error(qar_library_init(&library_init)))
	{
		fprintf(stderr, "The QAROS runtime failed to initialize.\n");
		qar_library_unload();
		return 1;
	}
	//! [local_app_load]

	//! [local_app_runtime]
	/* A local app stores no identity, so it needs no storage folder of its own,
	 * and the runtime binaries folder stays empty. */
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	QarRuntime* runtime = NULL;
	QarResult created = qar_runtime_create(&runtime_init, &runtime);
	if(qar_result_is_error(created))
	{
		log_result("qar_runtime_create", created);
		example_unload_library();
		return 1;
	}
	//! [local_app_runtime]

	//! [local_app_onboard]
	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = "Local App Onboarding Example";
	QarOnboardLocalAppExt local_app = qar_onboard_local_app_ext_default();
	local_app.approval_timeout_ms = approval_timeout_ms;
	init.header.next = &local_app.header;

	/* Blocks until the Hub answers, which can take minutes on the first run.
	 * The onboarding id comes back zero: a local app has nothing to persist. */
	QarOnboardingId onboarding_id = qar_onboarding_id_default();
	QarSession* session = NULL;
	QarResult joined = qar_runtime_onboard(
		runtime, &init, example_print_progress, NULL, NULL, &onboarding_id, &session
	);
	//! [local_app_onboard]

	//! [local_app_teardown]
	const int exit_code = example_local_app_exit_code(joined);
	if(exit_code == 0)
	{
		printf("Joined QAROS.\n");
	}
	if(session != NULL)
	{
		qar_session_handle_destroy(session);
	}
	qar_runtime_handle_destroy(runtime);
	example_unload_library();
	return exit_code;
	//! [local_app_teardown]
}
