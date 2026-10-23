#pragma once
/* Under strict C11 (CMAKE_C_EXTENSIONS OFF), glibc declares neither nanosleep,
 * used below, nor realpath, used by qar_streaming.h. X/Open 7, defined before
 * the first include, declares both. macOS declares them anyway, and the define
 * would hide the RTLD_NOLOAD that qar_streaming.h uses there. */
#if defined(__linux__) && !defined(_XOPEN_SOURCE)
#define _XOPEN_SOURCE 700
#endif
/* Every sample loads the runtime at run time; the define has to come before the
 * header, which is why it is here and not in each sample. */
#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include "qar_streaming.h"
#include <errno.h>
#include <inttypes.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifdef _WIN32
#include <wchar.h>
#else
#include <pthread.h>
#include <time.h>
#endif

/** \brief Print every line as it happens, also when the output goes to a pipe
 *  or a file. Call it first in main.
 *
 *  A console shows each line at once, but the C runtime block-buffers output
 *  that is redirected, so a captured log shows nothing until the program exits
 *  and loses its tail if the program is killed. MSVC treats _IOLBF as full
 *  buffering, so stdout is left unbuffered; the samples print little. */
static void
example_print_as_it_happens(void)
{
	setvbuf(stdout, NULL, _IONBF, 0);
}

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

/** \brief Log a result and tell whether it succeeded, so calls can be chained
 *  with && and the chain stops at the first failure. */
static bool
example_logged_success(const char* label, QarResult result)
{
	log_result(label, result);
	return qar_result_is_success(result);
}

/** \brief Like example_logged_success, but logs only a failure: for queries
 *  and getters, whose success says nothing new. */
static bool
example_quiet_success(const char* label, QarResult result)
{
	if(qar_result_is_error(result))
	{
		log_result(label, result);
	}
	return qar_result_is_success(result);
}

/** \brief The pairing code: the first argument that does not start with
 *  "--", else NULL. */
static const char*
example_pairing_code_argument(int argc, char** argv)
{
	for(int i = 1; i < argc; ++i)
	{
		if(strncmp(argv[i], "--", 2) != 0)
		{
			return argv[i];
		}
	}
	return NULL;
}

/** \brief Check the command line: at most one pairing code (an argument
 *  that does not start with "--"), and only flags that start with one of
 *  allowed_flags, such as "--frames=". Otherwise prints the usage to stderr
 *  and returns false, so "--frames 300" is refused instead of reading "300" as
 *  a pairing code. */
static bool
example_arguments_are_known(
	int argc,
	char** argv,
	const char* const* allowed_flags,
	size_t allowed_flag_count,
	const char* usage
)
{
	bool pairing_code_seen = false;
	for(int i = 1; i < argc; ++i)
	{
		bool known = false;
		if(strncmp(argv[i], "--", 2) != 0)
		{
			known = !pairing_code_seen;
			pairing_code_seen = true;
		}
		for(size_t flag = 0; !known && flag < allowed_flag_count; ++flag)
		{
			known = strncmp(argv[i], allowed_flags[flag], strlen(allowed_flags[flag])) == 0;
		}
		if(!known)
		{
			fprintf(stderr, "Unknown argument '%s'.\nUsage: %s %s\n", argv[i], argv[0], usage);
			return false;
		}
	}
	return true;
}

/** \brief The value of `--frames=N`, else default_frame_limit. Returns false
 *  for a value that is not a whole number. */
static bool
example_frame_limit_argument(
	int argc, char** argv, size_t default_frame_limit, size_t* out_frame_limit
)
{
	static const char flag[] = "--frames=";
	*out_frame_limit = default_frame_limit;
	for(int i = 1; i < argc; ++i)
	{
		if(strncmp(argv[i], flag, sizeof(flag) - 1) != 0)
		{
			continue;
		}
		const char* text = argv[i] + sizeof(flag) - 1;
		/* strtoull alone reads "many" as 0 and "-1" as a huge number. */
		if(text[0] < '0' || text[0] > '9')
		{
			return false;
		}
		char* end = NULL;
		errno = 0;
		const unsigned long long value = strtoull(text, &end, 10);
		if(errno != 0 || *end != '\0' || value > UINT32_MAX)
		{
			return false;
		}
		*out_frame_limit = (size_t)value;
	}
	return true;
}

/* A lock for what a callback on a library thread shares with the main thread,
 * and a sleep for polling loops. */
#ifdef _WIN32
typedef SRWLOCK ExampleLock;

static void
example_lock_init(ExampleLock* lock)
{
	InitializeSRWLock(lock);
}

static void
example_lock_acquire(ExampleLock* lock)
{
	AcquireSRWLockExclusive(lock);
}

static void
example_lock_release(ExampleLock* lock)
{
	ReleaseSRWLockExclusive(lock);
}

static void
example_lock_destroy(ExampleLock* lock)
{
	(void)lock; /* an SRW lock holds no resources */
}

static void
example_sleep_milliseconds(unsigned milliseconds)
{
	Sleep(milliseconds);
}
#else
typedef pthread_mutex_t ExampleLock;

static void
example_lock_init(ExampleLock* lock)
{
	pthread_mutex_init(lock, NULL);
}

static void
example_lock_acquire(ExampleLock* lock)
{
	pthread_mutex_lock(lock);
}

static void
example_lock_release(ExampleLock* lock)
{
	pthread_mutex_unlock(lock);
}

static void
example_lock_destroy(ExampleLock* lock)
{
	pthread_mutex_destroy(lock);
}

static void
example_sleep_milliseconds(unsigned milliseconds)
{
	struct timespec duration;
	duration.tv_sec = (time_t)(milliseconds / 1000u);
	duration.tv_nsec = (long)(milliseconds % 1000u) * 1000000L;
	nanosleep(&duration, NULL);
}
#endif

//! [example_wait_for_updates]
/* The Hub delivers the first update on its own schedule, which can take a few
 * seconds after the subscription. Ten seconds leaves room for that and still
 * ends a run in which no update comes. */
#define EXAMPLE_FIRST_UPDATE_TIMEOUT_MS 10000u
/* After the first update, how long the wait lasts without one more. */
#define EXAMPLE_UPDATE_QUIET_PERIOD_MS 500u
#define EXAMPLE_UPDATE_POLL_INTERVAL_MS 50u

static unsigned
example_read_locked_count(ExampleLock* lock, const unsigned* count)
{
	example_lock_acquire(lock);
	const unsigned value = *count;
	example_lock_release(lock);
	return value;
}

/** \brief Wait for the first update, then until no update has arrived for
 *  EXAMPLE_UPDATE_QUIET_PERIOD_MS. received_count is the counter a callback
 *  increments while holding lock. Returns false when no update arrived within
 *  EXAMPLE_FIRST_UPDATE_TIMEOUT_MS. */
static bool
example_wait_for_updates(ExampleLock* lock, const unsigned* received_count)
{
	unsigned waited_milliseconds = 0;
	while(example_read_locked_count(lock, received_count) == 0)
	{
		if(waited_milliseconds >= EXAMPLE_FIRST_UPDATE_TIMEOUT_MS)
		{
			return false;
		}
		example_sleep_milliseconds(EXAMPLE_UPDATE_POLL_INTERVAL_MS);
		waited_milliseconds += EXAMPLE_UPDATE_POLL_INTERVAL_MS;
	}

	unsigned last_count = example_read_locked_count(lock, received_count);
	unsigned quiet_milliseconds = 0;
	while(quiet_milliseconds < EXAMPLE_UPDATE_QUIET_PERIOD_MS)
	{
		example_sleep_milliseconds(EXAMPLE_UPDATE_POLL_INTERVAL_MS);
		const unsigned count = example_read_locked_count(lock, received_count);
		if(count != last_count)
		{
			last_count = count;
			quiet_milliseconds = 0;
		}
		else
		{
			quiet_milliseconds += EXAMPLE_UPDATE_POLL_INTERVAL_MS;
		}
	}
	return true;
}
//! [example_wait_for_updates]

//! [example_same_pose]
/* The Hub reports a value back that can differ from the one you set, and from
 * its own earlier report, below the precision anyone sees. Compared exactly,
 * such an echo looks like a change. So compare at the precision your app
 * cares about. These samples print centimeters; a millimeter is far below
 * what a viewer notices. */
#define EXAMPLE_SAME_LENGTH_METERS 0.001f
/* For each component of a unit quaternion: 0.0001 is about a hundredth of a
 * degree. */
#define EXAMPLE_SAME_ROTATION 0.0001f
/* For a factor such as a scale: one part in a thousand. */
#define EXAMPLE_SAME_FACTOR 0.001f

static bool
example_nearly_equal(float first, float second, float tolerance)
{
	return fabsf(first - second) <= tolerance;
}

/** \brief Whether two poses are the same at the precision above. */
static bool
example_same_pose(const QarPose* first, const QarPose* second)
{
	const float length = EXAMPLE_SAME_LENGTH_METERS;
	const float rotation = EXAMPLE_SAME_ROTATION;
	return example_nearly_equal(first->position.x, second->position.x, length)
		   && example_nearly_equal(first->position.y, second->position.y, length)
		   && example_nearly_equal(first->position.z, second->position.z, length)
		   && example_nearly_equal(first->orientation.x, second->orientation.x, rotation)
		   && example_nearly_equal(first->orientation.y, second->orientation.y, rotation)
		   && example_nearly_equal(first->orientation.z, second->orientation.z, rotation)
		   && example_nearly_equal(first->orientation.w, second->orientation.w, rotation);
}
//! [example_same_pose]

//! [example_is_running_target]
/** \brief A target app you can render for: role target, and online or not yet
 *  reconciled by the Hub. Offline, leaving and retired peers are gone. */
static bool
example_is_running_target(QarPeerSpec* peer)
{
	QarPeerRole role = QAR_PEER_ROLE_INTERNAL;
	QarPeerPresence presence = QAR_PEER_PRESENCE_UNKNOWN;
	return qar_result_is_success(qar_peer_spec_get_role(peer, &role))
		   && role == QAR_PEER_ROLE_TARGET
		   && qar_result_is_success(qar_peer_spec_get_presence(peer, &presence))
		   && (presence == QAR_PEER_PRESENCE_ONLINE || presence == QAR_PEER_PRESENCE_UNKNOWN);
}
//! [example_is_running_target]

//! [example_is_waiting]
/** \brief A stream that is paused, reconnecting or has no transfer yet is not
 *  broken, and an app volume that does not resolve yet is recoverable: keep the
 *  sender and try the next frame. qar_result_has_code also matches a wrapped
 *  cause, which is how begin-frame reports these. */
static bool
example_is_waiting(QarResult result)
{
	return qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_STREAM_IS_PAUSED)
		   || qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING)
		   || qar_result_has_code(result, QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS)
		   || qar_result_has_code(result, QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED);
}
//! [example_is_waiting]

/* Where a user finds the pairing code; printed whenever one is needed. */
#define EXAMPLE_WHERE_THE_CODE_IS                                                                  \
	"The Visualizer on the Hub machine shows the current code under Connection panel, "            \
	"Hubs > Pair another Hub > Let another hub join us."

//! [example_library_path]
/** \brief The value of `--qaros-library=PATH` among the arguments, else NULL.
 *
 *  NULL is what a shipped application passes to qar_library_load: the default
 *  lookup finds the runtime of the QAROS installed on this machine. A path is
 *  for one specific library file, such as a development build of QAROS. */
static const char*
example_library_path(int argc, char** argv)
{
	static const char flag[] = "--qaros-library=";
	for(int i = 1; i < argc; ++i)
	{
		if(strncmp(argv[i], flag, sizeof(flag) - 1) == 0)
		{
			return argv[i] + sizeof(flag) - 1;
		}
	}
	return NULL;
}
//! [example_library_path]

//! [example_print_loaded_runtime]
/** \brief Print which runtime file was loaded, which lookup step found it, and
 *  the runtime's version. Call it after qar_library_load succeeded.
 *
 *  The default lookup stops at the first step that has a runtime, and steps 2
 *  and 3 stop it even when they fail. So the step can be read back from what
 *  is set on this machine, in the lookup's own order. */
static void
example_print_loaded_runtime(const char* library_path)
{
#ifdef _WIN32
	wchar_t loaded_file[MAX_PATH] = { 0 };
	wchar_t executable_file[MAX_PATH] = { 0 };
	const DWORD loaded_length =
		GetModuleFileNameW(g_qar_dynamic_library_handle, loaded_file, MAX_PATH);
	const DWORD executable_length = GetModuleFileNameW(NULL, executable_file, MAX_PATH);
	wchar_t* loaded_separator = wcsrchr(loaded_file, L'\\');
	wchar_t* executable_separator = wcsrchr(executable_file, L'\\');
	const bool beside_executable =
		loaded_length > 0 && loaded_length < MAX_PATH && executable_length > 0
		&& executable_length < MAX_PATH && loaded_separator != NULL && executable_separator != NULL
		&& (size_t)(loaded_separator - loaded_file)
			   == (size_t)(executable_separator - executable_file)
		&& _wcsnicmp(loaded_file, executable_file, (size_t)(loaded_separator - loaded_file)) == 0;

	char folder[1024] = { 0 };
	const char* lookup_step = NULL;
	if(library_path != NULL)
	{
		lookup_step = "the file you named";
	}
	else if(beside_executable)
	{
		lookup_step = "beside the executable";
	}
	else if(qar_runtime_env_folder(folder, sizeof(folder)))
	{
		lookup_step = "QAROS_RUNTIME_DIR";
	}
	else if(qar_installed_bin_folder(folder, sizeof(folder)))
	{
		lookup_step = "the installed QAROS";
	}
	else
	{
		lookup_step = "PATH";
	}
	printf("Loaded %ls (%s)\n", loaded_file, lookup_step);
#else
	printf("Loaded %s\n", library_path != NULL ? library_path : "qar-streaming-c");
#endif

	QarRuntimeInfo runtime_info;
	memset(&runtime_info, 0, sizeof(runtime_info));
	QarResult info_result = qar_runtime_info(&runtime_info);
	if(qar_result_is_success(info_result))
	{
		printf(
			"Runtime %s (%s, C API %s)\n",
			runtime_info.version,
			runtime_info.flavor,
			runtime_info.c_api
		);
	}
	else
	{
		log_result("qar_runtime_info", info_result);
	}
}
//! [example_print_loaded_runtime]

//! [example_load_library]
/** \brief Load the runtime (NULL: the default lookup), print which one loaded,
 *  and initialize the library with console logging.
 *
 *  Returns 0, or the sample exit code after printing why: 2 when no runtime
 *  loaded, 1 when qar_library_init failed. On failure nothing stays loaded.
 *  Undo a success with example_unload_library. */
static int
example_load_library(const char* library_path)
{
	if(library_path != NULL)
	{
		printf("Loading qar-streaming-c from: %s\n", library_path);
	}
	else
	{
		printf("Loading the QAROS runtime (default lookup)\n");
	}
	if(!qar_library_load(library_path))
	{
		fprintf(
			stderr,
			"No QAROS runtime loaded. Install QAROS, set QAROS_RUNTIME_DIR to the folder of "
			"qar-streaming-c.dll, or pass --qaros-library=PATH.\n"
		);
		return 2;
	}
	example_print_loaded_runtime(library_path);

	QarLibraryInit library_init = qar_library_init_default();
	library_init.enable_console_logging = true;
	QarResult init_result = qar_library_init(&library_init);
	if(qar_result_is_error(init_result))
	{
		log_result("qar_library_init", init_result);
		qar_library_unload();
		return 1;
	}
	return 0;
}

/** \brief Destroy the library state, then unload the runtime file. Call it
 *  after every runtime handle is destroyed. */
static void
example_unload_library(void)
{
	log_result("qar_library_destroy", qar_library_destroy());
	qar_library_unload();
}
//! [example_load_library]

//! [example_create_runtime]
/** \brief Create a runtime that keeps its identities in storage_folder_path
 *  (NULL: the library's default folder).
 *
 *  runtime_binaries_folder_path stays empty, so the runtime uses the folder the
 *  library was loaded from. Returns false after printing why. */
static bool
example_create_runtime(const char* storage_folder_path, QarRuntime** out_runtime)
{
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	runtime_init.storage_folder_path = storage_folder_path;

	*out_runtime = NULL;
	QarResult runtime_result = qar_runtime_create(&runtime_init, out_runtime);
	log_result("qar_runtime_create", runtime_result);
	return qar_result_is_success(runtime_result) && *out_runtime != NULL;
}
//! [example_create_runtime]

//! [example_onboarding_persistence]
/** \brief Load a persisted onboarding id (UUID text) from a small state file.
 *  Returns false when the file does not exist yet (first run) or holds no
 *  readable id. */
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
	if(qar_result_is_error(parse_result))
	{
		/* Treated as "no id": the caller then asks for a code instead of failing. */
		printf("Ignoring unreadable onboarding id in '%s'\n", path);
		return false;
	}
	return true;
}

/** \brief Persist an onboarding id (UUID text) so the next run can rejoin.
 *  Returns false after printing why when the id is not saved. */
static bool
save_onboarding_id(const char* path, const QarOnboardingId* id)
{
	char text[64] = { 0 };
	if(qar_result_is_error(qar_uuid_to_string(id->data, text, sizeof(text))))
	{
		printf("Failed to serialize onboarding id\n");
		return false;
	}
	FILE* file = fopen(path, "w");
	if(!file)
	{
		printf("Failed to persist onboarding id to '%s'\n", path);
		return false;
	}
	/* A full disk or a lost network share shows up only here: an fprintf that
	 * fails, or an fclose that cannot flush. */
	const bool written = fprintf(file, "%s\n", text) >= 0;
	const bool closed = fclose(file) == 0;
	if(!written || !closed)
	{
		printf("Failed to persist onboarding id to '%s'\n", path);
		return false;
	}
	return true;
}
//! [example_onboarding_persistence]

//! [example_rejoin]
/** What example_rejoin_with_stored_id found. */
typedef enum ExampleRejoinOutcome
{
	/** The session is back; nothing else to do. */
	EXAMPLE_REJOINED,
	/** No readable id stored yet: onboard instead. */
	EXAMPLE_REJOIN_NO_SAVED_ID,
	/** An id is stored, but the runtime has no identity for it
	 *  (QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND): onboard instead. */
	EXAMPLE_REJOIN_IDENTITY_NOT_FOUND,
	/** A real error (no Hub answered, ...): onboarding would not help. */
	EXAMPLE_REJOIN_FAILED
} ExampleRejoinOutcome;

/** \brief Rejoin with the onboarding id stored in id_file_path. Shows no UI and
 *  needs no code. On EXAMPLE_REJOINED, *out_onboarding_id and *out_session are
 *  set. */
static ExampleRejoinOutcome
example_rejoin_with_stored_id(
	QarRuntime* runtime,
	const char* id_file_path,
	const QarPeerPresentation* presentation,
	QarOnboardingId* out_onboarding_id,
	QarSession** out_session
)
{
	QarOnboardingId saved_id = qar_onboarding_id_default();
	if(!load_onboarding_id(id_file_path, &saved_id))
	{
		return EXAMPLE_REJOIN_NO_SAVED_ID;
	}

	QarRejoinInit rejoin_init = qar_rejoin_init_default();
	rejoin_init.onboarding_id = saved_id;
	rejoin_init.presentation = *presentation;

	QarResult rejoin_result =
		qar_runtime_rejoin(runtime, &rejoin_init, NULL, NULL, NULL, out_session);
	log_result("qar_runtime_rejoin", rejoin_result);
	if(qar_result_is_success(rejoin_result))
	{
		*out_onboarding_id = saved_id;
		return EXAMPLE_REJOINED;
	}
	if(qar_result_has_code(rejoin_result, QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND))
	{
		/* Normal flow: the id was forgotten, or this storage folder is new. */
		printf(
			"No identity stored for the id in '%s' in this runtime's storage folder: it was "
			"forgotten, or the folder is new.\n",
			id_file_path
		);
		return EXAMPLE_REJOIN_IDENTITY_NOT_FOUND;
	}
	return EXAMPLE_REJOIN_FAILED;
}
//! [example_rejoin]

//! [example_onboard_with_code]
/** \brief Onboard with a pairing code and persist the new onboarding id.
 *
 *  hub names the Hub to reach. With hub NULL, no QarOnboardHostExt is chained
 *  and the code finds a Hub of this user: first through the hub-endpoint.json
 *  file the Hub writes, then through a discovery beacon whose owner is this
 *  user. Returns false after printing why. */
static bool
example_onboard_with_code(
	QarRuntime* runtime,
	const char* pairing_code,
	QarOnboardHostExt* hub, /* NULL: a Hub of this user */
	const char* id_file_path,
	const QarPeerPresentation* presentation,
	QarOnboardingId* out_onboarding_id,
	QarSession** out_session
)
{
	QarOnboardInit onboard_init = qar_onboard_init_default();
	QarOnboardCodeExt onboard_code = qar_onboard_code_ext_default();
	onboard_init.presentation = *presentation;
	onboard_code.code = pairing_code;
	onboard_init.header.next = &onboard_code.header;
	/* The Hub's address goes after the code extension in the chain. */
	onboard_code.header.next = hub != NULL ? &hub->header : NULL;

	QarOnboardingId new_id = qar_onboarding_id_default();
	QarResult onboard_result =
		qar_runtime_onboard(runtime, &onboard_init, NULL, NULL, NULL, &new_id, out_session);
	log_result("qar_runtime_onboard", onboard_result);
	if(qar_result_has_code(onboard_result, QAR_STATUS_PAKE_ERROR))
	{
		printf(
			"Pairing code rejected or expired: a code works once, for about ten seconds. "
			"Run again with the current code.\n" EXAMPLE_WHERE_THE_CODE_IS "\n"
		);
	}
	if(qar_result_is_error(onboard_result))
	{
		return false;
	}

	/* The id is the ticket for every later rejoin and forget: save it at once. */
	if(!save_onboarding_id(id_file_path, &new_id))
	{
		printf(
			"This run continues, but the next run cannot rejoin: it will need a new "
			"pairing code.\n"
		);
	}
	*out_onboarding_id = new_id;
	return true;
}
//! [example_onboard_with_code]

//! [example_obtain_session]
/** \brief The startup of an app that pairs with a code: rejoin with the
 *  persisted onboarding id, otherwise onboard with the pairing code (at hub,
 *  when given) and persist the new id.
 *
 *  Returns 0 on success; on success *out_session and *out_onboarding_id are
 *  set and the id is saved in id_file_path for the next run. */
static int
example_obtain_session(
	QarRuntime* runtime,
	const char* pairing_code, /* may be NULL when a persisted id exists */
	QarOnboardHostExt* hub,	  /* NULL: a Hub of this user */
	const char* id_file_path,
	const char* display_name,
	QarOnboardingId* out_onboarding_id,
	QarSession** out_session
)
{
	QarPeerPresentation presentation = qar_peer_presentation_default();
	presentation.display_name = display_name;

	const ExampleRejoinOutcome rejoin = example_rejoin_with_stored_id(
		runtime, id_file_path, &presentation, out_onboarding_id, out_session
	);
	if(rejoin == EXAMPLE_REJOINED)
	{
		return 0;
	}
	if(rejoin == EXAMPLE_REJOIN_FAILED)
	{
		return 1;
	}

	if(pairing_code == NULL && rejoin == EXAMPLE_REJOIN_IDENTITY_NOT_FOUND)
	{
		printf(
			"Pair again: pass a new pairing code as the first argument.\n" EXAMPLE_WHERE_THE_CODE_IS
			"\n"
		);
		return 1;
	}
	if(pairing_code == NULL)
	{
		printf(
			"No persisted onboarding id in '%s'. Pass the pairing code as the first "
			"argument.\n" EXAMPLE_WHERE_THE_CODE_IS "\n",
			id_file_path
		);
		return 1;
	}
	const bool onboarded = example_onboard_with_code(
		runtime, pairing_code, hub, id_file_path, &presentation, out_onboarding_id, out_session
	);
	return onboarded ? 0 : 1;
}
//! [example_obtain_session]

//! [example_local_app_result]
/** \brief Progress callback that prints each message. The library calls it on
 *  its own thread while the blocking call waits; it returns before the call
 *  does. */
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

/** \brief Exit code for the result of a local-app join, after telling the user
 *  what to do: 0 joined, 3 to 7 for the refusals below, 1 for anything else.
 *  Each refusal needs a different answer from the user, so each has its own
 *  code. */
static int
example_local_app_exit_code(QarResult result)
{
	if(qar_result_is_success(result))
	{
		return 0;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_UNREACHABLE))
	{
		fprintf(
			stderr,
			"Could not start or reach QAROS on this machine. Start QAROS from the Start menu, "
			"then start this app again.\n"
		);
		return 3;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_DENIED))
	{
		fprintf(stderr, "QAROS did not allow this app: the approval was denied or revoked.\n");
		return 4;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APPROVAL_TIMEOUT))
	{
		fprintf(
			stderr, "Nobody answered the approval prompt in QAROS in time. Start the app again.\n"
		);
		return 5;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_HUB_NOT_AUTHENTIC))
	{
		fprintf(
			stderr,
			"The program answering on this machine is not the installed QAROS. Reinstall QAROS.\n"
		);
		return 6;
	}
	if(qar_result_has_code(result, QAR_STATUS_ONBOARDING_APP_IDENTITY_MISMATCH))
	{
		fprintf(
			stderr,
			"This app no longer matches what QAROS approved. Revoke it in QAROS and approve it "
			"again.\n"
		);
		return 7;
	}
	log_result("qar_runtime_onboard", result);
	return 1;
}
//! [example_local_app_result]

//! [example_join_session]
/** How long the first run of an executable waits for someone to click Allow. */
#define EXAMPLE_LOCAL_APP_APPROVAL_TIMEOUT_MS 120000u

/** \brief The startup of the samples that use a session: rejoin with the
 *  persisted onboarding id, else onboard with the pairing code when one is
 *  given, else (Windows) join the Hub on this machine as a local app.
 *
 *  Returns 0 with *out_session set, or an exit code after printing why: 3 to 7
 *  for a refused local-app join (see example_local_app_exit_code), 1 for
 *  anything else. A local app stores nothing: its onboarding id is the zero
 *  id, so the next run joins as a local app again. */
static int
example_join_session(
	QarRuntime* runtime,
	const char* pairing_code, /* NULL: no code given */
	const char* id_file_path,
	const char* display_name,
	QarSession** out_session
)
{
	QarPeerPresentation presentation = qar_peer_presentation_default();
	presentation.display_name = display_name;
	QarOnboardingId onboarding_id = qar_onboarding_id_default();

	switch(example_rejoin_with_stored_id(
		runtime, id_file_path, &presentation, &onboarding_id, out_session
	))
	{
	case EXAMPLE_REJOINED:
		return 0;
	case EXAMPLE_REJOIN_FAILED:
		return 1;
	case EXAMPLE_REJOIN_NO_SAVED_ID:
	case EXAMPLE_REJOIN_IDENTITY_NOT_FOUND:
		break;
	}

	if(pairing_code != NULL)
	{
		const bool onboarded = example_onboard_with_code(
			runtime, pairing_code, NULL, id_file_path, &presentation, &onboarding_id, out_session
		);
		return onboarded ? 0 : 1;
	}

#ifdef _WIN32
	printf(
		"No session to rejoin and no pairing code: joining the QAROS Hub on this machine as a "
		"local app. On the first run of this executable, click Allow in the QAROS approval "
		"prompt (waiting up to %u s).\n",
		EXAMPLE_LOCAL_APP_APPROVAL_TIMEOUT_MS / 1000u
	);
	QarOnboardInit onboard_init = qar_onboard_init_default();
	QarOnboardLocalAppExt local_app = qar_onboard_local_app_ext_default();
	onboard_init.presentation = presentation;
	local_app.approval_timeout_ms = EXAMPLE_LOCAL_APP_APPROVAL_TIMEOUT_MS;
	onboard_init.header.next = &local_app.header;

	QarResult joined = qar_runtime_onboard(
		runtime, &onboard_init, example_print_progress, NULL, NULL, &onboarding_id, out_session
	);
	const int exit_code = example_local_app_exit_code(joined);
	if(exit_code == 0)
	{
		printf("qar_runtime_onboard: SUCCESS (local app)\n");
		printf(
			"A local app has no onboarding id to store (it is zero): the next run joins as a "
			"local app again, without a prompt while this executable is unchanged.\n"
		);
	}
	return exit_code;
#else
	printf(
		"No persisted onboarding id in '%s'. Pass the pairing code as the first "
		"argument.\n" EXAMPLE_WHERE_THE_CODE_IS "\n",
		id_file_path
	);
	return 1;
#endif
}
//! [example_join_session]
