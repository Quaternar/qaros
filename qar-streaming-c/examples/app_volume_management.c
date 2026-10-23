/** \file app_volume_management.c
 *  \brief Gets or creates an app volume, moves and resizes it while following
 *  its updates, reads its latest values, maps a room point into it, sets its
 *  gestures and world anchor, and lists the session's volumes.
 *  \example app_volume_management.c
 */

/* The page holds tables and command lines wider than the column limit. */
// clang-format off
/** \page qar_c_tutorial_app_volumes App Volume Management
 * \tableofcontents
 *
 * \section app_volume_what What you build
 *
 * A small program that gets or creates an app volume, the named box in the
 * room that holds your rendered content. It moves and resizes the box, moves
 * and shrinks your content inside it, and prints the distinct updates its
 * subscription delivers. It then reads the volume's latest values, maps a
 * room-space point into your content, changes the gesture rules, pins the
 * volume to a place on the Earth, lists every volume in the session and
 * clears the pin again.
 *
 * What you will learn:
 * - how a volume's common name makes creation idempotent,
 * - the two transforms: the box in the room, and your content in the box,
 * - why you read the latest values instead of applying an update's values,
 * - how a point in room space becomes a point in your content,
 * - how to change the gesture rules and set a world anchor,
 * - which handles you destroy, and in which order.
 *
 * \section app_volume_before Before you start
 *
 * - **Windows**, with QAROS installed on the Hub machine. The program runs on
 *   the Hub machine. Joining as a local app starts the Hub when it is not
 *   running.
 * - **Joining.** The program joins the Hub as a local app: click **Allow** in
 *   the QAROS approval prompt on the first run of the executable. Nothing is
 *   stored, and later runs join without a prompt while the executable is
 *   unchanged. To join with a pairing code instead, see
 *   \ref app_volume_code.
 * - **The examples built**, with the repository's CMake presets (MSVC).
 * - The \ref qar_c_tutorial_gui_panels tutorial explains subscriptions and
 *   the rule for ending them in more detail. This tutorial uses the same
 *   pattern.
 *
 * \section app_volume_run Run it
 *
 * From the repository root, in a Developer PowerShell for Visual Studio:
 *
 * \code{.ps1}
 * cmake --preset x64-windows
 * cmake --build --preset x64-windows-debug --target app_volume_management
 * .\build\x64-windows\qar-streaming-c\examples\Debug\app_volume_management.exe
 * \endcode
 *
 * The program loads the runtime of the installed QAROS. To load another one,
 * such as a development build, add `--qaros-library=PATH` with the path of its
 * `qar-streaming-c.dll`. Any other argument prints the usage and exits with 1.
 *
 * Expected output after the join messages (`<...>` stands for values of your
 * own run; the library's own log lines are left out):
 *
 * \code{.txt}
 * qar_app_volumes_get_or_create: SUCCESS (code=0)
 * App volume id: <16 hex bytes>
 * qar_app_volumes_subscribe_updates: SUCCESS (code=0)
 * qar_app_volumes_change_pose: SUCCESS (code=0)
 * qar_app_volumes_change_size: SUCCESS (code=0)
 * qar_app_volumes_change_app_pose: SUCCESS (code=0)
 * qar_app_volumes_change_app_scale: SUCCESS (code=0)
 * Update: center(<x>, <y>, <z>) size(<w> x <l> x <h>) app position(<x>, <y>, <z>) app scale <s> lifetime=ACTIVE
 * Update: ...
 * qar_cancel_token_cancel: SUCCESS (code=0)
 * Received <n> updates of this volume, printed <m>: the others matched the update printed before them.
 * Latest: center(0.00, 0.00, -1.50) size(0.80 x 0.80 x 0.80) app position(0.00, -0.25, 0.00) app scale 0.50
 * Room point (0.00, 0.25, -1.50) -> app (0.00, 1.00, 0.00), inside the volume
 * qar_app_volumes_change_gesture_configuration: SUCCESS (code=0)
 * qar_app_volumes_change_app_world_anchor: SUCCESS (code=0)
 * - Volume id: <16 hex bytes> name='Tutorial Volume' center(<x>, <y>, <z>) size(<w> x <l> x <h>) anchored=yes
 * qar_app_volumes_clear_app_world_anchor: SUCCESS (code=0)
 * qar_library_destroy: SUCCESS (code=0)
 * \endcode
 *
 * Every run prints the same id: it derives from the common name, and the
 * program leaves the volume open. The `Latest:` and `Room point` lines show
 * the values the program set, unless a user or another peer changed the
 * volume meanwhile. The program renders nothing, so in the room the box stays
 * empty.
 *
 * When no update of the volume arrives within 10 s of the changes, the line
 * `No update of the volume arrived within 10 s of the changes.` replaces the
 * `Update:` lines, the steps after the `Received` line are left out, and the
 * program exits with 1.
 *
 * The Hub delivers updates on its own schedule, reports your own changes back
 * to you, repeats them, and can deliver them out of order: the last update can
 * carry an app scale of 1.00 after you set 0.50, while the `Latest:` line
 * reads 0.50. That is why the program prints only an update that differs from
 * the last one it printed, and why it takes the current values from the latest
 * getters, not from an update. When no update repeated another, the `Received`
 * line ends after `printed <m>`.
 *
 * Exit codes: 0 when every step succeeded; 1 for an unknown argument, when a
 * call failed (the line before names it and its status), or when no update of
 * the volume arrived within 10 s of the changes; 2 when no runtime loaded; 3 to 7 for a
 * refused local-app join (the program prints what to do): 3 when it could
 * not start or reach the Hub, 4 when the approval was denied, 5 when nobody
 * answered in time, 6 when the program answering is not the installed QAROS,
 * 7 when the executable no longer matches its approval.
 *
 * \section app_volume_walkthrough Walkthrough
 *
 * \subsection app_volume_session 1. Get a session
 *
 * A volume is shared session state: every peer sees the same box at the same
 * place. The program gets its session with the helpers of `common.h`, as
 * \ref qar_c_tutorial_gui_panels does: rejoin with a stored onboarding id,
 * else onboard with a pairing code when you gave one, else join the Hub on
 * this machine as a local app.
 *
 * \snippet app_volume_management.c app_startup
 *
 * The \ref qar_c_tutorial_onboarding tutorial explains the rejoin and the
 * pairing code; the \ref qar_c_tutorial_local_app_onboarding tutorial
 * explains the local-app join and its approval prompt.
 *
 * \subsection app_volume_create 2. Get or create the volume
 *
 * Like a panel, a volume is identified by a stable **common name**.
 * qar_app_volumes_get_or_create() returns the volume with that name and
 * creates it the first time, so the call is safe on every start and a
 * restarted app finds its box again. The id derives from the name, and it is
 * what a render sender needs.
 *
 * Volumes persist: a volume stays in the session after your program ends,
 * until someone closes it. So the call returns the existing volume of that
 * common name, or a new one, and does not say which. Do not try to tell them
 * apart by listing the volumes first: right after joining, the list may not
 * hold every volume yet. Write your app so that both cases work.
 *
 * \snippet app_volume_management.c app_create
 *
 * - `pose` is the pose of the box's **center** in room space (a panel's pose
 *   is its top-edge midpoint).
 * - `size` is width (X), length (Z, the horizontal depth) and height (Y), in
 *   meters. The box covers half of each extent on either side of the center.
 * - `app_pose` and `app_scale`, left at their defaults here, place your
 *   content in the box (step 4).
 *
 * The init describes a new volume only: an existing volume keeps its own
 * values. Change what must change with the calls of step 4 and step 8.
 *
 * What can go wrong: the common name must be ASCII, and the runtime rejects
 * any other. When the call fails, the program stops: every later call needs
 * the volume id.
 *
 * \subsection app_volume_subscribe 3. Subscribe to volume updates
 *
 * Users grab and scale volumes with gestures, and other peers change them
 * too. A subscription tells you when. qar_app_volumes_subscribe_updates()
 * reports changes to **every** volume in the session; there is no per-volume
 * form, so the callback skips other volumes by id.
 *
 * The callback runs on a library thread. Its QarAppVolume handle belongs to
 * the library and is valid only during the call: read it with the
 * `qar_app_volume_get_*` getters, copy what you need, and do not destroy it.
 * The callback shares the update log with the main thread, so a lock guards
 * it.
 *
 * \snippet app_volume_management.c app_update_callback
 *
 * The callback prints an update only when it differs from the last one it
 * printed. A repeat need not be equal to the bit: its floats can differ from
 * the earlier report below any precision you print or see. So the program
 * compares poses, size and scale with a tolerance, the precision it cares
 * about, and the lifetime exactly:
 *
 * \snippet app_volume_management.c app_same_snapshot
 *
 * The tolerances are in `common.h`, shared with
 * \ref qar_c_tutorial_gui_panels :
 *
 * \snippet common.h example_same_pose
 *
 * \snippet app_volume_management.c app_subscribe
 *
 * \subsection app_volume_change 4. Move and resize the box, then your content
 *
 * A volume has two transforms
 * ([Coordinate Systems](/docs/developer-guide/coordinate-systems)):
 * - **The box in the room:** qar_app_volumes_change_pose() and
 *   qar_app_volumes_change_size(). Moving the box moves everything in it.
 *   Users usually do this by grabbing the volume.
 * - **Your content in the box:** qar_app_volumes_change_app_pose() and
 *   qar_app_volumes_change_app_scale(). They pan and zoom your content
 *   without your renderer changing anything.
 *
 * \snippet app_volume_management.c app_change
 *
 * Here the box moves half a meter further away and shrinks to a 0.8 m cube.
 * `app_pose` is in volume meters and is not scaled: your content's origin
 * sits 25 cm below the box's center. `app_scale` is room meters per app
 * meter: at 0.5 your content appears at half size.
 *
 * What can go wrong: `QAR_STATUS_APP_VOLUME_INVALID_ID` means no volume has
 * that id, for example because it was closed. Call
 * qar_app_volumes_get_or_create() again if your app still needs it.
 *
 * \subsection app_volume_receive 5. Receive the updates, then end the subscription
 *
 * The program waits up to 10 s for the first update of its volume, then until
 * no update of it has arrived for half a second, and then ends the
 * subscription. The Hub can take a few seconds to deliver the first update, so
 * a quiet period alone could end before anything arrived. When no update comes
 * within 10 s, the program says so, skips the remaining steps and exits with 1.
 * The wait is `example_wait_for_updates` from `common.h`, which
 * \ref qar_c_tutorial_gui_panels shows.
 *
 * \snippet app_volume_management.c app_receive
 *
 * \snippet app_volume_management.c app_end_subscription
 *
 * A real application does not wait for a quiet period: it handles updates in
 * its own loop. When qar_cancel_token_cancel() returns, no callback is
 * running and none starts, so the update log may be read without the lock
 * and freed. Never end a subscription while holding a lock its callback
 * takes: ending it waits for the running callback.
 *
 * \subsection app_volume_latest 6. Read the latest values
 *
 * \snippet app_volume_management.c app_latest
 *
 * The `qar_app_volume_get_latest_*` getters return the latest values this
 * peer knows, including changes by other peers and by gestures. An update
 * tells you that the volume changed, but its values can be older than the
 * latest ones. So on each update, read the getters again and apply what they
 * return.
 *
 * \subsection app_volume_point 7. Map a room-space point into your content
 *
 * Gesture points already arrive in app content space. A point you hold only in
 * room space, such as a GUI panel's position or a device pose, takes two
 * steps: room to app volume space through the volume pose, whose position is
 * the box's **center**, then app volume space to app content space through
 * the app pose and app scale. The test whether the point is inside the box
 * happens in app volume space, against half the extents.
 *
 * \snippet app_volume_management.c app_room_to_app
 *
 * The program maps the point 25 cm above the box's center. With the values of
 * step 4 it lands at (0, 1, 0) in your content: 50 cm above your content's
 * origin in the volume, which at app scale 0.5 is one app meter.
 *
 * \subsection app_volume_gestures 8. Change the gesture rules
 *
 * The gesture rules decide what a user's hands do to your content. Start from
 * qar_app_volume_gesture_configuration_default(), which mirrors what a volume
 * created without a configuration gets: two-hand distance to app scale,
 * one-hand drag to app pose (rotation off), hover, click. Rules are in
 * descending priority. Change a rule in place, then replace the volume's
 * default mode with qar_app_volumes_change_gesture_configuration().
 *
 * \snippet app_volume_management.c app_gestures
 *
 * What can go wrong: `QAR_STATUS_APP_VOLUME_GESTURE_CONFIGURATION_INVALID`
 * means a rule could never work, and the volume keeps its previous rules.
 *
 * \subsection app_volume_anchor 9. Pin the volume to the Earth
 *
 * A world anchor states where app volume space sits on the Earth, as an ECEF
 * WGS84 frame. Here the volume's +X points east, +Y up and +Z south of a
 * latitude and longitude, which keeps the frame right-handed.
 *
 * \snippet app_volume_management.c app_world_anchor
 *
 * The latitude, longitude and height are an example: use the place your
 * content belongs to. The anchor stays with the volume, for every peer, until
 * someone clears it. The listing of step 10 shows whether a volume has an
 * anchor, and step 11 clears it.
 *
 * What can go wrong: `QAR_STATUS_ARGUMENT_NOT_SUPPORTED` means the frame's
 * handedness is `QAR_HANDEDNESS_UNSPECIFIED`.
 *
 * \subsection app_volume_list 10. List the volumes
 *
 * \snippet app_volume_management.c app_list
 *
 * The listing uses the count-then-fetch pattern: ask for the count, allocate
 * that many handles, fetch them. You own each handle the fetch returns and
 * destroy it when done. A fetch that returns an error may still have written
 * some handles: destroy those too.
 *
 * \snippet app_volume_management.c app_list_helpers
 *
 * The listing is a snapshot of every volume the session knows. It can be
 * stale the moment it returns: use it at startup or for display, and the
 * subscription to follow changes.
 *
 * The program leaves its volume open, so the next run reuses it.
 * qar_app_volumes_close_volume() would retire it for every peer.
 *
 * \subsection app_volume_clear_anchor 11. Clear the anchor
 *
 * \snippet app_volume_management.c app_clear_world_anchor
 *
 * The volume is shared and stays after the program ends, so the example pin
 * would stay too, for every peer. qar_app_volumes_clear_app_world_anchor()
 * lets the volume float again. Clear an anchor of your own only when your
 * content should no longer be tied to that place.
 *
 * \section app_volume_rendering Using the volume for rendering
 *
 * A render sender streams your frames into a volume: point
 * `QarRenderSenderInit::app_volume_id` at the volume's id when you create the
 * sender
 * (\ref qar_c_tutorial_cpu_rendering). The volume also defines the space the
 * sender hands you poses in, app content space, under the volume's current
 * `app_pose` and `app_scale`. While the volume does not resolve where the
 * stream is received, beginning a frame fails with
 * `QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED`. That is normal
 * control flow: keep the sender and try again. A volume you closed never
 * resolves again; create it anew with qar_app_volumes_get_or_create().
 *
 * \section app_volume_ownership Ownership and teardown in C
 *
 * | Value | Who owns it | How it ends |
 * | --- | --- | --- |
 * | `QarRuntime*`, `QarSession*` | you | qar_session_handle_destroy(), then qar_runtime_handle_destroy(), then `example_unload_library` (qar_library_destroy() and the unload) |
 * | `QarCancelToken*` | you | qar_cancel_token_cancel() ends the subscription; qar_cancel_token_handle_destroy() frees the token and also ends it |
 * | the update log (`user_state`) | you | free it only after the subscription has ended |
 * | `QarAppVolume*` in the callback | the library | valid during the call only; never destroy it |
 * | `QarAppVolume*` from qar_query_app_volumes() | you | qar_app_volume_handle_destroy(), then free the array |
 * | `QarAppVolumeId` | a plain value | names the volume; not a handle, nothing to free |
 * | the volume itself | the session, shared by every peer | qar_app_volumes_close_volume(); destroying your session does not close it |
 *
 * qar_session_handle_destroy() and qar_library_destroy() also end every
 * subscription still running. Do not destroy the session or the library from
 * a callback.
 *
 * \section app_volume_troubleshooting Troubleshooting
 *
 * | Symptom | Cause | What to do |
 * | --- | --- | --- |
 * | `Could not start or reach QAROS on this machine. Start QAROS from the Start menu, then start this app again.`, exit code 3 | The local-app join could neither start the Hub nor reach it | Start QAROS from the Start menu and run again |
 * | Exit code 4 or 5 after `joining the QAROS Hub on this machine as a local app` | The approval prompt was denied, closed or not answered within 120 s | Run again and click **Allow** |
 * | The box already has another app's content or values on the first run of your own app | A volume with that common name already exists in the session, and get-or-create returned it | Expected: volumes persist. Pick a common name scoped to your app |
 * | `ERROR (code=325)` from a call or getter: `QAR_STATUS_APP_VOLUME_INVALID_ID` | The volume was closed, possibly by another peer | qar_app_volumes_get_or_create() again |
 * | `Latest:` values differ from what you set | A user or another peer moved or scaled the volume | Expected: apply the latest values, not your own copy |
 * | The last `Update:` line differs from `Latest:` | Updates arrive repeated and out of order | Read the latest getters after each update |
 * | Your content appears at the wrong size or offset | `app_pose` or `app_scale` set by a gesture or another peer | Read qar_app_volume_get_latest_app_pose() and qar_app_volume_get_latest_app_scale() again after each update |
 * | `Volume values not available` | The latest getters failed for this volume | Check the status in the line before; get or create the volume again |
 * | `No update of the volume arrived within 10 s of the changes.`, exit code 1 | The Hub delivered no update of the volume within 10 s after the changes | Check that the Hub is still running and run again. In your app, read updates as they come instead of waiting for them a fixed time |
 * | The program hangs while ending the subscription | A callback is waiting for a lock the ending thread holds | Release the lock before qar_cancel_token_cancel() |
 * | `No QAROS runtime loaded` | The default lookup found no runtime | Install QAROS, set `QAROS_RUNTIME_DIR`, or pass `--qaros-library=PATH` |
 *
 * \section app_volume_next Next
 *
 * - \ref qar_c_tutorial_cpu_rendering : render into the volume.
 * - [App Volumes and Gestures](/docs/developer-guide/app-volumes): gesture
 *   events, the editing lock and other ways peers coordinate on a volume.
 * - [Coordinate Systems](/docs/developer-guide/coordinate-systems): every
 *   space in one place.
 * - The same program in Rust:
 *   [App Volumes](/docs/developer-guide/tutorials/rust/app-volumes).
 *
 * \section app_volume_code With a pairing code
 *
 * Instead of the local-app join, you can pass a Hub's pairing code as the first
 * argument. The program passes no Hub address, so the code finds a Hub of your
 * own user, as the \ref qar_c_tutorial_onboarding tutorial explains. An app on
 * another machine also names the Hub with a QarOnboardHostExt; that tutorial
 * shows how.
 *
 * \code{.ps1}
 * .\build\x64-windows\qar-streaming-c\examples\Debug\app_volume_management.exe <PAIRING-CODE>
 * \endcode
 *
 * It then stores the onboarding id it gets in
 * `app_volume_management.onboarding-id.txt` in the current folder and rejoins
 * with that id on every later run. The \ref qar_c_tutorial_onboarding tutorial
 * explains where the code comes from and how long it lives.
 */
// clang-format on

#include "common.h"

#include <math.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

static bool
same_volume_id(const QarAppVolumeId* first, const QarAppVolumeId* second)
{
	return memcmp(first->data, second->data, QAR_MAX_ID_LENGTH) == 0;
}

//! [app_list_helpers]
/** \brief Destroy the first count handles of a listing, then the array. */
static void
destroy_app_volume_handles(QarAppVolume** handles, size_t count)
{
	for(size_t i = 0; i < count; ++i)
	{
		if(handles[i] != NULL)
		{
			qar_app_volume_handle_destroy(handles[i]);
		}
	}
	free(handles);
}

/** \brief Fetch a handle for every volume the session knows (count, then
 *  fetch). On success the caller owns the handles: release them with
 *  destroy_app_volume_handles. */
static bool
query_app_volume_handles(QarSession* session, QarAppVolume*** out_handles, size_t* out_count)
{
	*out_handles = NULL;
	*out_count = 0;
	size_t capacity = 0;
	if(!example_quiet_success(
		   "qar_query_app_volumes_count", qar_query_app_volumes_count(session, &capacity)
	   ))
	{
		return false;
	}
	if(capacity == 0)
	{
		return true;
	}

	QarAppVolume** handles = (QarAppVolume**)calloc(capacity, sizeof(QarAppVolume*));
	if(handles == NULL)
	{
		fprintf(stderr, "Out of memory listing %zu app volumes.\n", capacity);
		return false;
	}
	/* A volume closed between the two calls makes written smaller than
	 * capacity; one added meanwhile waits for the next listing. */
	size_t written = 0;
	const bool listed = example_quiet_success(
		"qar_query_app_volumes", qar_query_app_volumes(session, handles, capacity, &written)
	);
	const size_t owned = written < capacity ? written : capacity;
	if(!listed)
	{
		/* A failed query may still have written some handles: destroy those too. */
		destroy_app_volume_handles(handles, owned);
		return false;
	}
	*out_handles = handles;
	*out_count = owned;
	return true;
}
//! [app_list_helpers]

//! [app_create]
static bool
get_or_create_volume(QarSession* session, QarAppVolumeId* out_volume_id)
{
	QarAppVolumeInit volume_init = qar_app_volume_init_default();
	/* The identity: stable, ASCII, scoped to your app. */
	volume_init.common_name = "tutorial-volume.examples.qaros";
	volume_init.display_name = "Tutorial Volume";
	volume_init.pose = qar_pose_default();
	volume_init.pose.position.z = -1.0f;
	volume_init.size = qar_app_volume_size_default();
	volume_init.size.width_meters = 1.0f;
	volume_init.size.length_meters = 1.0f;
	volume_init.size.height_meters = 1.0f;

	/* Returns the existing volume of this common name, or a new one; the
	 * result does not say which. */
	*out_volume_id = qar_app_volume_id_default();
	if(!example_logged_success(
		   "qar_app_volumes_get_or_create",
		   qar_app_volumes_get_or_create(session, &volume_init, out_volume_id)
	   ))
	{
		return false;
	}
	printf("App volume id: ");
	print_hex_id(out_volume_id->data, QAR_MAX_ID_LENGTH);
	printf("\n");
	return true;
}
//! [app_create]

static const char*
lifetime_name(QarAppVolumeLifetimeStatus lifetime)
{
	switch(lifetime)
	{
	case QAR_APP_VOLUME_ACTIVE:
		return "ACTIVE";
	case QAR_APP_VOLUME_CLOSED:
		return "CLOSED";
	}
	return "UNKNOWN";
}

/** The values of a volume update this program prints and compares. */
typedef struct VolumeSnapshot
{
	QarPose pose;
	QarAppVolumeSize size;
	QarPose app_pose;
	float app_scale;
	QarAppVolumeLifetimeStatus lifetime;
} VolumeSnapshot;

/** \brief Copy a volume's values out of a handle; returns the first failure. */
static QarResult
read_volume_snapshot(QarAppVolume* volume, VolumeSnapshot* out_snapshot)
{
	memset(out_snapshot, 0, sizeof(*out_snapshot));
	QarResult result = qar_app_volume_get_pose(volume, &out_snapshot->pose);
	if(qar_result_is_success(result))
	{
		result = qar_app_volume_get_size(volume, &out_snapshot->size);
	}
	if(qar_result_is_success(result))
	{
		result = qar_app_volume_get_app_pose(volume, &out_snapshot->app_pose);
	}
	if(qar_result_is_success(result))
	{
		result = qar_app_volume_get_app_scale(volume, &out_snapshot->app_scale);
	}
	if(qar_result_is_success(result))
	{
		result = qar_app_volume_get_lifetime_status(volume, &out_snapshot->lifetime);
	}
	return result;
}

//! [app_same_snapshot]
/** \brief Whether two updates show the same volume: poses, size and scale at
 *  the precision of example_same_pose, the lifetime exactly. */
static bool
same_volume_snapshot(const VolumeSnapshot* first, const VolumeSnapshot* second)
{
	return example_same_pose(&first->pose, &second->pose)
		   && example_nearly_equal(
			   first->size.width_meters, second->size.width_meters, EXAMPLE_SAME_LENGTH_METERS
		   )
		   && example_nearly_equal(
			   first->size.length_meters, second->size.length_meters, EXAMPLE_SAME_LENGTH_METERS
		   )
		   && example_nearly_equal(
			   first->size.height_meters, second->size.height_meters, EXAMPLE_SAME_LENGTH_METERS
		   )
		   && example_same_pose(&first->app_pose, &second->app_pose)
		   && example_nearly_equal(first->app_scale, second->app_scale, EXAMPLE_SAME_FACTOR)
		   && first->lifetime == second->lifetime;
}
//! [app_same_snapshot]

//! [app_update_callback]
/** What the update callback shares with the main thread. */
typedef struct VolumeUpdateLog
{
	/* Set before subscribing and never changed: read without the lock. */
	QarAppVolumeId watched_volume_id;
	ExampleLock lock;
	unsigned received_count;
	unsigned printed_count;
	bool has_last_printed;
	VolumeSnapshot last_printed;
} VolumeUpdateLog;

/** \brief Runs on a library thread for every update of every volume. The
 *  handle belongs to the library and is valid during this call only: copy
 *  what you need, never destroy it. */
static void
on_volume_update(QarAppVolume* volume, void* user_state)
{
	VolumeUpdateLog* update_log = (VolumeUpdateLog*)user_state;
	QarAppVolumeId volume_id = qar_app_volume_id_default();
	if(qar_result_is_error(qar_app_volume_get_id(volume, &volume_id))
	   || !same_volume_id(&volume_id, &update_log->watched_volume_id))
	{
		return;
	}
	VolumeSnapshot snapshot;
	QarResult read_result = read_volume_snapshot(volume, &snapshot);

	example_lock_acquire(&update_log->lock);
	update_log->received_count += 1;
	if(qar_result_is_error(read_result))
	{
		log_result("reading an app volume update", read_result);
	}
	else if(
		!update_log->has_last_printed || !same_volume_snapshot(&update_log->last_printed, &snapshot)
	)
	{
		/* The Hub repeats updates; print only one that differs from the last
		 * one printed. */
		update_log->last_printed = snapshot;
		update_log->has_last_printed = true;
		update_log->printed_count += 1;
		printf(
			"Update: center(%.2f, %.2f, %.2f) size(%.2f x %.2f x %.2f) app position(%.2f, "
			"%.2f, %.2f) app scale %.2f lifetime=%s\n",
			snapshot.pose.position.x,
			snapshot.pose.position.y,
			snapshot.pose.position.z,
			snapshot.size.width_meters,
			snapshot.size.length_meters,
			snapshot.size.height_meters,
			snapshot.app_pose.position.x,
			snapshot.app_pose.position.y,
			snapshot.app_pose.position.z,
			snapshot.app_scale,
			lifetime_name(snapshot.lifetime)
		);
	}
	example_lock_release(&update_log->lock);
}
//! [app_update_callback]

//! [app_change]
static bool
move_and_resize_volume(QarSession* session, const QarAppVolumeId* volume_id)
{
	/* The box in the room: half a meter further away, a 0.8 m cube. */
	QarPose volume_pose = qar_pose_default();
	volume_pose.position.z = -1.5f;
	QarAppVolumeSize volume_size = qar_app_volume_size_default();
	volume_size.width_meters = 0.8f;
	volume_size.length_meters = 0.8f;
	volume_size.height_meters = 0.8f;

	/* Your content in the box: origin 25 cm below the center, half size. */
	QarPose app_pose = qar_pose_default();
	app_pose.position.y = -0.25f;
	const float app_scale = 0.5f;

	return example_logged_success(
			   "qar_app_volumes_change_pose",
			   qar_app_volumes_change_pose(session, volume_id, &volume_pose)
		   )
		   && example_logged_success(
			   "qar_app_volumes_change_size",
			   qar_app_volumes_change_size(session, volume_id, &volume_size)
		   )
		   && example_logged_success(
			   "qar_app_volumes_change_app_pose",
			   qar_app_volumes_change_app_pose(session, volume_id, &app_pose)
		   )
		   && example_logged_success(
			   "qar_app_volumes_change_app_scale",
			   qar_app_volumes_change_app_scale(session, volume_id, app_scale)
		   );
}
//! [app_change]

/** \brief Subscribe to volume updates, move and resize the volume, print the
 *  updates, and end the subscription. Returns false when a call failed or no
 *  update arrived. */
static bool
change_volume_and_follow_updates(QarSession* session, const QarAppVolumeId* volume_id)
{
	//! [app_subscribe]
	VolumeUpdateLog update_log;
	memset(&update_log, 0, sizeof(update_log));
	update_log.watched_volume_id = *volume_id;
	example_lock_init(&update_log.lock);

	QarCancelToken* subscription_token = NULL;
	if(!example_quiet_success(
		   "qar_cancel_token_create", qar_cancel_token_create(&subscription_token)
	   ))
	{
		example_lock_destroy(&update_log.lock);
		return false;
	}
	if(!example_logged_success(
		   "qar_app_volumes_subscribe_updates",
		   qar_app_volumes_subscribe_updates(
			   session, on_volume_update, &update_log, subscription_token
		   )
	   ))
	{
		qar_cancel_token_handle_destroy(subscription_token);
		example_lock_destroy(&update_log.lock);
		return false;
	}
	//! [app_subscribe]

	//! [app_receive]
	const bool changed = move_and_resize_volume(session, volume_id);
	bool updated = false;
	if(changed)
	{
		updated = example_wait_for_updates(&update_log.lock, &update_log.received_count);
		if(!updated)
		{
			printf(
				"No update of the volume arrived within %u s of the changes.\n",
				EXAMPLE_FIRST_UPDATE_TIMEOUT_MS / 1000u
			);
		}
	}
	//! [app_receive]

	//! [app_end_subscription]
	/* Once this returns, no callback runs and none starts: the log is ours. */
	const bool cancelled = example_logged_success(
		"qar_cancel_token_cancel", qar_cancel_token_cancel(subscription_token)
	);
	qar_cancel_token_handle_destroy(subscription_token);
	printf(
		"Received %u updates of this volume, printed %u",
		update_log.received_count,
		update_log.printed_count
	);
	if(update_log.printed_count < update_log.received_count)
	{
		printf(": the others matched the update printed before them");
	}
	printf(".\n");
	example_lock_destroy(&update_log.lock);
	//! [app_end_subscription]
	return changed && updated && cancelled;
}

//! [app_latest]
/** The values a peer currently knows for a volume. */
typedef struct LatestVolumeValues
{
	QarPose pose;
	QarAppVolumeSize size;
	QarPose app_pose;
	float app_scale;
} LatestVolumeValues;

static bool
read_latest_volume_values(
	QarSession* session, const QarAppVolumeId* volume_id, LatestVolumeValues* out_values
)
{
	out_values->pose = qar_pose_default();
	out_values->size = qar_app_volume_size_default();
	out_values->app_pose = qar_pose_default();
	out_values->app_scale = 1.0f;
	return example_quiet_success(
			   "qar_app_volume_get_latest_pose",
			   qar_app_volume_get_latest_pose(session, volume_id, &out_values->pose)
		   )
		   && example_quiet_success(
			   "qar_app_volume_get_latest_size",
			   qar_app_volume_get_latest_size(session, volume_id, &out_values->size)
		   )
		   && example_quiet_success(
			   "qar_app_volume_get_latest_app_pose",
			   qar_app_volume_get_latest_app_pose(session, volume_id, &out_values->app_pose)
		   )
		   && example_quiet_success(
			   "qar_app_volume_get_latest_app_scale",
			   qar_app_volume_get_latest_app_scale(session, volume_id, &out_values->app_scale)
		   );
}

static bool
print_latest_volume_values(QarSession* session, const QarAppVolumeId* volume_id)
{
	LatestVolumeValues latest;
	if(!read_latest_volume_values(session, volume_id, &latest))
	{
		return false;
	}
	printf(
		"Latest: center(%.2f, %.2f, %.2f) size(%.2f x %.2f x %.2f) app position(%.2f, %.2f, "
		"%.2f) app scale %.2f\n",
		latest.pose.position.x,
		latest.pose.position.y,
		latest.pose.position.z,
		latest.size.width_meters,
		latest.size.length_meters,
		latest.size.height_meters,
		latest.app_pose.position.x,
		latest.app_pose.position.y,
		latest.app_pose.position.z,
		latest.app_scale
	);
	return true;
}
//! [app_latest]

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
 *  center, so the result is relative to that center. */
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
 *  meters, unscaled), then divide by app_scale (room meters per app meter). */
static QarVector3
volume_to_app_content(const QarPose* app_pose, float app_scale, QarVector3 volume_point)
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

/** \brief Map the room point height_above_center meters above the volume's
 *  latest center into app content space, and test it against the box. */
static bool
map_point_above_volume_center(
	QarSession* session, const QarAppVolumeId* volume_id, float height_above_center
)
{
	LatestVolumeValues latest;
	if(!read_latest_volume_values(session, volume_id, &latest) || latest.app_scale <= 0.0f)
	{
		printf("Volume values not available\n");
		return false;
	}

	QarVector3 room_point = latest.pose.position;
	room_point.y += height_above_center;
	const QarVector3 in_volume = room_to_volume(&latest.pose, room_point);
	const QarVector3 in_app = volume_to_app_content(&latest.app_pose, latest.app_scale, in_volume);
	printf(
		"Room point (%.2f, %.2f, %.2f) -> app (%.2f, %.2f, %.2f), %s the volume\n",
		room_point.x,
		room_point.y,
		room_point.z,
		in_app.x,
		in_app.y,
		in_app.z,
		is_inside_volume(&latest.size, in_volume) ? "inside" : "outside"
	);
	return true;
}
//! [app_room_to_app]

//! [app_gestures]
static bool
configure_gestures(QarSession* session, const QarAppVolumeId* volume_id)
{
	QarAppVolumeGestureConfiguration config = qar_app_volume_gesture_configuration_default();

	for(size_t i = 0; i < config.mapping_rule_count; ++i)
	{
		QarAppVolumeGestureMappingRule* rule = &config.mapping_rules[i];
		if(rule->gesture_kind == QAR_GESTURE_SINGLE_POINTER_6DOF)
		{
			/* Finer one-hand drag (2 cm of hand -> 1 cm of app), sliding
			 * along the floor plane and turning about the vertical axis. */
			rule->precision = 2.0f;
			rule->translation_axes = QAR_APP_VOLUME_AXIS_X | QAR_APP_VOLUME_AXIS_Z;
			rule->rotation_axes = QAR_APP_VOLUME_AXIS_Y;
		}
		else if(rule->gesture_kind == QAR_GESTURE_CLICK)
		{
			/* Clicks only count when the ray starts inside the box. */
			rule->enable_gestures_from_outside_of_volume = false;
		}
	}

	return example_logged_success(
		"qar_app_volumes_change_gesture_configuration",
		qar_app_volumes_change_gesture_configuration(session, volume_id, &config)
	);
}
//! [app_gestures]

//! [app_world_anchor]
/** \brief Build an east/up/south anchor frame at a WGS84 latitude/longitude
 *  (degrees) and ellipsoid height (meters). */
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

static bool
pin_to_earth(QarSession* session, const QarAppVolumeId* volume_id)
{
	const QarGeoAnchorFrame anchor = geo_anchor_from_lat_lon(50.0870, 14.4208, 200.0);
	return example_logged_success(
		"qar_app_volumes_change_app_world_anchor",
		qar_app_volumes_change_app_world_anchor(session, volume_id, &anchor)
	);
}
//! [app_world_anchor]

//! [app_list]
static void
print_listed_volume(QarAppVolume* volume)
{
	QarAppVolumeId id = qar_app_volume_id_default();
	char name[QAR_MAX_STRING_LENGTH] = { 0 };
	QarPose pose = qar_pose_default();
	QarAppVolumeSize size = qar_app_volume_size_default();
	QarAppWorldAnchor world_anchor;
	memset(&world_anchor, 0, sizeof(world_anchor));

	QarResult read_result = qar_app_volume_get_id(volume, &id);
	if(qar_result_is_success(read_result))
	{
		read_result = qar_app_volume_get_display_name(volume, name, sizeof(name));
	}
	if(qar_result_is_success(read_result))
	{
		read_result = qar_app_volume_get_pose(volume, &pose);
	}
	if(qar_result_is_success(read_result))
	{
		read_result = qar_app_volume_get_size(volume, &size);
	}
	if(qar_result_is_success(read_result))
	{
		read_result = qar_app_volume_get_app_world_anchor(volume, &world_anchor);
	}
	if(qar_result_is_error(read_result))
	{
		log_result("reading a listed app volume", read_result);
		return;
	}

	printf("- Volume id: ");
	print_hex_id(id.data, QAR_MAX_ID_LENGTH);
	printf(
		" name='%s' center(%.2f, %.2f, %.2f) size(%.2f x %.2f x %.2f) anchored=%s\n",
		name,
		pose.position.x,
		pose.position.y,
		pose.position.z,
		size.width_meters,
		size.length_meters,
		size.height_meters,
		world_anchor.has_anchor ? "yes" : "no"
	);
}

static bool
list_volumes(QarSession* session)
{
	QarAppVolume** volumes = NULL;
	size_t volume_count = 0;
	if(!query_app_volume_handles(session, &volumes, &volume_count))
	{
		return false;
	}
	for(size_t i = 0; i < volume_count; ++i)
	{
		print_listed_volume(volumes[i]);
	}
	destroy_app_volume_handles(volumes, volume_count);
	return true;
}
//! [app_list]

//! [app_clear_world_anchor]
/* The volume is shared and persistent: without this, the example pin would
 * stay on it for every peer after the program ends. */
static bool
clear_earth_pin(QarSession* session, const QarAppVolumeId* volume_id)
{
	return example_logged_success(
		"qar_app_volumes_clear_app_world_anchor",
		qar_app_volumes_clear_app_world_anchor(session, volume_id)
	);
}
//! [app_clear_world_anchor]

static int
run_volume_steps(QarSession* session)
{
	QarAppVolumeId volume_id;
	if(!get_or_create_volume(session, &volume_id))
	{
		return 1;
	}

	/* Every step runs only when the one before succeeded, except the clear: a
	 * pin that was set is cleared even when the listing failed. */
	const bool pinned = change_volume_and_follow_updates(session, &volume_id)
						&& print_latest_volume_values(session, &volume_id)
						&& map_point_above_volume_center(session, &volume_id, 0.25f)
						&& configure_gestures(session, &volume_id)
						&& pin_to_earth(session, &volume_id);
	const bool listed = pinned && list_volumes(session);
	const bool cleared = pinned && clear_earth_pin(session, &volume_id);
	return listed && cleared ? 0 : 1;
}

int
main(int argc, char** argv)
{
	example_print_as_it_happens();
	//! [app_startup]
	static const char* const known_flags[] = { "--qaros-library=" };
	if(!example_arguments_are_known(
		   argc,
		   argv,
		   known_flags,
		   sizeof(known_flags) / sizeof(known_flags[0]),
		   "[pairing-code] [--qaros-library=PATH]"
	   ))
	{
		return 1;
	}
	const char* library_path = example_library_path(argc, argv);
	const char* pairing_code = example_pairing_code_argument(argc, argv);

	int exit_code = example_load_library(library_path);
	if(exit_code != 0)
	{
		return exit_code;
	}
	QarRuntime* runtime = NULL;
	if(!example_create_runtime(NULL, &runtime))
	{
		example_unload_library();
		return 1;
	}
	QarSession* session = NULL;
	exit_code = example_join_session(
		runtime,
		pairing_code,
		"app_volume_management.onboarding-id.txt",
		"App Volume Peer",
		&session
	);
	if(exit_code == 0)
	{
		exit_code = run_volume_steps(session);
	}

	/* Children before parents: session, runtime, then the library. Destroying
	 * the session leaves it; the volume stays in the Hub's session. */
	if(session != NULL)
	{
		qar_session_handle_destroy(session);
	}
	qar_runtime_handle_destroy(runtime);
	example_unload_library();
	return exit_code;
	//! [app_startup]
}
