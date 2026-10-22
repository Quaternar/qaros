//! The machine-readable strip of a frame stamp that a renderer draws into each eye, so the stub
//! can check which frame a shown image was rendered for. A viewer that streams one sender per
//! user draws it to prove each user receives the frames issued for them.
//!
//! Format (a renderer that draws the stamp must match it):
//! - a 32-bit word, `0xA` in the top four bits, the eye in bit 27 (1 = right), the low 27 bits
//!   of the frame id below;
//! - drawn at the eye's top-left corner as 32 cells, most significant bit first, each cell
//!   `cell_size` pixels wide and `2 * cell_size` tall, white for 1 and black for 0;
//! - `cell_size` is the eye width divided by 64, at least 1.

pub const MAGIC: u32 = 0xA;
pub const FRAME_ID_BITS: u32 = 27;
pub const WORD_BITS: u32 = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stamp {
    /// The low 27 bits of the frame id.
    pub frame_id: u32,
    pub right_eye: bool,
}

pub fn cell_size(view_width: u32) -> u32 {
    (view_width / 64).max(1)
}

pub fn frame_id_bits(frame_id: u64) -> u32 {
    let mask = (1_u64 << FRAME_ID_BITS) - 1;
    u32::try_from(frame_id & mask).unwrap_or(0)
}

pub fn encode(stamp: Stamp) -> u32 {
    (MAGIC << 28)
        | (u32::from(stamp.right_eye) << FRAME_ID_BITS)
        | frame_id_bits(stamp.frame_id.into())
}

pub fn decode(word: u32) -> Option<Stamp> {
    (word >> 28 == MAGIC).then_some(Stamp {
        frame_id: word & ((1 << FRAME_ID_BITS) - 1),
        right_eye: (word >> FRAME_ID_BITS) & 1 == 1,
    })
}

/// A view inside a CPU texture: rows `pitch` bytes apart, `bytes_per_pixel` per pixel.
#[derive(Clone, Copy, Debug)]
pub struct ViewArea {
    pub start_x: u32,
    pub start_y: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: usize,
    pub bytes_per_pixel: usize,
}

impl ViewArea {
    fn offset(&self, x: u32, y: u32) -> usize {
        (self.start_y + y) as usize * self.pitch
            + (self.start_x + x) as usize * self.bytes_per_pixel
    }

    fn cell_center(&self, bit: u32) -> (u32, u32) {
        let cell = cell_size(self.width);
        (bit * cell + cell / 2, cell)
    }
}

/// Reads the strip from the first channel of each cell's center pixel (gray, so the channel
/// order does not matter); `None` when the view is too small or holds no stamp.
pub fn read(texture: &[u8], area: ViewArea) -> Option<Stamp> {
    let cell = cell_size(area.width);
    if area.width < WORD_BITS * cell || area.height < 2 * cell {
        return None;
    }
    let mut word = 0_u32;
    for bit in 0..WORD_BITS {
        let (x, y) = area.cell_center(bit);
        let value = *texture.get(area.offset(x, y))?;
        word = (word << 1) | u32::from(value >= 128);
    }
    decode(word)
}

/// Writes the strip into an RGBA or BGRA texture; tests use it to stand in for the renderer.
pub fn write(texture: &mut [u8], area: ViewArea, stamp: Stamp) {
    let cell = cell_size(area.width);
    let word = encode(stamp);
    for bit in 0..WORD_BITS.min(area.width / cell) {
        let level = if (word >> (WORD_BITS - 1 - bit)) & 1 == 1 {
            255
        } else {
            0
        };
        for y in 0..(2 * cell).min(area.height) {
            for x in bit * cell..(bit + 1) * cell {
                let offset = area.offset(x, y);
                if let Some(pixel) = texture.get_mut(offset..offset + area.bytes_per_pixel) {
                    pixel.fill(level);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_round_trips_through_a_texture() {
        let area = ViewArea {
            start_x: 64,
            start_y: 0,
            width: 64,
            height: 64,
            pitch: 128 * 4,
            bytes_per_pixel: 4,
        };
        let mut texture = vec![0x40; 128 * 64 * 4];
        let stamp = Stamp {
            frame_id: 123_456,
            right_eye: true,
        };
        write(&mut texture, area, stamp);
        assert_eq!(read(&texture, area), Some(stamp));
        let left_view = ViewArea { start_x: 0, ..area };
        assert_eq!(read(&texture, left_view), None, "no magic, no stamp");
    }

    #[test]
    fn frame_ids_wrap_at_27_bits() {
        assert_eq!(frame_id_bits((1 << 27) + 5), 5);
        let word = encode(Stamp {
            frame_id: 5,
            right_eye: false,
        });
        assert_eq!(word >> 28, MAGIC);
        assert_eq!(
            decode(word),
            Some(Stamp {
                frame_id: 5,
                right_eye: false
            })
        );
    }
}
