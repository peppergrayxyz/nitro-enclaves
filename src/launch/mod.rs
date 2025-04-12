// SPDX-License-Identifier: Apache-2.0

mod error;
mod linux;
mod types;

pub use error::LaunchError;
pub use types::*;

use crate::device::Device;
use linux::*;
use std::{
    marker::PhantomData,
    os::fd::{AsRawFd, RawFd},
};

const NE_MAGIC: u64 = 0xAE;
const NE_CREATE_VM: u64 = nix::request_code_read!(NE_MAGIC, 0x20, size_of::<u64>()) as _;
const NE_ADD_VCPU: u64 = nix::request_code_readwrite!(NE_MAGIC, 0x21, size_of::<u32>()) as _;
const NE_GET_IMAGE_LOAD_INFO: u64 =
    nix::request_code_readwrite!(NE_MAGIC, 0x22, size_of::<ImageLoadInfo>()) as _;

/// Launcher type-state that indicates an initializing (not yet started) enclave.
pub struct Initializing;

/// Facilitates the correct execution of the nitro enclaves launch process.
pub struct Launcher<T> {
    vm_fd: RawFd,
    slot_uid: u64,
    image_memory_offset: u64,
    cpu_ids: Vec<u32>,
    state: PhantomData<T>,
}

impl Launcher<Initializing> {
    /// Begin the nitro enclaves launch process by creating a Launcher and issuing the NE_CREATE_VM
    /// ioctl.
    pub fn new(dev: &Device, image_type: ImageType) -> Result<Self, LaunchError> {
        let mut slot_uid: u64 = 0;
        let vm_fd = unsafe { libc::ioctl(dev.as_raw_fd(), NE_CREATE_VM as _, &mut slot_uid) };

        if vm_fd < 0 || slot_uid == 0 {
            return Err(LaunchError::from_errno());
        }

        // Load the VM's enclave image type and fetch the offset in enclave memory of where to
        // start placing the enclave image.
        let mut load_info = ImageLoadInfo::from(image_type);

        let ret = unsafe {
            libc::ioctl(
                vm_fd.as_raw_fd(),
                NE_GET_IMAGE_LOAD_INFO as _,
                &mut load_info,
            )
        };

        if ret < 0 {
            return Err(LaunchError::from_errno());
        }

        Ok(Self {
            vm_fd,
            slot_uid,
            image_memory_offset: load_info.memory_offset,
            cpu_ids: Vec::new(),
            state: PhantomData,
        })
    }

    /// Set a vCPU for an enclave. The vCPU can be auto-chosen from the NE CPU pool or it can be
    /// set by the caller.
    ///
    /// If set by the caller, the CPU needs to be available in the NE CPU pool.
    pub fn vcpu_add(&mut self, id: Option<u32>) -> Result<(), LaunchError> {
        let mut id = id.unwrap_or(0);

        let ret = unsafe { libc::ioctl(self.vm_fd, NE_ADD_VCPU as _, &mut id) };

        if ret < 0 {
            return Err(LaunchError::from_errno());
        }

        self.cpu_ids.push(id);

        Ok(())
    }
}
