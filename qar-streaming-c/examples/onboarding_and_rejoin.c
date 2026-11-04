/** \file onboarding_and_rejoin.c
 *  \brief A camera app on a PC without a QAROS hub pairs once with a code from
 *  the hub's operator, keeps its identity in the secure store and rejoins
 *  silently on every later start.
 *  \example onboarding_and_rejoin.c
 */

/** \page qar_c_tutorial_onboarding Code Pairing and Rejoin
 * \tableofcontents
 *
 * \section onboarding_overview What You Will Learn
 * Every QAROS session is obtained through onboarding; there is no direct
 * create or join. An app on a PC that has no hub of its own (a camera PC, a
 * capture box, a laptop next to the QAROS PC) cannot be identified by the
 * hub's Windows, so it pairs with a code once and keeps an identity:
 * - Try a silent rejoin with the persisted onboarding id
 * - Fall back to onboarding with a code from the hub's operator, read from
 *   the console, never from the command line
 * - Reach a hub on another PC with \ref QarOnboardHostExt
 * - Describe the app as a camera (\ref QarPeerDeviceIdentity)
 * - Persist the returned onboarding id for the next start
 * - Tear down by destroying the session handle, and forget only to un-pair
 *
 * \section onboarding_when When to Use It
 * The app runs on a PC or device where QAROS is installed (the app loads its
 * runtime) but no QAROS hub runs, or the hub it should join runs on another
 * PC. On the hub's own PC, join as a local app instead
 * (\ref qar_c_tutorial_local_app_onboarding): no code, nothing stored.
 *
 * \section onboarding_prereq Prerequisites
 * - The \ref qar_c_tutorial_dynamic_loading tutorial
 * - The QAROS hub on the other PC reachable over the network, and its host
 *   name or IP and onboarding port (shown with its pairing code)
 * - For the first start: a pairing code from the hub's operator, the code on
 *   the Visualizer's onboarding screen. Apps cannot mint codes.
 *
 * \section onboarding_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target onboarding_and_rejoin
 * ./build/x64-windows/Debug/onboarding_and_rejoin.exe [--host <host>:<port>] [--forget]
 * \endcode
 * The first start asks for the code on the console. Later starts rejoin with
 * no input. `--forget` un-pairs on exit.
 *
 * \section onboarding_args Step 1: Parse Arguments
 * Only the hub's address is an argument, used when pairing; a rejoin reaches
 * the hub the stored identity was paired with. The pairing code is a short-lived
 * secret: a command line is visible to other processes and ends up in shell
 * history, so the example reads it from the console.
 * \snippet onboarding_and_rejoin.c onboarding_args
 *
 * \section onboarding_persistence Step 2: Persist the Onboarding Id
 * The onboarding id is the only thing the app stores itself. It names the
 * identity slot; it is not a secret and grants nothing on its own:
 * \snippet common.h example_onboarding_persistence
 *
 * \section onboarding_rejoin Step 3: Rejoin With the Stored Identity
 * \snippet onboarding_and_rejoin.c onboarding_rejoin
 *
 * - The identity (key and certificate) lives in the QAROS secure store under
 *   \ref QarRuntimeInit::storage_folder_path (default: the user's QAROS data
 *   folder), protected for this Windows user. The app never sees the key.
 * - The certificate lives 24 hours. While the session runs, the runtime
 *   renews it in the background at 75 % of its life. A device that was off
 *   longer rejoins too: within 7 days after expiry, the rejoin renews it.
 * - The peer id is stored with the slot: every rejoin is the same peer.
 * - `QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND` is expected control flow (no
 *   slot for the id here): onboard instead.
 * - `QAR_STATUS_ONBOARDING_RE_PAIR_REQUIRED` (1809): the certificate expired
 *   beyond the renewal window, was revoked, or its hub no longer renews it.
 *   Retrying does not help; pair again with a new code.
 * - `QAR_STATUS_ONBOARDING_RENEWAL_POSSESSION_UNPROVEN` (1810): the hub
 *   refused a renewal that did not prove the key. Pair again with a new
 *   code; an app built with an SDK older than the hub must be updated first.
 *
 * \section onboarding_code Step 4: Pair With a Code
 * \snippet onboarding_and_rejoin.c onboarding_code
 *
 * - The code runs a password-authenticated key exchange with the hub: neither
 *   side sends the code, and a wrong or expired code fails with
 *   `QAR_STATUS_PAKE_ERROR` (the screen code changes about every 10 s).
 *   Re-prompt the user.
 * - Without \ref QarOnboardHostExt the runtime only finds a hub of this user
 *   on this PC (`QAR_STATUS_ONBOARDING_HUB_UNREACHABLE` on a PC without one).
 *   Pass the host and port; port 0 is refused. A code text that carries an
 *   invitation id (`qar-code:...`) needs them too.
 * - Re-pairing after 1809 or 1810 passes the stored onboarding id: the slot is
 *   re-enrolled instead of a second identity being created.
 * - An app joins with the source role. As a camera it chains
 *   \ref QarPeerDeviceIdentity so viewers show a camera, not a desktop PC.
 *
 * \section onboarding_obtain Step 5: Put It Together
 * \snippet onboarding_and_rejoin.c onboarding_obtain
 *
 * \section onboarding_stored What Is Stored, and for How Long
 * - The app: the onboarding id (a UUID, not secret), until un-paired.
 * - The secure store: key, 24-hour certificate and peer id; renewed while
 *   running, and by a rejoin up to 7 days after expiry.
 * - The hub: the device's entry, until the operator revokes it.
 *
 * \section onboarding_other_instances Other API Instances and Processes
 * A code-paired app requests no invite or code for another process or PC.
 * Each device pairs with its own code from the operator.
 *
 * \section onboarding_teardown Step 6: Destroy and Forget
 * Destroying the session handle keeps the stored identity, so the next start
 * rejoins. Forget erases it: only for an explicit "un-pair this device"
 * action, and only after the session was left
 * (`QAR_STATUS_ONBOARDING_SESSION_STILL_ACTIVE` otherwise):
 * \snippet onboarding_and_rejoin.c onboarding_teardown
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

#define ID_FILE "onboarding_and_rejoin.onboarding-id.txt"
/* Room for any code text the hub hands out. */
#define MAX_CODE_LENGTH 1024

static void
print_usage(const char* program_name)
{
	const char* name = program_name ? program_name : "onboarding_and_rejoin";
	printf("Usage: %s [--host <host>:<port>] [--forget]\n", name);
	printf(
		"The first start asks for the pairing code on the console; later "
		"starts rejoin with the stored identity.\n"
	);
	printf("Pass --forget to un-pair on exit.\n");
}

/* Parses "<host>:<port>" into host_buffer and *out_port. */
static bool
parse_host_port(const char* text, char* host_buffer, size_t host_buffer_size, uint16_t* out_port)
{
	const char* colon = strrchr(text, ':');
	if(colon == NULL || colon == text || (size_t)(colon - text) >= host_buffer_size)
	{
		return false;
	}
	char* end = NULL;
	const unsigned long port = strtoul(colon + 1, &end, 10);
	if(end == colon + 1 || *end != '\0' || port == 0 || port > 65535)
	{
		return false;
	}
	memcpy(host_buffer, text, (size_t)(colon - text));
	host_buffer[colon - text] = '\0';
	*out_port = (uint16_t)port;
	return true;
}

/* Asks for the pairing code on the console. The caller clears the buffer. */
static bool
prompt_for_code(char* code, size_t code_size)
{
	printf("Enter the pairing code from the QAROS hub's operator: ");
	fflush(stdout);
	if(fgets(code, (int)code_size, stdin) == NULL)
	{
		return false;
	}
	code[strcspn(code, "\r\n")] = '\0';
	return code[0] != '\0';
}

/* How this app describes itself: a camera, not the default desktop PC. */
static QarPeerPresentation
camera_presentation(QarPeerDeviceIdentity* device)
{
	*device = qar_peer_device_identity_default();
	device->category = QAR_DEVICE_CATEGORY_CAMERA;
	device->device_id = "ExampleCamera"; /* open string; viewers fall back */
	device->device_name = "Example camera";
	QarPeerPresentation presentation = qar_peer_presentation_default();
	presentation.display_name = "Onboarding and Rejoin Example";
	presentation.header.next = &device->header;
	return presentation;
}

//! [onboarding_rejoin]
/* Rejoins with the stored identity. */
static QarResult
rejoin(QarRuntime* runtime, const QarOnboardingId* saved_id, QarSession** out_session)
{
	QarPeerDeviceIdentity device;
	QarRejoinInit init = qar_rejoin_init_default();
	init.onboarding_id = *saved_id;
	init.presentation = camera_presentation(&device);
	return qar_runtime_rejoin(runtime, &init, NULL, NULL, NULL, out_session);
}
//! [onboarding_rejoin]

//! [onboarding_code]
/* Pairs with a code. `slot_id` zero creates a new identity; a stored id
 * re-enrolls that slot. Receives the id to persist in *out_id. */
static QarResult
pair_with_code(
	QarRuntime* runtime,
	const char* code,
	const char* hub_host, /* NULL: a hub of this user on this PC */
	uint16_t hub_port,
	const QarOnboardingId* slot_id,
	QarOnboardingId* out_id,
	QarSession** out_session
)
{
	QarPeerDeviceIdentity device;
	QarOnboardInit init = qar_onboard_init_default();
	init.onboarding_id = *slot_id;
	init.presentation = camera_presentation(&device);
	QarOnboardCodeExt code_ext = qar_onboard_code_ext_default();
	code_ext.code = code; /* copied before the call returns */
	init.header.next = &code_ext.header;
	QarOnboardHostExt host = qar_onboard_host_ext_default();
	if(hub_host != NULL)
	{
		host.hostname = hub_host;
		host.port = hub_port;
		code_ext.header.next = &host.header;
	}
	return qar_runtime_onboard(runtime, &init, NULL, NULL, NULL, out_id, out_session);
}
//! [onboarding_code]

/* Explains a pairing or rejoin failure in plain words. */
static void
explain(const char* label, QarResult result)
{
	log_result(label, result);
	if(qar_result_has_code(result, QAR_STATUS_PAKE_ERROR))
	{
		printf("The code was wrong or expired. Ask for the current code.\n");
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_UNREACHABLE))
	{
		printf("No hub answered. Pass --host <host>:<port> of the QAROS hub.\n");
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_RE_PAIR_REQUIRED))
	{
		printf("This device must be paired again with a new code.\n");
	}
	else if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_RENEWAL_POSSESSION_UNPROVEN))
	{
		printf("The stored identity could not renew. Update the app, or pair again.\n");
	}
}

//! [onboarding_obtain]
/* Rejoin when an identity is stored; otherwise, or when it must be paired
 * again, pair with a code read from the console and persist the id. */
static bool
obtain_session(
	QarRuntime* runtime,
	const char* hub_host,
	uint16_t hub_port,
	QarOnboardingId* out_id,
	QarSession** out_session
)
{
	QarOnboardingId saved_id = qar_onboarding_id_default();
	QarOnboardingId slot_id = qar_onboarding_id_default(); /* zero: new identity */
	if(load_onboarding_id(ID_FILE, &saved_id))
	{
		QarResult rejoined = rejoin(runtime, &saved_id, out_session);
		if(qar_result_is_success(rejoined))
		{
			*out_id = saved_id; /* the normal path: no input, same peer */
			return true;
		}
		const bool re_pair =
			qar_result_has_code(rejoined, QAR_STATUS_ONBOARDING_RE_PAIR_REQUIRED)
			|| qar_result_has_code(rejoined, QAR_STATUS_ONBOARDING_RENEWAL_POSSESSION_UNPROVEN);
		if(!re_pair && !qar_result_has_code(rejoined, QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND))
		{
			explain("qar_runtime_rejoin", rejoined); /* e.g. hub unreachable: retry later */
			return false;
		}
		if(!re_pair)
		{
			/* Expected control flow, not an error: no slot for the id here. */
			printf("No stored identity for this id; pairing with a code.\n");
		}
		else
		{
			explain("qar_runtime_rejoin", rejoined);
			slot_id = saved_id; /* re-enroll the stored slot */
		}
	}

	char code[MAX_CODE_LENGTH] = { 0 };
	bool joined = false;
	if(prompt_for_code(code, sizeof(code)))
	{
		QarOnboardingId new_id = qar_onboarding_id_default();
		QarResult paired =
			pair_with_code(runtime, code, hub_host, hub_port, &slot_id, &new_id, out_session);
		if(qar_result_is_success(paired))
		{
			save_onboarding_id(ID_FILE, &new_id); /* the ticket for every rejoin */
			*out_id = new_id;
			joined = true;
		}
		else
		{
			explain("qar_runtime_onboard", paired);
		}
	}
	else
	{
		printf("No pairing code entered.\n");
	}
	memset(code, 0, sizeof(code)); /* the code is a secret until it expires */
	return joined;
}
//! [onboarding_obtain]

int
main(int argc, char** argv)
{
	//! [onboarding_args]
	char host_buffer[256] = { 0 };
	const char* hub_host = NULL;
	uint16_t hub_port = 0;
	bool forget_on_exit = false;
	for(int i = 1; i < argc; ++i)
	{
		if(strcmp(argv[i], "--forget") == 0)
		{
			forget_on_exit = true;
		}
		else if(strcmp(argv[i], "--host") == 0 && i + 1 < argc
				&& parse_host_port(argv[i + 1], host_buffer, sizeof(host_buffer), &hub_port))
		{
			hub_host = host_buffer;
			++i;
		}
		else
		{
			print_usage(argv[0]);
			return 1;
		}
	}
	//! [onboarding_args]

	/* The runtime of the QAROS installed on this PC, even though no hub runs
	 * here. */
	if(!qar_library_load(NULL))
	{
		fprintf(
			stderr,
			"Could not load the QAROS runtime (reason above): install QAROS, or fix "
			"QAR_INSTANCE.\n"
		);
		return 2;
	}
	QarLibraryInit library_init = qar_library_init_default();
	QarResult library_result = qar_library_init(&library_init);
	if(qar_result_is_error(library_result))
	{
		log_result("qar_library_init", library_result);
		qar_library_unload();
		return 3;
	}

	/* storage_folder_path NULL: this user's QAROS data folder, where the
	 * secure store keeps the identity. */
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	QarRuntime* runtime = NULL;
	QarResult runtime_result = qar_runtime_create(&runtime_init, &runtime);
	if(qar_result_is_error(runtime_result))
	{
		log_result("qar_runtime_create", runtime_result);
		log_result("qar_library_destroy", qar_library_destroy());
		qar_library_unload();
		return 4;
	}

	int exit_code = 5;
	QarOnboardingId onboarding_id = qar_onboarding_id_default();
	QarSession* session = NULL;
	if(obtain_session(runtime, hub_host, hub_port, &onboarding_id, &session))
	{
		exit_code = example_print_own_peer_id("This camera", session) ? 0 : 5;

		//! [onboarding_teardown]
		/* Leaves the session and keeps the stored identity: the next start
		 * rejoins silently. */
		qar_session_handle_destroy(session);
		if(forget_on_exit)
		{
			/* Un-pair: erase the identity. The next start needs a new code. */
			QarForgetInit forget_init = qar_forget_init_default();
			forget_init.onboarding_id = onboarding_id;
			QarResult forgotten = qar_runtime_forget(runtime, &forget_init);
			log_result("qar_runtime_forget", forgotten);
			if(qar_result_is_success(forgotten) && remove(ID_FILE) != 0)
			{
				printf("Could not delete '%s'.\n", ID_FILE);
			}
		}
		//! [onboarding_teardown]
	}

	qar_runtime_handle_destroy(runtime);
	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	return exit_code;
}
