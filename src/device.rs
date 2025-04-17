// SPDX-License-Identifier: Apache-2.0

use std::{
    fs::{File, OpenOptions},
    os::fd::{AsRawFd, RawFd},
};

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

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
