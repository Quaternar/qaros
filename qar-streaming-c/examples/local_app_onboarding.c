/** \file local_app_onboarding.c
 *  \brief Smallest source app that joins the QAROS installed on this PC as a
 *  local app: no code, the user approves it once in QAROS.
 *  \example local_app_onboarding.c
 */

/** \page qar_c_tutorial_local_app_onboarding Local App Onboarding
 * \tableofcontents
 *
 * \section local_app_intro What You Will Learn
 * - Load the runtime of the QAROS installed on this PC
 * - Join it with \ref QarOnboardLocalAppExt (no code; QAROS asks the user once)
 * - Tell the user, in plain words, why joining failed
 *
 * \section local_app_run Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target local_app_onboarding
 * ./build/x64-windows/Debug/local_app_onboarding.exe [approval-timeout-ms]
 * \endcode
 * The first run opens "Allow local_app_onboarding.exe to join QAROS?" in the
 * QAROS Visualizer or as a QAROS tray window; later runs join silently while
 * the executable is unchanged.
 *
 * Exit codes (scripts and the QAROS install tests rely on them):
 * | Code | Meaning |
 * |---|---|
 * | 0 | joined |
 * | 1 | other error |
 * | 2 | QAROS is not installed (runtime not loaded) |
 * | 3 | no QAROS hub running on this PC (1802) |
 * | 4 | approval denied or revoked (1804) |
 * | 5 | nobody answered the approval in time (1805) |
 * | 6 | the hub is not the installed QAROS (1806) |
 * | 7 | the app no longer matches its approval (1807) |
 *
 * \section local_app_onboard Join as a Local App
 * \snippet local_app_onboarding.c local_app_onboard
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

#include <stdlib.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

static void
on_progress(
	QarActionSeverity severity, float percent, const char* message, void* state
)
{
	(void)severity;
	(void)percent;
	(void)state;
	if(message != NULL && message[0] != '\0')
	{
		printf("%s\n", message);
	}
}

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
	log_result("qar_runtime_onboard", result);
	return 1;
}

int
main(int argc, char** argv)
{
	const uint32_t approval_timeout_ms =
		argc >= 2 ? (uint32_t)strtoul(argv[1], NULL, 10) : 0;

	if(!qar_library_load(NULL))
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
		init.presentation.display_name = "Local App Onboarding Example";
		QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
		local.approval_timeout_ms = approval_timeout_ms;
		init.header.next = &local.header;

		QarOnboardingId onboarding_id;
		memset(&onboarding_id, 0, sizeof(onboarding_id));
		QarSession* session = NULL;
		QarResult joined = qar_runtime_onboard(
			runtime, &init, on_progress, NULL, NULL, &onboarding_id, &session
		);
		//! [local_app_onboard]
		exit_code = explain_result(joined);
		if(session != NULL)
		{
			qar_session_handle_destroy(session);
		}
		qar_runtime_destroy(runtime);
	}

	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	return exit_code;
}
