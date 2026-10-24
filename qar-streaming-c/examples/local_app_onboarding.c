/** \file local_app_onboarding.c
 *  \brief Smallest source app that joins the QAROS installed on this PC as a
 *  local app: no code, the user approves it once in QAROS, and an app tag
 *  keeps its peer id across restarts.
 *  \example local_app_onboarding.c
 */

/** \page qar_c_tutorial_local_app_onboarding Local App Onboarding
 * \tableofcontents
 *
 * \section local_app_intro What You Will Learn
 * - Load the runtime of the QAROS installed on this PC
 * - Join it with \ref QarOnboardLocalAppExt (no code; QAROS asks the user once)
 * - Name the instance with an app tag (\ref QarOnboardLocalAppTagExt), so the
 *   hub gives it the same peer id on every start
 * - Tell the user, in plain words, why joining failed
 *
 * \section local_app_when When to Use It
 * Your app runs on the PC where QAROS is installed. This is the normal mode
 * for a source app: nobody types a code, nothing is stored by your app, and
 * the user decides once whether the app may join. Windows only.
 *
 * \section local_app_run Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target local_app_onboarding
 * ./build/x64-windows/Debug/local_app_onboarding.exe [approval-timeout-ms] [app-tag]
 * \endcode
 * The first run opens "Allow local_app_onboarding.exe to join QAROS?" in the
 * QAROS Visualizer or as a QAROS tray window. Later runs join silently while
 * the executable is unchanged, and print the same peer id while no other
 * instance with the same tag runs. Start a second instance with another tag
 * (second argument) while the first runs to see two peers of one approval.
 *
 * Exit codes (scripts and the QAROS install tests rely on them):
 * | Code | Meaning |
 * |---|---|
 * | 0 | joined |
 * | 1 | other error |
 * | 2 | QAROS is not installed (runtime not loaded) |
 * | 3 | no QAROS hub running on this PC (1802) |
 * | 4 | approval denied or revoked (1805) |
 * | 5 | nobody answered the approval in time (1806) |
 * | 6 | the hub is not the installed QAROS (1807) |
 * | 7 | the app no longer matches its approval (1808) |
 *
 * \section local_app_load Step 1: Load the Installed Runtime
 * `qar_library_load(NULL)` loads the qar-streaming-c of the QAROS installed
 * on this PC. Use it: the hub refuses a runtime of another version or flavor,
 * such as a private copy shipped beside your app
 * (`QAR_STATUS_ONBOARDING_FAILED`, the reason in the message).
 * \snippet local_app_onboarding.c local_app_load
 *
 * \section local_app_onboard Step 2: Join as a Local App
 * \snippet local_app_onboarding.c local_app_onboard
 *
 * What happens during the call:
 * 1. The runtime connects to the hub of its QAROS installation over a local
 *    pipe and checks that the program answering is the installed QAROS hub,
 *    signed with the QAROS key (otherwise 1807). When no hub runs, it starts
 *    one.
 * 2. The hub asks Windows who is calling: executable path, file hash,
 *    Authenticode signature and Windows user. Your `display_name` is only
 *    shown next to it, labelled as what the app says about itself.
 * 3. Unknown app: the hub shows the approval prompt and the progress callback
 *    reports "Waiting for approval in QAROS hub". Deny, closing the prompt or
 *    no answer within `approval_timeout_ms` (0: 120 s) fails the call (1805,
 *    1806); Allow saves a trust record on the hub. A known app that still
 *    matches its record joins without a prompt; one that was moved, copied or
 *    re-signed is refused (1808).
 * 4. The hub picks the peer id (below) and signs a 30-minute certificate for
 *    a key that exists only in this process's memory. The runtime renews it
 *    in the background for as long as the session lives.
 *
 * \section local_app_tag The App Tag and the Peer Id
 * - The hub remembers one peer id per approval and app tag. The next start
 *   with the same tag gets the same peer id back, so viewers and your own
 *   bookkeeping see the same peer, not a new one. The hub picks the id; an app
 *   cannot choose it (\ref QarOnboardPeerIdExt is refused here).
 * - While another live process holds the remembered id (a second instance
 *   with the same tag), the newcomer gets a one-off id that is not
 *   remembered. Give instances that run together different tags.
 * - The tag is an identity key like the common name of an app volume:
 *   printable ASCII, no space at either end, at most
 *   \ref QAR_MAX_APP_TAG_LENGTH bytes, compared exactly, never starting with
 *   `member/`, and kept the same across versions of the app. Anything else is
 *   refused with `QAR_STATUS_ARGUMENT_NOT_SUPPORTED` before the hub is asked.
 *   Text for people goes in the display name.
 * - No tag is the default tag, remembered the same way.
 * - The hub keeps at most 64 tags per approval (least recently used dropped
 *   first) and forgets a tag unused for 30 days. Revoking or narrowing the
 *   approval revokes the running instances and forgets every remembered id.
 * - A tag is not a secret and not a permission: it only selects among the
 *   peer ids of your own approval.
 *
 * \section local_app_stored What Is Stored, and for How Long
 * - Your app: nothing; the onboarding id stays zero.
 * - This process's memory: the key and its certificate, 30 minutes, renewed
 *   while the session lives.
 * - The hub: the trust record (until revoked) and the remembered peer id per
 *   tag (until unused for 30 days).
 *
 * Every start therefore calls `qar_runtime_onboard` again;
 * `qar_runtime_rejoin` and `qar_runtime_forget` do not apply.
 *
 * \section local_app_errors Step 3: Tell the User What Went Wrong
 * \snippet local_app_onboarding.c local_app_explain
 *
 * Retry 1806 when the user asks. Never retry 1807 silently. 1808 needs the
 * operator to revoke the old approval first.
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

#include <stdlib.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

/* An app tag is an identity key: printable ASCII, constant across versions,
 * scoped by the app (DNS style). */
#define DEFAULT_APP_TAG "main.local-app-onboarding.examples.quaternar"

static void
on_progress(QarActionSeverity severity, float percent, const char* message, void* state)
{
	(void)severity;
	(void)percent;
	(void)state;
	if(message != NULL && message[0] != '\0')
	{
		printf("%s\n", message); /* "Waiting for approval in QAROS hub" */
	}
}

//! [local_app_explain]
/* Maps the onboarding result to the documented exit code and explains it. */
static int
explain_result(QarResult result)
{
	if(qar_result_is_success(result))
	{
		printf("Joined QAROS.\n");
		return 0;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_UNREACHABLE))
	{
		fprintf(
			stderr,
			"No QAROS hub is running on this PC. Start QAROS (tray icon) and "
			"try again.\n"
		);
		return 3;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_DENIED))
	{
		fprintf(
			stderr,
			"QAROS did not allow this app: the approval was denied or "
			"revoked.\n"
		);
		return 4;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT))
	{
		fprintf(
			stderr,
			"Nobody answered the approval prompt in QAROS in time. Start the "
			"app again.\n"
		);
		return 5;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC))
	{
		fprintf(
			stderr,
			"The program answering on this PC is not the installed QAROS. "
			"Reinstall QAROS.\n"
		);
		return 6;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH))
	{
		fprintf(
			stderr,
			"This app no longer matches what QAROS approved. Revoke it in "
			"QAROS and approve it again.\n"
		);
		return 7;
	}
	if(qar_result_has_code(result, QAR_STATUS_ARGUMENT_NOT_SUPPORTED))
	{
		fprintf(
			stderr,
			"An argument was refused (the message below names it). An app tag "
			"must be printable ASCII, no space at either end, not starting "
			"with member/.\n"
		);
	}
	log_result("qar_runtime_onboard", result);
	return 1;
}
//! [local_app_explain]

int
main(int argc, char** argv)
{
	const uint32_t approval_timeout_ms = argc >= 2 ? (uint32_t)strtoul(argv[1], NULL, 10) : 0;
	const char* app_tag = argc >= 3 ? argv[2] : DEFAULT_APP_TAG;

	//! [local_app_load]
	if(!qar_library_load(NULL)) /* NULL: the installed QAROS runtime */
	{
		fprintf(stderr, "QAROS is not installed on this PC.\n");
		return 2;
	}
	QarLibraryInit library_init = qar_library_init_default();
	library_init.enable_console_logging = false;
	if(qar_result_is_error(qar_library_init(&library_init)))
	{
		fprintf(stderr, "The QAROS runtime failed to initialize.\n");
		qar_library_unload();
		return 1;
	}
	//! [local_app_load]

	int exit_code = 1;
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	QarRuntime* runtime = NULL;
	QarResult created = qar_runtime_create(&runtime_init, &runtime);
	if(qar_result_is_error(created))
	{
		log_result("qar_runtime_create", created);
	}
	else
	{
		//! [local_app_onboard]
		QarOnboardInit init = qar_onboard_init_default();
		/* Any UTF-8; shown to people, never used as a key. */
		init.presentation.display_name = "Local App Onboarding Example";
		QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
		local.approval_timeout_ms = approval_timeout_ms; /* 0: hub default, 120 s */
		/* Optional: without it the app has the default tag. */
		QarOnboardLocalAppTagExt tag = qar_onboard_local_app_tag_ext_default();
		tag.app_tag = app_tag;
		init.header.next = &local.header; /* the one mode extension */
		local.header.next = &tag.header;

		QarOnboardingId onboarding_id = qar_onboarding_id_default(); /* stays zero */
		QarSession* session = NULL;
		/* Pass a QarCancelToken instead of NULL in a GUI app: the call can wait
		 * minutes for the user's answer. */
		QarResult joined =
			qar_runtime_onboard(runtime, &init, on_progress, NULL, NULL, &onboarding_id, &session);
		//! [local_app_onboard]
		exit_code = explain_result(joined);
		if(session != NULL)
		{
			/* The same tag prints the same peer id on the next start. Joining
			 * succeeded either way, so a failure here is only logged. */
			if(!example_print_own_peer_id("This app", session))
			{
				fprintf(stderr, "Could not read this app's peer id.\n");
			}
			/* Leaves the session; nothing to forget, nothing was stored. */
			qar_session_handle_destroy(session);
		}
		qar_runtime_handle_destroy(runtime);
	}

	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	return exit_code;
}
