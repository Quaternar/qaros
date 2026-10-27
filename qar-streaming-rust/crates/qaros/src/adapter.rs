//! The GPU adapter a session's devices and frame textures are created on
//! (`QarGraphicsDeviceId`, chained into onboard and rejoin).

use std::fmt;

use qaros_sys::{self as sys, defaults};

use crate::error::{Error, Result};

const LUID_BYTES: usize = 8;
const UUID_BYTES: usize = 16;

/// A graphics adapter, by the id bytes Vulkan and DXGI report for it.
///
/// Windows takes the LUID, other platforms the UUID (header,
/// `QarGraphicsDeviceId`). Take both from `VkPhysicalDeviceIDProperties`
/// (`device_luid`, valid when `device_luid_valid`, and `device_uuid`) of the
/// physical device the application renders on, so frames need no cross-GPU
/// copy. The C API supports NVIDIA adapters only; another adapter fails with
/// [`crate::StatusCode::ARGUMENT_NOT_SUPPORTED`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GraphicsAdapter {
    /// `VkPhysicalDeviceIDProperties::deviceLUID`, bytes in memory order.
    Luid([u8; LUID_BYTES]),
    /// `VkPhysicalDeviceIDProperties::deviceUUID`.
    Uuid([u8; UUID_BYTES]),
}

impl GraphicsAdapter {
    /// Parses the hex of the id bytes, as [`fmt::Display`] writes it: 16 hex
    /// digits are a LUID, 32 a UUID. Upper-case digits are accepted.
    pub fn parse(text: &str) -> Result<Self> {
        let digits = text.trim();
        match digits.len() {
            16 => Ok(Self::Luid(parse_hex_bytes(digits)?)),
            32 => Ok(Self::Uuid(parse_hex_bytes(digits)?)),
            other => Err(Error::InvalidArgument(format!(
                "adapter id {digits:?} has {other} characters; expected 16 hex digits (LUID) or 32 (UUID)"
            ))),
        }
    }

    /// Whether this id names the physical device with these
    /// `VkPhysicalDeviceIDProperties`: `locally_unique_id` is `device_luid`
    /// when `device_luid_valid`, else `None`; `universally_unique_id` is
    /// `device_uuid`.
    pub fn matches(
        &self,
        locally_unique_id: Option<[u8; LUID_BYTES]>,
        universally_unique_id: [u8; UUID_BYTES],
    ) -> bool {
        match self {
            Self::Luid(luid) => locally_unique_id == Some(*luid),
            Self::Uuid(uuid) => *uuid == universally_unique_id,
        }
    }

    pub(crate) fn to_raw(self) -> sys::QarGraphicsDeviceId {
        let mut raw = defaults::qar_graphics_device_id_default();
        match self {
            Self::Luid(luid) => {
                raw.id_type = sys::QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_LUID;
                write_luid_bytes(&mut raw.luid, luid);
            }
            Self::Uuid(uuid) => {
                raw.id_type = sys::QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_UUID;
                raw.uuid = uuid;
            }
        }
        raw
    }

    /// Reads the id type the library wrote; any other type is an error
    /// rather than a guessed adapter.
    pub(crate) fn from_raw(raw: &sys::QarGraphicsDeviceId) -> Result<Self> {
        if raw.id_type == sys::QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_LUID {
            Ok(Self::Luid(read_luid_bytes(&raw.luid)))
        } else if raw.id_type == sys::QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_UUID {
            Ok(Self::Uuid(raw.uuid))
        } else {
            Err(Error::InvalidArgument(format!(
                "graphics device id has unknown id type {}",
                raw.id_type.0
            )))
        }
    }
}

/// Lowercase hex of the id bytes, the format [`GraphicsAdapter::parse`] reads.
impl fmt::Display for GraphicsAdapter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: &[u8] = match self {
            Self::Luid(luid) => luid,
            Self::Uuid(uuid) => uuid,
        };
        for byte in bytes {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

const _: () = assert!(size_of::<sys::QarLuid>() == LUID_BYTES);

/// `QarLuid` is the Windows `LUID` struct on Windows and an 8-byte array
/// elsewhere; the header's own example fills it with `memcpy` from
/// `deviceLUID`, so the bytes are copied in memory order on both.
fn write_luid_bytes(target: &mut sys::QarLuid, luid: [u8; LUID_BYTES]) {
    // SAFETY: QarLuid is a plain-old-data struct of exactly LUID_BYTES bytes
    // (asserted above), so every byte pattern is a valid value.
    unsafe {
        std::ptr::copy_nonoverlapping(
            luid.as_ptr(),
            std::ptr::from_mut(target).cast::<u8>(),
            LUID_BYTES,
        );
    }
}

fn read_luid_bytes(source: &sys::QarLuid) -> [u8; LUID_BYTES] {
    let mut luid = [0_u8; LUID_BYTES];
    // SAFETY: QarLuid is a plain-old-data struct of exactly LUID_BYTES bytes
    // (asserted above).
    unsafe {
        std::ptr::copy_nonoverlapping(
            std::ptr::from_ref(source).cast::<u8>(),
            luid.as_mut_ptr(),
            LUID_BYTES,
        );
    }
    luid
}

fn parse_hex_bytes<const COUNT: usize>(digits: &str) -> Result<[u8; COUNT]> {
    let mut bytes = [0_u8; COUNT];
    for (index, byte) in bytes.iter_mut().enumerate() {
        let pair = digits
            .get(index * 2..index * 2 + 2)
            .ok_or_else(|| Error::InvalidArgument(format!("adapter id {digits:?} is not hex")))?;
        *byte = u8::from_str_radix(pair, 16)
            .map_err(|_| Error::InvalidArgument(format!("adapter id {digits:?} is not hex")))?;
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_digits_parse_as_a_luid_in_byte_order() {
        let adapter = GraphicsAdapter::parse("a1b2000000000000");
        assert_eq!(
            adapter.ok(),
            Some(GraphicsAdapter::Luid([0xa1, 0xb2, 0, 0, 0, 0, 0, 0]))
        );
    }

    #[test]
    fn thirty_two_digits_parse_as_a_uuid_and_print_back_lowercase() {
        let text = "00112233445566778899AABBCCDDEEFF";
        let Ok(adapter) = GraphicsAdapter::parse(text) else {
            panic!("the uuid did not parse");
        };
        assert_eq!(
            adapter,
            GraphicsAdapter::Uuid([
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
                0xee, 0xff
            ])
        );
        assert_eq!(adapter.to_string(), text.to_lowercase());
    }

    #[test]
    fn other_lengths_and_non_hex_digits_are_refused() {
        assert!(GraphicsAdapter::parse("a1b2").is_err());
        assert!(GraphicsAdapter::parse("zz00000000000000").is_err());
        assert!(GraphicsAdapter::parse("é100000000000000").is_err());
    }

    #[test]
    fn a_luid_is_stamped_and_copied_into_the_raw_structure() {
        let raw = GraphicsAdapter::Luid([1, 2, 3, 4, 5, 6, 7, 8]).to_raw();
        assert_eq!(
            raw.header.type_,
            sys::QarStructureType::QAR_STRUCTURE_TYPE_SESSION_GRAPHICS_DEVICE_ID
        );
        assert_eq!(
            raw.id_type,
            sys::QarGpuDeviceIdType::QAR_GPU_DEVICE_ID_TYPE_LUID
        );
        // SAFETY: QarLuid is 8 plain bytes.
        let bytes: [u8; 8] = unsafe { std::mem::transmute_copy(&raw.luid) };
        assert_eq!(bytes, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn matching_compares_the_id_kind_it_holds() {
        let uuid = [7; 16];
        assert!(GraphicsAdapter::Uuid(uuid).matches(None, uuid));
        assert!(!GraphicsAdapter::Luid([7; 8]).matches(None, uuid));
        assert!(GraphicsAdapter::Luid([7; 8]).matches(Some([7; 8]), uuid));
    }
}
