/** \file remote_app_onboarding.c
 *  \brief An app on the primary hub's PC grants one app on another PC: the
 *  invitation joins that PC's hub as a member and admits the app there with
 *  no prompt, under a peer id the primary remembers.
 *  \example remote_app_onboarding.c
 */

/** \page qar_c_tutorial_remote_app_onboarding Remote App on Another PC
 * \tableofcontents
 *
 * \section remote_app_intro What You Will Learn
 * - Create a member-hub invitation that carries a remote-app grant
 *   (\ref QarMemberHubInvitationAppGrantExt) on the primary hub's PC
 * - Carry it to the other PC over your application's own channel
 * - Join that PC's hub to the primary as a member
 *   (\ref qar_runtime_join_hub_as_member)
 * - Onboard the app there as a local app, redeeming the grant
 *   (\ref QarOnboardRemoteAppGrantExt), and later start it with nothing
 *
 * \section remote_app_when When to Use It
 * Your application already spans two PCs (a simulation server on the QAROS
 * PC and its clients on other PCs) and the part on the other PC should stream
 * into the same QAROS session, without an operator walking over to pair it.
 * The other PC needs QAROS installed; its hub becomes a member of the primary
 * hub. Windows only.
 *
 * An app cannot obtain a pairing code or an unbound invite for another PC:
 * the grant is the only app-issued way in, and it admits exactly one app.
 *
 * \section remote_app_run Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target remote_app_onboarding
 * # On the primary hub's PC (the app's approval needs "May bring in member hubs"):
 * remote_app_onboarding.exe issue | <your encrypted, authenticated channel>
 * # On the other PC, first start:
 * <your channel> | remote_app_onboarding.exe join
 * # On the other PC, every later start:
 * remote_app_onboarding.exe start
 * \endcode
 * `issue` writes the invitation to stdout and refuses to print it on a
 * console, so it never lands on screen or in a terminal log; its messages go
 * to stderr. To try it out by hand, redirect it to a file you delete right
 * after `join` read it.
 *
 * \section remote_app_issue Step 1: Issue the Invitation on the Primary's PC
 * \snippet remote_app_onboarding.c remote_app_issue
 *
 * What the primary hub checks:
 * - The calling process is an app it approved (the hub identifies it from
 *   Windows, as for a local app: \ref qar_c_tutorial_local_app_onboarding).
 *   This example joins as a local app first so the user can approve it.
 *   Unknown: `QAR_STATUS_ONBOARDING_APPROVAL_DENIED`.
 * - The operator gave that approval the right to bring in member hubs, with
 *   quota left: otherwise `QAR_STATUS_MEMBER_SPONSOR_NOT_ALLOWED` or
 *   `QAR_STATUS_MEMBER_SPONSOR_QUOTA_EXCEEDED`.
 * - The hub on this PC is a primary, not a member itself
 *   (`QAR_STATUS_SESSION_WIDE_ACTION_ON_MEMBER`).
 * - The grant's tag follows the app tag rules
 *   (`QAR_STATUS_ARGUMENT_NOT_SUPPORTED` otherwise).
 *
 * The invitation is signed by the primary hub, valid 24 h and for one hub by
 * default (\ref QarMemberHubInvitationInit), and listed in the primary's
 * visualizer, where the operator can revoke it. It is a **bearer secret**:
 * whoever holds it can join a hub until it expires or is used up. Send it only
 * over an encrypted, authenticated channel, never log it, never put it on a
 * command line.
 *
 * \section remote_app_join Step 2: Join the Other PC's Hub as a Member
 * \snippet remote_app_onboarding.c remote_app_join
 *
 * - This joins the **hub** of that PC, not the app. The call starts the
 *   installed hub when none runs, checks the invitation's signature and
 *   expiry, checks that this PC trusts the primary (an administrator's
 *   allow-list, else the primary this hub joined first), joins and waits
 *   through the hub's restart into the primary's session.
 * - Typical failures: `QAR_STATUS_MEMBER_INVITATION_EXPIRED`, `_USED_UP`,
 *   `_REVOKED` (ask for a new invitation), `_PRIMARY_NOT_ALLOWED` (this PC
 *   does not trust that primary), `QAR_STATUS_PRIMARY_UNREACHABLE`,
 *   `QAR_STATUS_TIMEOUT` (the join is committed; calling again succeeds once
 *   the hub is back).
 * - Called again for the primary it already belongs to, it succeeds at once.
 *
 * \section remote_app_redeem Step 3: Onboard, Redeeming the Grant
 * \snippet remote_app_onboarding.c remote_app_redeem
 *
 * - The first executable that redeems the grant is approved on the primary
 *   without a prompt and recorded by its publisher (signed) or file hash
 *   (unsigned). A different executable is refused afterwards
 *   (`QAR_STATUS_ONBOARDING_APPROVAL_DENIED`), also with a later invitation of
 *   the same app and tag.
 * - The primary picks the peer id and remembers it under the grant's tag; no
 *   \ref QarOnboardLocalAppTagExt is needed.
 * - With a grant, the issuing app's own approval is not extended to the
 *   member hub: the grant admits this one app only.
 *
 * \section remote_app_later Step 4: Every Later Start
 * \snippet remote_app_onboarding.c remote_app_start
 * A plain local onboard: the app is approved now, so no invitation and no
 * prompt are needed, and the primary gives it the same peer id while no other
 * live process holds it.
 *
 * \section remote_app_stored What Is Stored, and for How Long
 * - Either app: nothing; onboarding ids stay zero.
 * - The channel: the invitation, until used up or expired (default one hub,
 *   24 h).
 * - The remote app's memory: key and a 30-minute certificate, renewed while
 *   it runs, as for any local app.
 * - The primary hub: the granted approval (until revoked) and the remembered
 *   peer id (until unused for 30 days).
 *
 * Revoking the issuing app's approval, or the granted one, on the primary
 * revokes the remote app and forgets its peer id.
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#define example_isatty _isatty
#define example_fileno _fileno
#else
#include <unistd.h>
#define example_isatty isatty
#define example_fileno fileno
#endif

QAR_IMPLEMENT_DYNAMIC_LOADING()

/* The issuing app's own tag on the primary, and the tag the grant admits the
 * remote app under. One grant admits one app: issue one tag per remote app. */
#define ISSUER_TAG "server.remote-app-onboarding.examples.quaternar"
#define GRANT_TAG "client.remote-app-onboarding.examples.quaternar"
/* A serialized invitation is at most 16 KiB. */
#define MAX_INVITATION_BYTES (16 * 1024)

/* The invitation's bytes pass unchanged (no newline translation). */
static bool
set_binary(FILE* stream)
{
#ifdef _WIN32
	if(_setmode(_fileno(stream), _O_BINARY) == -1)
	{
		fprintf(stderr, "Could not switch a standard stream to binary mode.\n");
		return false;
	}
#else
	(void)stream; /* POSIX streams do not translate */
#endif
	return true;
}

/* Messages go to stderr: in the issue role stdout carries the invitation. */
static void
report(const char* label, QarResult result)
{
	char message[512] = { 0 };
	if(qar_result_is_success(result))
	{
		fprintf(stderr, "%s: ok\n", label);
		return;
	}
	qar_result_message(result, message, sizeof(message));
	fprintf(stderr, "%s: error %d: %s\n", label, (int)result.code, message);
}

static void
on_progress(QarActionSeverity severity, float percent, const char* message, void* state)
{
	(void)severity;
	(void)percent;
	(void)state;
	if(message != NULL && message[0] != '\0')
	{
		fprintf(stderr, "%s\n", message);
	}
}

/* A local onboard; `grant` (optional) redeems a remote-app grant. */
static QarResult
onboard_local(
	QarRuntime* runtime,
	const char* tag,
	const QarMemberHubInvitation* grant,
	QarSession** out_session
)
{
	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = "Remote App Onboarding Example";
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	QarOnboardLocalAppTagExt tag_ext = qar_onboard_local_app_tag_ext_default();
	QarOnboardRemoteAppGrantExt redeem = qar_onboard_remote_app_grant_ext_default();
	init.header.next = &local.header;
	if(grant != NULL)
	{
		redeem.invitation = grant; /* borrowed; the grant's tag applies */
		local.header.next = &redeem.header;
	}
	else if(tag != NULL)
	{
		tag_ext.app_tag = tag;
		local.header.next = &tag_ext.header;
	}
	QarOnboardingId unused = qar_onboarding_id_default(); /* stays zero */
	return qar_runtime_onboard(runtime, &init, on_progress, NULL, NULL, &unused, out_session);
}

/* Role "issue", on the primary hub's PC. */
static int
issue(QarRuntime* runtime)
{
	if(example_isatty(example_fileno(stdout)))
	{
		fprintf(
			stderr,
			"Refusing to print the invitation on a console: pipe stdout into your channel.\n"
		);
		return 1;
	}
	/* The issuing app normally is in the session anyway; joining first also
	 * lets the user approve it. */
	QarSession* session = NULL;
	QarResult joined = onboard_local(runtime, ISSUER_TAG, NULL, &session);
	report("qar_runtime_onboard (issuer)", joined);
	if(qar_result_is_error(joined))
	{
		return 1;
	}

	int exit_code = 1;
	//! [remote_app_issue]
	QarMemberHubInvitationInit init = qar_member_hub_invitation_init_default();
	init.label = "Remote app example"; /* shown to the primary's operator */
	/* Defaults: valid 24 h, one hub. Keep both as small as your flow allows. */
	QarMemberHubInvitationAppGrantExt grant = qar_member_hub_invitation_app_grant_ext_default();
	grant.app_tag = GRANT_TAG;
	init.header.next = &grant.header;

	QarMemberHubInvitation* invitation = NULL;
	QarResult created = qar_runtime_create_member_hub_invitation(runtime, &init, NULL, &invitation);
	report("qar_runtime_create_member_hub_invitation", created);
	if(qar_result_has_code(created, QAR_STATUS_MEMBER_SPONSOR_NOT_ALLOWED))
	{
		fprintf(stderr, "Ask the operator to allow this app to bring in member hubs.\n");
	}

	size_t size = 0;
	uint8_t* bytes = NULL;
	if(qar_result_is_success(created))
	{
		QarResult sized = qar_member_hub_invitation_serialized_size(invitation, &size);
		if(qar_result_is_success(sized) && size > 0 && size <= MAX_INVITATION_BYTES)
		{
			bytes = (uint8_t*)calloc(1, size);
		}
		else
		{
			report("qar_member_hub_invitation_serialized_size", sized);
		}
	}
	size_t written = 0;
	if(bytes != NULL)
	{
		QarResult serialized =
			qar_member_hub_invitation_serialize(invitation, bytes, size, &written);
		report("qar_member_hub_invitation_serialize", serialized); /* never the bytes */
		/* stdout stands in for your channel to the other PC. */
		if(qar_result_is_success(serialized) && set_binary(stdout)
		   && fwrite(bytes, 1, written, stdout) == written && fflush(stdout) == 0)
		{
			exit_code = 0;
		}
		else
		{
			fprintf(stderr, "The invitation was not handed over.\n");
		}
		memset(bytes, 0, size); /* clear the secret before freeing it */
		free(bytes);
	}
	qar_member_hub_invitation_handle_destroy(invitation); /* NULL is ignored */
	//! [remote_app_issue]

	qar_session_handle_destroy(session);
	return exit_code;
}

/* Reads the whole invitation from stdin into `buffer`. */
static bool
read_invitation(uint8_t* buffer, size_t buffer_size, size_t* out_size)
{
	if(!set_binary(stdin))
	{
		return false;
	}
	*out_size = fread(buffer, 1, buffer_size, stdin);
	if(ferror(stdin) || *out_size == 0 || *out_size == buffer_size)
	{
		return false; /* nothing, or more than an invitation can be */
	}
	return true;
}

/* Role "join", on the other PC, first start. */
static int
join(QarRuntime* runtime)
{
	int exit_code = 1;
	uint8_t* bytes = (uint8_t*)calloc(1, MAX_INVITATION_BYTES + 1);
	size_t size = 0;
	QarMemberHubInvitation* invitation = NULL;
	QarResult parsed = qar_result_success();
	const bool got_bytes = bytes != NULL && read_invitation(bytes, MAX_INVITATION_BYTES + 1, &size);
	if(got_bytes)
	{
		/* Checks the format only; the signature is checked by the join. */
		parsed = qar_member_hub_invitation_deserialize(bytes, size, &invitation);
		report("qar_member_hub_invitation_deserialize", parsed);
	}
	else
	{
		fprintf(stderr, "No invitation on stdin.\n");
	}
	if(bytes != NULL)
	{
		memset(bytes, 0, MAX_INVITATION_BYTES + 1); /* the handle keeps its own copy */
		free(bytes);
	}
	if(!got_bytes || qar_result_is_error(parsed))
	{
		return 1;
	}

	//! [remote_app_join]
	QarJoinHubAsMemberInit join_init = qar_join_hub_as_member_init_default();
	join_init.invitation = invitation; /* borrowed for the call */
	QarResult joined_hub =
		qar_runtime_join_hub_as_member(runtime, &join_init, on_progress, NULL, NULL);
	report("qar_runtime_join_hub_as_member", joined_hub);
	if(qar_result_has_code(joined_hub, QAR_STATUS_TIMEOUT))
	{
		fprintf(stderr, "The join is committed; run join again once the hub is back.\n");
	}
	//! [remote_app_join]

	if(qar_result_is_success(joined_hub))
	{
		//! [remote_app_redeem]
		QarSession* session = NULL;
		QarResult joined = onboard_local(runtime, NULL, invitation, &session);
		report("qar_runtime_onboard (remote app)", joined);
		if(qar_result_has_code(joined, QAR_STATUS_ONBOARDING_APPROVAL_DENIED))
		{
			fprintf(stderr, "The grant already admitted another app, or it was revoked.\n");
		}
		//! [remote_app_redeem]
		if(qar_result_is_success(joined))
		{
			exit_code = example_print_own_peer_id("Remote app", session) ? 0 : 1;
			qar_session_handle_destroy(session);
		}
	}
	qar_member_hub_invitation_handle_destroy(invitation);
	return exit_code;
}

/* Role "start", on the other PC, every later start. */
static int
start(QarRuntime* runtime)
{
	//! [remote_app_start]
	QarSession* session = NULL;
	/* No tag needed: the primary remembers a granted app under the grant's
	 * tag, whatever tag the app sends. */
	QarResult joined = onboard_local(runtime, NULL, NULL, &session);
	report("qar_runtime_onboard (remote app)", joined);
	//! [remote_app_start]
	if(qar_result_is_error(joined))
	{
		return 1;
	}
	const int exit_code = example_print_own_peer_id("Remote app", session) ? 0 : 1;
	qar_session_handle_destroy(session);
	return exit_code;
}

int
main(int argc, char** argv)
{
	const char* role = argc >= 2 ? argv[1] : "";
	if(strcmp(role, "issue") != 0 && strcmp(role, "join") != 0 && strcmp(role, "start") != 0)
	{
		fprintf(
			stderr, "Usage: %s issue|join|start\n", argc >= 1 ? argv[0] : "remote_app_onboarding"
		);
		return 1;
	}

	if(!qar_library_load(NULL))
	{
		fprintf(stderr, "QAROS is not installed on this PC.\n");
		return 2;
	}
	QarLibraryInit library_init = qar_library_init_default();
	library_init.enable_console_logging = false; /* keep stdout for the invitation */
	QarResult initialized = qar_library_init(&library_init);
	if(qar_result_is_error(initialized))
	{
		report("qar_library_init", initialized);
		qar_library_unload();
		return 1;
	}

	int exit_code = 1;
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	QarRuntime* runtime = NULL;
	QarResult created = qar_runtime_create(&runtime_init, &runtime);
	report("qar_runtime_create", created);
	if(qar_result_is_success(created))
	{
		if(strcmp(role, "issue") == 0)
		{
			exit_code = issue(runtime);
		}
		else if(strcmp(role, "join") == 0)
		{
			exit_code = join(runtime);
		}
		else
		{
			exit_code = start(runtime);
		}
		qar_runtime_handle_destroy(runtime);
	}

	report("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
	return exit_code;
}
