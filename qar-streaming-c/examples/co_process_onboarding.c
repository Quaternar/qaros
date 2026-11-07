/** \file co_process_onboarding.c
 *  \brief A local app requests one invite for its companion process, then
 *  starts the companion twice and hands both starts the same invite over
 *  stdin; the companion joins with nothing on disk.
 *  \example co_process_onboarding.c
 */

/** \page qar_c_tutorial_co_process_onboarding Co-Process Onboarding
 * \tableofcontents
 *
 * \section co_process_intro What You Will Learn
 * - Join as a local app with an app tag (\ref QarOnboardLocalAppTagExt)
 * - Request an invite for a companion process once per session
 *   (\ref QarRequestInviteInit) and reuse its bytes for every start of the
 *   companion
 * - Hand the invite over the companion's stdin, never on its command line
 * - Join from the invite with a runtime that keeps its identity in memory only
 *   (\ref QarRuntimeEphemeralStorageExt), in one onboarding call bounded by a
 *   cancel token
 *
 * \section co_process_when When to Use It
 * Your app runs on the QAROS PC and starts a helper in another process, for
 * example a companion written in C#. The helper is another executable, so it
 * cannot share your app's session (\ref qar_c_tutorial_api_instances_in_one_process)
 * and must not ask the user for an approval of its own or store an identity:
 * it joins under your app's approval with the invite your app hands it.
 * Windows and Linux, like local enrollment.
 *
 * \section co_process_run Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target co_process_onboarding
 * ./build/x64-windows/Debug/co_process_onboarding.exe
 * \endcode
 * The example joins, requests one invite, then starts itself twice with
 * `--co-process` as the companion and writes the same invite to both. Each
 * start prints its peer id: both are the same peer.
 *
 * \section co_process_request Step 1: The App Requests the Invite, Once
 * The app first joins as a local app (\ref qar_c_tutorial_local_app_onboarding),
 * then asks for an invite named by the companion's tag and serializes it:
 * \snippet co_process_onboarding.c co_process_request
 *
 * What is checked before the invite is made:
 * - The requester holds a certificate the hub signed for a local app on its
 *   own PC, or is a process the QAROS launcher started. Any other session is
 *   refused with `QAR_STATUS_ONBOARDING_INVITE_REFUSED` (final; nothing was
 *   made).
 * - The tag follows the app tag rules; a missing, empty or non-ASCII tag, or a
 *   \ref QarRequestInviteInit built from an older header, fails with
 *   `QAR_STATUS_ARGUMENT_NOT_SUPPORTED` ("app tag required") before anything
 *   is asked.
 *
 * What the invite is:
 * - It names your app on this PC, not the hub: a named pipe on Windows, a
 *   Unix socket under `/run/user/<uid>/qaros` on Linux. It reaches only this
 *   user on this PC.
 * - It works for every start of the companion while your app's session
 *   lives, and has no expiry of its own. Request it once per session and
 *   reuse the bytes; there is no timer to watch and nothing to refresh.
 * - Each onboarding attempt of the companion asks your app for a fresh
 *   one-time hub invite, so your app stays in the session while companions
 *   start.
 * - It is bound to the peer id the hub remembers for your app's approval and
 *   the companion's tag: the companion joins as the same peer on every start,
 *   while no other live process holds that id (then a one-off id).
 * - The companion's certificate renews only while your app is still in the
 *   session.
 *
 * The invite is a secret. Hand it over a private channel (stdin, a pipe,
 * shared memory), never on a command line, in an environment variable, a file
 * or a log, and clear your copy when your session ends.
 *
 * \section co_process_handover Step 2: Hand the Same Bytes to Every Start
 * \snippet co_process_onboarding.c co_process_handover
 *
 * \section co_process_child Step 3: The Companion Joins From the Invite
 * \snippet co_process_onboarding.c co_process_join
 *
 * - \ref QarRuntimeEphemeralStorageExt keeps the identity in memory: nothing
 *   is written under the storage folder and the returned onboarding id is
 *   zero. A runtime older than the extension skips it and stores the identity
 *   on disk: check \ref qar_runtime_info before relying on it.
 * - The companion makes one \ref qar_runtime_onboard call and does not retry
 *   it. While your app or the hub is away, the call tries again by itself,
 *   after 250 ms, doubling up to 5 s. The cancel token bounds that wait; when
 *   it runs out the call fails with `QAR_STATUS_TIMEOUT`.
 * - The call fails at once only for what no retry changes: a forged invite,
 *   a denied approval, or your app's session gone. Treat these as final.
 *
 * \section co_process_lifetime What Is Stored, and for How Long
 * - The app: nothing on disk. It keeps the invite bytes in memory for its
 *   session and clears them when it leaves.
 * - The companion: nothing on disk. Its memory holds key and certificate
 *   (local profile under your app's approval), 30 minutes, renewed only while
 *   your app is in the session.
 * - The hub: the remembered peer id for (your approval, companion tag), as
 *   for local app tags until unused for 30 days.
 *
 * - When your app leaves, the companion drops out at the latest when its
 *   certificate ends. Keep your app in the session while the companion runs;
 *   this example waits for each companion before it goes on.
 * - Revoking your app's approval revokes both and forgets their peer ids.
 * - Give the companion a tag of its own, not one your app's instances use.
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

#ifdef _WIN32
#include <wchar.h>
#include <windows.h>
#else
#include <signal.h>
#endif

QAR_IMPLEMENT_DYNAMIC_LOADING()

/* App tags are identity keys: printable ASCII, constant across versions. */
#define APP_TAG "main.co-process-onboarding.examples.quaternar"
#define COMPANION_TAG "companion.co-process-onboarding.examples.quaternar"
/* A serialized invite is one line of text, well below this. */
#define MAX_INVITE_LINE 16384
/* How often the example starts its companion with the same invite. */
#define COMPANION_STARTS 2
/* How long the companion's one onboarding call may wait for the app or the
 * hub before it gives up. */
#define COMPANION_JOIN_TIMEOUT_MS 120000u

/* Starts this executable again with --co-process; its stdin is the returned
 * stream. */
static FILE*
start_companion(const char* program)
{
#ifdef _WIN32
	(void)program;
	wchar_t exe[MAX_PATH * 4];
	const DWORD length = GetModuleFileNameW(NULL, exe, (DWORD)(sizeof(exe) / sizeof(exe[0])));
	if(length == 0 || length >= sizeof(exe) / sizeof(exe[0]))
	{
		return NULL;
	}
	/* _wpopen runs the command through cmd /c, which strips one pair of
	 * outer quotes: keep the path quoted inside them. */
	wchar_t command[MAX_PATH * 4 + 32];
	if(swprintf(command, sizeof(command) / sizeof(command[0]), L"\"\"%ls\" --co-process\"", exe)
	   < 0)
	{
		return NULL;
	}
	return _wpopen(command, L"w");
#else
	/* popen runs the command through sh: quote the path, and refuse one that
	 * would end the quotes. */
	if(program == NULL || strchr(program, '\'') != NULL)
	{
		return NULL;
	}
	char command[4096];
	const int length = snprintf(command, sizeof(command), "'%s' --co-process", program);
	if(length < 0 || (size_t)length >= sizeof(command))
	{
		return NULL;
	}
	/* A companion that exits before it read the invite must not end the app
	 * with SIGPIPE: the failed write is reported instead. */
	signal(SIGPIPE, SIG_IGN);
	return popen(command, "w");
#endif
}

static int
wait_for_companion(FILE* companion)
{
#ifdef _WIN32
	return _pclose(companion);
#else
	return pclose(companion);
#endif
}

/* Starts the companion, writes the invite line to its stdin and waits for it
 * to exit. Returns its exit code, or 1 when it could not be started or
 * handed the invite. */
static int
run_companion(const char* program, const char* line, size_t length)
{
	FILE* companion = start_companion(program);
	if(companion == NULL)
	{
		fprintf(stderr, "Could not start the companion.\n");
		return 1;
	}
	const bool sent = fwrite(line, 1, length, companion) == length
					  && fputc('\n', companion) != EOF && fflush(companion) == 0;
	if(!sent)
	{
		fprintf(stderr, "Could not hand the invite to the companion.\n");
	}
	/* Stay in the session until the companion exits: each of its onboarding
	 * attempts asks this app for a hub invite, and its certificate renews only
	 * while this app is in the session. */
	const int companion_exit = wait_for_companion(companion);
	return sent && companion_exit == 0 ? 0 : 1;
}

/* The app: join as a local app, request one invite for the companion, then
 * start the companion several times with the same invite. */
static int
run_app(QarRuntime* runtime, const char* program)
{
	QarSession* session = NULL;
	int exit_code =
		example_join_as_local_app(runtime, "Co-Process Onboarding Example", APP_TAG, &session);
	if(exit_code != 0)
	{
		return exit_code;
	}
	if(!example_print_own_peer_id("App", session))
	{
		qar_session_handle_destroy(session);
		return 1;
	}

	exit_code = 1;
	//! [co_process_request]
	/* Once per session: the invite works for every start of the companion
	 * while this session lives. */
	QarRequestInviteInit request = qar_request_invite_init_default();
	request.app_tag = COMPANION_TAG; /* required: names the companion */
	QarOnboardingInvite* invite = NULL;
	QarResult requested =
		qar_session_request_onboarding_invite(session, &request, NULL, NULL, NULL, &invite);
	if(qar_result_has_code(requested, QAR_STATUS_ONBOARDING_INVITE_REFUSED))
	{
		/* Final: this session did not join as a local app of this PC. */
		fprintf(stderr, "Only a local app may request an invite for its companion.\n");
	}
	log_result("qar_session_request_onboarding_invite", requested);

	/* Serialize into a buffer of its own so it can be cleared when the session
	 * ends. The result is one line of text; the companion reads it up to the
	 * newline. */
	size_t size = 0;
	char* line = NULL;
	if(qar_result_is_success(requested))
	{
		QarResult sized = qar_onboarding_invite_serialized_size(invite, &size);
		if(qar_result_is_error(sized))
		{
			log_result("qar_onboarding_invite_serialized_size", sized);
		}
		else if(size == 0 || size >= MAX_INVITE_LINE - 1)
		{
			fprintf(stderr, "The invite has an unexpected size (%zu bytes).\n", size);
		}
		else
		{
			line = (char*)calloc(1, size + 1);
		}
	}
	size_t written = 0;
	QarResult serialized = qar_result_success();
	if(line != NULL)
	{
		serialized = qar_onboarding_invite_serialize(invite, (uint8_t*)line, size, &written);
		log_result("qar_onboarding_invite_serialize", serialized); /* never the bytes */
	}
	qar_onboarding_invite_handle_destroy(invite); /* the bytes are what is kept; NULL is ignored */
	//! [co_process_request]

	//! [co_process_handover]
	if(line != NULL && qar_result_is_success(serialized))
	{
		/* The same bytes for every start while this session lives. The invite
		 * is a secret: hand it over a private channel such as the companion's
		 * stdin, never on its command line, in its environment, a file or a
		 * log. */
		exit_code = 0;
		for(int start = 1; start <= COMPANION_STARTS; ++start)
		{
			printf("Starting the companion (%d of %d) with the same invite\n", start, COMPANION_STARTS);
			if(run_companion(program, line, written) != 0)
			{
				exit_code = 1;
			}
		}
	}
	if(line != NULL)
	{
		memset(line, 0, size + 1); /* clear the secret before freeing it */
		free(line);
	}
	//! [co_process_handover]

	qar_session_handle_destroy(session);
	return exit_code;
}

/* Reads the invite line the app wrote to stdin. */
static bool
read_invite_line(char* line, size_t line_size, size_t* out_length)
{
	if(fgets(line, (int)line_size, stdin) == NULL)
	{
		return false;
	}
	size_t length = strlen(line);
	if(length == 0 || line[length - 1] != '\n')
	{
		return false; /* no complete line */
	}
	while(length > 0 && (line[length - 1] == '\n' || line[length - 1] == '\r'))
	{
		line[--length] = '\0';
	}
	*out_length = length;
	return length > 0;
}

/* The companion: a runtime with in-memory storage joins from the invite, in
 * one onboarding call bounded by a cancel token. */
static int
run_companion_process(void)
{
	//! [co_process_join]
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	QarRuntimeEphemeralStorageExt ephemeral = qar_runtime_ephemeral_storage_ext_default();
	runtime_init.header.next = &ephemeral.header; /* nothing written to disk */
	QarRuntime* runtime = NULL;
	QarResult created = qar_runtime_create(&runtime_init, &runtime);
	log_result("qar_runtime_create (companion)", created);
	if(qar_result_is_error(created) || runtime == NULL)
	{
		return 1;
	}

	int exit_code = 1;
	char* line = (char*)calloc(1, MAX_INVITE_LINE);
	size_t length = 0;
	QarOnboardingInvite* invite = NULL;
	QarResult parsed = qar_result_success();
	const bool got_line = line != NULL && read_invite_line(line, MAX_INVITE_LINE, &length);
	if(got_line)
	{
		parsed = qar_onboarding_invite_deserialize((const uint8_t*)line, length, &invite);
		log_result("qar_onboarding_invite_deserialize", parsed);
	}
	else
	{
		fprintf(stderr, "No invite on stdin.\n");
	}
	/* The call retries by itself while the app or the hub is away (250 ms,
	 * doubling up to 5 s): the token bounds that wait. */
	QarCancelToken* cancel = NULL;
	QarResult token_created = qar_result_success();
	if(got_line && qar_result_is_success(parsed))
	{
		token_created = qar_cancel_token_create_with_timeout(&cancel, COMPANION_JOIN_TIMEOUT_MS);
		log_result("qar_cancel_token_create_with_timeout", token_created);
	}
	if(got_line && qar_result_is_success(parsed) && qar_result_is_success(token_created))
	{
		QarOnboardInit init = qar_onboard_init_default();
		init.presentation.display_name = "Co-Process Onboarding Example (companion)";
		QarOnboardInviteExt invite_mode = qar_onboard_invite_ext_default();
		invite_mode.invite = invite; /* borrowed */
		init.header.next = &invite_mode.header;

		QarOnboardingId unused = qar_onboarding_id_default(); /* zero: in memory */
		QarSession* session = NULL;
		/* One call; the companion does not retry it. */
		QarResult joined =
			qar_runtime_onboard(runtime, &init, NULL, NULL, cancel, &unused, &session);
		log_result("qar_runtime_onboard (companion)", joined);
		if(qar_result_has_code(joined, QAR_STATUS_TIMEOUT))
		{
			fprintf(
				stderr,
				"Neither the app nor the hub answered within %u s.\n",
				COMPANION_JOIN_TIMEOUT_MS / 1000u
			);
		}
		if(qar_result_is_success(joined) && session != NULL)
		{
			exit_code = example_print_own_peer_id("Companion", session) ? 0 : 1;
			qar_session_handle_destroy(session);
		}
	}
	if(cancel != NULL)
	{
		qar_cancel_token_handle_destroy(cancel);
	}
	qar_onboarding_invite_handle_destroy(invite);
	if(line != NULL)
	{
		memset(line, 0, MAX_INVITE_LINE);
		free(line);
	}
	qar_runtime_handle_destroy(runtime);
	//! [co_process_join]
	return exit_code;
}

int
main(int argc, char** argv)
{
	example_print_as_it_happens();
	const bool is_companion = argc >= 2 && strcmp(argv[1], "--co-process") == 0;
	if(argc > 2 || (argc == 2 && !is_companion))
	{
		fprintf(stderr, "Usage: %s\n", argv[0]);
		return 1;
	}

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
	if(is_companion)
	{
		exit_code = run_companion_process();
	}
	else
	{
		QarRuntimeInit runtime_init = qar_runtime_init_default();
		QarRuntime* runtime = NULL;
		QarResult created = qar_runtime_create(&runtime_init, &runtime);
		log_result("qar_runtime_create (app)", created);
		if(qar_result_is_success(created) && runtime != NULL)
		{
			exit_code = run_app(runtime, argv[0]);
			qar_runtime_handle_destroy(runtime);
		}
	}

	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	return exit_code;
}
