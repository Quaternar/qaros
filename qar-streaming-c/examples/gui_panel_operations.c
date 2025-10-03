/** \file gui_panel_operations.c
 *  \brief Demonstrates creating, updating, and enumerating GUI panels.
 *  \example gui_panel_operations.c
 */

/** \page qar_c_tutorial_gui_panels GUI Panel Operations
 * \tableofcontents
 *
 * \section gui_panels_overview Overview
 * - Host and join a session
 * - Add a GUI panel to the scene
 * - Adjust pose, size, state, and navigate to content
 * - Enumerate GUI panels to inspect current values
 *
 * \section gui_panels_prereq Prerequisites
 * - Complete the \ref qar_c_tutorial_app_volumes tutorial
 *
 * \section gui_panels_build Build and Run
 * \code{.bash}
 * cmake --build --preset x64-windows-debug --target gui_panel_operations
 * ./build/x64-windows/Debug/gui_panel_operations.exe
 * <path-to-qar-streaming-c.dll> [runtime-dir]
 * \endcode
 *
 * \section gui_panels_args Parse Arguments
 * \snippet gui_panel_operations.c gui_args
 *
 * \section gui_panels_setup Set Up the Session
 * \snippet gui_panel_operations.c gui_setup
 *
 * \section gui_panels_create Create and Update the Panel
 * \snippet gui_panel_operations.c gui_create
 *
 * \section gui_panels_list Enumerate GUI Panels
 * \snippet gui_panel_operations.c gui_list
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
	const char* name = program_name ? program_name : "gui_panel_operations";
	printf(
		"Usage: %s <path-to-qar-streaming-c-library> [runtime-binaries-dir]\n",
		name
	);
}

int
main(int argc, char** argv)
{
	//! [gui_args]
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
	//! [gui_args]

	//! [gui_setup]
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

	QarSessionCreateInit create_init = qar_session_create_init_default();
	QarSessionInvite* invite = NULL;
	QarResult invite_result =
		qar_runtime_create_session(runtime, &create_init, &invite);
	if(qar_result_is_error(invite_result) || invite == NULL)
	{
		log_result("qar_runtime_create_session", invite_result);
		qar_runtime_destroy(runtime);
		qar_library_destroy();
		qar_library_unload();
		return 5;
	}

	QarSession* session = NULL;
	QarSessionJoinInit join_init = qar_session_join_init_default();
	join_init.invite_data = invite->data;
	join_init.invite_data_size = invite->data_size;
	join_init.peer_spec_init.display_name = "GUI Panel Peer";

	QarResult join_result = qar_session_join(&join_init, &session);
	log_result("qar_session_join", join_result);
	qar_session_invite_destroy(invite);
	if(qar_result_is_error(join_result) || session == NULL)
	{
		qar_runtime_destroy(runtime);
		qar_library_destroy();
		qar_library_unload();
		return 6;
	}
	//! [gui_setup]

	//! [gui_create]
	QarGuiPanelInit panel_init = qar_gui_panel_init_default();
	panel_init.display_name = "Tutorial Panel";
	panel_init.pose = qar_pose_default();
	panel_init.pose.position.x = 0.5f;
	panel_init.pose.position.y = 1.5f;
	panel_init.pose.position.z = -1.2f;
	panel_init.size = qar_gui_panel_size_default();
	panel_init.size.width_meters = 1.2f;
	panel_init.size.height_meters = 0.7f;

	QarGuiPanelId panel_id = qar_gui_panel_id_default();
	QarResult add_result =
		qar_gui_panels_add_panel(session, &panel_init, &panel_id);
	log_result("qar_gui_panels_add_panel", add_result);
	if(qar_result_is_success(add_result))
	{
		printf("Created GUI panel with id: ");
		print_hex_id(panel_id.data, QAR_MAX_ID_LENGTH);
		printf("\n");
	}

	QarPose new_pose = qar_pose_default();
	new_pose.position.x = 0.2f;
	new_pose.position.y = 1.6f;
	new_pose.position.z = -1.0f;
	log_result(
		"qar_gui_panels_update_pose",
		qar_gui_panels_update_pose(session, &panel_id, &new_pose)
	);

	QarGuiPanelSize new_size = qar_gui_panel_size_default();
	new_size.width_meters = 1.0f;
	new_size.height_meters = 0.6f;
	log_result(
		"qar_gui_panels_change_size",
		qar_gui_panels_change_size(session, &panel_id, &new_size)
	);

	QAR_ASSUME_SUCCESS(qar_gui_panels_set_state(
		session, &panel_id, QAR_GUI_PANEL_STATE_MINIMIZED
	));

	QAR_ASSUME_SUCCESS(qar_gui_panels_navigate_to_uri(
		session, &panel_id, "https://example.com/tutorial"
	));
	//! [gui_create]

	//! [gui_list]
	size_t panel_count = 0;
	QarResult count_result = qar_query_gui_panels_count(session, &panel_count);
	log_result("qar_query_gui_panels_count", count_result);
	if(qar_result_is_success(count_result) && panel_count > 0)
	{
		QarGuiPanel** handles =
			(QarGuiPanel**)calloc(panel_count, sizeof(QarGuiPanel*));
		if(handles)
		{
			size_t written = 0;
			QarResult list_result =
				qar_query_gui_panels(session, handles, panel_count, &written);
			log_result("qar_query_gui_panels", list_result);
			if(qar_result_is_success(list_result))
			{
				for(size_t i = 0; i < written; ++i)
				{
					QarGuiPanel* handle = handles[i];
					if(!handle)
					{
						continue;
					}

					QarGuiPanelId id = qar_gui_panel_id_default();
					QarGuiPanelSize size = qar_gui_panel_size_default();
					QarPose pose = qar_pose_default();
					QarGuiPanelState state = QAR_GUI_PANEL_STATE_VISIBLE;
					char name[QAR_MAX_STRING_LENGTH] = { 0 };

					(void)qar_gui_panel_get_id(handle, &id);
					(void)qar_gui_panel_get_display_name(
						handle, name, sizeof(name)
					);
					(void)qar_gui_panel_get_size(handle, &size);
					(void)qar_gui_panel_get_pose(handle, &pose);
					(void)qar_gui_panel_get_state(handle, &state);

					printf("- Panel id: ");
					print_hex_id(id.data, QAR_MAX_ID_LENGTH);
					printf(
						" name='%s' size(%.2f x %.2f) position(%.2f, %.2f, "
						"%.2f) state=%d\n",
						name,
						size.width_meters,
						size.height_meters,
						pose.position.x,
						pose.position.y,
						pose.position.z,
						(int)state
					);

					qar_gui_panel_handle_destroy(handle);
				}
			}

			free(handles);
		}
	}

	log_result(
		"qar_gui_panels_close_panel",
		qar_gui_panels_close_panel(session, &panel_id)
	);
	//! [gui_list]

	qar_session_destroy(session);
	qar_runtime_destroy(runtime);
	QarResult destroy_result = qar_library_destroy();
	log_result("qar_library_destroy", destroy_result);
	qar_library_unload();
	return 0;
}