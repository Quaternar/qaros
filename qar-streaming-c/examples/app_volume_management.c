/** \file app_volume_management.c
 *  \brief Creates an app volume and enumerates existing volumes in the session.
 *  \example app_volume_management.c
 */

/** \page qar_c_tutorial_app_volumes App Volume Management
 * \tableofcontents
 *
 * \section app_volume_overview Overview
 * - Obtain a session (rejoin, or onboard with the hub pairing code)
 * - Get-or-create a single app volume by its stable common name
 * - Enumerate and inspect every app volume handle
 *
 * \section app_volume_prereq Prerequisites
 * - Complete the \ref qar_c_tutorial_onboarding tutorial
 * - A running QarOS hub (for the first run, read the pairing code off its
 *   onboarding screen)
 *
 * \section app_volume_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target app_volume_management
 * ./build/x64-windows/Debug/app_volume_management.exe
 * <path-to-qar-streaming-c.dll> [runtime-dir] [pairing-code]
 * \endcode
 *
 * \section app_volume_args Parse Arguments
 * \snippet app_volume_management.c app_args
 *
 * \section app_volume_setup Load, Init, and Obtain a Session
 * The session is obtained with the shared rejoin-or-onboard helper described
 * in the \ref qar_c_tutorial_onboarding tutorial.
 * \snippet app_volume_management.c app_setup
 *
 * \section app_volume_create Create and List
 * \snippet app_volume_management.c app_create
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
	const char* name = program_name ? program_name : "app_volume_management";
	printf(
		"Usage: %s <path-to-qar-streaming-c-library> [runtime-binaries-dir] "
		"[pairing-code]\n",
		name
	);
	printf(
		"The pairing code is required on the first run only; later runs "
		"rejoin with the persisted onboarding id.\n"
	);
}

int
main(int argc, char** argv)
{
	//! [app_args]
	if(argc < 2)
	{
		print_usage(argv[0]);
		return 1;
	}

	const char* library_path = argv[1];
	const char* runtime_dir = NULL;
	char runtime_dir_buffer[1024] = { 0 };

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

	const char* pairing_code = (argc >= 4) ? argv[3] : NULL;
	//! [app_args]

	//! [app_setup]
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

	QarRuntime* runtime = NULL;
	QarRuntimeInit runtime_init = qar_runtime_init_default();
	runtime_init.runtime_binaries_folder_path = runtime_dir;

	QarResult runtime_result = qar_runtime_create(&runtime_init, &runtime);
	if(qar_result_is_error(runtime_result) || runtime == NULL)
	{
		log_result("qar_runtime_create", runtime_result);
		qar_library_destroy();
		qar_library_unload();
		return 4;
	}

	QarOnboardingId onboarding_id = qar_onboarding_id_default();
	QarSession* session = NULL;
	if(example_obtain_session(
		   runtime,
		   pairing_code,
		   "app_volume_management.onboarding-id.txt",
		   "App Volume Peer",
		   &onboarding_id,
		   &session
	   ) != 0
	   || session == NULL)
	{
		qar_runtime_destroy(runtime);
		qar_library_destroy();
		qar_library_unload();
		return 5;
	}
	//! [app_setup]

	//! [app_create]
	QarAppVolumeInit volume_init = qar_app_volume_init_default();
	volume_init.common_name = "tutorial-volume.examples.qaros";
	volume_init.display_name = "Tutorial Volume";
	volume_init.pose = qar_pose_default();
	volume_init.pose.position.z = -1.0f;
	volume_init.size = qar_app_volume_size_default();
	volume_init.size.width_meters = 1.0f;
	volume_init.size.length_meters = 1.0f;
	volume_init.size.height_meters = 1.0f;

	QarAppVolumeId volume_id = qar_app_volume_id_default();
	QarResult create_result =
		qar_app_volumes_get_or_create(session, &volume_init, &volume_id);
	log_result("qar_app_volumes_get_or_create", create_result);
	if(qar_result_is_success(create_result))
	{
		printf("Created app volume with id: ");
		print_hex_id(volume_id.data, QAR_MAX_ID_LENGTH);
		printf("\n");
	}

	size_t volume_count = 0;
	QarResult count_result =
		qar_query_app_volumes_count(session, &volume_count);
	log_result("qar_query_app_volumes_count", count_result);
	if(qar_result_is_success(count_result) && volume_count > 0)
	{
		QarAppVolume** handles =
			(QarAppVolume**)calloc(volume_count, sizeof(QarAppVolume*));
		if(handles)
		{
			size_t written = 0;
			QarResult list_result =
				qar_query_app_volumes(session, handles, volume_count, &written);
			log_result("qar_query_app_volumes", list_result);
			if(qar_result_is_success(list_result))
			{
				for(size_t i = 0; i < written; ++i)
				{
					QarAppVolume* handle = handles[i];
					if(!handle)
					{
						continue;
					}

					QarAppVolumeId id = qar_app_volume_id_default();
					QarAppVolumeSize desc = qar_app_volume_size_default();
					QarPose pose = qar_pose_default();
					char name[QAR_MAX_STRING_LENGTH] = { 0 };

					(void)qar_app_volume_get_id(handle, &id);
					(void)qar_app_volume_get_size(handle, &desc);
					(void)qar_app_volume_get_pose(handle, &pose);
					(void)qar_app_volume_get_display_name(
						handle, name, sizeof(name)
					);

					printf("- Volume id: ");
					print_hex_id(id.data, QAR_MAX_ID_LENGTH);
					printf(
						" name='%s' size(%.2f x %.2f x %.2f) position(%.2f, "
						"%.2f, %.2f)\n",
						name,
						desc.width_meters,
						desc.length_meters,
						desc.height_meters,
						pose.position.x,
						pose.position.y,
						pose.position.z
					);

					qar_app_volume_handle_destroy(handle);
				}
			}

			free(handles);
		}
	}
	//! [app_create]

	log_result("qar_session_leave", qar_session_leave(session));
	qar_session_handle_destroy(session);
	qar_runtime_destroy(runtime);
	QarResult destroy_result = qar_library_destroy();
	log_result("qar_library_destroy", destroy_result);
	qar_library_unload();
	return 0;
}
