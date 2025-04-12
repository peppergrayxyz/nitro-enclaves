// SPDX-License-Identifier: Apache-2.0

use super::types::*;

/// Info necessary for in-memory enclave image.
#[derive(Default)]
#[repr(C)]
pub struct ImageLoadInfo {
    /// Flags to determine the enclave image type (e.g. Enclave Image Format [EIF]).
    flags: u64,

    /// Offset in enclave memory where to start placing the enclave image.
    pub memory_offset: u64,
}

impl From<ImageType> for ImageLoadInfo {
    fn from(image_type: ImageType) -> Self {
        let flags = match image_type {
            ImageType::Eif => 0x01,
        };

        Self {
            flags,
            ..Default::default()
        }
    }
}
