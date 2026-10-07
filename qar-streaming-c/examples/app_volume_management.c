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
 * - Customise the volume's gesture mapping rules
 * - Set and clear a world anchor
 * - Map a room-space point into app content space and hit-test the cuboid
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
 * <installed|path-to-qar-streaming-c.dll> [runtime-dir] [pairing-code]
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
 *
 * \section app_volume_gestures Customise the Gesture Configuration
 * Start from qar_app_volume_gesture_configuration_default(), which mirrors
 * what a volume created without a configuration gets: two-hand distance ->
 * app scale, one-hand 6DoF -> app pose (rotation off), hover, click. Rules are
 * in descending priority. Change a rule in place, then replace the volume's
 * default mode with qar_app_volumes_change_gesture_configuration(). The same
 * struct can instead be passed in QarAppVolumeInit::gesture_configuration, but
 * get-or-create ignores it when the volume already exists.
 * \snippet app_volume_management.c app_gestures
 *
 * \section app_volume_anchor Pin to the Earth with a World Anchor
 * A world anchor states where app volume space sits on the Earth, as an ECEF
 * WGS84 frame. Here the volume's +X points east, +Y up and +Z south of a
 * latitude/longitude, which keeps the frame right-handed.
 * \snippet app_volume_management.c app_world_anchor
 *
 * \section app_volume_hit_test Room-Space Point to App Content Space
 * Gesture points already arrive in app content space. A point the application
 * has only in room space - a GUI panel's position, a device pose - is mapped
 * in two steps: room -> app volume space through the volume pose, whose
 * position is the cuboid **centre**, then app volume -> app content space
 * through the app pose and app scale. The cuboid test happens in app volume
 * space against the half extents.
 * \snippet app_volume_management.c app_room_to_app
 */

#include "common.h"

#include <math.h>

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
		"Usage: %s <installed|path-to-qar-streaming-c-library> "
		"[runtime-binaries-dir] "
		"[pairing-code]\n",
		name
	);
	printf(
		"The pairing code is required on the first run only; later runs "
		"rejoin with the persisted onboarding id.\n"
	);
}

//! [app_gestures]
static void
configure_gestures(QarSession* session, const QarAppVolumeId* volume_id)
{
	QarAppVolumeGestureConfiguration config =
		qar_app_volume_gesture_configuration_default();

	for(size_t i = 0; i < config.mapping_rule_count; ++i)
	{
		QarAppVolumeGestureMappingRule* rule = &config.mapping_rules[i];
		if(rule->gesture_kind == QAR_GESTURE_SINGLE_POINTER_6DOF)
		{
			/* Finer one-hand drag (2 cm of hand -> 1 cm of app), sliding
			 * along the floor plane and turning about the vertical axis. */
			rule->precision = 2.0f;
			rule->translation_axes =
				QAR_APP_VOLUME_AXIS_X | QAR_APP_VOLUME_AXIS_Z;
			rule->rotation_axes = QAR_APP_VOLUME_AXIS_Y;
		}
		else if(rule->gesture_kind == QAR_GESTURE_CLICK)
		{
			/* Clicks only count when the ray starts inside the box. */
			rule->enable_gestures_from_outside_of_volume = false;
		}
	}

	log_result(
		"qar_app_volumes_change_gesture_configuration",
		qar_app_volumes_change_gesture_configuration(
			session, volume_id, &config
		)
	);
}
//! [app_gestures]

//! [app_world_anchor]
/** \brief Build an east/up/south anchor frame at a WGS84 latitude/longitude
 *  (degrees) and ellipsoid height (metres). */
static QarGeoAnchorFrame
geo_anchor_from_lat_lon(double lat_deg, double lon_deg, double height_m)
{
	const double a = 6378137.0;			/* WGS84 semi-major axis */
	const double e2 = 6.69437999014e-3; /* WGS84 first eccentricity^2 */
	const double deg = 3.14159265358979323846 / 180.0;
	const double sin_lat = sin(lat_deg * deg);
	const double cos_lat = cos(lat_deg * deg);
	const double sin_lon = sin(lon_deg * deg);
	const double cos_lon = cos(lon_deg * deg);
	const double n = a / sqrt(1.0 - e2 * sin_lat * sin_lat);

	QarGeoAnchorFrame frame;
	frame.world_ref_system = QAR_WORLD_REF_SYSTEM_ECEF_WGS84_METERS;
	frame.handedness = QAR_HANDEDNESS_RIGHT;
	frame.origin_world.x = (n + height_m) * cos_lat * cos_lon;
	frame.origin_world.y = (n + height_m) * cos_lat * sin_lon;
	frame.origin_world.z = (n * (1.0 - e2) + height_m) * sin_lat;
	/* +X east */
	frame.axis_x_world.x = -sin_lon;
	frame.axis_x_world.y = cos_lon;
	frame.axis_x_world.z = 0.0;
	/* +Y up */
	frame.axis_y_world.x = cos_lat * cos_lon;
	frame.axis_y_world.y = cos_lat * sin_lon;
	frame.axis_y_world.z = sin_lat;
	/* +Z south = east x up */
	frame.axis_z_world.x = sin_lat * cos_lon;
	frame.axis_z_world.y = sin_lat * sin_lon;
	frame.axis_z_world.z = -cos_lat;
	return frame;
}

static void
pin_and_unpin_world_anchor(QarSession* session, const QarAppVolumeId* volume_id)
{
	QarGeoAnchorFrame anchor = geo_anchor_from_lat_lon(50.0870, 14.4208, 200.0);
	QarResult set_result =
		qar_app_volumes_change_app_world_anchor(session, volume_id, &anchor);
	log_result("qar_app_volumes_change_app_world_anchor", set_result);
	if(qar_result_is_error(set_result))
	{
		return;
	}

	/* ... the volume is now pinned; clear it when it should float again. */
	log_result(
		"qar_app_volumes_clear_app_world_anchor",
		qar_app_volumes_clear_app_world_anchor(session, volume_id)
	);
}
//! [app_world_anchor]

//! [app_room_to_app]
/** \brief Rotate v by the inverse of the unit quaternion q. */
static QarVector3
rotate_by_inverse(QarQuaternion q, QarVector3 v)
{
	/* v' = v + w t + u x t with u = -q.xyz and t = 2 (u x v) */
	const float ux = -q.x, uy = -q.y, uz = -q.z;
	const float tx = 2.0f * (uy * v.z - uz * v.y);
	const float ty = 2.0f * (uz * v.x - ux * v.z);
	const float tz = 2.0f * (ux * v.y - uy * v.x);
	QarVector3 r;
	r.x = v.x + q.w * tx + (uy * tz - uz * ty);
	r.y = v.y + q.w * ty + (uz * tx - ux * tz);
	r.z = v.z + q.w * tz + (ux * ty - uy * tx);
	return r;
}

/** \brief Room space -> app volume space. The volume pose is the cuboid
 *  centre, so the result is relative to that centre. */
static QarVector3
room_to_volume(const QarPose* volume_pose, QarVector3 room_point)
{
	QarVector3 d;
	d.x = room_point.x - volume_pose->position.x;
	d.y = room_point.y - volume_pose->position.y;
	d.z = room_point.z - volume_pose->position.z;
	return rotate_by_inverse(volume_pose->orientation, d);
}

/** \brief App volume space -> app content space: undo the app pose (volume
 *  metres, unscaled), then divide by app_scale (room metres per app metre). */
static QarVector3
volume_to_app_content(
	const QarPose* app_pose, float app_scale, QarVector3 volume_point
)
{
	QarVector3 app = room_to_volume(app_pose, volume_point);
	app.x /= app_scale;
	app.y /= app_scale;
	app.z /= app_scale;
	return app;
}

/** \brief True when a point in app volume space lies inside the cuboid. */
static bool
is_inside_volume(const QarAppVolumeSize* size, QarVector3 volume_point)
{
	return fabsf(volume_point.x) <= 0.5f * size->width_meters
		   && fabsf(volume_point.y) <= 0.5f * size->height_meters
		   && fabsf(volume_point.z) <= 0.5f * size->length_meters;
}

static void
hit_test_room_point(
	QarSession* session, const QarAppVolumeId* volume_id, QarVector3 room_point
)
{
	QarPose volume_pose = qar_pose_default();
	QarAppVolumeSize size = qar_app_volume_size_default();
	QarPose app_pose = qar_pose_default();
	float app_scale = 1.0f;

	if(qar_result_is_error(
		   qar_app_volume_get_latest_pose(session, volume_id, &volume_pose)
	   )
	   || qar_result_is_error(
		   qar_app_volume_get_latest_size(session, volume_id, &size)
	   )
	   || qar_result_is_error(
		   qar_app_volume_get_latest_app_pose(session, volume_id, &app_pose)
	   )
	   || qar_result_is_error(
		   qar_app_volume_get_latest_app_scale(session, volume_id, &app_scale)
	   )
	   || app_scale <= 0.0f)
	{
		printf("Volume state not available yet\n");
		return;
	}

	const QarVector3 in_volume = room_to_volume(&volume_pose, room_point);
	const QarVector3 in_app =
		volume_to_app_content(&app_pose, app_scale, in_volume);
	printf(
		"Room point (%.2f, %.2f, %.2f) -> app (%.2f, %.2f, %.2f), %s the "
		"volume\n",
		room_point.x,
		room_point.y,
		room_point.z,
		in_app.x,
		in_app.y,
		in_app.z,
		is_inside_volume(&size, in_volume) ? "inside" : "outside"
	);
}
//! [app_room_to_app]

int
main(int argc, char** argv)
{
	//! [app_args]
	if(argc < 2)
	{
		print_usage(argv[0]);
		return 1;
	}

	const char* library_path = library_path_argument(argv[1]);
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
		fprintf(stderr, "Failed to load %s.\n", describe_library(library_path));
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

	if(qar_result_is_success(create_result))
	{
		configure_gestures(session, &volume_id);
		pin_and_unpin_world_anchor(session, &volume_id);

		/* A room point 0.25 m above the volume's initial centre. */
		QarVector3 room_point = qar_vector3_default();
		room_point.x = volume_init.pose.position.x;
		room_point.y = volume_init.pose.position.y + 0.25f;
		room_point.z = volume_init.pose.position.z;
		hit_test_room_point(session, &volume_id, room_point);
	}

	/* Destroying the handle leaves the session. */
	qar_session_handle_destroy(session);
	qar_runtime_destroy(runtime);
	QarResult destroy_result = qar_library_destroy();
	log_result("qar_library_destroy", destroy_result);
	qar_library_unload();
	return 0;
}
