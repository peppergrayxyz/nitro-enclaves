// SPDX-License-Identifier: Apache-2.0

use std::fmt;

const NE_ERR_NOT_IN_INIT_STATE: i32 = 270;
const NE_ERR_NO_CPUS_AVAIL_IN_POOL: i32 = 272;
const NE_ERR_INVALID_FLAG_VALUE: i32 = 274;

#[derive(Debug)]
pub enum LaunchError {
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

impl From<std::io::Error> for LaunchError {
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

impl fmt::Display for LaunchError {
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
