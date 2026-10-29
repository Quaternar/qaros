/** \file gui_panel_operations.c
 *  \brief Creates a GUI panel, follows its updates while moving, resizing,
 *  minimizing and navigating it, lists the session's panels and closes it.
 *  \example gui_panel_operations.c
 */

/* The page holds tables and command lines wider than the column limit. */
// clang-format off
/** \page qar_c_tutorial_gui_panels GUI Panel Operations
 * \tableofcontents
 *
 * \section gui_panels_what What you build
 *
 * A small program that puts a web panel into the shared QAROS room. It
 * subscribes to the panel's updates, then moves, resizes, minimizes and
 * navigates the panel, and prints each distinct update the subscription
 * delivers. It then lists every panel in the session and closes its own.
 *
 * What you will learn:
 * - how a panel's common name makes creation idempotent,
 * - how to subscribe to one panel's updates, and why the callback needs a lock,
 * - why an update tells you that the panel changed, not what you should apply,
 * - when it is safe to end a subscription and free the state its callback uses,
 * - which handles you destroy, and in which order.
 *
 * \section gui_panels_before Before you start
 *
 * - **Windows**, with QAROS installed on the Hub machine. The program runs on
 *   the Hub machine. Joining as a local app starts the Hub when it is not
 *   running.
 * - **Joining.** The program joins the Hub as a local app: click **Allow** in
 *   the QAROS approval prompt on the first run of the executable. Nothing is
 *   stored, and later runs join without a prompt while the executable is
 *   unchanged. To join with a pairing code instead, see
 *   \ref gui_panels_code.
 * - **The examples built**, with the repository's CMake presets (MSVC).
 *
 * \section gui_panels_run Run it
 *
 * From the repository root, in a Developer PowerShell for Visual Studio:
 *
 * \code{.ps1}
 * cmake --preset x64-windows
 * cmake --build --preset x64-windows-debug --target gui_panel_operations
 * .\build\x64-windows\qar-streaming-c\examples\Debug\gui_panel_operations.exe
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
 * qar_gui_panels_get_or_create: SUCCESS (code=0)
 * GUI panel id: <16 hex bytes>
 * qar_gui_panels_subscribe_panel_updates: SUCCESS (code=0)
 * qar_gui_panels_update_pose: SUCCESS (code=0)
 * qar_gui_panels_change_size: SUCCESS (code=0)
 * qar_gui_panels_set_state: SUCCESS (code=0)
 * qar_gui_panels_navigate_to_uri: SUCCESS (code=0)
 * Update: position(<x>, <y>, <z>) size(<w> x <h>) state=<STATE> uri='<uri>'
 * Update: ...
 * qar_cancel_token_cancel: SUCCESS (code=0)
 * Received <n> updates, printed <m>: the others matched the update printed before them.
 * - Panel id: <16 hex bytes> name='<name>' position(<x>, <y>, <z>) size(<w> x <h>) state=<STATE> uri='<uri>'
 * qar_gui_panels_close_panel: SUCCESS (code=0)
 * qar_library_destroy: SUCCESS (code=0)
 * \endcode
 *
 * In the room you see the panel appear, shrink and minimize, then disappear
 * when the program closes it.
 *
 * When no update arrives within 10 s of the changes, the line
 * `No update of the panel arrived within 10 s of the changes.` replaces the
 * `Update:` lines, the listing is left out, and the program exits with 1.
 *
 * The Hub delivers updates on its own schedule and reports your own changes
 * back to you, often several times. A few changes can arrive as dozens of
 * updates, most of them the same to the millimeter, and an update can carry an
 * older state than the one you set. That is why the program prints only an
 * update that differs from the last one it printed, and why the `Update:`
 * lines need not match the calls one to one. When no update repeated another,
 * the `Received` line ends after `printed <m>`.
 *
 * Exit codes: 0 when every step succeeded; 1 for an unknown argument, when a
 * call failed (the line before names it and its status), or when no update of
 * the panel arrived within 10 s of the changes; 2 when no runtime loaded; 3 to 7 for a
 * refused local-app join (the program prints what to do): 3 when it could
 * not start or reach the Hub, 4 when the approval was denied, 5 when nobody
 * answered in time, 6 when the program answering is not the installed QAROS,
 * 7 when the executable no longer matches its approval.
 *
 * \section gui_panels_walkthrough Walkthrough
 *
 * \subsection gui_panels_session 1. Get a session
 *
 * A panel is shared session state: every peer sees the same panel at the same
 * place. So everything starts with a session. The program loads the runtime,
 * creates a runtime handle and joins with the helpers of `common.h`:
 * `example_join_session` rejoins with a stored onboarding id, else onboards
 * with a pairing code when you gave one, else joins the Hub on this machine
 * as a local app.
 *
 * \snippet gui_panel_operations.c gui_startup
 *
 * The \ref qar_c_tutorial_onboarding tutorial explains the rejoin and the
 * pairing code; the \ref qar_c_tutorial_local_app_onboarding tutorial
 * explains the local-app join and its approval prompt.
 *
 * \subsection gui_panels_create 2. Get or create the panel
 *
 * A panel is identified by its **common name**, not by a handle.
 * qar_gui_panels_get_or_create() returns the panel with that name and creates
 * it the first time, so the same call is safe on every start: a restarted app
 * reattaches to its panel instead of adding a second one. The id derives from
 * the name. Use a DNS-style, app-scoped name and keep it stable across
 * versions.
 *
 * \snippet gui_panel_operations.c gui_create
 *
 * The call does not say whether it created the panel or found it. This
 * program closes its panel at the end, so a normal run creates it; after a
 * run that ended early, it finds the one left behind.
 *
 * The pose is the **midpoint of the panel's top edge** in room space, not its
 * center; the surface hangs below it
 * ([Coordinate Systems](/docs/developer-guide/coordinate-systems)).
 * QarGuiPanelSize::content_scale zooms the web content; the default keeps it
 * at 1. With no content extension chained to the init, the panel starts with
 * the default website content until you navigate it.
 *
 * What can go wrong: the common name must be ASCII, and the runtime rejects
 * any other. Put other text in `display_name`. When the call fails, the
 * program stops: every later call needs the panel id.
 *
 * \subsection gui_panels_subscribe 3. Subscribe before you change anything
 *
 * Other peers, other apps and users grabbing the panel all change it. A
 * subscription tells you when. Subscribe before your own changes so you do not
 * miss any.
 *
 * The callback runs on a **library thread**, while your main thread does
 * something else. It receives a QarGuiPanel handle that the library owns and
 * that is valid only during the call: read what you need with the
 * `qar_gui_panel_get_*` getters, copy it, and do not destroy the handle. Keep
 * the callback short and do not call blocking QAROS functions from it.
 *
 * \snippet gui_panel_operations.c gui_update_callback
 *
 * The callback and the main thread share the update log, so a lock guards it.
 * The callback skips an update that matches the last one it printed: the Hub
 * repeats updates, and a repeat tells you nothing new.
 *
 * A repeat need not be equal to the bit. Its pose and size can differ from the
 * earlier report below any precision you print or see, so comparing floats
 * with `==` takes most repeats for changes. Compare at the precision your app
 * cares about. This program allows a millimeter for pose and size, and
 * compares the state and the address exactly:
 *
 * \snippet gui_panel_operations.c gui_same_snapshot
 *
 * The tolerances are in `common.h`, shared with
 * \ref qar_c_tutorial_app_volumes :
 *
 * \snippet common.h example_same_pose
 *
 * Every `qar_*_subscribe_*` call hands its subscription to a cancel token. The
 * token is how you end it (step 5).
 *
 * \snippet gui_panel_operations.c gui_subscribe
 *
 * qar_gui_panels_subscribe_updates() works the same way for every panel in the
 * session; qar_gui_panels_subscribe_panel_updates() filters to one.
 *
 * \subsection gui_panels_change 4. Change the panel
 *
 * Every change goes through the session and the panel id. Each call is
 * checked, and the first failure stops the chain.
 *
 * \snippet gui_panel_operations.c gui_change
 *
 * - qar_gui_panels_update_pose() moves the top-edge midpoint.
 * - qar_gui_panels_change_size() changes the physical size and the content
 *   scale together.
 * - qar_gui_panels_set_state() switches between `QAR_GUI_PANEL_STATE_VISIBLE`,
 *   `_MINIMIZED` and `_HIDDEN`. To remove a panel, close it (step 7).
 * - qar_gui_panels_navigate_to_uri() points the panel at web content.
 *
 * What can go wrong: `QAR_STATUS_GUI_PANEL_INVALID_ID` means no panel has that
 * id, for example because another peer closed it. Call
 * qar_gui_panels_get_or_create() again if your app still needs it.
 * `QAR_STATUS_GUI_PANEL_CHANGE_NOT_PERMITTED` from the navigate call means the
 * panel shows a stream, not a website.
 *
 * \subsection gui_panels_receive 5. Receive the updates, then end the subscription
 *
 * The program waits up to 10 s for the first update, then until no update has
 * arrived for half a second, and then ends the subscription. The Hub can take a
 * few seconds to deliver the first update, so a quiet period alone could end
 * before anything arrived. When no update comes within 10 s, the program says
 * so, skips the listing, closes the panel and exits with 1: a program that
 * follows updates and sees none should not report success.
 *
 * \snippet gui_panel_operations.c gui_receive
 *
 * The wait lives in `common.h`, shared with \ref qar_c_tutorial_app_volumes.
 * The callback counts updates under the lock, so the wait reads the count
 * under the same lock:
 *
 * \snippet common.h example_wait_for_updates
 *
 * \snippet gui_panel_operations.c gui_end_subscription
 *
 * A real application does not wait for a quiet period. It reads what the
 * callback stored from its own loop, such as a UI frame, and never blocks a
 * callback waiting for that loop.
 *
 * Treat an update as news that the panel changed, not as an instruction.
 * Expect your own changes to come back, repeated and sometimes older than what
 * you set: skip an update that matches the last one you handled, and do not
 * take an update for a user's action, since it may be the echo of your own
 * call.
 *
 * When qar_cancel_token_cancel() returns, a callback that was running on
 * another thread has finished, and no callback starts afterwards. That is
 * what makes it safe to read the counts without the lock, and to free the
 * update log once the function returns. The guarantee comes with one rule:
 * **never end a subscription while holding a lock its callback takes.**
 * Ending it waits for the running callback; if that callback waits for your
 * lock, neither returns. Ending a subscription from inside its own callback
 * is allowed, and does not wait for that callback. The full rules are under
 * \ref qar_c_subscription_end "Ending a subscription" in the API reference.
 *
 * \subsection gui_panels_list 6. List the panels
 *
 * The listing is a snapshot of every panel in the session, read with the
 * count-then-fetch pattern: ask for the count, allocate that many handles,
 * fetch them. You own each handle the fetch returns: read it, then destroy
 * it. A fetch that returns an error may still have written some handles:
 * destroy those too.
 *
 * \snippet gui_panel_operations.c gui_list
 *
 * The listing can be stale the moment it returns. Use it at startup or for
 * display, and the subscription to follow changes. A panel's display name can
 * come back empty from the Hub, so tell panels apart by id, which derives
 * from the common name, not by display name.
 *
 * \subsection gui_panels_close 7. Close the panel
 *
 * \snippet gui_panel_operations.c gui_close
 *
 * Closing removes the panel for every peer, since panels are shared. A later
 * qar_gui_panels_get_or_create() with the same name creates it again.
 *
 * \section gui_panels_ownership Ownership and teardown in C
 *
 * | Value | Who owns it | How it ends |
 * | --- | --- | --- |
 * | `QarRuntime*`, `QarSession*` | you | qar_session_handle_destroy(), then qar_runtime_handle_destroy(), then `example_unload_library` (qar_library_destroy() and the unload) |
 * | `QarCancelToken*` | you | qar_cancel_token_cancel() ends the subscription; qar_cancel_token_handle_destroy() frees the token and also ends it |
 * | the update log (`user_state`) | you | free it only after the subscription has ended; here it lives on the stack of the function that ends it |
 * | `QarGuiPanel*` in the callback | the library | valid during the call only; never destroy it |
 * | `QarGuiPanel*` from qar_query_gui_panels() | you | qar_gui_panel_handle_destroy() after reading it, then free the array |
 * | `QarGuiPanelId` | a plain value | names the panel; not a handle, nothing to free |
 * | the panel itself | the session, shared by every peer | qar_gui_panels_close_panel(); destroying your session does not close it |
 *
 * qar_session_handle_destroy() and qar_library_destroy() also end every
 * subscription still running, so a forgotten token does not outlive the
 * session. Do not destroy the session or the library from a callback.
 *
 * \section gui_panels_troubleshooting Troubleshooting
 *
 * | Symptom | Cause | What to do |
 * | --- | --- | --- |
 * | `Could not start or reach QAROS on this machine. Start QAROS from the Start menu, then start this app again.`, exit code 3 | The local-app join could neither start the Hub nor reach it | Start QAROS from the Start menu and run again |
 * | Exit code 4 or 5 after `joining the QAROS Hub on this machine as a local app` | The approval prompt was denied, closed or not answered within 120 s | Run again and click **Allow** |
 * | `qar_gui_panels_get_or_create: ERROR` | The common name is not ASCII, or the session is gone | Keep the name ASCII; put other text in `display_name` |
 * | `ERROR (code=305)` from a setter: `QAR_STATUS_GUI_PANEL_INVALID_ID` | The panel was closed, possibly by another peer | qar_gui_panels_get_or_create() again |
 * | `No update of the panel arrived within 10 s of the changes.`, exit code 1 | The Hub delivered no update of the panel within 10 s after the changes | Check that the Hub is still running and run again. In your app, read updates as they come instead of waiting for them a fixed time |
 * | Many more updates received than printed | The Hub reports your own changes back, repeated | Expected; skip repeats as the callback does |
 * | `Update:` or listing values differ from what you set | Updates arrive asynchronously and can be older than your last call | Do not apply an update's values blindly; treat it as a sign that the panel changed |
 * | ``name=''`` in the listing | The Hub returned an empty display name | Identify panels by id, not by display name |
 * | The program hangs while ending the subscription | A callback is waiting for a lock the ending thread holds | Release the lock before qar_cancel_token_cancel() |
 * | `No QAROS runtime loaded` | The default lookup found no runtime | Install QAROS, set `QAROS_RUNTIME_DIR`, or pass `--qaros-library=PATH` |
 *
 * \section gui_panels_next Next
 *
 * - \ref qar_c_tutorial_app_volumes : the 3D box your app renders into, with
 *   the same get-or-create and update patterns.
 * - [Custom GUI Panels](/docs/developer-guide/gui-panels): visibility per peer
 *   and integration patterns.
 * - [API Conventions](/docs/developer-guide/api-conventions): handles,
 *   results, callbacks and count-then-fetch in every language.
 * - The same program in Rust:
 *   [GUI Panels](/docs/developer-guide/tutorials/rust/gui-panels).
 *
 * \section gui_panels_code With a pairing code
 *
 * Instead of the local-app join, you can pass a Hub's pairing code as the first
 * argument. The program passes no Hub address, so the code finds a Hub of your
 * own user, as the \ref qar_c_tutorial_onboarding tutorial explains. An app on
 * another machine also names the Hub with a QarOnboardHostExt; that tutorial
 * shows how.
 *
 * \code{.ps1}
 * .\build\x64-windows\qar-streaming-c\examples\Debug\gui_panel_operations.exe <PAIRING-CODE>
 * \endcode
 *
 * It then stores the onboarding id it gets in
 * `gui_panel_operations.onboarding-id.txt` in the current folder and rejoins
 * with that id on every later run. The \ref qar_c_tutorial_onboarding tutorial
 * explains where the code comes from and how long it lives.
 */
// clang-format on

#include "common.h"

QAR_IMPLEMENT_DYNAMIC_LOADING()

static const char*
panel_state_name(QarGuiPanelState state)
{
	switch(state)
	{
	case QAR_GUI_PANEL_STATE_VISIBLE:
		return "VISIBLE";
	case QAR_GUI_PANEL_STATE_MINIMIZED:
		return "MINIMIZED";
	case QAR_GUI_PANEL_STATE_HIDDEN:
		return "HIDDEN";
	case QAR_GUI_PANEL_STATE_CLOSED:
		return "CLOSED";
	}
	return "UNKNOWN";
}

/** The values of a panel this program prints and compares. */
typedef struct PanelSnapshot
{
	QarPose pose;
	QarGuiPanelSize size;
	QarGuiPanelState state;
	char content_uri[QAR_MAX_STRING_LENGTH];
} PanelSnapshot;

/** \brief Copy a panel's values out of a handle; returns the first failure. */
static QarResult
read_panel_snapshot(QarGuiPanel* panel, PanelSnapshot* out_snapshot)
{
	memset(out_snapshot, 0, sizeof(*out_snapshot));
	QarResult result = qar_gui_panel_get_pose(panel, &out_snapshot->pose);
	if(qar_result_is_success(result))
	{
		result = qar_gui_panel_get_size(panel, &out_snapshot->size);
	}
	if(qar_result_is_success(result))
	{
		result = qar_gui_panel_get_state(panel, &out_snapshot->state);
	}
	if(qar_result_is_success(result))
	{
		result = qar_gui_panel_get_content_uri(
			panel, out_snapshot->content_uri, sizeof(out_snapshot->content_uri)
		);
	}
	return result;
}

//! [gui_same_snapshot]
/** \brief Whether two updates show the same panel: pose and size at the
 *  precision of example_same_pose, state and address exactly. */
static bool
same_panel_snapshot(const PanelSnapshot* first, const PanelSnapshot* second)
{
	return example_same_pose(&first->pose, &second->pose)
		   && example_nearly_equal(
			   first->size.width_meters, second->size.width_meters, EXAMPLE_SAME_LENGTH_METERS
		   )
		   && example_nearly_equal(
			   first->size.height_meters, second->size.height_meters, EXAMPLE_SAME_LENGTH_METERS
		   )
		   && example_nearly_equal(
			   first->size.content_scale, second->size.content_scale, EXAMPLE_SAME_FACTOR
		   )
		   && first->state == second->state && strcmp(first->content_uri, second->content_uri) == 0;
}
//! [gui_same_snapshot]

static void
print_panel_snapshot(const PanelSnapshot* snapshot)
{
	printf(
		"position(%.2f, %.2f, %.2f) size(%.2f x %.2f) state=%s uri='%s'",
		snapshot->pose.position.x,
		snapshot->pose.position.y,
		snapshot->pose.position.z,
		snapshot->size.width_meters,
		snapshot->size.height_meters,
		panel_state_name(snapshot->state),
		snapshot->content_uri
	);
}

//! [gui_update_callback]
/** What the update callback shares with the main thread. */
typedef struct PanelUpdateLog
{
	ExampleLock lock;
	unsigned received_count;
	unsigned printed_count;
	bool has_last_printed;
	PanelSnapshot last_printed;
} PanelUpdateLog;

/** \brief Runs on a library thread for every update of the subscribed panel.
 *  The handle belongs to the library and is valid during this call only:
 *  copy what you need, never destroy it. */
static void
on_panel_update(QarGuiPanel* panel, void* user_state)
{
	PanelUpdateLog* update_log = (PanelUpdateLog*)user_state;
	PanelSnapshot snapshot;
	QarResult read_result = read_panel_snapshot(panel, &snapshot);

	example_lock_acquire(&update_log->lock);
	update_log->received_count += 1;
	if(qar_result_is_error(read_result))
	{
		log_result("reading a GUI panel update", read_result);
	}
	else if(
		!update_log->has_last_printed || !same_panel_snapshot(&update_log->last_printed, &snapshot)
	)
	{
		/* The Hub repeats updates; print only one that differs from the last
		 * one printed. */
		update_log->last_printed = snapshot;
		update_log->has_last_printed = true;
		update_log->printed_count += 1;
		printf("Update: ");
		print_panel_snapshot(&snapshot);
		printf("\n");
	}
	example_lock_release(&update_log->lock);
}
//! [gui_update_callback]

//! [gui_create]
static bool
get_or_create_panel(QarSession* session, QarGuiPanelId* out_panel_id)
{
	QarGuiPanelInit panel_init = qar_gui_panel_init_default();
	/* The identity: stable, ASCII, scoped to your app. */
	panel_init.common_name = "tutorial-panel.examples.qaros";
	panel_init.display_name = "Tutorial Panel";
	panel_init.pose = qar_pose_default();
	panel_init.pose.position.x = 0.5f;
	panel_init.pose.position.y = 1.5f;
	panel_init.pose.position.z = -1.2f;
	panel_init.size = qar_gui_panel_size_default();
	panel_init.size.width_meters = 1.2f;
	panel_init.size.height_meters = 0.7f;

	*out_panel_id = qar_gui_panel_id_default();
	if(!example_logged_success(
		   "qar_gui_panels_get_or_create",
		   qar_gui_panels_get_or_create(session, &panel_init, out_panel_id)
	   ))
	{
		return false;
	}
	printf("GUI panel id: ");
	print_hex_id(out_panel_id->data, QAR_MAX_ID_LENGTH);
	printf("\n");
	return true;
}
//! [gui_create]

//! [gui_change]
static bool
change_panel(QarSession* session, const QarGuiPanelId* panel_id)
{
	QarPose new_pose = qar_pose_default();
	new_pose.position.x = 0.2f;
	new_pose.position.y = 1.6f;
	new_pose.position.z = -1.0f;

	QarGuiPanelSize new_size = qar_gui_panel_size_default();
	new_size.width_meters = 1.0f;
	new_size.height_meters = 0.6f;

	return example_logged_success(
			   "qar_gui_panels_update_pose",
			   qar_gui_panels_update_pose(session, panel_id, &new_pose)
		   )
		   && example_logged_success(
			   "qar_gui_panels_change_size",
			   qar_gui_panels_change_size(session, panel_id, &new_size)
		   )
		   && example_logged_success(
			   "qar_gui_panels_set_state",
			   qar_gui_panels_set_state(session, panel_id, QAR_GUI_PANEL_STATE_MINIMIZED)
		   )
		   && example_logged_success(
			   "qar_gui_panels_navigate_to_uri",
			   qar_gui_panels_navigate_to_uri(session, panel_id, "https://example.com/tutorial")
		   );
}
//! [gui_change]

/** \brief Subscribe to the panel, change it, print the updates, and end the
 *  subscription. Returns false when a call failed or no update arrived. */
static bool
change_panel_and_follow_updates(QarSession* session, const QarGuiPanelId* panel_id)
{
	//! [gui_subscribe]
	PanelUpdateLog update_log;
	memset(&update_log, 0, sizeof(update_log));
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
		   "qar_gui_panels_subscribe_panel_updates",
		   qar_gui_panels_subscribe_panel_updates(
			   session, panel_id, on_panel_update, &update_log, subscription_token
		   )
	   ))
	{
		qar_cancel_token_handle_destroy(subscription_token);
		example_lock_destroy(&update_log.lock);
		return false;
	}
	//! [gui_subscribe]

	//! [gui_receive]
	const bool changed = change_panel(session, panel_id);
	bool updated = false;
	if(changed)
	{
		updated = example_wait_for_updates(&update_log.lock, &update_log.received_count);
		if(!updated)
		{
			printf(
				"No update of the panel arrived within %u s of the changes.\n",
				EXAMPLE_FIRST_UPDATE_TIMEOUT_MS / 1000u
			);
		}
	}
	//! [gui_receive]

	//! [gui_end_subscription]
	/* Once this returns, no callback runs and none starts: the log is ours. */
	const bool cancelled = example_logged_success(
		"qar_cancel_token_cancel", qar_cancel_token_cancel(subscription_token)
	);
	qar_cancel_token_handle_destroy(subscription_token);
	printf("Received %u updates, printed %u", update_log.received_count, update_log.printed_count);
	if(update_log.printed_count < update_log.received_count)
	{
		printf(": the others matched the update printed before them");
	}
	printf(".\n");
	example_lock_destroy(&update_log.lock);
	//! [gui_end_subscription]
	return changed && updated && cancelled;
}

//! [gui_list]
static void
print_listed_panel(QarGuiPanel* panel)
{
	QarGuiPanelId id = qar_gui_panel_id_default();
	char name[QAR_MAX_STRING_LENGTH] = { 0 };
	PanelSnapshot snapshot;
	QarResult read_result = qar_gui_panel_get_id(panel, &id);
	if(qar_result_is_success(read_result))
	{
		read_result = qar_gui_panel_get_display_name(panel, name, sizeof(name));
	}
	if(qar_result_is_success(read_result))
	{
		read_result = read_panel_snapshot(panel, &snapshot);
	}

	if(qar_result_is_success(read_result))
	{
		printf("- Panel id: ");
		print_hex_id(id.data, QAR_MAX_ID_LENGTH);
		printf(" name='%s' ", name);
		print_panel_snapshot(&snapshot);
		printf("\n");
	}
	else
	{
		log_result("reading a listed GUI panel", read_result);
	}
}

static bool
list_panels(QarSession* session)
{
	size_t panel_count = 0;
	if(!example_quiet_success(
		   "qar_query_gui_panels_count", qar_query_gui_panels_count(session, &panel_count)
	   ))
	{
		return false;
	}
	if(panel_count == 0)
	{
		return true;
	}

	QarGuiPanel** panels = (QarGuiPanel**)calloc(panel_count, sizeof(QarGuiPanel*));
	if(panels == NULL)
	{
		fprintf(stderr, "Out of memory listing %zu panels.\n", panel_count);
		return false;
	}
	size_t written = 0;
	const bool listed = example_quiet_success(
		"qar_query_gui_panels", qar_query_gui_panels(session, panels, panel_count, &written)
	);
	/* The fetch handed us every handle it wrote, also when it then failed: we
	 * destroy them all. */
	const size_t owned = written < panel_count ? written : panel_count;
	for(size_t i = 0; i < owned; ++i)
	{
		if(panels[i] == NULL)
		{
			continue;
		}
		if(listed)
		{
			print_listed_panel(panels[i]);
		}
		qar_gui_panel_handle_destroy(panels[i]);
	}
	free(panels);
	return listed;
}
//! [gui_list]

static int
run_panel_steps(QarSession* session)
{
	QarGuiPanelId panel_id;
	if(!get_or_create_panel(session, &panel_id))
	{
		return 1;
	}

	bool succeeded = change_panel_and_follow_updates(session, &panel_id);
	succeeded = succeeded && list_panels(session);

	//! [gui_close]
	/* Close the panel even when a step failed, so no run leaves one behind. */
	succeeded = example_logged_success(
					"qar_gui_panels_close_panel", qar_gui_panels_close_panel(session, &panel_id)
				)
				&& succeeded;
	//! [gui_close]
	return succeeded ? 0 : 1;
}

int
main(int argc, char** argv)
{
	example_print_as_it_happens();
	//! [gui_startup]
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
		runtime, pairing_code, "gui_panel_operations.onboarding-id.txt", "GUI Panel Peer", &session
	);
	if(exit_code == 0)
	{
		exit_code = run_panel_steps(session);
	}

	/* Children before parents: session, runtime, then the library. */
	if(session != NULL)
	{
		qar_session_handle_destroy(session);
	}
	qar_runtime_handle_destroy(runtime);
	example_unload_library();
	return exit_code;
	//! [gui_startup]
}
