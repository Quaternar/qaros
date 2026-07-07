/** \file onboarding_and_rejoin.c
 *  \brief Obtains a session via rejoin-or-onboard, onboards a sibling runtime
 *  from a shared invite, and demonstrates destroy/forget teardown.
 *  \example onboarding_and_rejoin.c
 */

/** \page qar_c_tutorial_onboarding Onboarding and Rejoin
 * \tableofcontents
 *
 * \section onboarding_overview Overview
 * Every QarOS session is obtained through onboarding - there is no direct
 * create/join. This tutorial shows the canonical startup flow:
 * - Reuse dynamic loading from the previous tutorial
 * - Try a silent rejoin with the persisted onboarding id
 * - Fall back to onboarding with the pairing code shown by the hub
 * - Persist the returned onboarding id for the next run
 * - Mint a serialized invite for a sibling API instance
 * - Start a second runtime instance and onboard it from that invite
 * - Tear down by destroying the session handle (and optionally forget)
 *
 * \section onboarding_prereq Prerequisites
 * - Complete the \ref qar_c_tutorial_dynamic_loading tutorial
 * - A running QarOS hub on this machine (or reachable on the network)
 * - For the first run: the pairing code from the hub's onboarding screen
 *
 * \section onboarding_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target onboarding_and_rejoin
 * ./build/x64-windows/Debug/onboarding_and_rejoin.exe
 * <path-to-qar-streaming-c.dll> [runtime-dir] [pairing-code] [--forget]
 * \endcode
 *
 * \section onboarding_args Parse Arguments
 * \snippet onboarding_and_rejoin.c onboarding_args
 *
 * \section onboarding_persistence Persisting the Onboarding Id
 * The onboarding id is the only thing an application must store. These
 * shared helpers keep it in a small text file as a UUID string:
 * \snippet common.h example_onboarding_persistence
 *
 * \section onboarding_obtain Rejoin or Onboard
 * The canonical flow - rejoin silently when a persisted id exists, otherwise
 * onboard with the code and persist the returned id:
 * \snippet common.h example_obtain_session
 *
 * Which is used from main like this:
 * \snippet onboarding_and_rejoin.c onboarding_obtain
 *
 * \section onboarding_invite Share an Invite with a Second API Instance
 * A second API instance in the same application (another process, or a C#
 * binding next to native code) must not pair with a code again. The primary
 * onboarded instance requests an invite, serializes it, and the sibling
 * instance consumes it with qar_runtime_onboard_with_invite:
 * \snippet onboarding_and_rejoin.c onboarding_invite
 *
 * \section onboarding_teardown Destroy and Forget
 * Destroying the session handle keeps the persisted identity so the next run
 * can rejoin. Forget erases it - only do that for an explicit "un-pair this
 * device" action:
 * \snippet onboarding_and_rejoin.c onboarding_teardown
 */

#include "common.h"

#ifndef QAR_ENABLE_DYNAMIC_LOADING
#define QAR_ENABLE_DYNAMIC_LOADING
#endif
#include <qar_streaming.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

static void
print_usage(const char* program_name)
{
	const char* name = program_name ? program_name : "onboarding_and_rejoin";
	printf(
		"Usage: %s <path-to-qar-streaming-c-library> [runtime-binaries-dir] "
		"[pairing-code] [--forget]\n",
		name
	);
	printf(
		"The pairing code is required on the first run only; later runs "
		"rejoin with the persisted onboarding id.\n"
	);
	printf("Pass --forget to erase the persisted identity on exit.\n");
}

static QarRuntime*
create_runtime_or_null(const char* runtime_dir, const char* storage_dir)
{
	QarRuntime* runtime = NULL;
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	runtime_init.runtime_binaries_folder_path = runtime_dir;
	runtime_init.storage_folder_path = storage_dir;

	QarResult runtime_result = qar_runtime_create(&runtime_init, &runtime);
	log_result("qar_runtime_create", runtime_result);
	if(qar_result_is_error(runtime_result) || runtime == NULL)
	{
		return NULL;
	}

	return runtime;
}

int
main(int argc, char** argv)
{
	//! [onboarding_args]
	if(argc < 2)
	{
		print_usage(argv[0]);
		return 1;
	}

	const char* library_path = argv[1];
	const char* runtime_dir = NULL;
	char runtime_dir_buffer[1024] = { 0 };
	const char* pairing_code = NULL;
	bool forget_on_exit = false;

	if(argc >= 3)
	{
		runtime_dir = argv[2];
	}
	else if(get_dir_from_path(
				library_path, runtime_dir_buffer, sizeof(runtime_dir_buffer)
			))
	{
		runtime_dir = runtime_dir_buffer;
	}

	for(int i = 3; i < argc; ++i)
	{
		if(strcmp(argv[i], "--forget") == 0)
		{
			forget_on_exit = true;
		}
		else
		{
			pairing_code = argv[i];
		}
	}
	//! [onboarding_args]

	if(!qar_library_load(library_path))
	{
		fprintf(stderr, "Failed to load '%s'.\n", library_path);
		return 2;
	}

	QarLibraryInit library_init = qar_library_init_default();
	library_init.enable_console_logging = true;
	QarResult library_result = qar_library_init(&library_init);
	if(qar_result_is_error(library_result))
	{
		log_result("qar_library_init", library_result);
		qar_library_unload();
		return 3;
	}

	QarRuntime* primary_runtime = create_runtime_or_null(
		runtime_dir, "onboarding_and_rejoin.primary-state"
	);
	if(primary_runtime == NULL)
	{
		qar_library_destroy();
		qar_library_unload();
		return 4;
	}

	//! [onboarding_obtain]
	QarOnboardingId primary_onboarding_id = qar_onboarding_id_default();
	QarOnboardingId sibling_onboarding_id = qar_onboarding_id_default();
	bool sibling_onboarded = false;
	QarSession* primary_session = NULL;
	if(example_obtain_session(
		   primary_runtime,
		   pairing_code,
		   "onboarding_and_rejoin.primary-onboarding-id.txt",
		   "Primary Tutorial Peer",
		   &primary_onboarding_id,
		   &primary_session
	   ) != 0
	   || primary_session == NULL)
	{
		qar_runtime_destroy(primary_runtime);
		qar_library_destroy();
		qar_library_unload();
		return 5;
	}

	char primary_id_text[64] = { 0 };
	if(qar_result_is_success(qar_uuid_to_string(
		   primary_onboarding_id.data, primary_id_text, sizeof(primary_id_text)
	   )))
	{
		printf(
			"Primary session obtained with onboarding id %s\n", primary_id_text
		);
	}
	//! [onboarding_obtain]

	//! [onboarding_invite]
	QarRequestInviteInit request_init = qar_request_invite_init_default();
	QarOnboardingInvite* primary_invite = NULL;
	QarResult invite_result = qar_session_request_onboarding_invite(
		primary_session, &request_init, NULL, NULL, NULL, &primary_invite
	);
	log_result("qar_session_request_onboarding_invite", invite_result);
	if(qar_result_is_success(invite_result) && primary_invite != NULL)
	{
		size_t serialized_size = 0;
		QAR_ASSUME_SUCCESS(qar_onboarding_invite_serialized_size(
			primary_invite, &serialized_size
		));

		uint8_t* blob = (uint8_t*)calloc(1, serialized_size);
		if(blob != NULL)
		{
			size_t bytes_written = 0;
			QAR_ASSUME_SUCCESS(qar_onboarding_invite_serialize(
				primary_invite, blob, serialized_size, &bytes_written
			));

			int64_t expires_unix = 0;
			QAR_ASSUME_SUCCESS(qar_onboarding_invite_get_expires_unix(
				primary_invite, &expires_unix
			));
			printf(
				"Minted sibling invite (%zu bytes, expires at unix %lld).\n",
				bytes_written,
				(long long)expires_unix
			);

			QarRuntime* sibling_runtime = create_runtime_or_null(
				runtime_dir, "onboarding_and_rejoin.sibling-state"
			);
			if(sibling_runtime != NULL)
			{
				QarOnboardingInvite* sibling_invite = NULL;
				QarResult deserialize_result =
					qar_onboarding_invite_deserialize(
						blob, bytes_written, &sibling_invite
					);
				log_result(
					"qar_onboarding_invite_deserialize", deserialize_result
				);
				if(qar_result_is_success(deserialize_result)
				   && sibling_invite != NULL)
				{
					QarOnboardWithInviteInit sibling_init =
						qar_onboard_with_invite_init_default();
					sibling_init.presentation.display_name =
						"Sibling Tutorial Peer";
					sibling_init.invite = sibling_invite;

					QarOnboardingId sibling_onboarding_id =
						qar_onboarding_id_default();
					QarSession* sibling_session = NULL;
					QarResult sibling_result = qar_runtime_onboard_with_invite(
						sibling_runtime,
						&sibling_init,
						NULL,
						NULL,
						NULL,
						&sibling_onboarding_id,
						&sibling_session
					);
					log_result(
						"qar_runtime_onboard_with_invite", sibling_result
					);
					if(qar_result_is_success(sibling_result)
					   && sibling_session != NULL)
					{
						char sibling_id_text[64] = { 0 };
						if(qar_result_is_success(qar_uuid_to_string(
							   sibling_onboarding_id.data,
							   sibling_id_text,
							   sizeof(sibling_id_text)
						   )))
						{
							printf(
								"Sibling runtime onboarded with onboarding id "
								"%s\n",
								sibling_id_text
							);
						}

						save_onboarding_id(
							"onboarding_and_rejoin.sibling-onboarding-id.txt",
							&sibling_onboarding_id
						);
						sibling_onboarded = true;
						printf(
							"Destroying sibling session handle keeps its "
							"persisted onboarding state for rejoin.\n"
						);
						qar_session_handle_destroy(sibling_session);
					}

					qar_onboarding_invite_handle_destroy(sibling_invite);
				}

				qar_runtime_destroy(sibling_runtime);
			}

			free(blob);
		}

		qar_onboarding_invite_handle_destroy(primary_invite);
	}
	//! [onboarding_invite]

	//! [onboarding_teardown]
	/* Destroying the handle leaves the session and keeps persisted identity,
	 * so the next run can rejoin silently. */
	printf(
		"Destroying primary session handle keeps its persisted onboarding "
		"state for rejoin.\n"
	);
	qar_session_handle_destroy(primary_session);

	if(forget_on_exit)
	{
		/* Full un-pair: erase the identity slot. The next run needs a fresh
		 * pairing code. Must happen after the session was left. */
		if(sibling_onboarded)
		{
			QarRuntime* sibling_runtime = create_runtime_or_null(
				runtime_dir, "onboarding_and_rejoin.sibling-state"
			);
			if(sibling_runtime != NULL)
			{
				QarForgetInit sibling_forget_init = qar_forget_init_default();
				sibling_forget_init.onboarding_id = sibling_onboarding_id;
				log_result(
					"qar_runtime_forget(sibling)",
					qar_runtime_forget(sibling_runtime, &sibling_forget_init)
				);
				qar_runtime_destroy(sibling_runtime);
			}
			remove("onboarding_and_rejoin.sibling-onboarding-id.txt");
		}

		QarForgetInit forget_init = qar_forget_init_default();
		forget_init.onboarding_id = primary_onboarding_id;
		log_result(
			"qar_runtime_forget(primary)",
			qar_runtime_forget(primary_runtime, &forget_init)
		);
		remove("onboarding_and_rejoin.primary-onboarding-id.txt");
	}
	//! [onboarding_teardown]

	qar_runtime_destroy(primary_runtime);
	QarResult destroy_result = qar_library_destroy();
	log_result("qar_library_destroy", destroy_result);
	qar_library_unload();
	return 0;
}
