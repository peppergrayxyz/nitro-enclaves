// SPDX-License-Identifier: Apache-2.0

use bitflags::bitflags;
use std::fs::File;

/// The image type of the enclave.
pub enum ImageType {
    /// Enclave Image Format.
    Eif(File),
}

/// Data related to setting enclave memory.
pub struct MemoryInfo {
    /// Enclave image type.
    pub image_type: ImageType,

    /// Amount of memory (in MiB) to allocate to the enclave.
    pub size_mib: usize,
}

impl MemoryInfo {
    pub fn new(image_type: ImageType, size_mib: usize) -> Self {
        Self {
            image_type,
            size_mib,
        }
    }
}

bitflags! {
    /// Configuration flags for starting an enclave.
    pub struct StartFlags: u64 {
        /// Start enclave in debug mode.
        const DEBUG = 1;
    }
}
