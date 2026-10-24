/** \file co_process_onboarding.c
 *  \brief A local app starts a co-process and hands it a co-process invite
 *  over the co-process's stdin; the co-process joins with nothing on disk.
 *  \example co_process_onboarding.c
 */

/** \page qar_c_tutorial_co_process_onboarding Co-Process Onboarding
 * \tableofcontents
 *
 * \section co_process_intro What You Will Learn
 * - Join as a local app with an app tag (\ref QarOnboardLocalAppTagExt)
 * - Mint a co-process invite for a process the app starts on the same PC
 *   (\ref QarRequestInviteInit)
 * - Hand the invite over the co-process's stdin, never on its command line
 * - Join from the invite with a runtime that keeps its identity in memory only
 *   (\ref QarRuntimeEphemeralStorageExt)
 *
 * \section co_process_when When to Use It
 * Your app runs on the QAROS PC and starts a helper in another process, for
 * example a companion written in C#. The helper is another executable, so it
 * cannot share your app's session (\ref qar_c_tutorial_api_instances_in_one_process)
 * and must not ask the user for an approval of its own or store an identity:
 * it joins under your app's approval with the invite your app hands it.
 * Windows only, like local enrollment.
 *
 * \section co_process_run Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target co_process_onboarding
 * ./build/x64-windows/Debug/co_process_onboarding.exe
 * \endcode
 * The example starts itself again with `--co-process` as the co-process and
 * prints both peer ids. Run it twice: both ids come back.
 *
 * \section co_process_parent Step 1: The App Mints and Hands Over
 * The app first joins as a local app (\ref qar_c_tutorial_local_app_onboarding),
 * then asks its hub for an invite named by the co-process's tag:
 * \snippet co_process_onboarding.c co_process_mint
 *
 * What the hub checks before it mints:
 * - The requester holds a certificate the hub signed for a local app on its
 *   own PC, or is a process the QAROS launcher started. A device, or an app
 *   that joined with a code or an invite, is refused with
 *   `QAR_STATUS_ONBOARDING_INVITE_REFUSED` (final; nothing was minted).
 * - The tag follows the app tag rules; a missing, empty or non-ASCII tag, or a
 *   \ref QarRequestInviteInit built from an older header, fails with
 *   `QAR_STATUS_ARGUMENT_NOT_SUPPORTED` ("app tag required") before the hub is
 *   asked.
 *
 * What the hub binds into the invite:
 * - the peer id it remembers for your app's approval and the co-process's
 *   tag: the co-process joins as the same peer on every start, while no other
 *   live process holds that id (then a one-off id);
 * - single use, valid for 60 s (the hub sets it, the app cannot choose);
 * - redeemable only from the hub's PC: a redeemer from another address is
 *   refused;
 * - the parent: the co-process's certificate renews only while your app is
 *   still in the session.
 *
 * The invite is a secret until it is used. Hand it over a private channel
 * (stdin, a pipe, shared memory), never on a command line, in an environment
 * variable, a file or a log, and clear your copy afterwards.
 *
 * \section co_process_child Step 2: The Co-Process Joins From the Invite
 * \snippet co_process_onboarding.c co_process_join
 *
 * - \ref QarRuntimeEphemeralStorageExt keeps the identity in memory: nothing
 *   is written under the storage folder, the returned onboarding id is zero,
 *   and \ref qar_runtime_rejoin / \ref qar_runtime_forget return
 *   `QAR_STATUS_ARGUMENT_NOT_SUPPORTED`. A runtime older than the extension
 *   skips it and stores the identity on disk: check \ref qar_runtime_info
 *   before relying on it.
 * - A used or expired invite fails with `QAR_STATUS_PAKE_ERROR`: ask the app
 *   for a new one, never replay the old one.
 *
 * \section co_process_lifetime What Is Stored, and for How Long
 * - The app and the co-process: nothing on disk.
 * - The invite: in the pipe, then gone; one use, 60 s.
 * - The co-process's memory: key and certificate (local profile under your
 *   app's approval), 30 minutes, renewed only while your app is in the
 *   session.
 * - The hub: the remembered peer id for (your approval, co-process tag), as
 *   for local app tags until unused for 30 days.
 *
 * - When your app leaves, the co-process drops out at the latest when its
 *   certificate ends. Keep your app in the session while the co-process runs;
 *   this example waits for the co-process before it leaves.
 * - Revoking your app's approval revokes both and forgets their peer ids.
 * - A restarted co-process needs a fresh invite from your app and gets its
 *   peer id back.
 * - Give the co-process a tag of its own, not one your app's instances use.
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

#ifdef _WIN32
#include <wchar.h>
#include <windows.h>
#endif

QAR_IMPLEMENT_DYNAMIC_LOADING()

/* App tags are identity keys: printable ASCII, constant across versions. */
#define APP_TAG "main.co-process-onboarding.examples.quaternar"
#define CO_PROCESS_TAG "companion.co-process-onboarding.examples.quaternar"
/* A serialized invite is one line of text, well below this. */
#define MAX_INVITE_LINE 16384

/* Starts this executable again with --co-process; its stdin is the returned
 * stream. */
static FILE*
start_co_process(void)
{
#ifdef _WIN32
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
	return NULL;
#endif
}

static int
wait_for_co_process(FILE* co_process)
{
#ifdef _WIN32
	return _pclose(co_process);
#else
	(void)co_process;
	return 1;
#endif
}

/* The app: join as a local app, mint a co-process invite, start the
 * co-process and write the invite to its stdin. Stays in the session until
 * the co-process exits: the co-process's certificate renews only while the app
 * is in the session. */
static int
run_app(QarRuntime* runtime)
{
	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = "Co-Process Onboarding Example";
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	QarOnboardLocalAppTagExt tag = qar_onboard_local_app_tag_ext_default();
	tag.app_tag = APP_TAG;
	init.header.next = &local.header;
	local.header.next = &tag.header;

	QarOnboardingId unused = qar_onboarding_id_default(); /* stays zero */
	QarSession* session = NULL;
	QarResult joined = qar_runtime_onboard(runtime, &init, NULL, NULL, NULL, &unused, &session);
	log_result("qar_runtime_onboard (app)", joined);
	if(qar_result_is_error(joined) || session == NULL)
	{
		return 1;
	}
	if(!example_print_own_peer_id("App", session))
	{
		qar_session_handle_destroy(session);
		return 1;
	}

	int exit_code = 1;
	//! [co_process_mint]
	QarRequestInviteInit request = qar_request_invite_init_default();
	request.app_tag = CO_PROCESS_TAG; /* required: names the co-process */
	QarOnboardingInvite* invite = NULL;
	QarResult minted =
		qar_session_request_onboarding_invite(session, &request, NULL, NULL, NULL, &invite);
	if(qar_result_has_code(minted, QAR_STATUS_ONBOARDING_INVITE_REFUSED))
	{
		/* Final: this session did not join as a local app of this PC. */
		fprintf(stderr, "The hub mints co-process invites only for local apps.\n");
	}
	log_result("qar_session_request_onboarding_invite", minted);

	/* Serialize into a buffer of its own so it can be cleared afterwards. The
	 * result is one line of text; the co-process reads it up to the newline. */
	size_t size = 0;
	char* line = NULL;
	if(qar_result_is_success(minted))
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
	if(line != NULL && qar_result_is_success(serialized))
	{
		/* The invite is a secret: hand it over a private channel such as the
		 * co-process's stdin, never on its command line, in its environment,
		 * a file or a log. */
		FILE* co_process = start_co_process();
		if(co_process == NULL)
		{
			fprintf(stderr, "Could not start the co-process.\n");
		}
		else
		{
			const bool sent = fwrite(line, 1, written, co_process) == written
							  && fputc('\n', co_process) != EOF && fflush(co_process) == 0;
			if(!sent)
			{
				fprintf(stderr, "Could not hand the invite to the co-process.\n");
			}
			/* Stay in the session until the co-process exits: its certificate
			 * renews only while this app is in the session. */
			const int co_process_exit = wait_for_co_process(co_process);
			exit_code = sent ? co_process_exit : 1;
		}
	}
	if(line != NULL)
	{
		memset(line, 0, size + 1); /* clear the secret before freeing it */
		free(line);
	}
	qar_onboarding_invite_handle_destroy(invite); /* NULL is ignored */
	//! [co_process_mint]

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

/* The co-process: a runtime with in-memory storage joins from the invite. */
static int
run_co_process(void)
{
	//! [co_process_join]
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	QarRuntimeEphemeralStorageExt ephemeral = qar_runtime_ephemeral_storage_ext_default();
	runtime_init.header.next = &ephemeral.header; /* nothing written to disk */
	QarRuntime* runtime = NULL;
	QarResult created = qar_runtime_create(&runtime_init, &runtime);
	log_result("qar_runtime_create (co-process)", created);
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
	if(got_line && qar_result_is_success(parsed))
	{
		QarOnboardInit init = qar_onboard_init_default();
		init.presentation.display_name = "Co-Process Onboarding Example (companion)";
		QarOnboardInviteExt invite_mode = qar_onboard_invite_ext_default();
		invite_mode.invite = invite; /* borrowed */
		init.header.next = &invite_mode.header;

		QarOnboardingId unused = qar_onboarding_id_default(); /* zero: in memory */
		QarSession* session = NULL;
		QarResult joined = qar_runtime_onboard(runtime, &init, NULL, NULL, NULL, &unused, &session);
		log_result("qar_runtime_onboard (co-process)", joined);
		if(qar_result_has_code(joined, QAR_STATUS_PAKE_ERROR))
		{
			/* Used or expired: ask the app for a new
			 * invite, never replay this one. */
			fprintf(stderr, "The invite was refused; the app must mint a new one.\n");
		}
		if(qar_result_is_success(joined) && session != NULL)
		{
			exit_code = example_print_own_peer_id("Co-process", session) ? 0 : 1;
			qar_session_handle_destroy(session);
		}
		qar_onboarding_invite_handle_destroy(invite);
	}
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
	const bool is_co_process = argc >= 2 && strcmp(argv[1], "--co-process") == 0;

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
	if(is_co_process)
	{
		exit_code = run_co_process();
	}
	else
	{
		QarRuntimeInit runtime_init = qar_runtime_init_default();
		QarRuntime* runtime = NULL;
		QarResult created = qar_runtime_create(&runtime_init, &runtime);
		log_result("qar_runtime_create (app)", created);
		if(qar_result_is_success(created) && runtime != NULL)
		{
			exit_code = run_app(runtime);
			qar_runtime_handle_destroy(runtime);
		}
	}

	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	return exit_code;
}
