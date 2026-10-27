//! A CPU-rendered QAROS application after `cpu_rendering_visualizer.c`, following the header's
//! "Basic flow": it finds its targets in the peer list (running peers for which
//! `qar_peer_spec_is_target_app` holds), creates one render sender per target in its own app
//! volume with declared frame views, and renders a lit sphere at the volume's center for each
//! target from that frame's own eye poses and fields of view, with depth.
//!
//! ```text
//! cargo run -p qaros-examples --bin cpu_rendering_visualizer -- [pairing-code] [--frames=300]
//! ```
//!
//! It starts from a console. With a stored onboarding id it rejoins; with a pairing code it
//! onboards and stores the id, as the C sample does; with neither, on Windows, it joins the QAROS
//! on this PC as a local app (Allow once in QAROS, nothing stored), which the C sample, onboarding
//! by code only, does not. It ends after `--frames` frames.

use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};

use qaros::{
    AppVolumeConfig, AppVolumeId, AppVolumeSize, BlockingOptions, Eye, FrameLayout, FrameViewSpec,
    Library, PeerId, PeerInfo, PixelFormat, Pose, Quat, RenderSender, RenderSenderConfig, Session,
    ShowParams, Vec3, VideoFrameView, ViewKind,
};
use qaros_examples::{
    Arguments, ExampleResult, create_runtime, frame_limit, is_running_target, is_waiting,
    join_session, load_library, report,
};

const DEFAULT_FRAME_COUNT: usize = 300;
/// Near and far planes in app units, the units of the poses a sender bound to an app volume
/// hands out (`QarNearFar`).
const NEAR_PLANE: f32 = 0.1;
const FAR_PLANE: f32 = 10.0;
const SPHERE_RADIUS: f32 = 0.25;
/// How often the peer list is read without a peer update.
const PEER_LIST_INTERVAL: Duration = Duration::from_secs(2);
/// How long creating a sender may block before the target is tried again later.
const SENDER_CREATION_TIMEOUT: Duration = Duration::from_secs(10);
const IDLE_WAIT: Duration = Duration::from_millis(2);

// ANCHOR: frame_views
/// Color of both eyes side by side in texture 0, depth side by side in texture 1. View 0 and
/// view 1 are the left and the right eye for poses and fields of view.
const FRAME_VIEWS: [FrameViewSpec; 4] = [
    FrameViewSpec {
        kind: ViewKind::Color,
        eye: Eye::Left,
        texture_index: 0,
    },
    FrameViewSpec {
        kind: ViewKind::Color,
        eye: Eye::Right,
        texture_index: 0,
    },
    FrameViewSpec {
        kind: ViewKind::Depth,
        eye: Eye::Left,
        texture_index: 1,
    },
    FrameViewSpec {
        kind: ViewKind::Depth,
        eye: Eye::Right,
        texture_index: 1,
    },
];
// ANCHOR_END: frame_views

fn main() -> ExitCode {
    report(run())
}

fn run() -> ExampleResult {
    // ANCHOR: session
    let arguments = Arguments::parse();
    let frame_limit = frame_limit(&arguments, DEFAULT_FRAME_COUNT)?;

    let library = load_library()?;
    let runtime = create_runtime(&library, None)?;
    let session = join_session(
        &library,
        &runtime,
        arguments.pairing_code.as_deref(),
        Path::new("cpu_rendering_visualizer.onboarding-id.txt"),
        "CPU Renderer",
    )?;
    // ANCHOR_END: session

    // ANCHOR: volume
    // A render sender requires the app volume its stream is shown in.
    let mut volume_config = AppVolumeConfig::new("cpu-volume.examples.qaros");
    volume_config.display_name = Some("CPU Tutorial Volume".to_string());
    volume_config.pose = Pose::from_position(Vec3::new(0.0, 0.0, -1.0));
    volume_config.size = AppVolumeSize {
        width: 1.0,
        length: 1.0,
        height: 1.0,
    };
    let volume = session.app_volumes().get_or_create(&volume_config)?;
    // ANCHOR_END: volume

    // ANCHOR: render_loop
    // Subscribe first, then list, so a target joining in between is not missed.
    let (forward, peer_updates) = qaros::channel();
    let _peer_subscription = session.subscribe_peer_updates(forward)?;
    let started = Instant::now();
    let mut targets: Vec<Target> = Vec::new();
    let mut last_listing: Option<Instant> = None;
    let mut frames_shown = 0_usize;
    println!("Looking for target apps in the peer list (open a visualizer or start a headset) ...");

    while frames_shown < frame_limit {
        let updates_arrived = peer_updates.try_iter().count() > 0;
        if updates_arrived || last_listing.is_none_or(|last| last.elapsed() >= PEER_LIST_INTERVAL) {
            last_listing = Some(Instant::now());
            follow_targets(&library, &session, volume, &session.peers()?, &mut targets);
        }
        let mut showed_a_frame = false;
        targets.retain_mut(|target| match render_next_frame(target, started) {
            Ok(showed) => {
                showed_a_frame |= showed;
                frames_shown += usize::from(showed);
                true
            }
            Err(error) => {
                println!("Stopped streaming to {}: {error}", target.label);
                false
            }
        });
        if !showed_a_frame {
            std::thread::sleep(IDLE_WAIT);
        }
    }
    println!("Submitted {frames_shown} frames.");
    Ok(())
}
// ANCHOR_END: render_loop

// ANCHOR: follow_targets
/// One target app and the sender rendering for it.
struct Target {
    peer: PeerId,
    label: String,
    sender: RenderSender,
}

/// Creates a sender for every running target app the list shows and drops the senders of
/// targets it no longer shows running.
fn follow_targets(
    library: &Library,
    session: &Session,
    volume: AppVolumeId,
    listed: &[PeerInfo],
    targets: &mut Vec<Target>,
) {
    targets.retain(|target| {
        let still_running = listed
            .iter()
            .any(|peer| peer.id == target.peer && is_running_target(peer));
        if !still_running {
            println!("{} left", target.label);
        }
        still_running
    });
    for peer in listed.iter().filter(|peer| is_running_target(peer)) {
        if targets.iter().any(|target| target.peer == peer.id) {
            continue;
        }
        let label = if peer.display_name.is_empty() {
            peer.id.to_string()
        } else {
            peer.display_name.clone()
        };
        match create_sender(library, session, volume, peer.id) {
            Ok(sender) => {
                println!("Rendering for {label} (peer {})", peer.id);
                targets.push(Target {
                    peer: peer.id,
                    label,
                    sender,
                });
            }
            Err(error) => println!("No sender for {label} yet: {error}"),
        }
    }
}
// ANCHOR_END: follow_targets

// ANCHOR: create_sender
fn create_sender(
    library: &Library,
    session: &Session,
    volume: AppVolumeId,
    peer: PeerId,
) -> qaros::Result<RenderSender> {
    let config = RenderSenderConfig {
        views: FRAME_VIEWS.to_vec(),
        app_volume: Some(volume),
        color_format: PixelFormat::R8G8B8A8,
        depth_format: PixelFormat::D32Float,
        layout: FrameLayout::SideBySide,
        ..RenderSenderConfig::new(peer)
    };
    let timeout = library.create_cancel_token(Some(SENDER_CREATION_TIMEOUT))?;
    session.create_render_sender(
        &config,
        &BlockingOptions {
            cancel: Some(&timeout),
            ..BlockingOptions::default()
        },
    )
}
// ANCHOR_END: create_sender

// ANCHOR: render_next_frame
/// Renders and shows the target's next frame when its sender has one; false while none is
/// ready or the stream is waiting.
fn render_next_frame(target: &mut Target, started: Instant) -> qaros::Result<bool> {
    let mut frame = match target.sender.try_begin_owned_frame() {
        Ok(Some(frame)) => frame,
        Ok(None) => return Ok(false),
        Err(error) if is_waiting(&error) => return Ok(false),
        Err(error) => return Err(error),
    };
    let eyes = [frame.view(0)?, frame.view(1)?];
    let seconds = started.elapsed().as_secs_f32();
    let sphere_center = Vec3::new(0.0, 0.05 * (seconds * 1.5).sin(), 0.0);
    {
        let cpu_frame = frame.cpu_frame()?;
        let mut textures = cpu_frame.textures;
        for view in &cpu_frame.views {
            let eye_index = usize::from(view.eye == Eye::Right);
            let Some(texture) = textures.get_mut(view.texture_index as usize) else {
                continue;
            };
            let camera = EyeRay::new(eyes[eye_index].pose, eyes[eye_index].fov);
            render_view(
                texture.data,
                texture.pitch as usize,
                view,
                &camera,
                sphere_center,
                seconds,
            );
        }
    }
    match frame.show(&ShowParams {
        near: NEAR_PLANE,
        far: FAR_PLANE,
        depth_scale: 1.0,
    }) {
        Ok(()) => Ok(true),
        Err(error) if is_waiting(&error) => Ok(false),
        Err(error) => Err(error),
    }
}
// ANCHOR_END: render_next_frame

// ANCHOR: eye_ray
/// Rays through an eye's pixels, in app content space.
struct EyeRay {
    origin: Vec3,
    orientation: Quat,
    tangents: [f32; 4],
}

impl EyeRay {
    fn new(pose: Pose, fov: qaros::Fov) -> Self {
        Self {
            origin: pose.position,
            orientation: pose.orientation,
            tangents: [
                fov.angle_left.tan(),
                fov.angle_right.tan(),
                fov.angle_up.tan(),
                fov.angle_down.tan(),
            ],
        }
    }

    /// The direction through view-local pixel (`u`, `v`) of a `width` x `height` view, scaled
    /// so that its view-space Z is -1: the ray parameter at a hit is then the hit's depth
    /// along the optical axis.
    fn direction(&self, u: f32, v: f32, width: f32, height: f32) -> Vec3 {
        let [left, right, up, down] = self.tangents;
        let x = left + (u + 0.5) / width * (right - left);
        let y = up - (v + 0.5) / height * (up - down);
        rotate(self.orientation, Vec3::new(x, y, -1.0))
    }
}
// ANCHOR_END: eye_ray

/// Lambert-shaded sphere color and its depth along the optical axis, or `None` on a miss.
fn trace_sphere(
    origin: Vec3,
    direction: Vec3,
    center: Vec3,
    seconds: f32,
) -> Option<([u8; 4], f32)> {
    let offset = subtract(origin, center);
    let a = dot(direction, direction);
    let half_b = dot(offset, direction);
    let c = dot(offset, offset) - SPHERE_RADIUS * SPHERE_RADIUS;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return None;
    }
    let distance = (-half_b - discriminant.sqrt()) / a;
    if distance <= NEAR_PLANE || distance >= FAR_PLANE {
        return None;
    }
    let hit = add(origin, scale(direction, distance));
    let normal = scale(subtract(hit, center), 1.0 / SPHERE_RADIUS);
    let light = normalize(Vec3::new(0.4, 0.8, 0.5));
    let brightness = 0.25 + 0.75 * dot(normal, light).max(0.0);
    let hue = seconds * 0.4;
    let base = [
        0.55 + 0.45 * hue.sin(),
        0.55 + 0.45 * (hue + 2.1).sin(),
        0.55 + 0.45 * (hue + 4.2).sin(),
    ];
    let channel = |value: f32| {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let byte = (value * brightness * 255.0).clamp(0.0, 255.0) as u8;
        byte
    };
    Some((
        [channel(base[0]), channel(base[1]), channel(base[2]), 255],
        distance,
    ))
}

// ANCHOR: forward_depth
/// D3D/OpenXR-style forward depth: 0 at near, 1 at far.
fn forward_depth(distance: f32) -> f32 {
    FAR_PLANE * (distance - NEAR_PLANE) / (distance * (FAR_PLANE - NEAR_PLANE))
}
// ANCHOR_END: forward_depth

/// Writes one view's rectangle, in bands of rows on all cores: color views get the sphere on
/// transparent black, depth views its depth on the far plane. A view whose start lies past its
/// end is mirrored.
fn render_view(
    data: &mut [u8],
    pitch: usize,
    view: &VideoFrameView,
    camera: &EyeRay,
    sphere_center: Vec3,
    seconds: f32,
) {
    let (row_start, row_end) = (view.start_y.min(view.end_y), view.start_y.max(view.end_y));
    let first_byte = (row_start as usize * pitch).min(data.len());
    let end_byte = (row_end as usize * pitch).min(data.len());
    let thread_count = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let rows_per_band = ((row_end - row_start) as usize)
        .div_ceil(thread_count)
        .max(1);
    std::thread::scope(|scope| {
        for (band_index, band) in data[first_byte..end_byte]
            .chunks_mut(rows_per_band * pitch)
            .enumerate()
        {
            let band_first_row = row_start as usize + band_index * rows_per_band;
            scope.spawn(move || {
                render_band(
                    band,
                    band_first_row,
                    pitch,
                    view,
                    camera,
                    sphere_center,
                    seconds,
                );
            });
        }
    });
}

/// Renders the rows of `band`, whose first row is `band_first_row` of the texture.
fn render_band(
    band: &mut [u8],
    band_first_row: usize,
    pitch: usize,
    view: &VideoFrameView,
    camera: &EyeRay,
    sphere_center: Vec3,
    seconds: f32,
) {
    let (column_start, column_end) = (view.start_x.min(view.end_x), view.start_x.max(view.end_x));
    let (row_start, row_end) = (
        view.start_y.min(view.end_y) as usize,
        view.start_y.max(view.end_y) as usize,
    );
    #[allow(clippy::cast_precision_loss)]
    let (width, height) = (
        (column_end - column_start) as f32,
        (row_end - row_start) as f32,
    );
    for (row_in_band, row_bytes) in band.chunks_mut(pitch).enumerate() {
        let row = band_first_row + row_in_band;
        let local_row = if view.start_y <= view.end_y {
            row - row_start
        } else {
            row_end - 1 - row
        };
        for column in column_start..column_end {
            let local_column = if view.start_x <= view.end_x {
                column - column_start
            } else {
                column_end - 1 - column
            };
            #[allow(clippy::cast_precision_loss)]
            let direction = camera.direction(local_column as f32, local_row as f32, width, height);
            let hit = trace_sphere(camera.origin, direction, sphere_center, seconds);
            let offset = column as usize * 4;
            let Some(pixel) = row_bytes.get_mut(offset..offset + 4) else {
                continue;
            };
            write_pixel(pixel, view, hit);
        }
    }
}

// ANCHOR: write_pixel
fn write_pixel(pixel: &mut [u8], view: &VideoFrameView, hit: Option<([u8; 4], f32)>) {
    match view.format {
        PixelFormat::D32Float | PixelFormat::R32Float => {
            let depth = hit.map_or(1.0, |(_, distance)| forward_depth(distance));
            pixel.copy_from_slice(&depth.to_ne_bytes());
        }
        PixelFormat::R8G8B8A8 => {
            pixel.copy_from_slice(&hit.map_or([0; 4], |(color, _)| color));
        }
        PixelFormat::B8G8R8A8 => {
            let [red, green, blue, alpha] = hit.map_or([0; 4], |(color, _)| color);
            pixel.copy_from_slice(&[blue, green, red, alpha]);
        }
        other => log::warn!("leaving a {other:?} view untouched"),
    }
}
// ANCHOR_END: write_pixel

fn add(left: Vec3, right: Vec3) -> Vec3 {
    Vec3::new(left.x + right.x, left.y + right.y, left.z + right.z)
}

fn subtract(left: Vec3, right: Vec3) -> Vec3 {
    Vec3::new(left.x - right.x, left.y - right.y, left.z - right.z)
}

fn scale(vector: Vec3, factor: f32) -> Vec3 {
    Vec3::new(vector.x * factor, vector.y * factor, vector.z * factor)
}

fn dot(left: Vec3, right: Vec3) -> f32 {
    left.x * right.x + left.y * right.y + left.z * right.z
}

fn cross(left: Vec3, right: Vec3) -> Vec3 {
    Vec3::new(
        left.y * right.z - left.z * right.y,
        left.z * right.x - left.x * right.z,
        left.x * right.y - left.y * right.x,
    )
}

fn normalize(vector: Vec3) -> Vec3 {
    scale(vector, 1.0 / dot(vector, vector).sqrt())
}

/// `vector` rotated by the unit quaternion `rotation`.
fn rotate(rotation: Quat, vector: Vec3) -> Vec3 {
    let axis = Vec3::new(rotation.x, rotation.y, rotation.z);
    let twice_cross = scale(cross(axis, vector), 2.0);
    add(
        add(vector, scale(twice_cross, rotation.w)),
        cross(axis, twice_cross),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_center_pixel_of_an_eye_looking_at_the_sphere_hits_it_at_the_right_depth() {
        let eye = EyeRay::new(
            Pose::from_position(Vec3::new(0.0, 0.0, 1.0)),
            qaros::Fov {
                angle_left: -0.7,
                angle_right: 0.7,
                angle_up: 0.7,
                angle_down: -0.7,
            },
        );
        let direction = eye.direction(49.5, 49.5, 100.0, 100.0);
        let Some((color, distance)) =
            trace_sphere(eye.origin, direction, Vec3::new(0.0, 0.0, 0.0), 0.0)
        else {
            panic!("the center ray misses the sphere");
        };
        assert!(
            (distance - (1.0 - SPHERE_RADIUS)).abs() < 1e-4,
            "{distance}"
        );
        assert_eq!(color[3], 255);
        let corner = eye.direction(0.0, 0.0, 100.0, 100.0);
        assert!(trace_sphere(eye.origin, corner, Vec3::new(0.0, 0.0, 0.0), 0.0).is_none());
    }

    #[test]
    #[allow(clippy::cast_precision_loss)]
    fn banded_rendering_writes_every_texel_of_a_mirrored_right_eye_like_a_single_pass() {
        let (eye_width, height, padding) = (40_u32, 31_u32, 16_usize);
        let pitch = (2 * eye_width) as usize * 4 + padding;
        let mut data = vec![0xAB_u8; pitch * height as usize];
        let view = VideoFrameView {
            start_x: eye_width,
            start_y: height,
            end_x: 2 * eye_width,
            end_y: 0,
            texture_index: 0,
            array_layer_index: 0,
            format: PixelFormat::R8G8B8A8,
            kind: ViewKind::Color,
            eye: Eye::Right,
        };
        let eye = EyeRay::new(
            Pose::from_position(Vec3::new(0.0, 0.0, 1.0)),
            qaros::Fov {
                angle_left: -0.5,
                angle_right: 0.5,
                angle_up: 0.4,
                angle_down: -0.4,
            },
        );
        let center = Vec3::new(0.0, 0.0, 0.0);
        render_view(&mut data, pitch, &view, &eye, center, 1.0);

        let mut sphere_texels = 0;
        for row in 0..height as usize {
            let local_row = height as usize - 1 - row;
            for local_column in 0..eye_width as usize {
                let direction = eye.direction(
                    local_column as f32,
                    local_row as f32,
                    eye_width as f32,
                    height as f32,
                );
                let expected = trace_sphere(eye.origin, direction, center, 1.0)
                    .map_or([0; 4], |(color, _)| color);
                sphere_texels += usize::from(expected[3] == 255);
                let offset = row * pitch + (eye_width as usize + local_column) * 4;
                assert_eq!(
                    data[offset..offset + 4],
                    expected,
                    "row {row} column {local_column}"
                );
            }
            let left_eye_and_padding_untouched = data
                [row * pitch..row * pitch + eye_width as usize * 4]
                .iter()
                .chain(&data[row * pitch + pitch - padding..row * pitch + pitch])
                .all(|byte| *byte == 0xAB);
            assert!(left_eye_and_padding_untouched, "row {row}");
        }
        assert!(sphere_texels > 0);
    }

    #[test]
    fn forward_depth_spans_near_to_far() {
        assert!(forward_depth(NEAR_PLANE).abs() < 1e-6);
        assert!((forward_depth(FAR_PLANE) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rotation_by_a_quarter_turn_about_y_maps_minus_z_to_minus_x() {
        let half = std::f32::consts::FRAC_PI_4;
        let quarter_turn = Quat {
            x: 0.0,
            y: half.sin(),
            z: 0.0,
            w: half.cos(),
        };
        let rotated = rotate(quarter_turn, Vec3::new(0.0, 0.0, -1.0));
        assert!(
            (rotated.x + 1.0).abs() < 1e-6 && rotated.z.abs() < 1e-6,
            "{rotated:?}"
        );
    }
}
