//! The D3D11 render sender, emulated without Direct3D.
//!
//! The real runtime adopts the application's `ID3D11Device1` and immediate context, creates
//! shared textures with keyed mutexes on that device, and hands them out per frame through
//! `QarVideoFrameD3D11`. The stub creates no D3D11 object: it cannot on macOS, and on Windows it
//! keeps to the same emulation so the tests mean the same on both. It checks the chained
//! `QarStreamParamsD3D11` (stamp, end of chain, device and context present), reports the
//! parameters it was given in the call log, and stands in a heap allocation for every shared
//! texture and keyed mutex. Those stand-ins are not COM objects: a caller that calls a method
//! on one, or releases it, crashes, so the tests read their addresses and nothing else.
//!
//! Keyed mutexes: with `acquire_keyed_mutex_sync` true the record carries no synchronization
//! entry (the library drives the keyed mutexes); with it false, one entry per texture whose
//! keys change with every frame. The keys are the stub's own and say nothing about the keys
//! QAROS uses; they show that the reader copies what the record carries, frame by frame.

#[allow(clippy::wildcard_imports)]
use qaros_sys::*;

/// What stands in for a shared texture or its keyed mutex. Not zero-sized, so each one has an
/// address of its own.
struct StandIn {
    _marker: u64,
}

/// The stand-ins live in vectors that never grow after creation, so their addresses hold for
/// the sender's life.
pub struct D3D11Sender {
    acquire_keyed_mutex_sync: bool,
    textures: Vec<StandIn>,
    keyed_mutexes: Vec<StandIn>,
}

impl D3D11Sender {
    /// Checks the extension `init.header.next` points to and creates `texture_count` texture
    /// stand-ins and as many keyed-mutex stand-ins. The returned report goes to the call log.
    ///
    /// # Safety
    ///
    /// `extension` is null or a stamped extension structure.
    pub unsafe fn create(
        extension: *const QarStructureHeader,
        texture_count: usize,
    ) -> Result<(Self, String), String> {
        if extension.is_null() {
            return Err(
                "QAR_GRAPHICS_API_D3D11 requires QarStreamParamsD3D11 in header.next".into(),
            );
        }
        // SAFETY: every extension starts with QarStructureHeader.
        let structure_type = unsafe { (*extension).type_ };
        if structure_type != QarStructureType::QAR_STRUCTURE_TYPE_STREAM_D3D11_PARAMS_EXT {
            return Err(format!(
                "QarStreamParamsD3D11 has structure type {}, expected {}",
                structure_type.0,
                QarStructureType::QAR_STRUCTURE_TYPE_STREAM_D3D11_PARAMS_EXT.0
            ));
        }
        // SAFETY: the type identifies the structure.
        let params = unsafe { &*extension.cast::<QarStreamParamsD3D11>() };
        // Where the bindings have the colour extensions, the chain walk has checked what follows.
        #[cfg(not(qaros_color_extensions))]
        if !params.header.next.is_null() {
            return Err("QarStreamParamsD3D11 header.next must be NULL".into());
        }
        if params.d3d11_device.is_null() || params.d3d11_context.is_null() {
            return Err("QarStreamParamsD3D11 needs both d3d11_device and d3d11_context".into());
        }
        let report = format!(
            "d3d11 sender created textures={texture_count} device={:#x} context={:#x} color_bind={:#x} depth_bind={:#x} acquire_keyed_mutex_sync={}",
            params.d3d11_device as usize,
            params.d3d11_context as usize,
            params.color_bind_flags,
            params.depth_bind_flags,
            params.acquire_keyed_mutex_sync
        );
        let stand_ins = |count: usize| {
            (0..count)
                .map(|index| StandIn {
                    _marker: index as u64,
                })
                .collect::<Vec<_>>()
        };
        Ok((
            Self {
                acquire_keyed_mutex_sync: params.acquire_keyed_mutex_sync,
                textures: stand_ins(texture_count),
                keyed_mutexes: stand_ins(texture_count),
            },
            report,
        ))
    }

    /// Fills the frame record: the views and texture sizes the sender was built with, the
    /// stand-ins' addresses, and the keyed-mutex entries for `frame_index`.
    pub fn fill_record(
        &self,
        views: &[QarVideoFrameView],
        sizes: &[QarTextureSize],
        frame_index: u64,
        record: &mut QarVideoFrameD3D11,
    ) {
        for (slot, view) in record.texture_views.iter_mut().zip(views) {
            *slot = *view;
        }
        record.texture_views_count = views.len().min(record.texture_views.len());
        for ((slot, size), stand_in) in record.textures.iter_mut().zip(sizes).zip(&self.textures) {
            slot.size = *size;
            slot.texture = stand_in_address::<ID3D11Texture2D>(stand_in);
        }
        record.textures_count = sizes
            .len()
            .min(self.textures.len())
            .min(record.textures.len());
        if self.acquire_keyed_mutex_sync {
            record.synchronization.texture_sync_count = 0;
            return;
        }
        let acquire_key = u32::try_from(frame_index % u64::from(u32::MAX)).unwrap_or(0);
        for (slot, stand_in) in record
            .synchronization
            .texture_sync
            .iter_mut()
            .zip(&self.keyed_mutexes)
            .take(record.textures_count)
        {
            slot.mutex = stand_in_address::<IDXGIKeyedMutex>(stand_in);
            slot.acquire = acquire_key;
            slot.release = acquire_key.wrapping_add(1);
        }
        record.synchronization.texture_sync_count = record.textures_count;
    }
}

fn stand_in_address<Interface>(stand_in: &StandIn) -> *mut Interface {
    std::ptr::from_ref(stand_in).cast_mut().cast::<Interface>()
}
