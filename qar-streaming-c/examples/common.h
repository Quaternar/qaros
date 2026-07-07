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

//! [example_obtain_session]
/** \brief Obtain a session the canonical way: rejoin with the persisted
 *  onboarding id when one exists, otherwise onboard with the pairing code
 *  currently shown on the hub's onboarding screen.
 *
 *  Returns 0 on success; on success *out_session and *out_onboarding_id are
 *  set and the id has been persisted to id_file_path for the next run. */
static int
example_obtain_session(
	QarRuntime* runtime,
	const char* pairing_code, /* may be NULL when a persisted id exists */
	const char* id_file_path,
	const char* display_name,
	QarOnboardingId* out_onboarding_id,
	QarSession** out_session
)
{
	QarPeerPresentation presentation = qar_peer_presentation_default();
	presentation.display_name = display_name;

	/* 1. Silent rejoin with the persisted identity — the normal path. */
	QarOnboardingId saved_id = qar_onboarding_id_default();
	if(load_onboarding_id(id_file_path, &saved_id))
	{
		QarRejoinInit rejoin_init = qar_rejoin_init_default();
		rejoin_init.onboarding_id = saved_id;
		rejoin_init.presentation = presentation;

		QarResult rejoin_result = qar_runtime_rejoin(
			runtime, &rejoin_init, NULL, NULL, NULL, out_session
		);
		log_result("qar_runtime_rejoin", rejoin_result);
		if(qar_result_is_success(rejoin_result))
		{
			*out_onboarding_id = saved_id;
			return 0;
		}
		if(!qar_result_has_code(
			   rejoin_result, QAR_STATUS_ONBOARDING_SESSION_NOT_FOUND
		   ))
		{
			return 1; /* real error (hub unreachable, ...) */
		}
		/* No persisted slot for the id -> fall through to onboarding. */
	}

	/* 2. First connect: onboard with the code shown by the hub. */
	if(!pairing_code)
	{
		printf(
			"No persisted onboarding id in '%s'. Pass the pairing code shown "
			"on the hub's onboarding screen.\n",
			id_file_path
		);
		return 1;
	}

	QarOnboardWithCodeInit onboard_init = qar_onboard_with_code_init_default();
	onboard_init.presentation = presentation;
	onboard_init.code = pairing_code;

	QarOnboardingId new_id = qar_onboarding_id_default();
	QarResult onboard_result = qar_runtime_onboard_with_code(
		runtime, &onboard_init, NULL, NULL, NULL, &new_id, out_session
	);
	log_result("qar_runtime_onboard_with_code", onboard_result);
	if(qar_result_is_error(onboard_result))
	{
		if(qar_result_has_code(onboard_result, QAR_STATUS_PAKE_ERROR))
		{
			printf(
				"Pairing code rejected or expired — read the current code "
				"off the hub screen and retry.\n"
			);
		}
		return 1;
	}

	/* 3. Persist the id — it is the ticket for every later rejoin/forget. */
	save_onboarding_id(id_file_path, &new_id);
	*out_onboarding_id = new_id;
	return 0;
}
//! [example_obtain_session]

/** \brief Extract directory portion from a path (both '/' and '\\' on Windows).
 */
static const char*
get_dir_from_path(const char* path, char* out_dir, size_t out_sz)
{
	if(!path || !out_dir || out_sz == 0)
	{
		return NULL;
	}
	size_t n = strlen(path);
	size_t i = n;
	while(i > 0)
	{
		char c = path[i - 1];
		if(c == '/' || c == '\\')
		{
			break;
		}
		--i;
	}
	if(i == 0)
	{
		// No separator found; use current directory
		snprintf(out_dir, out_sz, ".");
		return out_dir;
	}
	size_t dir_len = (i > out_sz - 1) ? (out_sz - 1) : i;
	memcpy(out_dir, path, dir_len);
	out_dir[dir_len] = '\0';
	return out_dir;
}