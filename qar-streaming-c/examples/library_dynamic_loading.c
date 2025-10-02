
/** \file tutorial_dynamic_loading.c
 * \brief End-to-end C tutorial for qar-streaming-c using dynamic loading.
 *
 * This source doubles as the single truth for the tutorial. Doxygen parses the
 * narrative documentation blocks below and doxygen2docusaurus converts them to
 * Markdown, so docs stay in sync with the runnable code.
 *
 * \par What you will learn
 *  - Dynamically load the qar-streaming-c shared library
 *  - Initialize the library
 *  - Create a runtime and a session (host + join via invite)
 *  - Create a GUI panel and an App Volume
 *  - Enumerate GUI panels and App Volumes, logging their properties
 *  - Clean up all resources
 *
 * \par Build notes
 *  - This example uses the dynamic loading mode. Define \c
 * QAR_ENABLE_DYNAMIC_LOADING and ensure the shared library (e.g., \c
 * qar-streaming-c.dll/.so/.dylib) is present.
 *  - No link-time dependency on the qar-streaming-c shared library is required.
 *
 * \par Runtime notes
 *  - Pass the path to the \c qar-streaming-c shared library as the first
 * argument. Optionally pass a second argument with the runtime binaries folder
 * path. If not provided, the example will try to use the directory of the
 * library for runtime binaries discovery as well.
 *
 * \par Doxygen
 *  - This file is included as an example in the API documentation.
 *  - \example tutorial_dynamic_loading.c
 */

/** \page qar_c_tutorial_dynamic_loading qar-streaming-c: Dynamic Loading
 * \note **Important:** note.
 * \warning **Important:** warning.
 * \attention attention
 * \todo add todo here
 *
 * \tableofcontents
 *
 * \section dlt_overview Overview
 * Demonstrates the C API workflow using dynamic loading across these groups:
 * \ref qar_c_dynamic_loading, \ref qar_c_library, \ref qar_c_runtime,
 * \ref qar_c_session, \ref qar_c_gui, and \ref qar_c_app_volumes.
 *
 * \section dlt_prereq Prerequisites
 * - Build the SDK with the provided CMake presets.
 * - Locate the built shared library (e.g., Windows: bin/qar-streaming-c.dll).
 *
 * \section dlt_build_run Build and Run
 * Windows:
 * \code{.bash}
 * ./run_with_vs2022.sh cmake --build --preset windows-x64-debug --target
 * qar-streaming-c # Run: tutorial_dynamic_loading.exe
 * <path-to-qar-streaming-c.dll> [runtime-binaries-dir]
 * \endcode
 * Linux:
 * \code{.bash}
 * cmake --build --preset linux-x64-debug --target qar-streaming-c
 * ./tutorial_dynamic_loading <path-to-libqar-streaming-c.so>
 * [runtime-binaries-dir]
 * \endcode
 *
 * \section dlt_includes Includes and Dynamic Loading Setup
 * We enable dynamic loading, include the C API headers, and declare storage for
 * function pointers. This lets your app load the library at runtime without a
 * link-time dependency.
 *
 * Why: decouples your app from the SDK binary location and makes deployment
 * more flexible (e.g., choose a DLL path at runtime).
 * \snippet tutorial_dynamic_loading.c includes_and_setup
 *
 * \section dlt_helpers Small Utilities
 * A few helpers keep the tutorial readable: formatted id printing, result
 * logging, and a path utility to derive the runtime folder from the library
 * path.
 *
 * Why: not part of the API, just quality-of-life utilities for a clearer demo.
 * \snippet tutorial_dynamic_loading.c helpers
 *
 * \section dlt_args Arguments and Paths
 * We parse the command-line arguments to get the shared library path and an
 * optional runtime binaries directory (defaults to the library’s directory).
 *
 * Why: the runtime discovers backends and assets from this directory.
 * \snippet tutorial_dynamic_loading.c args_and_paths
 *
 * \section dlt_load Load the Library
 * Load the shared library and initialize the dynamic dispatch tables.
 *
 * Why: all subsequent API calls route through dynamically loaded functions.
 * \snippet tutorial_dynamic_loading.c load_library
 *
 * \section dlt_init Initialize the Library
 * Initialize global state and enable console logging for visibility.
 *
 * Why: required once per process before creating runtimes or sessions.
 * \snippet tutorial_dynamic_loading.c init_library
 *
 * \section dlt_runtime Create a Runtime
 * Create a runtime and point it to the runtime binaries folder for backend
 * discovery.
 *
 * Why: the runtime hosts sessions and services used by your app.
 * \snippet tutorial_dynamic_loading.c create_runtime
 *
 * \section dlt_session_host Host a Session Invite
 * Create a session invite on the host side; this blob can be shared with peers
 * who wish to join.
 *
 * Why: forms the handshake payload to establish a session.
 * \snippet tutorial_dynamic_loading.c create_session_invite
 *
 * \section dlt_session_join Join the Session
 * Join the session using the invite and provide a peer display name.
 *
 * Why: obtains an active \ref QarSession handle to use GUI and app volumes.
 * \snippet tutorial_dynamic_loading.c join_session
 *
 * \section dlt_gui Create a GUI Panel
 * Add a GUI panel with a pose and size. Panels are 2D UI surfaces in the 3D
 * scene.
 *
 * Why: GUI panels are often used for HUDs, dashboards, or web content.
 * \snippet tutorial_dynamic_loading.c create_gui_panel
 *
 * \section dlt_volume Create an App Volume
 * Add a 3D app volume with a pose and size.
 *
 * Why: app volumes host content in 3D space and can be shared among peers.
 * \snippet tutorial_dynamic_loading.c create_app_volume
 *
 * \section dlt_enum_gui Enumerate GUI Panels
 * Query panels and log their properties (id, name, size, pose, state).
 *
 * Why: shows how to discover and inspect existing panels.
 * \snippet tutorial_dynamic_loading.c enumerate_gui_panels
 *
 * \section dlt_enum_vol Enumerate App Volumes
 * Query app volumes and log their properties (id, name, size, pose, lifetime).
 *
 * Why: shows how to discover and inspect existing volumes.
 * \snippet tutorial_dynamic_loading.c enumerate_app_volumes
 *
 * \section dlt_cleanup Cleanup
 * Close created entities (best-effort), destroy session and runtime, and
 * unload the library.
 *
 * Why: release resources deterministically and avoid leaks.
 * \snippet tutorial_dynamic_loading.c cleanup
 *
 * \section dlt_code Full Listing
 * The full listing for reference:
 * \snippet tutorial_dynamic_loading.c dynamic_loading_example
 *
 * \section dlt_steps Steps
 * 1. Load the shared library with \ref qar_library_load (\ref
 * qar_c_dynamic_loading).
 * 2. Initialize with \ref qar_library_init (\ref qar_c_library).
 * 3. Create a runtime via \ref qar_runtime_create (\ref qar_c_runtime).
 * 4. Create a session invite with \ref qar_runtime_create_session.
 * 5. Join the session using \ref qar_session_join (\ref qar_c_session).
 * 6. Create a GUI panel with \ref qar_gui_panels_add_panel (\ref qar_c_gui).
 * 7. Create an App Volume with \ref qar_app_volumes_add_volume (\ref
 * qar_c_app_volumes).
 * 8. Enumerate panels/volumes: \ref qar_query_gui_panels, \ref
 * qar_query_app_volumes and their getters.
 * 9. Clean up: destroy session/runtime, \ref qar_library_destroy, and \ref
 * qar_library_unload.
 */

//! [dynamic_loading_example]
//! [includes_and_setup]
#include "common.h"

// Enable dynamic loading and provide storage for dispatch pointers
#define QAR_ENABLE_DYNAMIC_LOADING

#include <qar_streaming.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

//! [includes_and_setup]

/**
 * \brief Entry point for the dynamic-loading tutorial.
 *
 * Arguments:
 *  - argv[1]: Path to the shared library (e.g., qar-streaming-c.dll/.so/.dylib)
 *  - argv[2]: Optional runtime binaries directory (defaults to library
 * directory)
 *
 * Follows the steps outlined in \ref qar_c_tutorial_dynamic_loading.
 * \return 0 on success, non-zero on failure.
 */
int
main(int argc, char** argv)
{
	//! [args_and_paths]
	const char* lib_path = NULL;
	const char* runtime_bins = NULL;

	if(argc >= 2)
	{
		lib_path = argv[1];
	}
	else
	{
		printf(
			"Usage: %s <path-to-qar-streaming-c-library> "
			"[runtime-binaries-dir]\n",
			argv[0]
		);
		printf(
			"Example (Windows): %s "
			"build\\windows-x64-debug\\bin\\qar-streaming-c.dll\n",
			argv[0]
		);
		return 1;
	}

	if(argc >= 3)
	{
		runtime_bins = argv[2];
	}
	else
	{
		// Default to the directory of the library
		static char dir_buf[1024];
		if(get_dir_from_path(lib_path, dir_buf, sizeof(dir_buf)))
		{
			runtime_bins = dir_buf;
		}
	}
	//! [args_and_paths]

	//! [load_library]
	printf("Loading library: %s\n", lib_path);
	if(!qar_library_load(lib_path))
	{
		fprintf(stderr, "Failed to load library. Aborting.\n");
		return 2;
	}
	//! [load_library]

	//! [init_library]
	QarLibraryInit lib_init = qar_library_init_default();
	lib_init.enable_console_logging = true;
	QarResult r = qar_library_init(&lib_init);
	if(qar_result_is_error(r))
	{
		log_result("qar_library_init", r);
		qar_library_unload();
		return 3;
	}
	//! [init_library]

	//! [create_runtime]
	QarRuntime* runtime = NULL;
	QarRuntimeInit rt_init = qar_runtime_init_default();
	rt_init.runtime_binaries_folder_path = runtime_bins;
	r = qar_runtime_create(&rt_init, &runtime);
	if(qar_result_is_error(r) || runtime == NULL)
	{
		log_result("qar_runtime_create", r);
		qar_library_destroy();
		qar_library_unload();
		return 4;
	}
	//! [create_runtime]

	//! [create_session_invite]
	QarSessionCreateInit s_create = qar_session_create_init_default();
	QarSessionInvite* invite = NULL;
	r = qar_runtime_create_session(runtime, &s_create, &invite);
	if(qar_result_is_error(r) || !invite)
	{
		log_result("qar_runtime_create_session", r);
		qar_runtime_destroy(runtime);
		qar_library_destroy();
		qar_library_unload();
		return 5;
	}
	//! [create_session_invite]

	//! [join_session]
	QarSession* session = NULL;
	QarSessionJoinInit join = qar_session_join_init_default();
	join.invite_data = invite->data;
	join.invite_data_size = invite->data_size;
	join.peer_spec_init = qar_peer_spec_init_default();
	join.peer_spec_init.display_name = "C Tutorial Peer";
	r = qar_session_join(&join, &session);
	// Invite no longer needed after join
	qar_session_invite_destroy(invite);
	if(qar_result_is_error(r) || !session)
	{
		log_result("qar_session_join", r);
		qar_runtime_destroy(runtime);
		qar_library_destroy();
		qar_library_unload();
		return 6;
	}
	//! [join_session]

	//! [create_gui_panel]
	QarGuiPanelId panel_id = qar_gui_panel_id_default();
	QarGuiPanelInit p_init = qar_gui_panel_init_default();
	p_init.display_name = "Tutorial Panel";
	p_init.pose = qar_pose_default();
	p_init.size = qar_gui_panel_size_default();
	p_init.size.width_meters = 1.2f;
	p_init.size.height_meters = 0.7f;
	r = qar_gui_panels_add_panel(session, &p_init, &panel_id);
	log_result("qar_gui_panels_add_panel", r);
	//! [create_gui_panel]

	//! [create_app_volume]
	QarAppVolumeId volume_id = qar_app_volume_id_default();
	QarAppVolumeInit v_init = qar_app_volume_init_default();
	v_init.display_name = "Tutorial Volume";
	v_init.pose = qar_pose_default();
	v_init.size = qar_app_volume_size_default();
	v_init.size.width_meters = 1.0f;
	v_init.size.height_meters = 1.0f;
	v_init.size.length_meters = 1.0f;
	r = qar_app_volumes_add_volume(session, &v_init, &volume_id);
	log_result("qar_app_volumes_add_volume", r);
	//! [create_app_volume]

	//! [enumerate_gui_panels]
	size_t panel_count = 0;
	if(qar_result_is_success(qar_query_gui_panels_count(session, &panel_count)))
	{
		printf("GUI Panels (%zu)\n", panel_count);
		if(panel_count > 0)
		{
			QarGuiPanel** handles =
				(QarGuiPanel**)calloc(panel_count, sizeof(QarGuiPanel*));

			size_t written = 0;
			r = qar_query_gui_panels(session, handles, panel_count, &written);
			if(qar_result_is_success(r))
			{
				for(size_t i = 0; i < written; ++i)
				{
					QarGuiPanel* h = handles[i];
					QarGuiPanelId id = qar_gui_panel_id_default();
					char name[QAR_MAX_STRING_LENGTH] = { 0 };
					QarGuiPanelSize sz = qar_gui_panel_size_default();
					QarPose pose = qar_pose_default();
					QarGuiPanelState st = QAR_GUI_PANEL_STATE_VISIBLE;
					(void)qar_gui_panel_get_id(h, &id);
					(void)qar_gui_panel_get_display_name(h, name, sizeof(name));
					(void)qar_gui_panel_get_size(h, &sz);
					(void)qar_gui_panel_get_pose(h, &pose);
					(void)qar_gui_panel_get_state(h, &st);
					printf("- Panel ID: ");
					print_hex_id(id.data, QAR_MAX_ID_LENGTH);
					printf(
						" name='%s' size(%.2f x %.2f) pos(%.2f,%.2f,%.2f) "
						"state=%d\n",
						name,
						sz.width_meters,
						sz.height_meters,
						pose.position.x,
						pose.position.y,
						pose.position.z,
						(int)st
					);
					qar_gui_panel_handle_destroy(h);
				}
			}
			else
			{
				log_result("qar_query_gui_panels", r);
			}
			free(handles);
		}
	}
	//! [enumerate_gui_panels]

	//! [enumerate_app_volumes]
	size_t vol_count = 0;
	if(qar_result_is_success(qar_query_app_volumes_count(session, &vol_count)))
	{
		printf("App Volumes (%zu)\n", vol_count);
		if(vol_count > 0)
		{
			QarAppVolume** vhandles =
				(QarAppVolume**)calloc(vol_count, sizeof(QarAppVolume*));
			size_t written = 0;
			r = qar_query_app_volumes(session, vhandles, vol_count, &written);
			if(qar_result_is_success(r))
			{
				for(size_t i = 0; i < written; ++i)
				{
					QarAppVolume* h = vhandles[i];
					QarAppVolumeId id = qar_app_volume_id_default();
					char name[QAR_MAX_STRING_LENGTH] = { 0 };
					QarAppVolumeSize sz = qar_app_volume_size_default();
					QarPose pose = qar_pose_default();
					(void)qar_app_volume_get_id(h, &id);
					(void)qar_app_volume_get_display_name(
						h, name, sizeof(name)
					);
					(void)qar_app_volume_get_size(h, &sz);
					(void)qar_app_volume_get_pose(h, &pose);
					printf("- Volume ID: ");
					print_hex_id(id.data, QAR_MAX_ID_LENGTH);
					printf(
						" name='%s' size(%.2f x %.2f x %.2f) "
						"pos(%.2f,%.2f,%.2f)",
						name,
						sz.width_meters,
						sz.length_meters,
						sz.height_meters,
						pose.position.x,
						pose.position.y,
						pose.position.z
					);
					qar_app_volume_handle_destroy(h);
				}
			}
			else
			{
				log_result("qar_query_app_volumes", r);
			}
			free(vhandles);
		}
	}
	//! [enumerate_app_volumes]

	//! [cleanup]

	// Step 9: Cleanup
	qar_session_destroy(session);
	qar_runtime_destroy(runtime);
	(void)qar_library_destroy();
	qar_library_unload();

	printf("Tutorial finished.\n");
	return 0;
}
//! [cleanup]

//! [dynamic_loading_example]
