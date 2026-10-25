/** \file session_lifecycle.c
 *  \brief A local app that follows its session through a hub's new session,
 *  a join to another hub, and a reset that requires onboarding again.
 *  \example session_lifecycle.c
 */

/** \page qar_c_tutorial_session_lifecycle Session Lifecycle
 * \tableofcontents
 *
 * \section session_lifecycle_intro What You Will Learn
 * - Read the session's lifecycle state with
 *   qar_session_get_lifecycle_state()
 * - Subscribe to its changes with qar_session_subscribe_lifecycle(), keep the
 *   callback short and react on the app's own thread
 * - Notice a move into a new session by a change of the session id, not by
 *   one particular state, and get or create what the app owns again
 * - Recover from QAR_SESSION_LIFECYCLE_STATE_RE_PAIR_REQUIRED: destroy the
 *   session, onboard again, recreate what the app owns
 *
 * \section session_lifecycle_when When It Matters
 * A session follows its hub by itself. When the hub's operator starts a new
 * session, or the hub joins another hub as its member, the same QarSession
 * moves into the new session and keeps everything the app created or took
 * over with a get_or_create call: its app volumes, GUI panels and rooms are
 * proposed again, senders renegotiate and frames pause for a few seconds.
 * Objects of other peers disappear until their owners have moved too. Only
 * the session id changes.
 *
 * On each new session id the app still calls get_or_create again for every
 * common name it owns. The calls are idempotent: each returns the same id,
 * takes the object over when it is there (a previous run of the app may have
 * left it) and re-creates it when it is missing. After onboarding again they
 * are the only way the app's objects come back.
 *
 * A hub reset to a new install, a revoked approval, or a peer that missed too
 * many rotations ends the session for good: RE_PAIR_REQUIRED. Nothing is
 * retried, and the app must onboard again.
 *
 * \section session_lifecycle_run Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target session_lifecycle
 * ./build/x64-windows/Debug/session_lifecycle.exe [run-seconds]
 * \endcode
 * While it runs (default 300 s), start a new session in the QAROS Visualizer
 * to see a move, or revoke the app's approval to see RE_PAIR_REQUIRED and the
 * new approval prompt.
 *
 * \section session_lifecycle_callback Step 1: A Short Callback
 * The callback runs on a library thread. It copies the state into a mailbox
 * and returns; `info` and its `reason` are valid only during the call. Never
 * destroy the session or the library from it.
 * \snippet session_lifecycle.c lifecycle_callback
 *
 * \section session_lifecycle_subscribe Step 2: Subscribe
 * The first callback carries the current state. The subscription belongs to
 * its cancel token: destroying the token ends it and waits for a running
 * callback.
 * \snippet session_lifecycle.c lifecycle_subscribe
 *
 * \section session_lifecycle_react Step 3: React on the App's Thread
 * - A move reports MIGRATING with the old id, then CONNECTED with the new id,
 *   the previous id and a cause. A move whose new session answers late reports
 *   HUB_UNREACHABLE with the new id first, and a callback that falls behind
 *   sees only the newest state. Compare the session id with the last one seen.
 * - The cause (ROTATED, JOINED_PRIMARY) is for logs only: the move is the
 *   same, and a join can be reported as ROTATED.
 * - HUB_UNREACHABLE alone needs nothing: the session keeps trying.
 * - On a new session id, get or create what the app owns again.
 * \snippet session_lifecycle.c lifecycle_react
 *
 * \section session_lifecycle_repair Step 4: Onboard Again After RE_PAIR_REQUIRED
 * \snippet session_lifecycle.c lifecycle_repair
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

#define APP_TAG "main.session-lifecycle.examples.quaternar"

//! [lifecycle_callback]
/** \brief The newest lifecycle state, handed from the library thread to main. */
typedef struct LifecycleMailbox
{
	CRITICAL_SECTION lock;
	bool has_news;
	QarSessionLifecycleInfo latest; /* `reason` is not kept: see below */
	char reason[256];
} LifecycleMailbox;

static void
on_lifecycle(const QarSessionLifecycleInfo* info, void* user_state)
{
	LifecycleMailbox* mailbox = (LifecycleMailbox*)user_state;
	EnterCriticalSection(&mailbox->lock);
	mailbox->latest = *info;
	mailbox->latest.reason = NULL; /* valid only during this callback */
	snprintf(
		mailbox->reason, sizeof(mailbox->reason), "%s", info->reason != NULL ? info->reason : ""
	);
	mailbox->has_news = true;
	LeaveCriticalSection(&mailbox->lock);
}
//! [lifecycle_callback]

static const char*
state_name(QarSessionLifecycleState state)
{
	switch(state)
	{
	case QAR_SESSION_LIFECYCLE_STATE_CONNECTED:
		return "CONNECTED";
	case QAR_SESSION_LIFECYCLE_STATE_MIGRATING:
		return "MIGRATING";
	case QAR_SESSION_LIFECYCLE_STATE_RE_PAIR_REQUIRED:
		return "RE_PAIR_REQUIRED";
	default: /* a value this header does not name reads as unreachable */
		return "HUB_UNREACHABLE";
	}
}

static const char*
cause_name(QarSessionChangeCause cause)
{
	switch(cause)
	{
	case QAR_SESSION_CHANGE_CAUSE_NONE:
		return "none";
	case QAR_SESSION_CHANGE_CAUSE_JOINED_PRIMARY:
		return "the hub joined another hub";
	default: /* a value this header does not name reads as ROTATED */
		return "the hub started a new session";
	}
}

static void
print_session_id(const char* label, const QarSessionId* id)
{
	char text[64] = { 0 };
	QarResult formatted = qar_uuid_to_string(id->data, text, sizeof(text));
	if(qar_result_is_error(formatted))
	{
		log_result("qar_uuid_to_string", formatted);
		return;
	}
	printf("%s %s\n", label, text);
}

/** \brief Get or create what this app owns in the session: here one app
 *  volume. Idempotent: the same id every time; an existing volume is taken
 *  over and carried into the hub's next session, a missing one is created.
 *  Called on every start, on every new session id and after onboarding
 *  again. */
static bool
get_or_create_owned_objects(QarSession* session)
{
	QarAppVolumeInit volume_init = qar_app_volume_init_default();
	volume_init.common_name = "lifecycle-volume.examples.qaros";
	volume_init.display_name = "Lifecycle Tutorial Volume";
	volume_init.pose = qar_pose_default();
	volume_init.pose.position.z = -1.0f;
	volume_init.size = qar_app_volume_size_default();
	QarAppVolumeId volume_id = qar_app_volume_id_default();
	QarResult created = qar_app_volumes_get_or_create(session, &volume_init, &volume_id);
	if(qar_result_is_error(created))
	{
		log_result("qar_app_volumes_get_or_create", created);
		return false;
	}
	return true;
}

//! [lifecycle_subscribe]
/** \brief Subscribe to the session's lifecycle. Returns the token owning the
 *  subscription, or NULL on failure. */
static QarCancelToken*
subscribe_lifecycle(QarSession* session, LifecycleMailbox* mailbox)
{
	QarCancelToken* token = NULL;
	QarResult created = qar_cancel_token_create(&token);
	if(qar_result_is_error(created) || token == NULL)
	{
		log_result("qar_cancel_token_create", created);
		return NULL;
	}
	QarResult subscribed = qar_session_subscribe_lifecycle(session, on_lifecycle, mailbox, token);
	if(qar_result_is_error(subscribed))
	{
		log_result("qar_session_subscribe_lifecycle", subscribed);
		qar_cancel_token_handle_destroy(token);
		return NULL;
	}
	return token;
}
//! [lifecycle_subscribe]

/** \brief Join as a local app, create what the app owns and subscribe.
 *  Returns false, with nothing left open, on failure. */
static bool
start_session(
	QarRuntime* runtime,
	LifecycleMailbox* mailbox,
	QarSession** out_session,
	QarCancelToken** out_token,
	QarSessionId* out_seen_id
)
{
	*out_session = NULL;
	*out_token = NULL;
	if(example_join_as_local_app(runtime, "Session Lifecycle Example", APP_TAG, out_session) != 0
	   || *out_session == NULL)
	{
		return false;
	}
	/* The state now, without a subscription: the id to compare against. */
	QarSessionLifecycleInfo now = qar_session_lifecycle_info_default();
	QarResult got_state = qar_session_get_lifecycle_state(*out_session, &now);
	if(qar_result_is_error(got_state))
	{
		log_result("qar_session_get_lifecycle_state", got_state);
	}
	else
	{
		printf("State %s\n", state_name(now.state));
		print_session_id("In session", &now.session_id);
		*out_seen_id = now.session_id;
	}
	if(qar_result_is_success(got_state) && get_or_create_owned_objects(*out_session))
	{
		*out_token = subscribe_lifecycle(*out_session, mailbox);
	}
	if(*out_token == NULL)
	{
		qar_session_handle_destroy(*out_session);
		*out_session = NULL;
		return false;
	}
	return true;
}

/** \brief End the subscription before the session: the token waits for a
 *  running callback, so no callback touches the mailbox afterwards. A state
 *  the old session delivered since main last looked is dropped with it. */
static void
stop_session(QarSession* session, QarCancelToken* token, LifecycleMailbox* mailbox)
{
	qar_cancel_token_handle_destroy(token);
	qar_session_handle_destroy(session);
	mailbox->has_news = false;
}

int
main(int argc, char** argv)
{
	const int requested_seconds = argc > 1 ? atoi(argv[1]) : 0;
	const int run_seconds = requested_seconds > 0 ? requested_seconds : 300;

	if(!qar_library_load(NULL)) /* NULL: the default lookup, normally the installed QAROS */
	{
		fprintf(
			stderr,
			"Could not load the QAROS runtime (reason above): install QAROS, or fix "
			"QAROS_RUNTIME_DIR.\n"
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
	QarRuntime* runtime = NULL;
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	QarResult runtime_result = qar_runtime_create(&runtime_init, &runtime);
	if(qar_result_is_error(runtime_result) || runtime == NULL)
	{
		log_result("qar_runtime_create", runtime_result);
		log_result("qar_library_destroy", qar_library_destroy());
		qar_library_unload();
		return 4;
	}

	LifecycleMailbox mailbox;
	InitializeCriticalSection(&mailbox.lock);
	mailbox.has_news = false;
	mailbox.latest = qar_session_lifecycle_info_default();
	mailbox.reason[0] = '\0';

	QarSession* session = NULL;
	QarCancelToken* token = NULL;
	QarSessionId seen_id;
	memset(&seen_id, 0, sizeof(seen_id));
	int exit_code = 0;
	int rejoins = 0;
	if(!start_session(runtime, &mailbox, &session, &token, &seen_id))
	{
		exit_code = 5;
	}

	const ULONGLONG deadline = GetTickCount64() + (ULONGLONG)run_seconds * 1000;
	while(session != NULL && GetTickCount64() < deadline)
	{
		Sleep(200);
		//! [lifecycle_react]
		EnterCriticalSection(&mailbox.lock);
		const bool has_news = mailbox.has_news;
		const QarSessionLifecycleInfo info = mailbox.latest;
		char reason[sizeof(mailbox.reason)];
		memcpy(reason, mailbox.reason, sizeof(reason));
		mailbox.has_news = false;
		LeaveCriticalSection(&mailbox.lock);
		if(!has_news)
		{
			continue;
		}
		printf("State %s%s%s\n", state_name(info.state), reason[0] != '\0' ? ": " : "", reason);
		if(memcmp(info.session_id.data, seen_id.data, sizeof(seen_id.data)) != 0)
		{
			/* A new session: re-read whatever the app keyed by the session id,
			 * and get or create what the app owns again (the same ids; a
			 * missing object comes back). Handles and senders are still there. */
			seen_id = info.session_id;
			print_session_id("Now in session", &info.session_id);
			if(info.has_previous_session)
			{
				print_session_id("  moved from", &info.previous_session_id);
				printf("  because %s\n", cause_name(info.cause));
			}
			if(info.state != QAR_SESSION_LIFECYCLE_STATE_RE_PAIR_REQUIRED
			   && !get_or_create_owned_objects(session))
			{
				printf("Could not get or create the app's volume in the new session.\n");
			}
		}
		//! [lifecycle_react]

		//! [lifecycle_repair]
		if(info.state == QAR_SESSION_LIFECYCLE_STATE_RE_PAIR_REQUIRED)
		{
			/* Terminal: nothing is retried. Leave the old session, join again
			 * (the user may be asked to approve the app again) and recreate
			 * what the app owns. Here, on the app's thread, never in the
			 * callback. A hub that keeps refusing is not asked forever. */
			printf("The hub no longer admits this app. Joining again ...\n");
			stop_session(session, token, &mailbox);
			session = NULL;
			if(++rejoins > 3)
			{
				printf("Still refused after 3 joins; giving up.\n");
				exit_code = 6;
			}
			else if(!start_session(runtime, &mailbox, &session, &token, &seen_id))
			{
				exit_code = 6;
			}
		}
		//! [lifecycle_repair]
	}

	if(session != NULL)
	{
		stop_session(session, token, &mailbox);
	}
	DeleteCriticalSection(&mailbox.lock);
	qar_runtime_handle_destroy(runtime);
	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	return exit_code;
}
