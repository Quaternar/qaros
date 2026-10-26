//! The known picture the sender writes and the receiver checks: every color
//! view a flat color that names its eye and the frame, every depth view a
//! constant per eye. Flat areas survive any codec, so the check samples
//! points inside each view and reports the largest channel error.

#![allow(dead_code)]

use qaros::{CpuFrame, Eye, PixelFormat, ViewKind};
use qaros_sys::{
    QarPixelFormat, QarTextureSize, QarVideoFrameLayout, QarVideoFrameView, QarVideoFrameViewEye,
    QarVideoFrameViewType,
};

/// Largest accepted difference per 8-bit channel after the transfer.
pub const COLOR_TOLERANCE: u8 = 12;
pub const DEPTH_TOLERANCE: f32 = 0.002;

/// RGB of a color view: red-ish left, green-ish right, blue counts frames.
pub fn expected_rgb(left_eye: bool, frame_number: u64) -> [u8; 3] {
    let frame_channel = u8::try_from(32 + (frame_number * 41) % 192).unwrap_or(0);
    if left_eye {
        [220, 40, frame_channel]
    } else {
        [40, 220, frame_channel]
    }
}

pub fn expected_depth(left_eye: bool) -> f32 {
    if left_eye { 0.25 } else { 0.75 }
}

pub fn texel_bytes(format: QarPixelFormat, rgb: [u8; 3], depth: f32) -> Vec<u8> {
    match format {
        QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8A8 | QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8X8 => {
            vec![rgb[2], rgb[1], rgb[0], 255]
        }
        QarPixelFormat::QAR_PIXEL_FORMAT_R8G8B8A8 => vec![rgb[0], rgb[1], rgb[2], 255],
        QarPixelFormat::QAR_PIXEL_FORMAT_D32_FLOAT | QarPixelFormat::QAR_PIXEL_FORMAT_R32_FLOAT => {
            depth.to_le_bytes().to_vec()
        }
        other => panic!("the pattern has no texel for pixel format {}", other.0),
    }
}

pub fn raw_format(format: PixelFormat) -> QarPixelFormat {
    match format {
        PixelFormat::R32Float => QarPixelFormat::QAR_PIXEL_FORMAT_R32_FLOAT,
        PixelFormat::R8G8B8A8 => QarPixelFormat::QAR_PIXEL_FORMAT_R8G8B8A8,
        PixelFormat::B8G8R8A8 => QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8A8,
        PixelFormat::R16G16B16A16 => QarPixelFormat::QAR_PIXEL_FORMAT_R16G16B16A16,
        PixelFormat::B8G8R8X8 => QarPixelFormat::QAR_PIXEL_FORMAT_B8G8R8X8,
        PixelFormat::D32Float => QarPixelFormat::QAR_PIXEL_FORMAT_D32_FLOAT,
        // By value, as the safe layer names them: only the public header declares them.
        PixelFormat::D16Unorm => QarPixelFormat(100),
        PixelFormat::D24UnormS8Uint => QarPixelFormat(102),
        PixelFormat::D32FloatS8X24Uint => QarPixelFormat(103),
        PixelFormat::Other(raw) => raw,
    }
}

/// Writes the pattern into every view of a CPU frame the sender handed out.
pub fn fill_cpu_frame(frame: &mut CpuFrame<'_>, frame_number: u64) {
    for view in &frame.views {
        let left_eye = view.eye != Eye::Right;
        let texture = &mut frame.textures[view.texture_index as usize];
        let format = raw_format(texture.size.format);
        let texel = match view.kind {
            ViewKind::Depth => texel_bytes(format, [0; 3], expected_depth(left_eye)),
            _ => texel_bytes(format, expected_rgb(left_eye, frame_number), 0.0),
        };
        let rectangle = (
            ordered(view.start_x, view.end_x),
            ordered(view.start_y, view.end_y),
        );
        fill_rectangle(texture.data, texture.pitch as usize, rectangle, &texel);
    }
}

/// Writes the pattern into tightly packed texture buffers, one per texture,
/// for the given views (the Vulkan path uploads these).
pub fn packed_textures(
    frame_views: &[QarVideoFrameView],
    texture_sizes: &[QarTextureSize],
    frame_number: u64,
) -> Vec<Vec<u8>> {
    let mut buffers: Vec<Vec<u8>> = texture_sizes
        .iter()
        .map(|size| {
            vec![0_u8; size.width as usize * size.height as usize * texel_size(size.format)]
        })
        .collect();
    for view in frame_views {
        let left_eye = view.eye != QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_RIGHT;
        let size = texture_sizes[view.texture_index as usize];
        let texel = if view.data_type == QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH {
            texel_bytes(size.format, [0; 3], expected_depth(left_eye))
        } else {
            texel_bytes(size.format, expected_rgb(left_eye, frame_number), 0.0)
        };
        let pitch = size.width as usize * texel.len();
        let rectangle = (
            ordered(view.start_x, view.end_x),
            ordered(view.start_y, view.end_y),
        );
        fill_rectangle(
            &mut buffers[view.texture_index as usize],
            pitch,
            rectangle,
            &texel,
        );
    }
    buffers
}

/// Fills a rectangle (columns, rows) with one texel: one row is built once and
/// copied into every row, which keeps a debug build fast enough.
fn fill_rectangle(
    texture: &mut [u8],
    pitch: usize,
    ((x_start, x_end), (y_start, y_end)): ((usize, usize), (usize, usize)),
    texel: &[u8],
) {
    let row_bytes: Vec<u8> = texel
        .iter()
        .copied()
        .cycle()
        .take((x_end - x_start) * texel.len())
        .collect();
    for row in y_start..y_end {
        let offset = row * pitch + x_start * texel.len();
        texture[offset..offset + row_bytes.len()].copy_from_slice(&row_bytes);
    }
}

fn texel_size(format: QarPixelFormat) -> usize {
    match format {
        QarPixelFormat::QAR_PIXEL_FORMAT_R16G16B16A16 => 8,
        _ => 4,
    }
}

fn ordered(start: u32, end: u32) -> (usize, usize) {
    (start.min(end) as usize, start.max(end) as usize)
}

pub fn views(layout: &QarVideoFrameLayout) -> &[QarVideoFrameView] {
    &layout.views[..layout.views_count.min(layout.views.len())]
}

/// The outcome of checking one received frame.
#[derive(Debug, Default)]
pub struct Verdict {
    pub mismatches: Vec<String>,
    pub largest_color_error: u8,
    pub largest_depth_error: f32,
    pub color_samples: usize,
    pub depth_samples: usize,
}

/// Samples nine points inside every view of a received frame, given each
/// texture's bytes and pitch by texture index, and compares them with the pattern.
pub fn check_frame(
    layout: &QarVideoFrameLayout,
    textures: &[(&[u8], usize)],
    frame_number: u64,
) -> Verdict {
    let mut verdict = Verdict::default();
    for view in views(layout) {
        let left_eye = view.eye != QarVideoFrameViewEye::QAR_VIDEO_FRAME_VIEW_EYE_RIGHT;
        let texture_index = view.texture_index as usize;
        let Some(&(bytes, pitch)) = textures.get(texture_index) else {
            verdict
                .mismatches
                .push(format!("texture {texture_index} missing"));
            continue;
        };
        let format = layout.textures[texture_index].format;
        let (x_start, x_end) = ordered(view.start_x, view.end_x);
        let (y_start, y_end) = ordered(view.start_y, view.end_y);
        for (column, row) in sample_points(x_start, x_end, y_start, y_end) {
            let offset = row * pitch + column * texel_size(format);
            let Some(texel) = bytes.get(offset..offset + 4) else {
                verdict.mismatches.push(format!(
                    "texture {texture_index} too small for ({column}, {row})"
                ));
                continue;
            };
            if view.data_type == QarVideoFrameViewType::QAR_VIDEO_FRAME_VIEW_TYPE_DEPTH {
                let depth = f32::from_le_bytes([texel[0], texel[1], texel[2], texel[3]]);
                let error = (depth - expected_depth(left_eye)).abs();
                verdict.largest_depth_error = verdict.largest_depth_error.max(error);
                verdict.depth_samples += 1;
                if error > DEPTH_TOLERANCE {
                    verdict.mismatches.push(format!(
                        "depth view eye {} at ({column}, {row}): {depth}, expected {}",
                        view.eye.0,
                        expected_depth(left_eye)
                    ));
                }
            } else {
                let rgb = match format {
                    QarPixelFormat::QAR_PIXEL_FORMAT_R8G8B8A8 => [texel[0], texel[1], texel[2]],
                    _ => [texel[2], texel[1], texel[0]],
                };
                let expected = expected_rgb(left_eye, frame_number);
                let error = rgb
                    .iter()
                    .zip(expected)
                    .map(|(actual, wanted)| actual.abs_diff(wanted))
                    .max()
                    .unwrap_or(0);
                verdict.largest_color_error = verdict.largest_color_error.max(error);
                verdict.color_samples += 1;
                if error > COLOR_TOLERANCE {
                    verdict.mismatches.push(format!(
                        "color view eye {} at ({column}, {row}): {rgb:?}, expected {expected:?}",
                        view.eye.0
                    ));
                }
            }
        }
    }
    verdict
}

fn sample_points(
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
) -> Vec<(usize, usize)> {
    let width = x_end - x_start;
    let height = y_end - y_start;
    let mut points = Vec::new();
    for x_fraction in [1, 2, 3] {
        for y_fraction in [1, 2, 3] {
            points.push((
                x_start + width * x_fraction / 4,
                y_start + height * y_fraction / 4,
            ));
        }
    }
    points
}
