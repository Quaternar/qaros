/** \file cpu_rendering_visualizer.c
 *  \brief Finds the target apps in the peer list, gives each one a CPU render
 *  sender, and renders a lit sphere with depth from each frame's own eye poses.
 *  \example cpu_rendering_visualizer.c
 */

/* The page holds tables and command lines wider than the column limit. */
// clang-format off
/** \page qar_c_tutorial_cpu_rendering CPU Rendering with a Visualizer
 * \tableofcontents
 *
 * \section cpu_rendering_build What you build
 * A source application that floats a lit sphere, 25 cm in radius, in the
 * middle of its own app volume. It finds every target app in the session (a
 * Visualizer, a headset), gives each one a render sender, and renders every
 * frame on the CPU from that target's eye poses and fields of view, with color
 * and depth for both eyes.
 *
 * What you will learn:
 * - why a source app finds its targets in the peer list instead of waiting
 *   for render requests
 * - how to declare the frame views of a stereo color-and-depth stream
 * - the frame cycle: begin a frame, read the pose and field of view of each
 *   view, write pixels, show
 * - which status codes mean "keep the sender and try again" and which mean
 *   "this sender is finished"
 * - which handles you destroy, and in which order
 *
 * \section cpu_rendering_prereq Before you start
 * - **Windows**, with QAROS installed on the Hub machine. The program runs on
 *   the Hub machine. Read \ref qar_c_tutorial_dynamic_loading "Dynamic Loading"
 *   first: this program loads the runtime the same way.
 * - **A target.** At least one target app in the session: open the
 *   Visualizer, or start a headset.
 * - **Joining.** The program joins the Hub on this machine as a local app.
 *   The first run of the executable opens the QAROS approval prompt; click
 *   **Allow** once. Later runs of the same executable join without a prompt.
 *   A local app has no onboarding id, so nothing is stored. Joining as a
 *   local app starts the Hub when it is not running. A pairing code works too;
 *   see \ref cpu_rendering_code "With a pairing code" at the end of this page.
 * - **This repository built** with CMake: `cmake --preset x64-windows`, then
 *   `cmake --build --preset x64-windows-debug --target cpu_rendering_visualizer`.
 *
 * \section cpu_rendering_run Run it
 * \code{.bash}
 * .\build\x64-windows\qar-streaming-c\examples\Debug\cpu_rendering_visualizer.exe --frames=300
 * \endcode
 *
 * `--frames=N` stops the program after N shown frames, counted over all
 * targets; without it, the program stops after 300. `--qaros-library=PATH`
 * loads one library file instead of the installed runtime. Flags take their
 * value after `=`: `--frames 300` is refused. Any other argument, or a second
 * pairing code, prints the usage and exits with 1. An executable that
 * changes after a rebuild asks for **Allow** again.
 *
 * Expected output, without the library's own log lines and the approval
 * progress messages:
 * \code{.txt}
 * Loading the QAROS runtime (default lookup)
 * Loaded <path>\qar-streaming-c.dll (the installed QAROS)
 * Runtime <version> (<flavor>, C API <version>)
 * qar_runtime_create: SUCCESS (code=0)
 * No session to rejoin and no pairing code: joining the QAROS Hub on this machine as a local app. On the first run of this executable, click Allow in the QAROS approval prompt (waiting up to 120 s).
 * qar_runtime_onboard: SUCCESS (local app)
 * A local app has no onboarding id to store (it is zero): the next run joins as a local app again, without a prompt while this executable is unchanged.
 * Looking for target apps in the peer list (open a visualizer or start a headset) ...
 * Rendering for <target name> (peer <peer id>)
 * Submitted 300 frames.
 * qar_library_destroy: SUCCESS (code=0)
 * \endcode
 *
 * On each target, the "CPU Tutorial Volume" app volume shows the sphere.
 *
 * Exit codes:
 * | Code | Meaning |
 * |---|---|
 * | 0 | the frames were shown |
 * | 1 | an unknown argument, a bad `--frames` value, `qar_library_init` or `qar_runtime_create` failed, a failed rejoin or code onboarding, or a failure after joining |
 * | 2 | no runtime loaded |
 * | 3 | the local-app join could not start or reach the Hub |
 * | 4 | the approval was denied |
 * | 5 | nobody answered the approval prompt in time |
 * | 6 | the program answering on this machine is not the installed QAROS |
 * | 7 | this executable no longer matches what QAROS approved |
 *
 * \section cpu_rendering_walkthrough Walkthrough
 *
 * \subsection cpu_rendering_views 1. Declare the frame views
 * A render sender streams one video frame per display frame. The frame views
 * say what is in it and where: which eye, color or depth, and in which
 * texture. This program declares four views: both eyes' color side by side in
 * texture 0, both eyes' depth side by side in texture 1.
 * \snippet cpu_rendering_visualizer.c cpu_frame_views
 *
 * The order matters. Every frame lists its views in the order you declared
 * them, and the view index you pass to
 * \ref qar_render_frame_info_get_view_pose is that position: view 0 is the
 * left eye, view 1 the right eye. A sender created with no views hands out
 * frames without poses: reading the pose of view 0 then fails with
 * `QAR_STATUS_LOGIC_ERROR`.
 *
 * Send depth whenever you can. QAROS uses it for occlusion between app
 * volumes and to warp your frame to the viewer's latest head pose.
 *
 * \subsection cpu_rendering_session 2. Load the runtime and join
 * Every session comes from onboarding: there is no "connect" call. The
 * program reads its arguments, loads the runtime, creates a runtime, and
 * joins with the helpers in `common.h`:
 * \snippet cpu_rendering_visualizer.c cpu_startup
 *
 * `example_join_session` tries three ways in order:
 * rejoin with a stored onboarding id, onboard with a pairing code when you
 * passed one, else join the Hub on this machine as a local app:
 * \snippet common.h example_join_session
 *
 * \ref qar_c_tutorial_local_app_onboarding "Local App Onboarding"
 * explains the local-app join and its exit codes, 3 to 7.
 * \ref qar_c_tutorial_onboarding "Code Pairing and Rejoin"
 * explains the rejoin and the code.
 *
 * \subsection cpu_rendering_volume 3. Create the app volume
 * A render sender needs an app volume. The mixer composites the stream inside
 * that box, and a sender for a target is what attaches the target to it.
 * \snippet cpu_rendering_visualizer.c cpu_volume
 *
 * \ref qar_app_volumes_get_or_create is idempotent: the common name
 * identifies the volume, so a second run gets the same one back. The pose
 * puts its center 1 m in front of the room origin, in room space. The default
 * size is 1 m along each axis.
 *
 * \subsection cpu_rendering_targets 4. Follow the target apps in the peer list
 * Your application decides which targets it renders for; QAROS does not
 * choose for it. This program serves every running target app.
 *
 * Do not wait for render requests to find targets. A target requests your
 * content only once it is attached to your app volume, and a sender is what
 * attaches it. Nothing is attached to a new application's own volume, so an
 * application that waits for requests waits forever.
 *
 * A peer is a target to render for when its role is `QAR_PEER_ROLE_TARGET`
 * and its presence is `QAR_PEER_PRESENCE_ONLINE`, or
 * `QAR_PEER_PRESENCE_UNKNOWN` on a Hub that has not reconciled it yet. The
 * check is in `common.h`, shared with the D3D11 program:
 * \snippet common.h example_is_running_target
 *
 * The program collects those peers from the list:
 * \snippet cpu_rendering_visualizer.c cpu_list_targets
 *
 * \ref qar_query_peer_specs takes an array you size with
 * \ref qar_query_peer_specs_count. Each spec it writes is a handle: destroy it
 * with \ref qar_peer_spec_handle_destroy once you have read it. A call that
 * returns an error may still have written some specs: destroy those too. The
 * list also holds this program's own peer, a source, which the role check
 * skips.
 *
 * On every listing, the program drops the senders of targets that left or
 * stopped running, and creates a sender for every new one:
 * \snippet cpu_rendering_visualizer.c cpu_follow_targets
 *
 * The main loop subscribes to peer updates before it lists the peers, so a
 * target that joins in between is not missed. The callback runs on a library
 * thread. It does nothing but raise a flag, with an Interlocked call because
 * two threads touch it. The loop reads the list again after any update, and
 * at least every 2 s:
 * \snippet cpu_rendering_visualizer.c cpu_peer_updates
 * \snippet cpu_rendering_visualizer.c cpu_render_loop
 *
 * One thread does all the QAROS work. The loop serves the targets in turn
 * with the non-blocking begin (step 6), and sleeps 2 ms only when no target
 * had a frame ready.
 *
 * \subsection cpu_rendering_sender 5. Create a render sender per target
 * \snippet cpu_rendering_visualizer.c cpu_create_sender
 *
 * \ref qar_render_sender_init_default starts with auto-reconnect on, the
 * side-by-side layout, BGRA8 color and D32 depth. The program sets the CPU
 * graphics API, the target, the app volume, RGBA8 color and its views.
 * `QAR_GRAPHICS_API_CPU` takes no extension in `header.next`.
 *
 * \ref qar_render_sender_create blocks until the target answers. A cancel
 * token with a 10 s timeout bounds that wait. A target that does not answer
 * gets no sender; the program prints `No sender for ... yet` and tries that
 * target again 30 s later at the earliest.
 *
 * This program creates senders on the thread that renders, to keep the code
 * short. While a creation waits, no other target gets a frame: up to 10 s for
 * each attempt. A target that keeps not answering therefore blocks the loop
 * for up to 10 s, then leaves it alone for at least 30 s. A real application
 * creates senders off its render thread, with
 * \ref qar_render_sender_create_async or with a thread per target, as the
 * \ref qar_c_tutorial_vulkan_rendering "Vulkan cube" does.
 *
 * \subsection cpu_rendering_frames 6. Begin, render, show
 * \snippet cpu_rendering_visualizer.c cpu_render_next_frame
 *
 * \ref qar_render_sender_try_begin_frame starts the begin-frame wait on its
 * first call and polls it on later calls. `ready` false means no frame yet:
 * the target does not want one, or the receiver paused the stream. Poll again
 * later.
 *
 * A frame you get carries the pose and field of view to render each view
 * with. They are in your app volume's app content space, the space you render
 * in. \ref qar_render_frame_info_get_view_pose and
 * \ref qar_render_frame_info_get_view_fov return them for views 0 and 1, the
 * two eyes, as declared in step 1. A view of no eye, which the layout may add
 * after the declared ones, gets the head pose and the left eye's field of
 * view; the program renders such views from the head, or from the left eye
 * when their pose or field of view cannot be read. The program turns each
 * pose into rays through the pixels:
 * \snippet cpu_rendering_visualizer.c cpu_eye_ray
 *
 * \ref qar_render_sender_frame_cpu hands out the frame's textures as bytes
 * with their pitch, and the views with their pixel rectangles. A view whose
 * start lies past its end is mirrored. The program writes color views as RGBA
 * or BGRA bytes and depth views as 32-bit floats:
 * \snippet cpu_rendering_visualizer.c cpu_write_pixel
 *
 * The header does not specify the depth encoding. The program writes forward
 * depth, 0 at the near plane and 1 at the far plane, as the Vulkan cube does:
 * \snippet cpu_rendering_visualizer.c cpu_forward_depth
 *
 * \ref qar_render_sender_show_frame presents the frame with the near and far
 * planes the frame was rendered with, in app units. QAROS reprojects with
 * exactly this metadata and your depth. Wrong values show up as swimming or
 * misplaced content, never as an error code. Destroy the frame info after
 * the show, not before.
 *
 * A frame that began is always shown, because the show closes the frame
 * cycle that begin opened. When the frame record or the view poses cannot be
 * read, the program renders nothing into the frame, shows it anyway, and then
 * treats the sender as finished: it destroys it and prints
 * `Stopped streaming to ...`. The D3D11 and Vulkan programs follow the same
 * rule.
 *
 * Tracing rays is what costs time: a frame has a ray for each of millions of
 * pixels. So the program traces only where the sphere can be. Seen from
 * above, the sphere is a circle, and the two rays from the eye that touch it
 * bound the columns it can cover; seen from the side, two more rays bound the
 * rows. Every other pixel keeps the background, which the program writes into
 * a view's first row and copies to the others:
 * \snippet cpu_rendering_visualizer.c cpu_sphere_pixels
 *
 * The color view and the depth view of an eye show the same pixels, so each
 * ray is traced once and its hit written into both. The direction of a pixel's
 * ray is the previous pixel's plus a fixed step, not rotated anew:
 * \snippet cpu_rendering_visualizer.c cpu_render_eye
 *
 * The frame rate follows how much of the view the sphere covers: up
 * close, most pixels need a ray. The program renders on one thread to keep
 * the code short. A Debug build calls each small vector function for every
 * pixel; a Release build inlines them and runs several times faster. A real
 * renderer splits the rows across threads, or renders on the GPU (see
 * \ref qar_c_tutorial_d3d11_rendering "D3D11 Rendering").
 *
 * \subsection cpu_rendering_status 7. Keep the sender, or end it
 * Some failures are part of the normal flow. The stream may be reconnecting,
 * may have no transfer yet, or your app volume may not resolve yet where the
 * receiver runs. For these, keep the sender and try the next frame. The check
 * is in `common.h`:
 * \snippet common.h example_is_waiting
 *
 * | Status | What the program does |
 * |---|---|
 * | `QAR_STATUS_MEDIA_STREAMING_STREAM_IS_PAUSED` | keeps the sender; begin-frame waits through a pause, so it mostly shows up as `ready` false |
 * | `QAR_STATUS_MEDIA_STREAMING_STREAM_IS_RECONNECTING` | keeps the sender, tries the next frame |
 * | `QAR_STATUS_MEDIA_STREAMING_NO_ACTIVE_TRANSFERS` | keeps the sender, tries the next frame |
 * | `QAR_STATUS_RENDERING_PRODUCER_APP_VOLUME_UNRESOLVED` | keeps the sender, tries the next frame |
 * | `QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED` | destroys the sender; the next listing creates a new one if the target still runs |
 * | anything else | destroys the sender and prints `Stopped streaming to ...` |
 *
 * Begin-frame reports the waiting codes wrapped inside
 * `QAR_STATUS_RENDERING_PRODUCER_UNABLE_TO_DO_BEGIN_FRAME`.
 * \ref qar_result_has_code matches a wrapped code too, so check with it, not
 * by comparing `QarResult::code`.
 *
 * \section cpu_rendering_ownership Ownership and teardown in C
 * \snippet cpu_rendering_visualizer.c cpu_teardown
 *
 * - **Senders first.** Destroy each sender with
 *   \ref qar_render_stream_handle_destroy before its session.
 * - **The subscription before its state.** The peer callback writes to
 *   `peer_watch`, which lives on `main`'s stack.
 *   \ref qar_cancel_token_handle_destroy ends the subscription and waits for a
 *   running callback, so after it returns nothing writes to `peer_watch`.
 * - **Then the session, the runtime and the library**, in that order.
 *   \ref qar_session_handle_destroy leaves the session.
 * - **One frame info per cycle.** Destroy it with
 *   \ref qar_render_frame_info_handle_destroy after the show. The texture
 *   bytes of \ref qar_render_sender_frame_cpu belong to the sender: write
 *   them only between begin and show, and never free them.
 * - **Peer specs are yours.** Destroy every spec
 *   \ref qar_query_peer_specs wrote, also when it returned an error.
 * - **Callbacks run on library threads.** Keep them short, and never call
 *   blocking QAROS functions or destroy the session from one.
 *
 * \section cpu_rendering_troubleshooting Troubleshooting
 * | What you see | Cause | What to do |
 * |---|---|---|
 * | exit 4 or 5 after `joining the QAROS Hub on this machine as a local app` | the approval prompt was denied, closed or not answered within 120 s | run again and click **Allow** |
 * | `Could not start or reach QAROS on this machine. Start QAROS from the Start menu, then start this app again.`, exit 3 | the local-app join could neither start the Hub nor reach it | start QAROS from the Start menu and run again |
 * | `No QAROS runtime loaded.`, exit 2 | no runtime found | install QAROS, fix or unset `QAR_INSTANCE`, or pass `--qaros-library=PATH` |
 * | stays at `Looking for target apps ...` | no running target app in the session | open the Visualizer or start a headset |
 * | `No sender for <target> yet; trying again in 30 s`, repeated | the target does not answer the sender creation within 10 s | check that the target is running; each attempt blocks the loop for up to 10 s |
 * | `Stopped streaming to <target>` | a failure that is not part of the normal flow, `QAR_STATUS_RENDERING_PRODUCER_STREAM_IS_CLOSED` included | the next listing creates a new sender while the target still runs |
 * | content swims or sits at the wrong depth | the near and far planes of the show do not match the frame, or poses not taken from this frame | render with this frame's poses and report the planes you used |
 *
 * \section cpu_rendering_next Next
 * - \ref qar_c_tutorial_d3d11_rendering "D3D11 Rendering": the same frame
 *   cycle with shared GPU textures, on a D3D11 device of your own.
 * - <a href="/docs/developer-guide/rendering-streams">Rendering Streams</a>:
 *   layouts, layout changes, hand tracking.
 * - <a href="/docs/developer-guide/tutorials/rust/cpu-rendering">CPU Rendering in Rust</a>:
 *   the same program on the Rust binding.
 *
 * \section cpu_rendering_code With a pairing code
 * The program also takes a pairing code, as its first argument that does not
 * start with `--`. It passes no Hub address, so the code finds a Hub of your
 * own user, as \ref qar_c_tutorial_onboarding "Code Pairing and Rejoin"
 * explains. An app on another machine also names the Hub with a
 * QarOnboardHostExt; that tutorial shows how.
 * \code{.bash}
 * .\build\x64-windows\qar-streaming-c\examples\Debug\cpu_rendering_visualizer.exe <PAIRING-CODE> --frames=300
 * \endcode
 * It stores the onboarding id it gets back in
 * `cpu_rendering_visualizer.onboarding-id.txt` in the working directory, and
 * later runs rejoin with it.
 */
// clang-format on

#include "common.h"

#include <math.h>
#include <windows.h>

QAR_IMPLEMENT_DYNAMIC_LOADING()

#define DEFAULT_FRAME_COUNT 300u
/* Near and far planes in app units, the units of the poses a sender bound to an
 * app volume hands out (QarNearFar). */
#define NEAR_PLANE 0.1f
#define FAR_PLANE 10.0f
#define SPHERE_RADIUS 0.25f
/* How often the peer list is read when no peer update arrived. */
#define PEER_LIST_INTERVAL_MS 2000u
/* How long creating a sender may block the render loop. */
#define SENDER_CREATION_TIMEOUT_MS 10000u
/* How long a target whose sender creation failed waits for the next attempt,
 * so a silent target blocks the loop for one timeout at most this often. */
#define SENDER_RETRY_DELAY_MS 30000u
#define IDLE_WAIT_MS 2u
#define MAX_SERVED_TARGETS 8u
#define MAX_LISTED_TARGETS 64u
#define TARGET_NAME_SIZE 128u

//! [cpu_frame_views]
/* Color of both eyes side by side in texture 0, depth side by side in texture
 * 1. Views 0 and 1 are the left and the right eye for poses and fields of
 * view. */
static const QarRenderFrameView FRAME_VIEWS[] = {
	{ QAR_VIDEO_FRAME_VIEW_TYPE_COLOR, QAR_VIDEO_FRAME_VIEW_EYE_LEFT, 0 },
	{ QAR_VIDEO_FRAME_VIEW_TYPE_COLOR, QAR_VIDEO_FRAME_VIEW_EYE_RIGHT, 0 },
	{ QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH, QAR_VIDEO_FRAME_VIEW_EYE_LEFT, 1 },
	{ QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH, QAR_VIDEO_FRAME_VIEW_EYE_RIGHT, 1 },
};
//! [cpu_frame_views]

/** One target app and, once it is served, the sender rendering for it. */
typedef struct Target
{
	QarPeerId peer;
	char name[TARGET_NAME_SIZE];
	QarRenderSender* sender;
	bool head_ray_failure_logged;
} Target;

/** A target whose sender creation failed, and when to try it again. */
typedef struct RetryTime
{
	QarPeerId peer;
	ULONGLONG retry_at_ms;
} RetryTime;

typedef struct TargetList
{
	Target items[MAX_SERVED_TARGETS];
	size_t count;
	RetryTime retries[MAX_LISTED_TARGETS];
	size_t retry_count;
} TargetList;

typedef enum FrameOutcome
{
	FRAME_SHOWN,
	FRAME_NOT_SHOWN,
	FRAME_FAILED
} FrameOutcome;

static void
print_target_name(const Target* target)
{
	if(target->name[0] != '\0')
	{
		printf("%s", target->name);
	}
	else
	{
		print_hex_id(target->peer.data, QAR_MAX_ID_LENGTH);
	}
}

//! [cpu_peer_updates]
/** Raised by the peer callback on a library thread, read and cleared by the
 *  render loop: two threads, so only Interlocked calls touch it. */
typedef struct PeerWatch
{
	volatile LONG changed;
} PeerWatch;

static void
on_peer_update(QarPeerSpec* peer, void* user_state)
{
	(void)peer; /* the render loop reads the whole list itself */
	PeerWatch* watch = (PeerWatch*)user_state;
	InterlockedExchange(&watch->changed, 1);
}

static bool
take_peer_update(PeerWatch* watch)
{
	return InterlockedExchange(&watch->changed, 0) != 0;
}
//! [cpu_peer_updates]

//! [cpu_list_targets]
static void
collect_running_target(QarPeerSpec* peer, Target* listed, size_t capacity, size_t* count)
{
	Target target = { qar_peer_id_default(), { 0 }, NULL, false };
	if(*count == capacity || !example_is_running_target(peer)
	   || qar_result_is_error(qar_peer_spec_get_id(peer, &target.peer)))
	{
		return;
	}
	if(qar_result_is_error(qar_peer_spec_get_display_name(peer, target.name, sizeof(target.name))))
	{
		target.name[0] = '\0'; /* shown by its id instead */
	}
	listed[(*count)++] = target;
}

/** \brief The running target apps the session lists. Returns false after
 *  printing why when the list cannot be read. */
static bool
list_running_targets(QarSession* session, Target* listed, size_t capacity, size_t* out_count)
{
	*out_count = 0;
	size_t peer_count = 0;
	QarResult counted = qar_query_peer_specs_count(session, &peer_count);
	if(qar_result_is_error(counted))
	{
		log_result("qar_query_peer_specs_count", counted);
		return false;
	}
	if(peer_count == 0)
	{
		return true; /* right after joining: the peer updates report them */
	}

	QarPeerSpec** peers = (QarPeerSpec**)calloc(peer_count, sizeof(*peers));
	if(peers == NULL)
	{
		fprintf(stderr, "Out of memory listing %zu peers\n", peer_count);
		return false;
	}
	size_t written = 0;
	QarResult queried = qar_query_peer_specs(session, peers, peer_count, &written);
	const bool listed_all = qar_result_is_success(queried);
	if(!listed_all)
	{
		log_result("qar_query_peer_specs", queried);
	}
	/* A failed query may still have written some specs: destroy those too. */
	const size_t owned = written < peer_count ? written : peer_count;
	for(size_t i = 0; i < owned; ++i)
	{
		if(peers[i] == NULL)
		{
			continue;
		}
		if(listed_all)
		{
			collect_running_target(peers[i], listed, capacity, out_count);
		}
		qar_peer_spec_handle_destroy(peers[i]);
	}
	free(peers);
	return listed_all;
}
//! [cpu_list_targets]

//! [cpu_create_sender]
static QarResult
create_sender(
	QarSession* session,
	const QarAppVolumeId* volume_id,
	QarPeerId target_peer,
	QarRenderSender** out_sender
)
{
	QarRenderSenderInit sender_init = qar_render_sender_init_default();
	sender_init.graphics_api = QAR_GRAPHICS_API_CPU;
	sender_init.peer_id = target_peer;
	sender_init.app_volume_id = volume_id;
	sender_init.texture_layout = QAR_FRAME_LAYOUT_SIDE_BY_SIDE;
	sender_init.color_format = QAR_PIXEL_FORMAT_R8G8B8A8;
	sender_init.depth_format = QAR_PIXEL_FORMAT_D32_FLOAT;
	memcpy(sender_init.frame_views, FRAME_VIEWS, sizeof(FRAME_VIEWS));
	sender_init.frame_views_count = sizeof(FRAME_VIEWS) / sizeof(FRAME_VIEWS[0]);

	/* Creation blocks until the target answers, and no target renders
	 * meanwhile. The timeout bounds that wait, and a target whose creation
	 * failed is tried again only after SENDER_RETRY_DELAY_MS. */
	QarCancelToken* timeout = NULL;
	QarResult result = qar_cancel_token_create_with_timeout(&timeout, SENDER_CREATION_TIMEOUT_MS);
	if(qar_result_is_error(result))
	{
		return result;
	}
	result = qar_render_sender_create(session, &sender_init, timeout, out_sender);
	qar_cancel_token_handle_destroy(timeout);
	return result;
}
//! [cpu_create_sender]

static Target*
find_target(Target* targets, size_t count, const QarPeerId* peer)
{
	for(size_t i = 0; i < count; ++i)
	{
		if(qar_peer_id_equals(&targets[i].peer, peer))
		{
			return &targets[i];
		}
	}
	return NULL;
}

static void
remove_target(TargetList* targets, size_t index)
{
	qar_render_stream_handle_destroy(targets->items[index].sender);
	targets->items[index] = targets->items[targets->count - 1];
	--targets->count;
}

static RetryTime*
find_retry(TargetList* targets, const QarPeerId* peer)
{
	for(size_t i = 0; i < targets->retry_count; ++i)
	{
		if(qar_peer_id_equals(&targets->retries[i].peer, peer))
		{
			return &targets->retries[i];
		}
	}
	return NULL;
}

static bool
is_waiting_to_retry(TargetList* targets, const QarPeerId* peer, ULONGLONG now_ms)
{
	const RetryTime* retry = find_retry(targets, peer);
	return retry != NULL && now_ms < retry->retry_at_ms;
}

static void
delay_retry(TargetList* targets, const QarPeerId* peer, ULONGLONG now_ms)
{
	RetryTime* retry = find_retry(targets, peer);
	if(retry == NULL)
	{
		if(targets->retry_count == MAX_LISTED_TARGETS)
		{
			return; /* this target is then tried again on the next listing */
		}
		retry = &targets->retries[targets->retry_count++];
		retry->peer = *peer;
	}
	retry->retry_at_ms = now_ms + SENDER_RETRY_DELAY_MS;
}

/** \brief Drop the retry times of targets the list no longer shows. */
static void
forget_unlisted_retries(TargetList* targets, Target* listed, size_t listed_count)
{
	size_t index = 0;
	while(index < targets->retry_count)
	{
		if(find_target(listed, listed_count, &targets->retries[index].peer) != NULL)
		{
			++index;
			continue;
		}
		targets->retries[index] = targets->retries[--targets->retry_count];
	}
}

static void
start_target(
	QarSession* session, const QarAppVolumeId* volume_id, const Target* listed, TargetList* targets
)
{
	if(targets->count == MAX_SERVED_TARGETS)
	{
		printf("No sender for ");
		print_target_name(listed);
		printf(": this program serves at most %u target apps\n", MAX_SERVED_TARGETS);
		return;
	}
	QarRenderSender* sender = NULL;
	QarResult created = create_sender(session, volume_id, listed->peer, &sender);
	if(qar_result_is_error(created))
	{
		log_result("qar_render_sender_create", created);
		printf("No sender for ");
		print_target_name(listed);
		printf(" yet; trying again in %u s\n", SENDER_RETRY_DELAY_MS / 1000u);
		delay_retry(targets, &listed->peer, GetTickCount64());
		return;
	}
	Target* target = &targets->items[targets->count++];
	*target = *listed;
	target->sender = sender;
	printf("Rendering for ");
	print_target_name(target);
	printf(" (peer ");
	print_hex_id(target->peer.data, QAR_MAX_ID_LENGTH);
	printf(")\n");
}

//! [cpu_follow_targets]
/** \brief Create a sender for every running target app the list shows, and
 *  destroy the senders of targets it no longer shows running. A target whose
 *  sender creation failed is tried again only after SENDER_RETRY_DELAY_MS.
 *  Returns false when the list cannot be read. */
static bool
follow_targets(QarSession* session, const QarAppVolumeId* volume_id, TargetList* targets)
{
	Target listed[MAX_LISTED_TARGETS];
	size_t listed_count = 0;
	if(!list_running_targets(session, listed, MAX_LISTED_TARGETS, &listed_count))
	{
		return false;
	}

	size_t index = 0;
	while(index < targets->count)
	{
		if(find_target(listed, listed_count, &targets->items[index].peer) != NULL)
		{
			++index;
			continue;
		}
		print_target_name(&targets->items[index]);
		printf(" left\n");
		remove_target(targets, index);
	}
	forget_unlisted_retries(targets, listed, listed_count);

	const ULONGLONG now_ms = GetTickCount64();
	for(size_t i = 0; i < listed_count; ++i)
	{
		if(find_target(targets->items, targets->count, &listed[i].peer) == NULL
		   && !is_waiting_to_retry(targets, &listed[i].peer, now_ms))
		{
			start_target(session, volume_id, &listed[i], targets);
		}
	}
	return true;
}
//! [cpu_follow_targets]

static QarVector3
vector_add(QarVector3 left, QarVector3 right)
{
	QarVector3 sum = { left.x + right.x, left.y + right.y, left.z + right.z };
	return sum;
}

static QarVector3
vector_subtract(QarVector3 left, QarVector3 right)
{
	QarVector3 difference = { left.x - right.x, left.y - right.y, left.z - right.z };
	return difference;
}

static QarVector3
vector_scale(QarVector3 vector, float factor)
{
	QarVector3 scaled = { vector.x * factor, vector.y * factor, vector.z * factor };
	return scaled;
}

static float
vector_dot(QarVector3 left, QarVector3 right)
{
	return left.x * right.x + left.y * right.y + left.z * right.z;
}

static QarVector3
vector_cross(QarVector3 left, QarVector3 right)
{
	QarVector3 cross = {
		left.y * right.z - left.z * right.y,
		left.z * right.x - left.x * right.z,
		left.x * right.y - left.y * right.x,
	};
	return cross;
}

static QarVector3
vector_normalize(QarVector3 vector)
{
	return vector_scale(vector, 1.0f / sqrtf(vector_dot(vector, vector)));
}

/** \brief `vector` rotated by the unit quaternion `rotation`. */
static QarVector3
vector_rotate(QarQuaternion rotation, QarVector3 vector)
{
	const QarVector3 axis = { rotation.x, rotation.y, rotation.z };
	const QarVector3 twice_cross = vector_scale(vector_cross(axis, vector), 2.0f);
	return vector_add(
		vector_add(vector, vector_scale(twice_cross, rotation.w)), vector_cross(axis, twice_cross)
	);
}

//! [cpu_eye_ray]
/** Rays through an eye's pixels, in app content space. */
typedef struct EyeRay
{
	QarVector3 origin;
	QarQuaternion orientation;
	float tangent_left;
	float tangent_right;
	float tangent_up;
	float tangent_down;
} EyeRay;

static EyeRay
eye_ray_from(QarPose pose, QarFov fov)
{
	EyeRay eye;
	eye.origin = pose.position;
	eye.orientation = pose.orientation;
	eye.tangent_left = tanf(fov.angle_left);
	eye.tangent_right = tanf(fov.angle_right);
	eye.tangent_up = tanf(fov.angle_up);
	eye.tangent_down = tanf(fov.angle_down);
	return eye;
}

/** \brief The direction through view-local pixel (u, v) of a width x height
 *  view, scaled so that its view-space Z is -1: the ray parameter at a hit is
 *  then the hit's depth along the optical axis. */
static QarVector3
eye_ray_direction(const EyeRay* eye, float u, float v, float width, float height)
{
	const QarVector3 view_space = {
		eye->tangent_left + (u + 0.5f) / width * (eye->tangent_right - eye->tangent_left),
		eye->tangent_up - (v + 0.5f) / height * (eye->tangent_up - eye->tangent_down),
		-1.0f,
	};
	return vector_rotate(eye->orientation, view_space);
}

/** \brief The rays of one view, from its pose and field of view. Logs a
 *  failed read when log_failure is true. */
static bool
read_view_ray(QarRenderFrameInfo* frame_info, size_t view_index, bool log_failure, EyeRay* out_ray)
{
	QarPose pose = qar_pose_default();
	QarFov fov = { 0 };
	QarResult pose_result = qar_render_frame_info_get_view_pose(frame_info, view_index, &pose);
	QarResult fov_result = qar_render_frame_info_get_view_fov(frame_info, view_index, &fov);
	if(qar_result_is_error(pose_result) || qar_result_is_error(fov_result))
	{
		if(log_failure)
		{
			log_result("qar_render_frame_info_get_view_pose", pose_result);
			log_result("qar_render_frame_info_get_view_fov", fov_result);
		}
		return false;
	}
	*out_ray = eye_ray_from(pose, fov);
	return true;
}

/** The rays to render a frame's views with. */
typedef struct FrameRays
{
	EyeRay left;
	EyeRay right;
	/* For the views of no eye, which the layout may add after the declared
	 * ones: such a view gets the head pose and the left eye's field of view.
	 * The left eye's ray when their pose or field of view cannot be read. */
	EyeRay head;
} FrameRays;

/** \brief The rays of views 0 and 1, the left and the right eye as declared
 *  in FRAME_VIEWS, and of the first view of no eye when the frame has one.
 *  Only the eyes are required: a view of no eye falls back to the left eye's
 *  ray, and the first such fallback on a sender is logged. */
static bool
read_frame_rays(
	Target* target,
	const QarVideoFrameCpu* frame,
	QarRenderFrameInfo* frame_info,
	FrameRays* out_rays
)
{
	if(!read_view_ray(frame_info, 0, true, &out_rays->left)
	   || !read_view_ray(frame_info, 1, true, &out_rays->right))
	{
		return false;
	}
	out_rays->head = out_rays->left;
	for(size_t view_index = 0; view_index < frame->texture_views_count; ++view_index)
	{
		if(frame->texture_views[view_index].eye != QAR_VIDEO_FRAME_VIEW_EYE_NONE)
		{
			continue;
		}
		const bool log_failure = !target->head_ray_failure_logged;
		if(!read_view_ray(frame_info, view_index, log_failure, &out_rays->head) && log_failure)
		{
			printf("Rendering the views of no eye for ");
			print_target_name(target);
			printf(" from the left eye: their pose or field of view cannot be read\n");
			target->head_ray_failure_logged = true;
		}
		break;
	}
	return true;
}
//! [cpu_eye_ray]

/** What one frame shows: the sphere bobs and changes color with the time. */
typedef struct Scene
{
	QarVector3 sphere_center;
	/* Red, green and blue of a fully lit point, 0 to 1. */
	float sphere_color[3];
	/* Unit length, toward the light. */
	QarVector3 light_direction;
} Scene;

/** \brief The scene at `seconds` after the start. Computed once per frame, not
 *  per pixel. */
static Scene
scene_at(float seconds)
{
	const float hue = seconds * 0.4f;
	const QarVector3 light_direction = { 0.4f, 0.8f, 0.5f };
	Scene scene;
	scene.sphere_center.x = 0.0f;
	scene.sphere_center.y = 0.05f * sinf(seconds * 1.5f);
	scene.sphere_center.z = 0.0f;
	scene.sphere_color[0] = 0.55f + 0.45f * sinf(hue);
	scene.sphere_color[1] = 0.55f + 0.45f * sinf(hue + 2.1f);
	scene.sphere_color[2] = 0.55f + 0.45f * sinf(hue + 4.2f);
	scene.light_direction = vector_normalize(light_direction);
	return scene;
}

typedef struct SphereHit
{
	uint8_t color[4];
	/* Along the optical axis, in app units. */
	float distance;
} SphereHit;

static uint8_t
color_channel(float value)
{
	const float scaled = value * 255.0f;
	return (uint8_t)(scaled < 0.0f ? 0.0f : (scaled > 255.0f ? 255.0f : scaled));
}

/** \brief Lambert-shaded sphere color and its depth, or false on a miss. */
static bool
trace_sphere(QarVector3 origin, QarVector3 direction, const Scene* scene, SphereHit* out_hit)
{
	const QarVector3 center = scene->sphere_center;
	const QarVector3 offset = vector_subtract(origin, center);
	const float a = vector_dot(direction, direction);
	const float half_b = vector_dot(offset, direction);
	const float c = vector_dot(offset, offset) - SPHERE_RADIUS * SPHERE_RADIUS;
	const float discriminant = half_b * half_b - a * c;
	if(discriminant < 0.0f)
	{
		return false;
	}
	const float distance = (-half_b - sqrtf(discriminant)) / a;
	if(distance <= NEAR_PLANE || distance >= FAR_PLANE)
	{
		return false;
	}
	const QarVector3 hit = vector_add(origin, vector_scale(direction, distance));
	const QarVector3 normal = vector_scale(vector_subtract(hit, center), 1.0f / SPHERE_RADIUS);
	const float lambert = vector_dot(normal, scene->light_direction);
	const float brightness = 0.25f + 0.75f * (lambert > 0.0f ? lambert : 0.0f);
	out_hit->color[0] = color_channel(scene->sphere_color[0] * brightness);
	out_hit->color[1] = color_channel(scene->sphere_color[1] * brightness);
	out_hit->color[2] = color_channel(scene->sphere_color[2] * brightness);
	out_hit->color[3] = 255;
	out_hit->distance = distance;
	return true;
}

//! [cpu_forward_depth]
/** \brief D3D/OpenXR-style forward depth: 0 at near, 1 at far. */
static float
forward_depth(float distance)
{
	return FAR_PLANE * (distance - NEAR_PLANE) / (distance * (FAR_PLANE - NEAR_PLANE));
}
//! [cpu_forward_depth]

//! [cpu_write_pixel]
/** \brief Formats with four bytes a pixel that write_pixel knows. */
static bool
is_known_format(QarPixelFormat format)
{
	return format == QAR_PIXEL_FORMAT_D32_FLOAT || format == QAR_PIXEL_FORMAT_R32_FLOAT
		   || format == QAR_PIXEL_FORMAT_R8G8B8A8 || format == QAR_PIXEL_FORMAT_B8G8R8A8;
}

/** \brief Color views get the sphere on transparent black, depth views its
 *  depth on the far plane. `hit` is NULL on a miss. */
static void
write_pixel(uint8_t* pixel, QarPixelFormat format, const SphereHit* hit)
{
	static const uint8_t transparent_black[4] = { 0, 0, 0, 0 };
	const uint8_t* color = hit != NULL ? hit->color : transparent_black;
	if(format == QAR_PIXEL_FORMAT_D32_FLOAT || format == QAR_PIXEL_FORMAT_R32_FLOAT)
	{
		const float depth = hit != NULL ? forward_depth(hit->distance) : 1.0f;
		memcpy(pixel, &depth, sizeof(depth));
	}
	else if(format == QAR_PIXEL_FORMAT_R8G8B8A8)
	{
		memcpy(pixel, color, 4);
	}
	else if(format == QAR_PIXEL_FORMAT_B8G8R8A8)
	{
		pixel[0] = color[2];
		pixel[1] = color[1];
		pixel[2] = color[0];
		pixel[3] = color[3];
	}
}
//! [cpu_write_pixel]

//! [cpu_sphere_pixels]
/** View-local pixels: columns from left and rows from top, up to but not
 *  including right and bottom. */
typedef struct PixelRect
{
	uint32_t left;
	uint32_t top;
	uint32_t right;
	uint32_t bottom;
} PixelRect;

/** \brief Where the ray with view-space tangent `tangent` crosses a view
 *  `size` pixels long whose edges have the tangents `first_edge` and
 *  `last_edge`, clamped to the view: eye_ray_direction's mapping in reverse. */
static float
pixel_edge(float tangent, float first_edge, float last_edge, float size)
{
	const float position = (tangent - first_edge) / (last_edge - first_edge) * size;
	/* Written so that NaN, from a field of view of zero width, gives 0. */
	if(!(position > 0.0f))
	{
		return 0.0f;
	}
	return position < size ? position : size;
}

/** \brief The view-space tangents, lowest first, of the two rays from the
 *  eye that touch the sphere, looking along one view axis.
 *
 *  Along that axis the sphere is a circle, `across` to the side of the eye's
 *  optical axis and `ahead` in front of the eye. Each touching ray is the
 *  direction to the center turned by the angle whose sine is radius over
 *  distance. Returns false when there is no such pair: the eye is inside the
 *  circle, or a touching ray points sideways or behind the eye. */
static bool
touching_tangents(float across, float ahead, float* out_lowest, float* out_highest)
{
	const float quarter_turn = 1.57079633f;
	const float distance = sqrtf(across * across + ahead * ahead);
	if(distance <= SPHERE_RADIUS)
	{
		return false;
	}
	const float center_angle = atan2f(across, ahead);
	const float touching_angle = asinf(SPHERE_RADIUS / distance);
	if(fabsf(center_angle) + touching_angle >= quarter_turn)
	{
		return false;
	}
	*out_lowest = tanf(center_angle - touching_angle);
	*out_highest = tanf(center_angle + touching_angle);
	return true;
}

/** \brief The pixels of a width x height view whose rays can hit the sphere.
 *
 *  The columns lie between the two rays that touch the sphere seen from
 *  above, and the rows between the two that touch it seen from the side.
 *  Rays through the other pixels miss the sphere. When the touching rays do
 *  not bound the view, the whole view is traced. */
static PixelRect
sphere_pixels(const EyeRay* eye, QarVector3 sphere_center, uint32_t width, uint32_t height)
{
	const PixelRect whole_view = { 0, 0, width, height };
	const PixelRect no_pixels = { 0, 0, 0, 0 };
	/* The inverse rotation takes app content space into view space, where the
	 * eye looks along -Z. */
	const QarQuaternion to_view_space = {
		-eye->orientation.x, -eye->orientation.y, -eye->orientation.z, eye->orientation.w
	};
	const QarVector3 center =
		vector_rotate(to_view_space, vector_subtract(sphere_center, eye->origin));
	const float ahead = -center.z;
	if(ahead + SPHERE_RADIUS <= NEAR_PLANE)
	{
		return no_pixels; /* trace_sphere finds no hit closer than the near plane */
	}
	float lowest_x = 0.0f;
	float highest_x = 0.0f;
	float lowest_y = 0.0f;
	float highest_y = 0.0f;
	if(!touching_tangents(center.x, ahead, &lowest_x, &highest_x)
	   || !touching_tangents(center.y, ahead, &lowest_y, &highest_y))
	{
		return whole_view;
	}
	const float left_column =
		pixel_edge(lowest_x, eye->tangent_left, eye->tangent_right, (float)width);
	const float right_column =
		pixel_edge(highest_x, eye->tangent_left, eye->tangent_right, (float)width);
	const float top_row = pixel_edge(highest_y, eye->tangent_up, eye->tangent_down, (float)height);
	const float bottom_row =
		pixel_edge(lowest_y, eye->tangent_up, eye->tangent_down, (float)height);
	/* Rounded outward, so a pixel the edge cuts through is traced. fminf and
	 * fmaxf keep the order for a field of view whose sides are swapped. */
	PixelRect pixels;
	pixels.left = (uint32_t)floorf(fminf(left_column, right_column));
	pixels.right = (uint32_t)ceilf(fmaxf(left_column, right_column));
	pixels.top = (uint32_t)floorf(fminf(top_row, bottom_row));
	pixels.bottom = (uint32_t)ceilf(fmaxf(top_row, bottom_row));
	return pixels;
}
//! [cpu_sphere_pixels]

/** \brief The texture column or row of view-local column or row `local`. A
 *  view whose start lies past its end is mirrored: its first pixel is the one
 *  just before start. */
static uint32_t
texture_coordinate(uint32_t local, uint32_t start, uint32_t end)
{
	return start <= end ? start + local : start - 1 - local;
}

/** \brief The bytes of view-local pixel (column, row). */
static uint8_t*
view_pixel(
	const QarVideoFrameCpu* frame, const QarVideoFrameView* view, uint32_t column, uint32_t row
)
{
	const QarVideoTextureCpu* texture = &frame->textures[view->texture_index];
	return texture->texture_data
		   + (size_t)texture_coordinate(row, view->start_y, view->end_y) * texture->pitch
		   + (size_t)texture_coordinate(column, view->start_x, view->end_x) * 4;
}

static uint32_t
view_width(const QarVideoFrameView* view)
{
	return view->start_x < view->end_x ? view->end_x - view->start_x : view->start_x - view->end_x;
}

static uint32_t
view_height(const QarVideoFrameView* view)
{
	return view->start_y < view->end_y ? view->end_y - view->start_y : view->start_y - view->end_y;
}

/** \brief A view in a format write_pixel knows, inside a texture it fits. */
static bool
is_writable_view(const QarVideoFrameCpu* frame, const QarVideoFrameView* view)
{
	if(view->texture_index >= frame->textures_count || !is_known_format(view->texture_format)
	   || frame->textures[view->texture_index].texture_data == NULL)
	{
		return false;
	}
	const QarVideoTextureCpu* texture = &frame->textures[view->texture_index];
	const uint32_t column_end = view->start_x < view->end_x ? view->end_x : view->start_x;
	const uint32_t row_end = view->start_y < view->end_y ? view->end_y : view->start_y;
	if((size_t)row_end * texture->pitch > texture->texture_data_size
	   || (size_t)column_end * 4 > texture->pitch)
	{
		printf("Leaving a view untouched that does not fit its texture\n");
		return false;
	}
	return true;
}

/** \brief Write the background into every pixel of a view: into its first
 *  row, then copy that row to the others. Copying a row costs far less than
 *  writing its pixels one by one. */
static void
fill_background(const QarVideoFrameCpu* frame, const QarVideoFrameView* view)
{
	const uint32_t width = view_width(view);
	const uint32_t height = view_height(view);
	if(width == 0 || height == 0)
	{
		return;
	}
	const QarVideoTextureCpu* texture = &frame->textures[view->texture_index];
	const uint32_t column_start = view->start_x < view->end_x ? view->start_x : view->end_x;
	const uint32_t row_start = view->start_y < view->end_y ? view->start_y : view->end_y;
	uint8_t* first_row =
		texture->texture_data + (size_t)row_start * texture->pitch + (size_t)column_start * 4;
	for(uint32_t column = 0; column < width; ++column)
	{
		write_pixel(first_row + (size_t)column * 4, view->texture_format, NULL);
	}
	for(uint32_t row = 1; row < height; ++row)
	{
		memcpy(first_row + (size_t)row * texture->pitch, first_row, (size_t)width * 4);
	}
}

/** \brief Write a hit into the same pixel of each view. */
static void
write_hit(
	const QarVideoFrameCpu* frame,
	const QarVideoFrameView* const* views,
	size_t view_count,
	uint32_t column,
	uint32_t row,
	const SphereHit* hit
)
{
	for(size_t i = 0; i < view_count; ++i)
	{
		write_pixel(view_pixel(frame, views[i], column, row), views[i]->texture_format, hit);
	}
}

//! [cpu_render_eye]
/** \brief Render one eye into its views: the background everywhere, then the
 *  sphere where a ray can hit it. The eye's color view and depth view show
 *  the same pixels, so each ray is traced once and its hit written into
 *  both. */
static void
render_eye(
	const QarVideoFrameCpu* frame,
	QarVideoFrameViewEye which_eye,
	const EyeRay* eye,
	const Scene* scene
)
{
	const QarVideoFrameView* views[QAR_MAX_FRAME_VIEWS];
	size_t view_count = 0;
	for(size_t i = 0; i < frame->texture_views_count; ++i)
	{
		const QarVideoFrameView* view = &frame->texture_views[i];
		if(view->eye != which_eye || !is_writable_view(frame, view))
		{
			continue;
		}
		fill_background(frame, view);
		if(view_count > 0
		   && (view_width(view) != view_width(views[0])
			   || view_height(view) != view_height(views[0])))
		{
			printf(
				"Leaving a view at the background: its size differs from its eye's other view\n"
			);
			continue;
		}
		views[view_count++] = view;
	}
	if(view_count == 0)
	{
		return;
	}

	const uint32_t width = view_width(views[0]);
	const uint32_t height = view_height(views[0]);
	/* eye_ray_direction is linear in the column and the row, so the next
	 * pixel's direction is this one's plus a fixed step. Adding the step costs
	 * far less than rotating every pixel's direction. */
	const float width_pixels = (float)width;
	const float height_pixels = (float)height;
	const QarVector3 first_direction =
		eye_ray_direction(eye, 0.0f, 0.0f, width_pixels, height_pixels);
	const QarVector3 column_step = vector_subtract(
		eye_ray_direction(eye, 1.0f, 0.0f, width_pixels, height_pixels), first_direction
	);
	const QarVector3 row_step = vector_subtract(
		eye_ray_direction(eye, 0.0f, 1.0f, width_pixels, height_pixels), first_direction
	);
	const PixelRect pixels = sphere_pixels(eye, scene->sphere_center, width, height);
	for(uint32_t row = pixels.top; row < pixels.bottom; ++row)
	{
		QarVector3 direction = vector_add(
			vector_add(first_direction, vector_scale(row_step, (float)row)),
			vector_scale(column_step, (float)pixels.left)
		);
		for(uint32_t column = pixels.left; column < pixels.right; ++column)
		{
			SphereHit hit;
			if(trace_sphere(eye->origin, direction, scene, &hit))
			{
				write_hit(frame, views, view_count, column, row, &hit);
			}
			direction = vector_add(direction, column_step);
		}
	}
}
//! [cpu_render_eye]

static void
render_frame(const QarVideoFrameCpu* frame, const FrameRays* rays, float seconds)
{
	const Scene scene = scene_at(seconds);
	render_eye(frame, QAR_VIDEO_FRAME_VIEW_EYE_LEFT, &rays->left, &scene);
	render_eye(frame, QAR_VIDEO_FRAME_VIEW_EYE_RIGHT, &rays->right, &scene);
	render_eye(frame, QAR_VIDEO_FRAME_VIEW_EYE_NONE, &rays->head, &scene);
}

//! [cpu_render_next_frame]
/** \brief Render and show the target's next frame when its sender has one.
 *  FRAME_NOT_SHOWN while none is ready or the stream is waiting; FRAME_FAILED
 *  when this sender is finished. */
static FrameOutcome
render_next_frame(Target* target, float seconds)
{
	QarRenderFrameInfo* frame_info = NULL;
	bool ready = false;
	QarResult begun = qar_render_sender_try_begin_frame(target->sender, &frame_info, &ready);
	if(example_is_waiting(begun))
	{
		return FRAME_NOT_SHOWN;
	}
	if(qar_result_is_error(begun))
	{
		log_result("qar_render_sender_try_begin_frame", begun);
		return FRAME_FAILED;
	}
	if(!ready)
	{
		return FRAME_NOT_SHOWN;
	}

	FrameRays rays;
	QarVideoFrameCpu frame = qar_video_frame_cpu_default();
	QarResult frame_result = qar_render_sender_frame_cpu(target->sender, &frame);
	if(qar_result_is_error(frame_result))
	{
		log_result("qar_render_sender_frame_cpu", frame_result);
	}
	const bool readable =
		qar_result_is_success(frame_result) && read_frame_rays(target, &frame, frame_info, &rays);
	if(readable)
	{
		render_frame(&frame, &rays, seconds);
	}

	/* Show closes the frame cycle that begin opened, so it runs even when the
	 * frame could not be read. The frame info lives until then. */
	QarRenderFrameShow show = qar_render_frame_show_default();
	show.rendered_near_far.near_plane = NEAR_PLANE;
	show.rendered_near_far.far_plane = FAR_PLANE;
	QarResult shown = qar_render_sender_show_frame(target->sender, &show);
	qar_render_frame_info_handle_destroy(frame_info);
	if(!readable)
	{
		return FRAME_FAILED;
	}
	if(example_is_waiting(shown))
	{
		return FRAME_NOT_SHOWN;
	}
	if(qar_result_is_error(shown))
	{
		log_result("qar_render_sender_show_frame", shown);
		return FRAME_FAILED;
	}
	return FRAME_SHOWN;
}
//! [cpu_render_next_frame]

static void
teardown(QarRuntime* runtime, QarSession* session)
{
	if(session != NULL)
	{
		/* Destroying the handle leaves the session. */
		qar_session_handle_destroy(session);
	}
	qar_runtime_handle_destroy(runtime);
	example_unload_library();
}

int
main(int argc, char** argv)
{
	example_print_as_it_happens();
	//! [cpu_startup]
	static const char* const known_flags[] = { "--frames=", "--qaros-library=" };
	if(!example_arguments_are_known(
		   argc,
		   argv,
		   known_flags,
		   sizeof(known_flags) / sizeof(known_flags[0]),
		   "[pairing-code] [--frames=N] [--qaros-library=PATH]"
	   ))
	{
		return 1;
	}
	size_t frame_limit = 0;
	if(!example_frame_limit_argument(argc, argv, DEFAULT_FRAME_COUNT, &frame_limit))
	{
		fprintf(stderr, "--frames takes a whole number of frames, as in --frames=300\n");
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
		"cpu_rendering_visualizer.onboarding-id.txt",
		"CPU Renderer",
		&session
	);
	if(exit_code != 0)
	{
		teardown(runtime, session);
		return exit_code;
	}
	//! [cpu_startup]

	//! [cpu_volume]
	/* A render sender requires the app volume its stream is shown in. */
	QarAppVolumeInit volume_init = qar_app_volume_init_default();
	volume_init.common_name = "cpu-volume.examples.qaros";
	volume_init.display_name = "CPU Tutorial Volume";
	volume_init.pose = qar_pose_default();
	volume_init.pose.position.z = -1.0f;
	volume_init.size = qar_app_volume_size_default();

	QarAppVolumeId volume_id = qar_app_volume_id_default();
	QarResult volume_result = qar_app_volumes_get_or_create(session, &volume_init, &volume_id);
	if(qar_result_is_error(volume_result))
	{
		log_result("qar_app_volumes_get_or_create", volume_result);
		teardown(runtime, session);
		return 1;
	}
	//! [cpu_volume]

	//! [cpu_render_loop]
	/* Subscribe first, then list, so a target joining in between is not missed.
	 * The subscription needs a token: destroying it ends the subscription. */
	PeerWatch peer_watch = { 0 };
	QarCancelToken* peer_subscription = NULL;
	QarResult subscribed = qar_cancel_token_create(&peer_subscription);
	if(qar_result_is_success(subscribed))
	{
		subscribed =
			qar_peer_subscribe_updates(session, on_peer_update, &peer_watch, peer_subscription);
	}
	if(qar_result_is_error(subscribed))
	{
		log_result("qar_peer_subscribe_updates", subscribed);
		if(peer_subscription != NULL)
		{
			qar_cancel_token_handle_destroy(peer_subscription);
		}
		teardown(runtime, session);
		return 1;
	}

	TargetList targets = { 0 };
	const ULONGLONG started_ms = GetTickCount64();
	ULONGLONG last_listing_ms = 0;
	bool listed_once = false;
	size_t frames_shown = 0;
	printf("Looking for target apps in the peer list (open a visualizer or start a headset) ...\n");
	while(frames_shown < frame_limit)
	{
		const ULONGLONG now_ms = GetTickCount64();
		if(take_peer_update(&peer_watch) || !listed_once
		   || now_ms - last_listing_ms >= PEER_LIST_INTERVAL_MS)
		{
			listed_once = true;
			last_listing_ms = now_ms;
			if(!follow_targets(session, &volume_id, &targets))
			{
				exit_code = 1;
				break;
			}
		}

		const float seconds = (float)(now_ms - started_ms) / 1000.0f;
		bool showed_a_frame = false;
		size_t index = 0;
		while(index < targets.count)
		{
			const FrameOutcome outcome = render_next_frame(&targets.items[index], seconds);
			if(outcome == FRAME_FAILED)
			{
				printf("Stopped streaming to ");
				print_target_name(&targets.items[index]);
				printf("\n");
				remove_target(&targets, index);
				continue;
			}
			if(outcome == FRAME_SHOWN)
			{
				showed_a_frame = true;
				++frames_shown;
			}
			++index;
		}
		if(!showed_a_frame)
		{
			Sleep(IDLE_WAIT_MS);
		}
	}
	printf("Submitted %zu frames.\n", frames_shown);
	//! [cpu_render_loop]

	//! [cpu_teardown]
	while(targets.count > 0)
	{
		remove_target(&targets, targets.count - 1);
	}
	/* Ends the subscription and waits for a running callback: peer_watch is not
	 * written after this returns. */
	qar_cancel_token_handle_destroy(peer_subscription);
	teardown(runtime, session);
	return exit_code;
	//! [cpu_teardown]
}
