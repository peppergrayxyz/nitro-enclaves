// SPDX-License-Identifier: Apache-2.0

use std::{fmt, io};

const NE_ERR_NOT_IN_INIT_STATE: i32 = 270;
const NE_ERR_NO_CPUS_AVAIL_IN_POOL: i32 = 272;
const NE_ERR_INVALID_FLAG_VALUE: i32 = 274;

/// Error that may occur during the launch process.
#[derive(Debug)]
pub enum LaunchError {
    /// /dev/nitro_enclaves ioctl error.
    Ioctl(IoctlError),

    /// Memory initialization error.
    MemInit(MemInitError),

    /// Error occuring when randomly-generating an enclave CID.
    CidRandomGenerate,
}

impl LaunchError {
    /// Error on ioctl, return an IoctlError from errno.
    pub fn ioctl_err_from_errno() -> Self {
        Self::Ioctl(IoctlError::from_errno())
    }
}

impl fmt::Display for LaunchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::Ioctl(e) => format!("ioctl error: {e}"),
            Self::MemInit(e) => format!("memory initialization error: {e}"),
            Self::CidRandomGenerate => "unable to randomly-generate enclave CID".to_string(),
        };

        write!(f, "{}", msg)
    }
}

/// Error that may occur when issuing /dev/nitro_enclaves ioctls.
#[derive(Debug)]
pub enum IoctlError {
    /// copy_to_user() failure.
    CopyToUser,

    /// Memory allocation failure for internal bookkeeping variables.
    InternalMemoryAllocation,

    /// The value of the provided flag is invalid.
    InvalidFlags,

    /// No nitro enclave CPU pool set or no CPUs available in the pool.
    NoCpuAvailable,

    /// The enclave is not in "init" (not yet started) state.
    NotInInitState,

    /// Unknown.
    Unknown(std::io::Error),
}

impl IoctlError {
    /// Parse an error from errno.
    pub fn from_errno() -> Self {
        Self::from(std::io::Error::last_os_error())
    }
}

impl From<std::io::Error> for IoctlError {
    fn from(err: std::io::Error) -> Self {
        match err.raw_os_error() {
            Some(mut e) => {
                if e < 0 {
                    e = -e;
                }

                match e {
                    libc::EFAULT => Self::CopyToUser,
                    libc::ENOMEM => Self::InternalMemoryAllocation,
                    NE_ERR_NOT_IN_INIT_STATE => Self::NotInInitState,
                    NE_ERR_NO_CPUS_AVAIL_IN_POOL => Self::NoCpuAvailable,
                    NE_ERR_INVALID_FLAG_VALUE => Self::InvalidFlags,
                    _ => Self::Unknown(err),
                }
            }
            None => Self::Unknown(err),
        }
    }
}

impl fmt::Display for IoctlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::CopyToUser => "unable to copy data to/from userspace".to_string(),
            Self::InternalMemoryAllocation => {
                "memory allocation failure for internal bookkeeping variables".to_string()
            }
            Self::InvalidFlags => "the value of the provided flag is invalid".to_string(),
            Self::NoCpuAvailable => {
                "no nitro enclave CPU pool set or no CPUs available in the pool".to_string()
            }
            Self::NotInInitState => "not in init (not yet started) state".to_string(),
            Self::Unknown(e) => format!("unknown error: {e}"),
        };

        write!(f, "{}", msg)
    }
}

/// Error that may occur when allocating and configuring enclave memory.
#[derive(Debug)]
pub enum MemInitError {
    /// A valid combination of hugepages could not be found for the requested size.
    NoHugePageFound,

    /// Unable to retrieve image metadata.
    ImageMetadata(io::Error),

    /// Unable to rewind image to beginning of image file.
    ImageRewind(io::Error),

    /// Unable to write total image file to memory regions.
    ImageWriteIncomplete,

    /// Unable to read bytes from image file.
    ImageRead(io::Error),

    /// Overflow when checking if memory region write was greater than image offset.
    OffsetCheckOverflow,

    /// Overflow when calculating end of image region in guest memory.
    ImagePlacementOverflow,
}

impl fmt::Display for MemInitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::NoHugePageFound => {
                "a valid combination of hugepages could not be found for the requested size"
                    .to_string()
            }
            Self::ImageMetadata(e) => format!("unable to retrieve image metadata: {e}"),
            Self::ImageRewind(e) => {
                format!("unable to rewind image to beginning of image file: {e}")
            }
            Self::ImageWriteIncomplete => {
                "unable to write total image file to memory regions".to_string()
            }
            Self::ImageRead(e) => format!("unable to read bytes from image file: {e}"),
            Self::OffsetCheckOverflow => {
                "overflow when checking if memory region write was greater than image offset"
                    .to_string()
            }
            Self::ImagePlacementOverflow => {
                "overflow when calculating end of image region in guest memory".to_string()
            }
        };

        write!(f, "{}", msg)
    }
}
