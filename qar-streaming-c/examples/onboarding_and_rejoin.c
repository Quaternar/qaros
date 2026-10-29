/** \file onboarding_and_rejoin.c
 *  \brief Obtains a session by rejoin-or-onboard with a pairing code, on this
 *  machine or at a Hub address, joins a second runtime with an invite, and
 *  leaves with destroy or forget.
 *  \example onboarding_and_rejoin.c
 */

/* The page holds tables and command lines wider than the column limit. */
// clang-format off
/** \page qar_c_tutorial_onboarding Onboarding and Rejoin
 * \tableofcontents
 *
 * \section onboarding_build What you build
 * A program that gets a QAROS session with a pairing code. On its first run
 * it pairs with the code the Hub shows and saves the onboarding id it gets
 * back. On every later run it rejoins with that id, silently. It also joins a
 * second runtime in the same process with an invite instead of a second code,
 * then leaves, keeping both identities, or erasing them with `--forget`.
 *
 * Without a Hub address, the code finds a Hub of your own user: first through
 * the `hub-endpoint.json` file the Hub writes, then through a discovery beacon
 * whose owner is your user. That is how it runs on the Hub machine. On
 * another machine, give the Hub's address with `--hub=HOST:PORT`: the program
 * then chains a QarOnboardHostExt after the code extension, with the Hub's
 * `hostname` and onboarding `port`.
 *
 * What you will learn:
 * - the difference between a session handle, which lives for one run, and
 *   the onboarding id, which you persist
 * - the rejoin-or-onboard startup, and which status codes are normal flow
 * - how to reach a Hub on another machine with QarOnboardHostExt
 * - how a second API instance joins with an invite, and why it rejoins
 *   afterwards instead of asking for a new invite
 * - what destroying a session does, and when to call \ref qar_runtime_forget
 *
 * A source app on the Hub machine needs no code: it joins as a local app with
 * nothing to persist
 * (\ref qar_c_tutorial_local_app_onboarding "Local App Onboarding").
 * This program pairs with a code anyway, to show the path of an app on
 * another machine.
 *
 * \section onboarding_prereq Before you start
 * - Work through \ref qar_c_tutorial_dynamic_loading "Dynamic Loading"
 *   first: this program loads the runtime the same way.
 * - **Windows**, with QAROS installed. On the Hub machine, the Hub runs under
 *   your Windows user.
 * - **A pairing code** for the first run. The Visualizer on the Hub machine
 *   shows the current one: open the Connection panel, then **Hubs** >
 *   **Pair another Hub** > **Let another hub join us**
 *   (<a href="/docs/operator-guide/managing-devices#reading-this-hubs-code">Reading this Hub's code</a>).
 *   A code works once and lives about ten seconds, so type it quickly.
 * - **The Hub's address**, to pair from another machine. The same card shows
 *   the Hub's addresses, and the onboarding port is shown with the code
 *   (<a href="/docs/operator-guide/running-the-hub#running-several-hubs-on-one-pc">Running several Hubs</a>).
 *   A Hub has no fixed onboarding port.
 *
 * \section onboarding_run Run it
 * The program writes its state into the working directory: a storage folder
 * per runtime (`onboarding_and_rejoin.primary-state`,
 * `onboarding_and_rejoin.sibling-state`) and the two saved ids
 * (`onboarding_and_rejoin.primary-onboarding-id.txt`,
 * `onboarding_and_rejoin.sibling-onboarding-id.txt`).
 * \code{.bash}
 * # First run: pair with the code the Hub shows.
 * .\build\x64-windows\qar-streaming-c\examples\Debug\onboarding_and_rejoin.exe <PAIRING-CODE>
 *
 * # First run on another machine: also name the Hub.
 * .\build\x64-windows\qar-streaming-c\examples\Debug\onboarding_and_rejoin.exe <PAIRING-CODE> --hub=<HOST>:<PORT>
 *
 * # Later runs: rejoin with the saved ids. No code.
 * .\build\x64-windows\qar-streaming-c\examples\Debug\onboarding_and_rejoin.exe
 *
 * # Leave and erase both identities. The next run needs a code again.
 * .\build\x64-windows\qar-streaming-c\examples\Debug\onboarding_and_rejoin.exe --forget
 * \endcode
 *
 * The expected output of the first run, between the runtime's own log lines,
 * with your runtime's version, ids and invite details in place of the
 * placeholders:
 * \code{.txt}
 * Loading the QAROS runtime (default lookup)
 * Loaded C:\Program Files\Quaternar\QAROS\bin\qar-streaming-c.dll (the installed QAROS)
 * Runtime <version> (release, C API v0)
 * qar_runtime_create: SUCCESS (code=0)
 * qar_runtime_onboard: SUCCESS (code=0)
 * Primary session obtained with onboarding id <primary-id>
 * qar_runtime_create: SUCCESS (code=0)
 * qar_session_request_onboarding_invite: SUCCESS (code=0)
 * Minted sibling invite (<size> bytes, expires at unix <time>).
 * qar_onboarding_invite_deserialize: SUCCESS (code=0)
 * qar_runtime_onboard: SUCCESS (code=0)
 * Sibling session obtained with onboarding id <sibling-id>
 * Destroying the session handles keeps both identities for the next run.
 * qar_library_destroy: SUCCESS (code=0)
 * \endcode
 *
 * A later run prints `qar_runtime_rejoin: SUCCESS (code=0)` for each runtime
 * where the first run onboarded, with the same two ids, and mints no invite.
 * The run with `--forget` ends with:
 * \code{.txt}
 * Destroying the session handles keeps both identities for the next run.
 * qar_runtime_forget(sibling): SUCCESS (code=0)
 * qar_runtime_forget(primary): SUCCESS (code=0)
 * qar_library_destroy: SUCCESS (code=0)
 * \endcode
 *
 * A run without a code and without a saved id stops before it reaches the
 * Hub:
 * \code{.txt}
 * qar_runtime_create: SUCCESS (code=0)
 * No persisted onboarding id in 'onboarding_and_rejoin.primary-onboarding-id.txt'. Pass the pairing code as the first argument.
 * The Visualizer on the Hub machine shows the current code under Connection panel, Hubs > Pair another Hub > Let another hub join us.
 * qar_library_destroy: SUCCESS (code=0)
 * \endcode
 *
 * Exit codes:
 * | Code | Meaning |
 * |---|---|
 * | 0 | both sessions obtained and left |
 * | 1 | an unknown argument, more than one code, a `--hub` value that is not `HOST:PORT`, `qar_library_init` or `qar_runtime_create` failed, or no session: no code, a rejected code, a real rejoin error, or a failed invite |
 * | 2 | no runtime loaded |
 *
 * \section onboarding_walkthrough Walkthrough
 *
 * \subsection onboarding_args_step 1. Read the arguments
 * \snippet onboarding_and_rejoin.c onboarding_args
 *
 * The first argument that does not start with `--` is the pairing code;
 * `--forget` asks for the full un-pair at the end. A code on the command line
 * is fine for a tutorial. In an application, ask for it in your UI and never
 * log it.
 *
 * `--hub=HOST:PORT` names the Hub to pair with. The program splits it at the
 * last colon into a QarOnboardHostExt:
 * \snippet onboarding_and_rejoin.c onboarding_hub
 *
 * `hostname` is a plain host name or IP address, without a URL scheme.
 * `port` is the Hub's onboarding port. There is no default port, and a `port`
 * of 0 is rejected with `QAR_STATUS_ARGUMENT_NOT_SUPPORTED`, so the program
 * refuses it at once.
 *
 * \subsection onboarding_primary_step 2. Create a runtime with its own storage, and obtain a session
 * The program loads the library with the helper from
 * \ref qar_c_tutorial_dynamic_loading "Dynamic Loading"
 * (`example_load_library` in `common.h`), then:
 * \snippet onboarding_and_rejoin.c onboarding_primary
 *
 * A runtime keeps its identities (certificates and session state) in a
 * storage folder. Without one, the library uses its default folder (see
 * QarRuntimeInit::storage_folder_path). Give every runtime in one
 * application a folder of its own, as this program does for both of its
 * runtimes, so their identities never mix:
 * \snippet onboarding_and_rejoin.c onboarding_files
 * \snippet common.h example_create_runtime
 *
 * `example_obtain_session` returns two things with different lifetimes. The
 * QarSession pointer is a handle for this run only. The QarOnboardingId is a
 * 16-byte key to the identity stored in the runtime's storage folder, and it
 * outlives the process: it is the one thing your app must save.
 *
 * \subsection onboarding_obtain_step 3. Rejoin, or onboard with the code
 * This is the canonical startup:
 * \snippet common.h example_obtain_session
 *
 * - **Rejoin first.** \ref qar_runtime_rejoin shows no UI and needs no code.
 *   Call it on every start that has a saved id.
 * - **`QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND` is normal flow.** It means the
 *   id has no stored identity in this runtime's storage folder, for example
 *   after a forget or with a new folder. Onboard instead; without a code, the
 *   program asks for a new one.
 * - **Any other rejoin error is real.** `QAR_STATUS_ONBOARDING_HUB_UNREACHABLE`,
 *   for example, means no Hub answered. Report it and stop; asking for a
 *   pairing code would not help.
 * \snippet common.h example_rejoin
 *
 * - **Onboard with the code.** QarOnboardCodeExt without a QarOnboardHostExt
 *   finds a Hub of your own user: first through the `hub-endpoint.json` file
 *   the Hub writes into the QAROS data folder, then through a discovery beacon
 *   whose owner is your user, listening up to 6 seconds. With `--hub`, the
 *   QarOnboardHostExt chained after the code extension names the Hub instead.
 *   A rejoin needs no address: QarRejoinInit has no field for one.
 * - **`QAR_STATUS_PAKE_ERROR` is normal flow too.** The code was wrong or has
 *   expired. Read the current code and try again.
 * - **Save the id at once.** \ref qar_runtime_onboard writes the new id to
 *   its `out_onboarding_id` argument. Without the saved id the next start
 *   must pair again.
 * \snippet common.h example_onboard_with_code
 *
 * Both calls block until the Hub answers. This program passes no progress
 * callback and no cancel token; an application with a UI passes a
 * QarCancelToken so the user can abort.
 *
 * \subsection onboarding_persist_step 4. Save the onboarding id
 * \snippet common.h example_onboarding_persistence
 *
 * The id is written in its UUID text form (\ref qar_uuid_to_string) and read
 * back with \ref qar_uuid_from_string. An unreadable file is reported and
 * treated as no id, so the program asks for a code instead of failing.
 *
 * A write that fails, also at the close that flushes it, is reported too. The
 * session of this run still works, but the next run finds no id: it needs a
 * new pairing code, and the sibling a new invite. The identity onboarded now
 * stays in the storage folder, and without its id the program cannot forget
 * it.
 *
 * \subsection onboarding_sibling_step 5. Join a second runtime
 * A second QAROS API instance of the same application, such as a helper
 * process or a plug-in host, must not pair with a code again
 * (<a href="/docs/developer-guide/onboarding-and-sessions#sibling-onboarding-one-app-several-api-instances">Sibling onboarding</a>).
 * It gets its own runtime and storage folder, and an onboarding id of its
 * own:
 * \snippet onboarding_and_rejoin.c onboarding_sibling
 *
 * Like the primary, the sibling rejoins with its saved id first. Only when it
 * has no identity yet does it ask the primary for an invite. An invite onboard
 * creates a new stored identity every time, so asking on every run would
 * leave one more identity behind in the sibling's storage folder per run:
 * \snippet onboarding_and_rejoin.c onboarding_sibling_join
 *
 * \subsection onboarding_invite_step 6. Mint an invite
 * \snippet onboarding_and_rejoin.c onboarding_mint_invite
 *
 * The onboarded instance asks its session for an invite with
 * \ref qar_session_request_onboarding_invite and serializes it to bytes:
 * first the size, then the bytes, the API's count-then-fetch convention. Your
 * application moves those bytes to the other instance over whatever channel
 * it has. Invites expire: mint one when it is needed and hand it over at
 * once.
 *
 * \subsection onboarding_invite_onboard_step 7. Onboard the sibling from the invite
 * \snippet onboarding_and_rejoin.c onboarding_sibling_onboard
 *
 * \ref qar_onboarding_invite_deserialize turns the bytes back into an invite
 * handle, and QarOnboardInviteExt chains it into the onboard call. The call
 * only borrows the invite, so the program destroys it right after. A
 * rejected or expired invite fails with `QAR_STATUS_PAKE_ERROR`: mint a fresh
 * one and try again.
 *
 * \subsection onboarding_teardown_step 8. Leave the sessions, keeping the identities
 * \snippet onboarding_and_rejoin.c onboarding_teardown
 *
 * \ref qar_session_handle_destroy ends the session for this process, and the
 * stored identity stays, so the next run rejoins silently. This is every
 * normal shutdown
 * (<a href="/docs/developer-guide/onboarding-and-sessions#teardown-destroy-vs-forget">Teardown: destroy vs. forget</a>).
 *
 * \subsection onboarding_forget_step 9. Forget, only when the user un-pairs
 * \snippet onboarding_and_rejoin.c onboarding_forget
 *
 * \ref qar_runtime_forget erases the stored identity: its certificate and
 * state. The next start needs a new pairing code. Call it only for an
 * explicit "sign out" or "unpair this device", never on a normal exit. It
 * needs the runtime that holds the identity, which is why the program keeps
 * both runtimes until the end. Delete your saved id together with the
 * identity, and only once the forget succeeded, as the program does with its
 * files: an id file kept after a failed forget lets you try again.
 *
 * \section onboarding_ownership Ownership and teardown in C
 * - **Handles start NULL and are destroyed in reverse order:** sessions, then
 *   runtimes, then the library. The program jumps to one teardown block from
 *   any failure, so every exit path destroys exactly what was created.
 * - **A session before its forget.** \ref qar_runtime_forget fails with
 *   `QAR_STATUS_ONBOARDING_SESSION_STILL_ACTIVE` while a session of that
 *   identity is alive.
 * - **The onboarding id is a plain value.** Copy it freely; it stays valid
 *   after the session and the runtime are gone.
 * - **The invite is yours twice.** Destroy each handle with
 *   \ref qar_onboarding_invite_handle_destroy, and free the serialized bytes
 *   yourself.
 * - **Init structures are copied.** QarRejoinInit, QarOnboardInit and their
 *   extensions live on the stack; the code string is copied before the call
 *   returns.
 *
 * \section onboarding_troubleshooting Troubleshooting
 * | What you see | Cause | What to do |
 * |---|---|---|
 * | ``No persisted onboarding id in '...'. Pass the pairing code ...``, exit 1 | first run, or the id file was removed | pass the code the Hub shows |
 * | `qar_runtime_onboard: ERROR (code=1600)`, then `Pairing code rejected or expired` | the code was wrong or older than about ten seconds | read the current code and run again |
 * | `qar_runtime_onboard: ERROR (code=1802)` | without `--hub`: no Hub of your Windows user was found; with `--hub`: no Hub answered at that address | start QAROS; for a Hub on another machine, pass `--hub=HOST:PORT` and check the address and port the Hub's card shows |
 * | `qar_runtime_rejoin: ERROR (code=1800)`, then `` No identity stored for the id in '...' ``, then `Pair again: pass a new pairing code ...` | the saved id has no identity in this storage folder: it was forgotten, or the folder was deleted or changed | pass a new code; keep the id file and the storage folder together |
 * | ``Failed to persist onboarding id to '...'``, then `This run continues, but the next run cannot rejoin ...` | the id file could not be written: a read-only working directory, a full disk, a lost network share | run from a working directory you can write to; the next run needs a new pairing code |
 * | `--hub takes the Hub's address as HOST:PORT ...`, exit 1 | no colon, no host, or a port that is not a number from 1 to 65535 | pass the address and port the Hub's card shows |
 * | `qar_runtime_forget(...): ERROR (code=1803)` | a session of that identity is still alive | destroy the session first |
 * | the program loads the wrong runtime or none | see \ref qar_c_tutorial_dynamic_loading "Dynamic Loading" | |
 *
 * <a href="/docs/developer-guide/troubleshooting#status-codes-that-are-part-of-normal-flow">Status codes that are part of normal flow</a>
 * lists the others.
 *
 * \section onboarding_next Next
 * - \ref qar_c_tutorial_local_app_onboarding "Local App Onboarding":
 *   how an app on the Hub machine joins without a code.
 * - <a href="/docs/developer-guide/onboarding-and-sessions">Onboarding and Sessions</a>:
 *   every onboarding mode, one-time codes for several devices, choosing the
 *   GPU.
 * - <a href="/docs/developer-guide/tutorials/rust/onboarding-and-rejoin">Onboarding and Rejoin in Rust</a>:
 *   the same program with the Rust binding.
 */
// clang-format on

#include "common.h"

QAR_IMPLEMENT_DYNAMIC_LOADING()

//! [onboarding_files]
/* Each runtime keeps its identities in a storage folder of its own; each id
 * file holds the onboarding id of one of those identities. */
#define PRIMARY_STATE "onboarding_and_rejoin.primary-state"
#define SIBLING_STATE "onboarding_and_rejoin.sibling-state"
#define PRIMARY_ID_FILE "onboarding_and_rejoin.primary-onboarding-id.txt"
#define SIBLING_ID_FILE "onboarding_and_rejoin.sibling-onboarding-id.txt"
//! [onboarding_files]

static void
print_usage(const char* program_name)
{
	printf(
		"Usage: %s [pairing-code] [--hub=HOST:PORT] [--forget] [--qaros-library=PATH]\n",
		program_name
	);
	printf(
		"The pairing code is needed on the first run only; later runs rejoin with the "
		"persisted onboarding id.\n" EXAMPLE_WHERE_THE_CODE_IS "\n"
	);
	printf("--hub names the Hub to pair with from another machine; the same card shows it.\n");
	printf("--forget erases both stored identities on exit.\n");
}

static void
print_onboarding_id(const char* label, const QarOnboardingId* onboarding_id)
{
	char text[64] = { 0 };
	if(qar_result_is_success(qar_uuid_to_string(onboarding_id->data, text, sizeof(text))))
	{
		printf("%s %s\n", label, text);
	}
}

//! [onboarding_hub]
/** \brief Read `HOST:PORT` into a QarOnboardHostExt. The host name is copied
 *  into host_buffer, which must outlive the onboarding call. Returns false for
 *  text without a host, or a port that is not a whole number from 1 to 65535. */
static bool
parse_hub_address(
	const char* text, char* host_buffer, size_t host_buffer_size, QarOnboardHostExt* out_hub
)
{
	/* The last colon, so the port is what follows it. */
	const char* colon = strrchr(text, ':');
	if(colon == NULL || colon == text || (size_t)(colon - text) >= host_buffer_size)
	{
		return false;
	}
	const char* port_text = colon + 1;
	/* strtoul alone reads "" as 0 and "-1" as a huge number. */
	if(port_text[0] < '0' || port_text[0] > '9')
	{
		return false;
	}
	char* end = NULL;
	errno = 0;
	const unsigned long port = strtoul(port_text, &end, 10);
	if(errno != 0 || *end != '\0' || port == 0 || port > UINT16_MAX)
	{
		return false;
	}
	memcpy(host_buffer, text, (size_t)(colon - text));
	host_buffer[colon - text] = '\0';

	*out_hub = qar_onboard_host_ext_default();
	out_hub->hostname = host_buffer;
	out_hub->port = (uint16_t)port;
	return true;
}
//! [onboarding_hub]

//! [onboarding_mint_invite]
/** \brief Ask the primary session for an invite and serialize it to bytes,
 *  the form that travels to another instance. Returns NULL after printing why;
 *  free the bytes with free(). */
static uint8_t*
mint_invite_bytes(QarSession* primary_session, size_t* out_size)
{
	QarRequestInviteInit request_init = qar_request_invite_init_default();
	QarOnboardingInvite* invite = NULL;
	QarResult invite_result = qar_session_request_onboarding_invite(
		primary_session, &request_init, NULL, NULL, NULL, &invite
	);
	log_result("qar_session_request_onboarding_invite", invite_result);
	if(qar_result_is_error(invite_result))
	{
		return NULL;
	}

	/* Count, then fetch: the API's convention for variable-size output. */
	size_t size = 0;
	int64_t expires_unix = 0;
	QarResult result = qar_onboarding_invite_serialized_size(invite, &size);
	uint8_t* bytes = qar_result_is_success(result) ? (uint8_t*)malloc(size) : NULL;
	if(bytes != NULL)
	{
		result = qar_onboarding_invite_serialize(invite, bytes, size, out_size);
	}
	if(bytes != NULL && qar_result_is_success(result))
	{
		result = qar_onboarding_invite_get_expires_unix(invite, &expires_unix);
	}
	/* The bytes stay valid without the handle. */
	qar_onboarding_invite_handle_destroy(invite);

	if(qar_result_is_error(result))
	{
		log_result("serializing the invite", result);
		free(bytes);
		return NULL;
	}
	if(bytes == NULL)
	{
		printf("No memory for the %zu invite bytes\n", size);
		return NULL;
	}
	printf(
		"Minted sibling invite (%zu bytes, expires at unix %lld).\n",
		*out_size,
		(long long)expires_unix
	);
	return bytes;
}
//! [onboarding_mint_invite]

//! [onboarding_sibling_onboard]
/** \brief Onboard the sibling runtime from invite bytes and save its new
 *  onboarding id. Returns false after printing why. */
static bool
onboard_sibling_from_invite(
	QarRuntime* sibling_runtime,
	const uint8_t* invite_bytes,
	size_t invite_size,
	const QarPeerPresentation* presentation,
	QarOnboardingId* out_onboarding_id,
	QarSession** out_session
)
{
	QarOnboardingInvite* invite = NULL;
	QarResult deserialize_result =
		qar_onboarding_invite_deserialize(invite_bytes, invite_size, &invite);
	log_result("qar_onboarding_invite_deserialize", deserialize_result);
	if(qar_result_is_error(deserialize_result))
	{
		return false;
	}

	QarOnboardInit onboard_init = qar_onboard_init_default();
	QarOnboardInviteExt invite_mode = qar_onboard_invite_ext_default();
	onboard_init.presentation = *presentation;
	invite_mode.invite = invite; /* borrowed for the call only */
	onboard_init.header.next = &invite_mode.header;

	QarResult onboard_result = qar_runtime_onboard(
		sibling_runtime, &onboard_init, NULL, NULL, NULL, out_onboarding_id, out_session
	);
	log_result("qar_runtime_onboard", onboard_result);
	qar_onboarding_invite_handle_destroy(invite);
	if(qar_result_is_error(onboard_result))
	{
		return false;
	}
	if(!save_onboarding_id(SIBLING_ID_FILE, out_onboarding_id))
	{
		printf(
			"This run continues, but the next run cannot rejoin the sibling: it will "
			"onboard it with a new invite.\n"
		);
	}
	return true;
}
//! [onboarding_sibling_onboard]

//! [onboarding_sibling_join]
/** \brief Join the sibling runtime: rejoin with its saved id, and only when it
 *  has no identity yet, onboard it with an invite from the primary session.
 *  Asking for an invite on every run would leave one more stored identity
 *  behind each time. Returns false after printing why. */
static bool
join_sibling(
	QarSession* primary_session,
	QarRuntime* sibling_runtime,
	QarOnboardingId* out_onboarding_id,
	QarSession** out_session
)
{
	QarPeerPresentation presentation = qar_peer_presentation_default();
	presentation.display_name = "Sibling Tutorial Peer";

	switch(example_rejoin_with_stored_id(
		sibling_runtime, SIBLING_ID_FILE, &presentation, out_onboarding_id, out_session
	))
	{
	case EXAMPLE_REJOINED:
		return true;
	case EXAMPLE_REJOIN_FAILED:
		return false;
	case EXAMPLE_REJOIN_NO_SAVED_ID:
	case EXAMPLE_REJOIN_IDENTITY_NOT_FOUND:
		break;
	}

	size_t invite_size = 0;
	uint8_t* invite_bytes = mint_invite_bytes(primary_session, &invite_size);
	if(invite_bytes == NULL)
	{
		return false;
	}
	/* Both instances live in this process, so "sending" the invite is passing
	 * a buffer. Between processes, send the same bytes over a pipe or socket. */
	const bool onboarded = onboard_sibling_from_invite(
		sibling_runtime, invite_bytes, invite_size, &presentation, out_onboarding_id, out_session
	);
	free(invite_bytes);
	return onboarded;
}
//! [onboarding_sibling_join]

//! [onboarding_forget]
/** \brief Erase a stored identity and its id file. Only for an explicit
 *  "unpair this device"; the session of that identity must be gone. */
static void
forget_identity(
	const char* label,
	QarRuntime* runtime,
	const QarOnboardingId* onboarding_id,
	const char* id_file_path
)
{
	QarForgetInit forget_init = qar_forget_init_default();
	forget_init.onboarding_id = *onboarding_id;
	/* Keep the id file while the identity is still stored, so a later run can
	 * rejoin with it or forget it again. */
	if(example_logged_success(label, qar_runtime_forget(runtime, &forget_init)))
	{
		remove(id_file_path);
	}
}
//! [onboarding_forget]

int
main(int argc, char** argv)
{
	example_print_as_it_happens();
	//! [onboarding_args]
	const char* library_path = example_library_path(argc, argv);
	const char* pairing_code = NULL;
	bool forget_on_exit = false;
	char hub_hostname[256] = { 0 };
	QarOnboardHostExt hub_address = qar_onboard_host_ext_default();
	QarOnboardHostExt* hub = NULL; /* NULL: a Hub of your own user */
	for(int i = 1; i < argc; ++i)
	{
		if(strcmp(argv[i], "--forget") == 0)
		{
			forget_on_exit = true;
		}
		else if(strncmp(argv[i], "--hub=", strlen("--hub=")) == 0)
		{
			if(!parse_hub_address(
				   argv[i] + strlen("--hub="), hub_hostname, sizeof(hub_hostname), &hub_address
			   ))
			{
				fprintf(
					stderr,
					"--hub takes the Hub's address as HOST:PORT, with a port from 1 to 65535.\n"
				);
				return 1;
			}
			hub = &hub_address;
		}
		else if(strncmp(argv[i], "--", 2) != 0 && pairing_code == NULL)
		{
			pairing_code = argv[i];
		}
		else if(strncmp(argv[i], "--qaros-library=", strlen("--qaros-library=")) != 0)
		{
			print_usage(argv[0]);
			return 1;
		}
	}
	//! [onboarding_args]

	int exit_code = example_load_library(library_path);
	if(exit_code != 0)
	{
		return exit_code;
	}

	/* Every handle starts NULL, so the teardown at the end can run from any
	 * point of failure. */
	QarRuntime* primary_runtime = NULL;
	QarRuntime* sibling_runtime = NULL;
	QarSession* primary_session = NULL;
	QarSession* sibling_session = NULL;
	QarOnboardingId primary_id = qar_onboarding_id_default();
	QarOnboardingId sibling_id = qar_onboarding_id_default();
	exit_code = 1; /* until both sessions are obtained */

	//! [onboarding_primary]
	if(!example_create_runtime(PRIMARY_STATE, &primary_runtime))
	{
		goto teardown;
	}
	if(example_obtain_session(
		   primary_runtime,
		   pairing_code,
		   hub,
		   PRIMARY_ID_FILE,
		   "Primary Tutorial Peer",
		   &primary_id,
		   &primary_session
	   )
	   != 0)
	{
		goto teardown;
	}
	print_onboarding_id("Primary session obtained with onboarding id", &primary_id);
	//! [onboarding_primary]

	//! [onboarding_sibling]
	if(!example_create_runtime(SIBLING_STATE, &sibling_runtime))
	{
		goto teardown;
	}
	if(!join_sibling(primary_session, sibling_runtime, &sibling_id, &sibling_session))
	{
		goto teardown;
	}
	print_onboarding_id("Sibling session obtained with onboarding id", &sibling_id);
	exit_code = 0;
	//! [onboarding_sibling]

	//! [onboarding_teardown]
	/* Destroying a session handle leaves the session and keeps the stored
	 * identity, so the next run rejoins silently. */
	printf("Destroying the session handles keeps both identities for the next run.\n");
	qar_session_handle_destroy(sibling_session);
	sibling_session = NULL;
	qar_session_handle_destroy(primary_session);
	primary_session = NULL;

	/* Forget only after the sessions are gone: while one is alive, forget fails
	 * with QAR_STATUS_ONBOARDING_SESSION_STILL_ACTIVE. */
	if(forget_on_exit)
	{
		forget_identity(
			"qar_runtime_forget(sibling)", sibling_runtime, &sibling_id, SIBLING_ID_FILE
		);
		forget_identity(
			"qar_runtime_forget(primary)", primary_runtime, &primary_id, PRIMARY_ID_FILE
		);
	}
	//! [onboarding_teardown]

teardown:
	/* Sessions, then runtimes, then the library: each needs the next one. */
	if(sibling_session != NULL)
	{
		qar_session_handle_destroy(sibling_session);
	}
	if(primary_session != NULL)
	{
		qar_session_handle_destroy(primary_session);
	}
	if(sibling_runtime != NULL)
	{
		qar_runtime_handle_destroy(sibling_runtime);
	}
	if(primary_runtime != NULL)
	{
		qar_runtime_handle_destroy(primary_runtime);
	}
	example_unload_library();
	return exit_code;
}
