use std::ffi::NulError;
use std::os::raw::{c_int, c_uint};

use displaydoc::Display;
use thiserror::Error;

/// The generic result type.
pub type Result<T> = std::result::Result<T, Error>;

/// A libnspire error.
#[derive(Display, Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Input/output error
    Io,
    /// Access denied (insufficient permissions)
    Access,
    /// Operation not supported or unimplemented on this platform
    NotSupported,
    /// Timeout
    Timeout,
    /// Out of memory
    OutOfMemory,
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
    DoesNotExist,
    /// Null byte in string: `{0}`
    NulError(#[from] NulError),
    /// Rusb error: `{0}`
    Usb(#[from] rusb::Error),
    /// Unknown bits-per-pixel value: `{0}`
    UnknownBpp(u8),
    /// unknown error
    Unknown,
}

pub(crate) fn err(code: c_int) -> Result<()> {
    use libnspire_sys::*;
    use libusb1_sys::constants::*;
    #[forbid(unreachable_patterns)]
    match code as c_uint {
        NSPIRE_ERR_SUCCESS => Ok(()),
        NSPIRE_ERR_TIMEOUT => Err(Error::Timeout),
        NSPIRE_ERR_NOMEM => Err(Error::OutOfMemory),
        NSPIRE_ERR_LIBUSB => Err(Error::LibUsb),
        NSPIRE_ERR_NODEVICE => Err(Error::NoDevice),
        NSPIRE_ERR_INVALPKT => Err(Error::InvalidPacket),
        NSPIRE_ERR_NACK => Err(Error::Nack),
        NSPIRE_ERR_BUSY => Err(Error::Busy),
        NSPIRE_ERR_INVALID => Err(Error::Invalid),
        NSPIRE_ERR_EXISTS => Err(Error::Exists),
        NSPIRE_ERR_NONEXIST => Err(Error::DoesNotExist),
        _ => match code {
            LIBUSB_ERROR_IO => Err(Error::Io),
            LIBUSB_ERROR_INVALID_PARAM => Err(Error::Invalid),
            LIBUSB_ERROR_ACCESS => Err(Error::Access),
            LIBUSB_ERROR_NO_DEVICE => Err(Error::NoDevice),
            LIBUSB_ERROR_NOT_FOUND => Err(Error::NoDevice),
            LIBUSB_ERROR_BUSY => Err(Error::Busy),
            LIBUSB_ERROR_TIMEOUT => Err(Error::Timeout),
            LIBUSB_ERROR_OVERFLOW => Err(Error::LibUsb),
            LIBUSB_ERROR_PIPE => Err(Error::LibUsb),
            LIBUSB_ERROR_INTERRUPTED => Err(Error::LibUsb),
            LIBUSB_ERROR_NO_MEM => Err(Error::OutOfMemory),
            LIBUSB_ERROR_NOT_SUPPORTED => Err(Error::NotSupported),
            _ => Err(Error::Unknown),
        },
    }
}
