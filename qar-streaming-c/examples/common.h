#pragma once
#include "qar_streaming.h"
#include <inttypes.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/** \brief Print a fixed-size identifier as hex bytes separated by colons. */
static void
print_hex_id(const uint8_t* data, size_t len)
{
	for(size_t i = 0; i < len; ++i)
	{
		printf("%02X", (unsigned)data[i]);
		if(i + 1 < len)
		{
			printf(":");
		}
	}
}

/** \brief Log a \ref QarResult with a label to stdout. */
static void
log_result(const char* label, QarResult r)
{
	if(qar_result_is_success(r))
	{
		printf("%s: SUCCESS (code=%d)\n", label, (int)r.code);
	}
	else
	{
		char msg[256] = { 0 };
		qar_result_message(r, msg, sizeof(msg));
		printf("%s: ERROR (code=%d) msg='%s'\n", label, (int)r.code, msg);
	}
}

//! [example_own_peer_id]
/** \brief Read the peer id this session joined under. The hub picks it (for a
 *  local app: the id it remembers for the app's approval and tag). Returns
 *  false and logs the reason when it cannot be read. */
static bool
example_get_own_peer_id(QarSession* session, QarPeerId* out_peer_id)
{
	QarPeerSpec* spec = NULL;
	QarResult got_spec = qar_session_get_my_spec(session, &spec);
	if(qar_result_is_error(got_spec) || spec == NULL)
	{
		log_result("qar_session_get_my_spec", got_spec);
		return false;
	}
	memset(out_peer_id, 0, sizeof(*out_peer_id));
	QarResult got_id = qar_peer_spec_get_id(spec, out_peer_id);
	qar_peer_spec_handle_destroy(spec);
	if(qar_result_is_error(got_id))
	{
		log_result("qar_peer_spec_get_id", got_id);
		return false;
	}
	return true;
}

/** \brief Print "<who> joined as peer <uuid>". A peer id is not a secret. */
static bool
example_print_own_peer_id(const char* who, QarSession* session)
{
	QarPeerId peer_id;
	char text[64] = { 0 };
	if(!example_get_own_peer_id(session, &peer_id))
	{
		return false;
	}
	QarResult formatted = qar_uuid_to_string(peer_id.data, text, sizeof(text));
	if(qar_result_is_error(formatted))
	{
		log_result("qar_uuid_to_string", formatted);
		return false;
	}
	printf("%s joined as peer %s\n", who, text);
	return true;
}
//! [example_own_peer_id]

//! [example_onboarding_persistence]
/** \brief Load a persisted onboarding id (UUID text) from a small state file.
 *  Returns false when the file does not exist yet (first run). */
static bool
load_onboarding_id(const char* path, QarOnboardingId* out_id)
{
	FILE* file = fopen(path, "r");
	if(!file)
	{
		return false;
	}
	char text[64] = { 0 };
	const bool read_ok = fgets(text, sizeof(text), file) != NULL;
	fclose(file);
	if(!read_ok)
	{
		return false;
	}
	text[strcspn(text, "\r\n")] = '\0';
	QarResult parse_result = qar_uuid_from_string(text, out_id->data);
	return qar_result_is_success(parse_result);
}

/** \brief Persist an onboarding id (UUID text) so the next run can rejoin. */
static void
save_onboarding_id(const char* path, const QarOnboardingId* id)
{
	char text[64] = { 0 };
	if(qar_result_is_error(qar_uuid_to_string(id->data, text, sizeof(text))))
	{
		printf("Failed to serialize onboarding id\n");
		return;
	}
	FILE* file = fopen(path, "w");
	if(!file)
	{
		printf("Failed to persist onboarding id to '%s'\n", path);
		return;
	}
	fprintf(file, "%s\n", text);
	fclose(file);
}
//! [example_onboarding_persistence]

//! [example_join_local_app]
/** \brief Print the onboarding progress ("Waiting for approval in QAROS hub"
 *  while the approval prompt is open). */
static void
example_print_progress(QarActionSeverity severity, float percent, const char* message, void* state)
{
	(void)severity;
	(void)percent;
	(void)state;
	if(message != NULL && message[0] != '\0')
	{
		printf("%s\n", message);
	}
}

/** \brief Join the QAROS hub on this PC as a local app: no code, the user
 *  approves the app once in QAROS, and the app tag keeps the peer id across
 *  restarts. Nothing is stored, so every start calls it again. See the
 *  \ref qar_c_tutorial_local_app_onboarding tutorial.
 *
 *  Returns 0 on success, with *out_session set. */
static int
example_join_as_local_app(
	QarRuntime* runtime, const char* display_name, const char* app_tag, QarSession** out_session
)
{
	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = display_name; /* any UTF-8 */
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	QarOnboardLocalAppTagExt tag = qar_onboard_local_app_tag_ext_default();
	tag.app_tag = app_tag;			  /* printable ASCII, constant across versions */
	init.header.next = &local.header; /* the one mode extension */
	local.header.next = &tag.header;

	QarOnboardingId unused = qar_onboarding_id_default(); /* stays zero */
	/* A GUI app passes a QarCancelToken: the call can wait minutes for the
	 * user's answer. */
	QarResult joined = qar_runtime_onboard(
		runtime, &init, example_print_progress, NULL, NULL, &unused, out_session
	);
	log_result("qar_runtime_onboard", joined);
	if(qar_result_is_success(joined))
	{
		return 0;
	}
	/* What to tell the user: see the local_app_onboarding tutorial. */
	if(qar_result_has_code(joined, QAR_STATUS_ONBOARDING_HUB_UNREACHABLE))
	{
		printf("QAROS is not running on this PC. Start QAROS, then run again.\n");
	}
	else if(qar_result_has_code(joined, QAR_STATUS_ONBOARDING_APPROVAL_DENIED)
			|| qar_result_has_code(joined, QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT))
	{
		printf("QAROS did not allow this app. Run again and click Allow in QAROS.\n");
	}
	else if(qar_result_has_code(joined, QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH))
	{
		printf("This app changed since it was approved. Revoke it in QAROS, then run again.\n");
	}
	return 1;
}
//! [example_join_local_app]

#ifdef _WIN32
#include <windows.h>

//! [example_find_target]
/** \brief The first render target found in the session's peer list. The peer
 *  subscription fills it on a library thread while main waits for it, so a
 *  critical section guards it. */
typedef struct ExampleTargetPicker
{
	CRITICAL_SECTION lock;
	bool found;
	QarPeerId peer_id;
} ExampleTargetPicker;

/** \brief True for a peer a source app renders for: a target app (a headset's
 *  serving app, a Visualizer receiver, a camera's view in the Visualizer) that
 *  the hub does not report as gone. Writes its id to *out_peer_id. */
static bool
example_peer_is_render_target(QarPeerSpec* peer, QarPeerId* out_peer_id)
{
	QarPeerRole role = QAR_PEER_ROLE_INTERNAL;
	QarResult got_role = qar_peer_spec_get_role(peer, &role);
	if(qar_result_is_error(got_role))
	{
		log_result("qar_peer_spec_get_role", got_role);
		return false;
	}
	/* Hubs, other source apps and pipeline infrastructure show no frames. */
	if(role != QAR_PEER_ROLE_TARGET)
	{
		return false;
	}
	QarPeerPresence presence = QAR_PEER_PRESENCE_UNKNOWN;
	QarResult got_presence = qar_peer_spec_get_presence(peer, &presence);
	if(qar_result_is_error(got_presence))
	{
		log_result("qar_peer_spec_get_presence", got_presence);
		return false;
	}
	/* OFFLINE, LEAVING and RETIRED peers are gone. UNKNOWN is a peer the hub
	 * has not reconciled yet: nothing says it is gone, so it counts. */
	if(presence != QAR_PEER_PRESENCE_ONLINE && presence != QAR_PEER_PRESENCE_UNKNOWN)
	{
		return false;
	}
	QarResult got_id = qar_peer_spec_get_id(peer, out_peer_id);
	if(qar_result_is_error(got_id))
	{
		log_result("qar_peer_spec_get_id", got_id);
		return false;
	}
	return true;
}

/** \brief Keep the peer when it is the first render target seen. */
static void
example_offer_peer(ExampleTargetPicker* picker, QarPeerSpec* peer)
{
	QarPeerId peer_id = qar_peer_id_default();
	if(!example_peer_is_render_target(peer, &peer_id))
	{
		return;
	}
	EnterCriticalSection(&picker->lock);
	const bool first = !picker->found;
	if(first)
	{
		picker->found = true;
		picker->peer_id = peer_id;
	}
	LeaveCriticalSection(&picker->lock);
	if(!first)
	{
		return;
	}
	/* The device name is what a GUI shows the user to choose a target by. */
	char name[128] = { 0 };
	QarResult got_name = qar_peer_spec_get_device_name(peer, name, sizeof(name));
	if(qar_result_is_error(got_name))
	{
		log_result("qar_peer_spec_get_device_name", got_name);
	}
	printf("Rendering for target '%s'\n", name[0] != '\0' ? name : "unnamed device");
}

/** \brief qar_peer_subscribe_updates() callback. The spec handle belongs to
 *  the library: read it during the callback, do not destroy it. */
static void
example_on_peer_update(QarPeerSpec* peer, void* user_state)
{
	example_offer_peer((ExampleTargetPicker*)user_state, peer);
}

/** \brief Offer every peer already in the session. Right after onboarding the
 *  list may still be empty; the subscription reports peers as they arrive. */
static bool
example_offer_current_peers(QarSession* session, ExampleTargetPicker* picker)
{
	size_t count = 0;
	QarResult counted = qar_query_peer_specs_count(session, &count);
	if(qar_result_is_error(counted))
	{
		log_result("qar_query_peer_specs_count", counted);
		return false;
	}
	if(count == 0)
	{
		return true;
	}
	QarPeerSpec** peers = (QarPeerSpec**)calloc(count, sizeof(QarPeerSpec*));
	if(peers == NULL)
	{
		printf("Out of memory listing %zu peers\n", count);
		return false;
	}
	size_t written = 0;
	QarResult listed = qar_query_peer_specs(session, peers, count, &written);
	if(qar_result_is_error(listed))
	{
		log_result("qar_query_peer_specs", listed);
	}
	for(size_t i = 0; i < written; ++i)
	{
		example_offer_peer(picker, peers[i]);
		qar_peer_spec_handle_destroy(peers[i]); /* listed handles are ours */
	}
	free(peers);
	return qar_result_is_success(listed);
}

/** \brief Wait up to timeout_ms for a render target to be in the session and
 *  write its id to *out_peer_id.
 *
 *  Which targets to serve is the application's decision; QAROS does not pick
 *  them. This example takes the first one. A real application keeps the
 *  subscription for its whole session, serves every target it chooses with a
 *  sender of its own and stops a sender whose target turns OFFLINE, LEAVING or
 *  RETIRED (the vulkan_cube example does).
 *
 *  Waiting for a render stream request instead would wait forever: a target
 *  requests an app's content only once a sender has attached it to the app's
 *  volume. */
static bool
example_wait_for_target(QarSession* session, DWORD timeout_ms, QarPeerId* out_peer_id)
{
	ExampleTargetPicker picker;
	InitializeCriticalSection(&picker.lock);
	picker.found = false;
	picker.peer_id = qar_peer_id_default();

	/* A subscription needs a cancel token: destroying it ends the
	 * subscription and waits for a running callback, so `picker` may go out
	 * of scope afterwards. */
	QarCancelToken* subscription = NULL;
	QarResult created = qar_cancel_token_create(&subscription);
	if(qar_result_is_error(created) || subscription == NULL)
	{
		log_result("qar_cancel_token_create", created);
		DeleteCriticalSection(&picker.lock);
		return false;
	}
	/* Subscribe before listing the current peers, so a target joining in
	 * between is not missed. */
	QarResult subscribed =
		qar_peer_subscribe_updates(session, example_on_peer_update, &picker, subscription);
	bool ok = qar_result_is_success(subscribed);
	if(!ok)
	{
		log_result("qar_peer_subscribe_updates", subscribed);
	}
	/* A failed listing is logged and not fatal: the subscription still
	 * reports every peer as it changes. */
	if(ok)
	{
		example_offer_current_peers(session, &picker);
	}

	if(ok)
	{
		printf(
			"Waiting for a target app in the session (a headset, or a receiver opened "
			"in the QAROS Visualizer) ...\n"
		);
	}
	const ULONGLONG deadline = GetTickCount64() + timeout_ms;
	bool waiting = ok;
	while(waiting && GetTickCount64() < deadline)
	{
		Sleep(100);
		EnterCriticalSection(&picker.lock);
		waiting = !picker.found;
		LeaveCriticalSection(&picker.lock);
	}

	qar_cancel_token_handle_destroy(subscription);
	/* No callback runs any more: the picker can be read without the lock. */
	const bool found = picker.found;
	*out_peer_id = picker.peer_id;
	DeleteCriticalSection(&picker.lock);
	if(ok && !found)
	{
		printf("No target app joined within %lu s.\n", (unsigned long)(timeout_ms / 1000));
	}
	return found;
}
//! [example_find_target]
#endif /* _WIN32 */

/** \brief Library path from the first argument. "installed" (what a shipped
 *  application does) returns NULL, so qar_library_load(NULL) loads the
 *  runtime of the QAROS installed on this PC and the runtime binaries folder
 *  defaults to its bin/. Any other value is the library file of a
 *  development build. */
static const char*
library_path_argument(const char* argument)
{
	return strcmp(argument, "installed") == 0 ? NULL : argument;
}

/** \brief Printable name of what qar_library_load was given. */
static const char*
describe_library(const char* library_path)
{
	return library_path != NULL ? library_path : "the installed QAROS runtime";
}
