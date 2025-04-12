// SPDX-License-Identifier: Apache-2.0

use crate::device::Device;

use std::{
    marker::PhantomData,
    os::fd::{AsRawFd, RawFd},
};

const NE_MAGIC: u64 = 0xAE;
const NE_CREATE_VM: u64 = nix::request_code_read!(NE_MAGIC, 0x20, size_of::<u64>()) as _;

/// Facilitates the correct execution of the nitro enclaves launch process.
pub struct Launcher<T> {
    vm_fd: RawFd,
    dev: Device,
    slot_uid: u64,
    state: PhantomData<T>,
}

impl<T> Launcher<T> {
    /// Begin the nitro enclaves launch process by creating a Launcher and issuing the NE_CREATE_VM
    /// ioctl.
    pub fn new(dev: Device) -> Result<Self, LaunchError> {
        let mut slot_uid: u64 = 0;
        let vm_fd = unsafe { libc::ioctl(dev.as_raw_fd(), NE_CREATE_VM as _, &mut slot_uid) };

        Self {
            vm_fd,
            dev,
            slot_uid,
            state: PhantomData,
        }
    }
}
