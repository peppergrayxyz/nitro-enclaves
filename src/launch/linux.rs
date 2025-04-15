// SPDX-License-Identifier: Apache-2.0

use super::types::*;

pub const NE_MAGIC: u64 = 0xAE;

// Create a slot that is associated with an enclave VM.
pub const NE_CREATE_VM: u64 = nix::request_code_read!(NE_MAGIC, 0x20, size_of::<u64>()) as _;

// Set a vCPU for an enclave.
pub const NE_ADD_VCPU: u64 = nix::request_code_readwrite!(NE_MAGIC, 0x21, size_of::<u32>()) as _;

// Get information needed for in-memory enclave image loading.
pub const NE_GET_IMAGE_LOAD_INFO: u64 =
    nix::request_code_readwrite!(NE_MAGIC, 0x22, size_of::<ImageLoadInfo>()) as _;

/// Info necessary for in-memory enclave image.
#[derive(Debug, Default)]
#[repr(C)]
pub struct ImageLoadInfo {
    /// Flags to determine the enclave image type (e.g. Enclave Image Format [EIF]).
    flags: u64,

    /// Offset in enclave memory where to start placing the enclave image.
    pub memory_offset: u64,
}

impl From<&ImageType> for ImageLoadInfo {
    fn from(image_type: &ImageType) -> Self {
        let flags = match image_type {
            ImageType::Eif(_) => 0x01,
        };

        Self {
            flags,
            ..Default::default()
        }
    }
}
