/** \file launched_app_onboarding.c
 *  \brief A QAROS API app: an app the operator added in the Visualizer, which
 *  QAROS starts, stops and restarts. It joins under the peer id QAROS planned
 *  for it, prompts for nothing and stops when its stdin closes.
 *  \example launched_app_onboarding.c
 */

/** \page qar_c_tutorial_launched_app_onboarding App Launched by QAROS
 * \tableofcontents
 *
 * \section launched_intro What You Will Learn
 * - The contract between QAROS and an app it starts (a "QAROS API app")
 * - Joining as a local app with nothing to read, under the peer id QAROS
 *   planned for the process
 * - Cancelling a pending onboard when QAROS stops the app (its stdin closes)
 * - Where local enrollment is not available: joining with the launch invite
 *   QAROS writes to stdin
 * - Creating the graphics device on the session's GPU
 *
 * \section launched_when When to Use It
 * The operator adds your executable as a QAROS API app in the Visualizer's
 * Source Applications panel. QAROS then starts one process of it for all
 * targets, passes the arguments the operator entered on the command line,
 * restarts it with backoff when it exits within 10 s of starting, and stops
 * it by closing its stdin. The same executable must also work when a user
 * starts it by hand.
 *
 * Target apps (the players on HoloLens and other headsets) and the processes
 * QAROS starts for its own pipeline also join under planned peer ids, but they
 * are QAROS's own programs, not built on this API. An app brings a headset in
 * with qar_session_invite_target_app instead; it does not onboard it.
 *
 * \section launched_run Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target launched_app_onboarding
 * \endcode
 * Add `launched_app_onboarding.exe` as a QAROS API app in the Visualizer and
 * start it there, or run it from a console (end it with Ctrl+Z, Enter on
 * Windows, Ctrl+D elsewhere: that closes stdin as QAROS's Stop does).
 *
 * \section launched_windows Step 1: Join as a Local App, Cancellable
 * \snippet launched_app_onboarding.c launched_join
 *
 * What the hub does:
 * - It recognizes the process it started (by the process, not by anything the
 *   app sends) and gives it the peer id it planned and announced to the
 *   session, so the visualizer shows the app under its tile from the start. A
 *   restart keeps that peer id. A user-started copy is an ordinary local app
 *   (\ref qar_c_tutorial_local_app_onboarding).
 * - The approval prompt applies as for every local app: the first start
 *   waits for the user. Nobody is at the app's console, so it prompts for
 *   nothing itself and exits with an error on
 *   `QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT` instead of offering a retry.
 * - Nothing is stored: a key in memory, a 30-minute certificate renewed while
 *   the session lives.
 *
 * \section launched_stop Step 2: Stop When stdin Closes
 * \snippet launched_app_onboarding.c launched_stop
 * QAROS closes stdin on Stop and kills the process about 1 s later. A closed
 * stdin cancels a pending onboard (its call returns with the cancel status)
 * and ends a running app; whatever arrives on stdin is ignored.
 *
 * \section launched_invite Step 3: Without Local Enrollment, the Launch Invite
 * \snippet launched_app_onboarding.c launched_invite
 * Where local enrollment is not available (not Windows), QAROS writes a launch
 * invite as the first line of stdin: one line of JSON starting with `{`, with
 * a one-time code and the target's GPU. It is valid for 2 minutes, for this
 * start only, and works once; each restart brings a new one. It is a secret:
 * never log it or pass it on. Pass it unchanged to
 * \ref qar_onboarding_invite_deserialize. A line not starting with `{` is a
 * code someone typed at the console. A restart onboards with its new invite,
 * so the example keeps the identity in memory
 * (\ref QarRuntimeEphemeralStorageExt) instead of leaving a slot on disk per
 * start.
 *
 * \section launched_gpu Step 4: Render on the Session's GPU
 * \snippet launched_app_onboarding.c launched_gpu
 * Onboard without chaining a \ref QarGraphicsDeviceId, and the session picks
 * the GPU QAROS selected for the app (from the launch invite) or one the
 * library picks. Create your graphics device on the adapter it reports. Then
 * create your own app volume and a sender per target you serve, as in
 * \ref qar_c_tutorial_cpu_rendering.
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

#ifdef _WIN32
#include <windows.h>
#endif

QAR_IMPLEMENT_DYNAMIC_LOADING()

/* A launch invite is one line of JSON, well below this. */
#define MAX_LAUNCH_LINE 16384

static void
on_progress(QarActionSeverity severity, float percent, const char* message, void* state)
{
	(void)severity;
	(void)percent;
	(void)state;
	if(message != NULL && message[0] != '\0')
	{
		printf("%s\n", message); /* stdout goes to the hub's log folder */
	}
}

//! [launched_stop]
/* Reads stdin until it closes, ignoring what arrives: its end is the stop
 * request. */
static void
read_stdin_until_closed(void)
{
	char buffer[256];
	while(fread(buffer, 1, sizeof(buffer), stdin) > 0)
	{
	}
}

#ifdef _WIN32
/* True when stdin can carry a stop request: a pipe (QAROS started the app) or
 * a console (a user did). Not for NUL or a missing stdin, which read as
 * closed at once. */
static bool
stdin_can_stop(void)
{
	const HANDLE input = GetStdHandle(STD_INPUT_HANDLE);
	DWORD console_mode = 0;
	if(input == NULL || input == INVALID_HANDLE_VALUE)
	{
		return false;
	}
	return GetFileType(input) == FILE_TYPE_PIPE || GetConsoleMode(input, &console_mode);
}

/* Thread: ends when stdin closes. It calls nothing of QAROS, so the process
 * may end while it still blocks; the main thread waits on it and cancels a
 * pending onboard from there. */
static DWORD WINAPI
stop_when_stdin_closes(LPVOID unused)
{
	(void)unused;
	read_stdin_until_closed();
	return 0;
}
#endif
//! [launched_stop]

/* Tells the operator, through the log, why joining failed. */
static void
explain(QarResult result)
{
	log_result("qar_runtime_onboard", result);
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT))
	{
		printf(
			"Nobody approved this app in QAROS in time. Start it again once someone can "
			"approve it.\n"
		);
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_DENIED))
	{
		printf("QAROS did not allow this app.\n");
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH))
	{
		printf("This app changed since it was approved: revoke it in QAROS.\n");
	}
	else if(qar_result_has_code(result, QAR_STATUS_PAKE_ERROR))
	{
		printf("The launch invite or code was refused: it works once.\n");
	}
}

#ifdef _WIN32
/* What the async onboard's result callback hands back. */
typedef struct OnboardWait
{
	HANDLE done;
	QarResult status;
	QarSession* session;
} OnboardWait;

static void
on_onboarded(
	QarResult status, const QarOnboardingId* onboarding_id, QarSession* session, void* state
)
{
	(void)onboarding_id; /* zero for a local app */
	OnboardWait* wait = (OnboardWait*)state;
	wait->status = status;
	wait->session = qar_result_is_success(status) ? session : NULL;
	if(!SetEvent(wait->done))
	{
		fprintf(stderr, "Could not signal the onboarding result.\n");
	}
}

//! [launched_join]
/* Joins as a local app. Nothing to read from stdin: the hub knows the process
 * it started. No QarOnboardPeerIdExt either: the hub assigns the planned peer
 * id. The onboard runs async so that a closed stdin (the `watcher` thread
 * ends) cancels the wait for the approval; with no watcher (NULL) it is not
 * cancellable. Returns false when the onboard
 * could not run; otherwise *out_result is its result. */
static bool
join(QarRuntime* runtime, HANDLE watcher, QarResult* out_result, QarSession** out_session)
{
	QarCancelToken* cancel = NULL;
	QarResult created = qar_cancel_token_create(&cancel);
	if(qar_result_is_error(created))
	{
		log_result("qar_cancel_token_create", created);
		return false;
	}
	OnboardWait wait = { CreateEventW(NULL, TRUE, FALSE, NULL), qar_result_success(), NULL };
	if(wait.done == NULL)
	{
		fprintf(stderr, "Could not create the onboarding event.\n");
		qar_cancel_token_handle_destroy(cancel);
		return false;
	}

	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = "Launched App Example";
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	init.header.next = &local.header;
	QarResult started =
		qar_runtime_onboard_async(runtime, &init, on_onboarded, on_progress, &wait, cancel);
	bool finished = qar_result_is_success(started);
	if(!finished)
	{
		log_result("qar_runtime_onboard_async", started);
	}
	else
	{
		HANDLE events[2] = { wait.done, watcher };
		const DWORD count = watcher != NULL ? 2 : 1;
		const DWORD first = WaitForMultipleObjects(count, events, FALSE, INFINITE);
		if(first == WAIT_OBJECT_0 + 1) /* stdin closed: QAROS stops the app */
		{
			log_result("qar_cancel_token_cancel", qar_cancel_token_cancel(cancel));
		}
		/* The result callback fires exactly once, also when cancelled. */
		finished = WaitForSingleObject(wait.done, INFINITE) == WAIT_OBJECT_0;
	}
	*out_result = wait.status;
	*out_session = wait.session;
	qar_cancel_token_handle_destroy(cancel);
	if(!CloseHandle(wait.done))
	{
		fprintf(stderr, "Could not close the onboarding event.\n");
	}
	return finished;
}
//! [launched_join]
#else
/* Reads the first stdin line (the launch invite, or a typed code). */
static bool
read_first_line(char* line, size_t line_size)
{
	if(fgets(line, (int)line_size, stdin) == NULL)
	{
		return false;
	}
	line[strcspn(line, "\r\n")] = '\0';
	return line[0] != '\0';
}

//! [launched_invite]
/* Joins with the launch invite on stdin, or a code typed at the console.
 * Returns false when there was nothing to join with; otherwise *out_result is
 * the onboarding's result. */
static bool
join(QarRuntime* runtime, QarResult* out_result, QarSession** out_session)
{
	char* line = (char*)calloc(1, MAX_LAUNCH_LINE);
	if(line == NULL || !read_first_line(line, MAX_LAUNCH_LINE))
	{
		free(line);
		fprintf(stderr, "No launch invite on stdin.\n");
		return false;
	}
	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = "Launched App Example";
	QarOnboardInviteExt invite_ext = qar_onboard_invite_ext_default();
	QarOnboardCodeExt code_ext = qar_onboard_code_ext_default();
	QarOnboardingInvite* invite = NULL;
	QarResult result = qar_result_success();
	if(line[0] == '{')
	{
		result = qar_onboarding_invite_deserialize((const uint8_t*)line, strlen(line), &invite);
		log_result("qar_onboarding_invite_deserialize", result); /* never the line */
		invite_ext.invite = invite;
		init.header.next = &invite_ext.header;
	}
	else
	{
		code_ext.code = line; /* copied before the call returns */
		init.header.next = &code_ext.header;
	}
	if(qar_result_is_success(result))
	{
		QarOnboardingId onboarding_id = qar_onboarding_id_default();
		result = qar_runtime_onboard(
			runtime, &init, on_progress, NULL, NULL, &onboarding_id, out_session
		);
	}
	qar_onboarding_invite_handle_destroy(invite); /* NULL is ignored */
	memset(line, 0, MAX_LAUNCH_LINE);			  /* clear the secret */
	free(line);
	*out_result = result;
	return true;
}
//! [launched_invite]
#endif

/* Prints the adapter the session's GPU devices live on. */
static bool
report_session_gpu(QarSession* session)
{
	//! [launched_gpu]
	QarGraphicsDeviceId gpu = qar_graphics_device_id_default();
	bool present = false;
	QarResult got = qar_session_get_graphics_device_id(session, &gpu, &present);
	if(qar_result_is_error(got))
	{
		log_result("qar_session_get_graphics_device_id", got);
		return false;
	}
	if(present && gpu.id_type == QAR_GPU_DEVICE_ID_TYPE_UUID)
	{
		printf("Create the graphics device on the adapter with UUID ");
		print_hex_id(gpu.uuid, sizeof(gpu.uuid));
		printf(".\n");
	}
#ifdef _WIN32
	else if(present)
	{
		printf(
			"Create the graphics device on the adapter with LUID %08lX:%08lX.\n",
			(unsigned long)gpu.luid.HighPart,
			(unsigned long)gpu.luid.LowPart
		);
	}
#endif
	//! [launched_gpu]
	return true;
}

static int
run(QarRuntime* runtime)
{
	QarResult joined = qar_result_success();
	QarSession* session = NULL;
#ifdef _WIN32
	/* NULL when stdin cannot carry a stop request (redirected from NUL, no
	 * stdin at all): then the onboard is not cancellable and the app leaves
	 * right after joining. */
	HANDLE watcher = NULL;
	if(stdin_can_stop())
	{
		watcher = CreateThread(NULL, 0, stop_when_stdin_closes, NULL, 0, NULL);
		if(watcher == NULL)
		{
			fprintf(stderr, "Could not watch stdin.\n");
			return 1;
		}
	}
	const bool attempted = join(runtime, watcher, &joined, &session);
#else
	const bool attempted = join(runtime, &joined, &session);
#endif

	int exit_code = 1;
	if(attempted && qar_result_is_success(joined) && session != NULL)
	{
		const bool ready =
			example_print_own_peer_id("This app", session) && report_session_gpu(session);
		printf("Running. QAROS stops the app by closing stdin.\n");
		fflush(stdout);
		/* ... create an app volume and render for targets here ... */
#ifdef _WIN32
		if(watcher == NULL)
		{
			printf("stdin carries no stop request: leaving now.\n");
		}
		else if(WaitForSingleObject(watcher, INFINITE) != WAIT_OBJECT_0)
		{
			fprintf(stderr, "Waiting for the stop request failed.\n");
		}
#else
		read_stdin_until_closed();
#endif
		exit_code = ready ? 0 : 1;
		qar_session_handle_destroy(session);
	}
	else if(attempted)
	{
		explain(joined);
	}
#ifdef _WIN32
	if(watcher != NULL && !CloseHandle(watcher))
	{
		fprintf(stderr, "Could not close the stdin watcher handle.\n");
	}
#endif
	fflush(stdout);
	return exit_code;
}

int
main(void)
{
	if(!qar_library_load(NULL))
	{
		fprintf(stderr, "QAROS is not installed on this PC.\n");
		return 2;
	}
	QarLibraryInit library_init = qar_library_init_default();
	QarResult initialized = qar_library_init(&library_init);
	if(qar_result_is_error(initialized))
	{
		log_result("qar_library_init", initialized);
		qar_library_unload();
		return 1;
	}

	int exit_code = 1;
	QarRuntimeInit runtime_init = qar_runtime_init_default();
#ifndef _WIN32
	/* Every start brings a new launch invite: keep the identity it onboards
	 * with in memory instead of leaving a slot on disk per start. */
	QarRuntimeEphemeralStorageExt ephemeral = qar_runtime_ephemeral_storage_ext_default();
	runtime_init.header.next = &ephemeral.header;
#endif
	QarRuntime* runtime = NULL;
	QarResult created = qar_runtime_create(&runtime_init, &runtime);
	log_result("qar_runtime_create", created);
	if(qar_result_is_success(created))
	{
		exit_code = run(runtime);
		qar_runtime_handle_destroy(runtime);
	}

	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	return exit_code;
}
