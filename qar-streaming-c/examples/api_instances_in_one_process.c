/** \file api_instances_in_one_process.c
 *  \brief Two users of the QAROS API in one process (for example native code
 *  and a C# plug-in) onboard as a local app with the same tag and share one
 *  session; a third with another tag is a peer of its own.
 *  \example api_instances_in_one_process.c
 */

/** \page qar_c_tutorial_api_instances_in_one_process Several API Instances in One Process
 * \tableofcontents
 *
 * \section api_instances_intro What You Will Learn
 * - Why a second API instance in the same process needs no invite and no
 *   approval of its own
 * - That two local onboards with the same app tag return two handles to one
 *   session, under one peer id
 * - That a different tag is a different peer with a session of its own
 * - When the shared session ends
 *
 * \section api_instances_when When to Use It
 * Your process uses QAROS from more than one place: an application and a
 * plug-in it hosts, native code and a C# or Unity layer, two libraries that
 * each wrap the C API. Each place creates its own \ref QarRuntime and onboards
 * as a local app (\ref qar_c_tutorial_local_app_onboarding). The runtime
 * library notices that the process already holds a session with that hub and
 * tag and hands out a new handle to it. One approval, one enrollment, one
 * peer. Windows only, like local enrollment.
 *
 * A process you start is not covered: it is another executable, so it joins
 * with a co-process invite (\ref qar_c_tutorial_co_process_onboarding).
 *
 * \section api_instances_run Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target api_instances_in_one_process
 * ./build/x64-windows/Debug/api_instances_in_one_process.exe
 * \endcode
 * The first run asks for the approval once, as any local app does.
 *
 * \section api_instances_onboard Step 1: Each Instance Onboards as a Local App
 * Both "instances" run the same function with their own runtime, as two
 * libraries in one process would:
 * \snippet api_instances_in_one_process.c api_instances_onboard
 *
 * \section api_instances_compare Step 2: One Session, One Peer
 * \snippet api_instances_in_one_process.c api_instances_compare
 *
 * What the runtime does for the second onboard with the same hub and tag:
 * - It sends nothing to the hub: no enrollment, no prompt, no new
 *   certificate. The progress callback reports one INFO message saying the
 *   session is shared.
 * - The returned onboarding id is zero, as for every local app.
 * - The first onboard's options win: the second one's presentation and
 *   \ref QarGraphicsDeviceId are not applied. Agree on them inside your process,
 *   or let the instance that starts first decide.
 * - Two onboards running at the same time share one enrollment: the second
 *   waits for the first and then shares its session (or fails with
 *   `QAR_STATUS_TIMEOUT` when the first does not finish within the approval
 *   timeout plus two minutes).
 *
 * A different tag is a different peer: the hub remembers a peer id per
 * approval and tag, so the third onboard below enrolls on its own and gets a
 * session of its own.
 *
 * \section api_instances_teardown Step 3: Release in Any Order
 * \snippet api_instances_in_one_process.c api_instances_teardown
 * - Each handle is released on its own with \ref qar_session_handle_destroy.
 *   The session is left when the last handle to it is released, whichever
 *   instance created it; destroying the runtime that enrolled it first is
 *   fine.
 * - If the hub refuses to renew the session's certificate, every handle sees
 *   the same session end; it stays shared until its last handle goes.
 * - The next onboard after the last handle enrolls again and, with no other
 *   live process holding it, gets the same remembered peer id.
 *
 * \section api_instances_stored What Is Stored
 * Nothing, as for any local app: the key lives in this process's memory with
 * a 30-minute certificate the runtime renews; the hub keeps the approval and
 * the remembered peer id per tag.
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

#define SHARED_TAG "main.api-instances.examples.quaternar"
#define OTHER_TAG "second-view.api-instances.examples.quaternar"

static void
on_progress(QarActionSeverity severity, float percent, const char* message, void* state)
{
	(void)percent;
	const char* who = (const char*)state;
	if(message != NULL && message[0] != '\0')
	{
		printf(
			"[%s] %s%s\n",
			who,
			(severity == QAR_ACTION_SEVERITY_WARNING || severity == QAR_ACTION_SEVERITY_ERROR)
				? "! "
				: "",
			message
		);
	}
}

//! [api_instances_onboard]
/* What every API user in the process does on its own: create a runtime and
 * onboard as a local app with its tag. Returns false and logs the reason on
 * failure; on success the caller owns both handles. */
static bool
onboard_instance(
	const char* who, const char* app_tag, QarRuntime** out_runtime, QarSession** out_session
)
{
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	QarResult created = qar_runtime_create(&runtime_init, out_runtime);
	if(qar_result_is_error(created))
	{
		log_result("qar_runtime_create", created);
		return false;
	}

	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = "API Instances Example";
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	QarOnboardLocalAppTagExt tag = qar_onboard_local_app_tag_ext_default();
	tag.app_tag = app_tag;
	init.header.next = &local.header;
	local.header.next = &tag.header;

	QarOnboardingId onboarding_id = qar_onboarding_id_default(); /* stays zero */
	QarResult joined = qar_runtime_onboard(
		*out_runtime, &init, on_progress, (void*)who, NULL, &onboarding_id, out_session
	);
	if(qar_result_is_error(joined))
	{
		log_result(who, joined);
		qar_runtime_handle_destroy(*out_runtime);
		*out_runtime = NULL;
		return false;
	}
	return true;
}
//! [api_instances_onboard]

/* Reads the session id and the own peer id of a handle. */
static bool
read_ids(QarSession* session, QarSessionId* out_session_id, QarPeerId* out_peer_id)
{
	QarResult got_session = qar_session_get_id(session, out_session_id);
	if(qar_result_is_error(got_session))
	{
		log_result("qar_session_get_id", got_session);
		return false;
	}
	return example_get_own_peer_id(session, out_peer_id);
}

static bool
same_id(const uint8_t* a, const uint8_t* b)
{
	return memcmp(a, b, QAR_MAX_ID_LENGTH) == 0;
}

/* Instance A and B share a tag, C has another one. */
static int
run(void)
{
	QarRuntime* runtime_a = NULL;
	QarRuntime* runtime_b = NULL;
	QarRuntime* runtime_c = NULL;
	QarSession* session_a = NULL;
	QarSession* session_b = NULL;
	QarSession* session_c = NULL;
	int exit_code = 1;

	if(onboard_instance("native code", SHARED_TAG, &runtime_a, &session_a)
	   && onboard_instance("plug-in", SHARED_TAG, &runtime_b, &session_b)
	   && onboard_instance("second view", OTHER_TAG, &runtime_c, &session_c))
	{
		//! [api_instances_compare]
		QarSessionId session_id_a, session_id_b, session_id_c;
		QarPeerId peer_a, peer_b, peer_c;
		if(read_ids(session_a, &session_id_a, &peer_a)
		   && read_ids(session_b, &session_id_b, &peer_b)
		   && read_ids(session_c, &session_id_c, &peer_c))
		{
			/* Same tag: two handles, one session, one peer. */
			const bool shared =
				same_id(session_id_a.data, session_id_b.data) && same_id(peer_a.data, peer_b.data);
			/* Another tag: a peer of its own. */
			const bool separate = !same_id(peer_a.data, peer_c.data);
			printf(
				"Same tag shares one peer: %s. Other tag is another peer: %s.\n",
				shared ? "yes" : "NO",
				separate ? "yes" : "NO"
			);
			exit_code = shared && separate ? 0 : 1;
		}
		//! [api_instances_compare]
	}

	//! [api_instances_teardown]
	/* Order does not matter. Releasing A (whose runtime enrolled the session)
	 * first leaves B's handle working; the shared session is left with B. */
	if(session_a != NULL)
	{
		qar_session_handle_destroy(session_a);
	}
	if(runtime_a != NULL)
	{
		qar_runtime_handle_destroy(runtime_a);
	}
	if(session_b != NULL)
	{
		/* Still joined: B shares the session A created. */
		if(!example_print_own_peer_id("Plug-in (native code already left)", session_b))
		{
			exit_code = 1;
		}
		qar_session_handle_destroy(session_b); /* last handle: session left */
	}
	if(runtime_b != NULL)
	{
		qar_runtime_handle_destroy(runtime_b);
	}
	if(session_c != NULL)
	{
		qar_session_handle_destroy(session_c);
	}
	if(runtime_c != NULL)
	{
		qar_runtime_handle_destroy(runtime_c);
	}
	//! [api_instances_teardown]
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

	const int exit_code = run();

	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	return exit_code;
}
