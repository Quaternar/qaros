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

/** \brief Check the command line: only flags that start with one of
 *  allowed_flags, such as "--frames=". Any other argument, a positional one
 *  included, prints the usage to stderr and returns false, so "--frames 300"
 *  is refused instead of ignoring "300". */
static bool
example_arguments_are_known(
	int argc,
	char** argv,
	const char* const* allowed_flags,
	size_t allowed_flag_count,
	const char* usage
)
{
	for(int i = 1; i < argc; ++i)
	{
		bool known = false;
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
	wchar_t instance[QAR_INSTANCE_NAME_MAX_LENGTH + 2] = { 0 };
	const char* lookup_step = NULL;
	if(library_path != NULL)
	{
		lookup_step = "the file you named";
	}
	else if(beside_executable)
	{
		lookup_step = "beside the executable";
	}
	else if(qar_requested_instance_w(instance, QAR_INSTANCE_NAME_MAX_LENGTH + 2))
	{
		lookup_step = "QAR_INSTANCE";
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
			"No QAROS runtime loaded. Install QAROS, fix or unset QAR_INSTANCE, or pass "
			"--qaros-library=PATH.\n"
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
			"Could not start or reach QAROS on this machine. Start QAROS, "
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

//! [example_join_local_app]
/** How long the first run of an executable waits for someone to click Allow. */
#define EXAMPLE_LOCAL_APP_APPROVAL_TIMEOUT_MS 120000u

/** \brief Join the QAROS Hub on this machine as a local app with an app tag:
 *  no code, the user approves the app once in QAROS, and the tag keeps the
 *  peer id across restarts. Nothing is stored, so every start calls it again.
 *  See the \ref qar_c_tutorial_local_app_onboarding tutorial.
 *
 *  Returns 0 with *out_session set, or the exit code of
 *  example_local_app_exit_code after printing why. */
static int
example_join_as_local_app(
	QarRuntime* runtime, const char* display_name, const char* app_tag, QarSession** out_session
)
{
	QarOnboardInit init = qar_onboard_init_default();
	init.presentation.display_name = display_name; /* any UTF-8 */
	QarOnboardLocalAppExt local = qar_onboard_local_app_ext_default();
	local.approval_timeout_ms = EXAMPLE_LOCAL_APP_APPROVAL_TIMEOUT_MS;
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
	return example_local_app_exit_code(joined);
}
//! [example_join_local_app]

//! [example_join_session]
/** \brief The startup of the samples that use a session: join the QAROS Hub
 *  on this machine as a local app with the sample's app tag.
 *
 *  Returns 0 with *out_session set, or an exit code after printing why: 3 to 7
 *  for a refused join (see example_local_app_exit_code), 1 for anything else.
 *  A local app stores nothing, so every run joins this way again. */
static int
example_join_session(
	QarRuntime* runtime, const char* display_name, const char* app_tag, QarSession** out_session
)
{
	printf(
		"Joining the QAROS Hub on this machine as a local app. On the first run of this "
		"executable, click Allow in the QAROS approval prompt (waiting up to %u s).\n",
		EXAMPLE_LOCAL_APP_APPROVAL_TIMEOUT_MS / 1000u
	);
	const int exit_code = example_join_as_local_app(runtime, display_name, app_tag, out_session);
	if(exit_code == 0)
	{
		printf(
			"qar_runtime_onboard: SUCCESS (local app). Nothing is stored: the next run joins "
			"the same way, without a prompt while this executable is unchanged.\n"
		);
	}
	return exit_code;
}
//! [example_join_session]

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
