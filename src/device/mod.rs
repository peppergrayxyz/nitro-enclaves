// SPDX-License-Identifier: Apache-2.0

use std::fs::{File, OpenOptions};

#[allow(dead_code)]
pub struct Device(File);

impl Device {
    // Create a handle to the Nitro Enclaves device (/dev/nitro_enclaves).
    pub fn open() -> std::io::Result<Self> {
        Ok(Self(
            OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/nitro_enclaves")?,
        ))
    }
}
