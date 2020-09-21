use std::ffi::NulError;
use std::os::raw::{c_int, c_uint};

use displaydoc::Display;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Display, Error, Debug)]
pub enum Error {
    /// Timeout
    Timeout,
    /// Out of memory
    Nomem,
    /// LibUSB error
    LibUsb,
    /// No device found
    NoDevice,
    /// Invalid packet received
    InvalidPacket,
    /// NACK received
    Nack,
    /// Busy
    Busy,
    /// Invalid input
    Invalid,
    /// Already exists
    Exists,
    /// Path does not exist
    Nonexist,
    /// Null byte in string: `{0}`
    NulError(#[from] NulError),
    /// unknown error
    Unknown,
}

pub(crate) fn err(code: c_int) -> Result<()> {
    use libnspire_sys::*;
    #[forbid(unreachable_patterns)]
    match code as c_uint {
        NSPIRE_ERR_SUCCESS => Ok(()),
        NSPIRE_ERR_TIMEOUT => Err(Error::Timeout),
        NSPIRE_ERR_NOMEM => Err(Error::Nomem),
        NSPIRE_ERR_LIBUSB => Err(Error::LibUsb),
        NSPIRE_ERR_NODEVICE => Err(Error::NoDevice),
        NSPIRE_ERR_INVALPKT => Err(Error::InvalidPacket),
        NSPIRE_ERR_NACK => Err(Error::Nack),
        NSPIRE_ERR_BUSY => Err(Error::Busy),
        NSPIRE_ERR_INVALID => Err(Error::Invalid),
        NSPIRE_ERR_EXISTS => Err(Error::Exists),
        NSPIRE_ERR_NONEXIST => Err(Error::Nonexist),
        _ => Err(Error::Unknown),
    }
}
