//! Start with [`Handle::new`]

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::ptr::{null_mut, NonNull};

use rusb::{DeviceHandle, UsbContext};

use crate::callback::CallbackData;
use array_iterator::ArrayIterator;
use dir::{DirItem, DirList};
pub use error::*;
use info::Info;
use libnspire_sys::{
    free, nspire_attr, nspire_device_info, nspire_devinfo, nspire_dir_create, nspire_dir_delete,
    nspire_dirlist, nspire_file_copy, nspire_file_delete, nspire_file_move, nspire_file_read,
    nspire_file_write, nspire_free, nspire_handle, nspire_image, nspire_init, nspire_os_send,
    nspire_screenshot,
};
use std::convert::TryFrom;

mod callback;
pub mod dir;
mod error;
pub mod info;

/// The USB vendor ID used by all Nspire calculators.
pub const VID: u16 = 0x0451;
/// The USB vendor ID used by all non-CX and original CX calculators.
pub const PID: u16 = 0xe012;
/// The USB vendor ID used by all CX II calculators.
pub const PID_CX2: u16 = 0xe022;

/// A handle to a calculator.
pub struct Handle<T: UsbContext> {
    handle: NonNull<nspire_handle>,
    device: DeviceHandle<T>,
}

fn is_cx_ii<T: UsbContext>(device: &DeviceHandle<T>) -> Result<bool> {
    Ok(device.device().device_descriptor()?.product_id() == PID_CX2)
}

impl<T: UsbContext> Handle<T> {
    /// Create a new handle to a USB device.
    pub fn new(device: DeviceHandle<T>) -> Result<Self> {
        let mut handle: *mut nspire_handle = null_mut();
        err(unsafe { nspire_init(&mut handle, device.as_raw() as _, is_cx_ii(&device)?) })?;
        Ok(Handle {
            handle: NonNull::new(handle).ok_or(Error::NoDevice)?,
            device,
        })
    }

    /// Whether this device is a CX II, CAS or non-CAS.
    pub fn is_cx_ii(&self) -> Result<bool> {
        is_cx_ii(&self.device)
    }

    pub fn info(&self) -> Result<Info> {
        unsafe {
            let mut info: nspire_devinfo = mem::zeroed();
            err(nspire_device_info(self.handle.as_ptr(), &mut info))?;
            Ok(info.into())
        }
    }

    /// Take a screenshot.
    pub fn screenshot(&self) -> Result<Image> {
        unsafe {
            let mut image: *mut nspire_image = null_mut();
            err(nspire_screenshot(self.handle.as_ptr(), &mut image))?;
            let width = (*image).width;
            let height = (*image).height;
            let bbp = (*image).bbp;
            let len = (width as u32 * height as u32 * bbp as u32) / 8;
            let data = (*image).data.as_slice(len as usize).into();
            free(image as _);
            Ok(Image {
                width,
                height,
                bpp: bbp,
                data,
            })
        }
    }

    /// Move/rename a file.
    pub fn move_file(&self, src: &str, dest: &str) -> Result<()> {
        let src = CString::new(src)?;
        let dest = CString::new(dest)?;
        unsafe {
            err(nspire_file_move(
                self.handle.as_ptr(),
                src.as_ptr(),
                dest.as_ptr(),
            ))
        }
    }

    /// Get the attributes of a file or directory.
    pub fn file_attr(&self, src: &str) -> Result<DirItem> {
        let src = CString::new(src)?;
        unsafe {
            let mut item = mem::zeroed();
            err(nspire_attr(self.handle.as_ptr(), src.as_ptr(), &mut item))?;
            Ok(item.into())
        }
    }

    /// Copy a file.
    pub fn copy_file(&self, src: &str, dest: &str) -> Result<()> {
        let src = CString::new(src)?;
        let dest = CString::new(dest)?;
        unsafe {
            err(nspire_file_copy(
                self.handle.as_ptr(),
                src.as_ptr(),
                dest.as_ptr(),
            ))
        }
    }

    /// Delete a file.
    pub fn delete_file(&self, path: &str) -> Result<()> {
        let path = CString::new(path)?;
        unsafe { err(nspire_file_delete(self.handle.as_ptr(), path.as_ptr())) }
    }

    /// Read a file. Returns the number of bytes read. You must pass a buffer
    /// large enough to read the entire file (or smaller if that's all you care
    /// about).
    pub fn read_file(
        &self,
        path: &str,
        buf: &mut [u8],
        progress: &mut dyn FnMut(usize),
    ) -> Result<usize> {
        let path = CString::new(path)?;
        let mut bytes = 0;
        let mut cb = CallbackData(progress);
        unsafe {
            err(nspire_file_read(
                self.handle.as_ptr(),
                path.as_ptr(),
                buf.as_mut_ptr() as _,
                buf.len() as _,
                &mut bytes,
                Some(CallbackData::callback),
                cb.as_mut_void(),
            ))?;
        }
        Ok(bytes as usize)
    }

    /// Write a file.
    pub fn write_file(
        &self,
        path: &str,
        buf: &[u8],
        progress: &mut dyn FnMut(usize),
    ) -> Result<()> {
        let path = CString::new(path)?;
        let mut cb = CallbackData(progress);
        unsafe {
            err(nspire_file_write(
                self.handle.as_ptr(),
                path.as_ptr(),
                buf.as_ptr() as _,
                buf.len() as _,
                Some(CallbackData::callback),
                cb.as_mut_void(),
            ))
        }
    }

    /// Send an OS update.
    pub fn send_os(&self, buf: &[u8], progress: &mut dyn FnMut(usize)) -> Result<()> {
        let mut cb = CallbackData(progress);
        unsafe {
            err(nspire_os_send(
                self.handle.as_ptr(),
                buf.as_ptr() as _,
                buf.len() as _,
                Some(CallbackData::callback),
                cb.as_mut_void(),
            ))
        }
    }

    /// Create a directory.
    pub fn create_dir(&self, path: &str) -> Result<()> {
        let path = CString::new(path)?;
        unsafe { err(nspire_dir_create(self.handle.as_ptr(), path.as_ptr())) }
    }

    /// Delete a directory.
    pub fn delete_dir(&self, path: &str) -> Result<()> {
        let path = CString::new(path)?;
        unsafe { err(nspire_dir_delete(self.handle.as_ptr(), path.as_ptr())) }
    }

    /// Get the contents of a directory.
    pub fn list_dir(&self, path: &str) -> Result<DirList> {
        let path = CString::new(path)?;
        unsafe {
            let mut list = null_mut();
            err(nspire_dirlist(
                self.handle.as_ptr(),
                path.as_ptr(),
                &mut list,
            ))?;
            Ok(DirList::from_raw(list))
        }
    }
}

unsafe impl<T: UsbContext> Send for Handle<T> {}
unsafe impl<T: UsbContext> Sync for Handle<T> {}

impl<T: UsbContext> TryFrom<DeviceHandle<T>> for Handle<T> {
    type Error = Error;

    fn try_from(device: DeviceHandle<T>) -> Result<Self> {
        Handle::new(device)
    }
}

impl<T: UsbContext> Drop for Handle<T> {
    fn drop(&mut self) {
        unsafe { nspire_free(self.handle.as_ptr()) }
    }
}

/// An image from a screenshot.
pub struct Image {
    pub width: u16,
    pub height: u16,
    /// The number of bits per pixel. Either 8 for non-color calculators or 16
    /// for color calculators.
    pub bpp: u8,
    pub data: Vec<u8>,
}
const MAX_R: u8 = ((1usize << 5) - 1) as u8;
const MAX_G: u8 = ((1usize << 6) - 1) as u8;
const MAX_B: u8 = ((1usize << 5) - 1) as u8;
/// Convert color channel values from one bit depth to another.
const fn convert_channel(value: u8, from_max: u8) -> u8 {
    ((value as u16 * 255u16 + from_max as u16 / 2) / from_max as u16) as u8
}

#[cfg(feature = "image")]
impl TryFrom<Image> for image::DynamicImage {
    type Error = Error;

    /// Currently broken.
    fn try_from(image: Image) -> Result<Self> {
        use image::ImageBuffer;
        match image.bpp {
            8 => Ok(image::DynamicImage::ImageLuma8(
                ImageBuffer::from_vec(image.width as u32, image.height as u32, image.data).unwrap(),
            )),
            16 => {
                let data: Vec<u8> = image
                    .data
                    .chunks(2)
                    .flat_map(|d| {
                        let color = u16::from_ne_bytes([d[0], d[1]]);
                        ArrayIterator::new([
                            convert_channel(color as u8 & MAX_R, MAX_R),
                            convert_channel((color >> 5) as u8 & MAX_G, MAX_G),
                            convert_channel((color >> 11) as u8 & MAX_B, MAX_B),
                        ])
                    })
                    .collect();
                dbg!(data.len());
                Ok(image::DynamicImage::ImageRgb8(
                    ImageBuffer::from_vec(image.width as u32, image.height as u32, data).unwrap(),
                ))
            }
            other => Err(Error::UnknownBpp(other)),
        }
    }
}

unsafe fn c_str(s: &[c_char]) -> String {
    CStr::from_ptr(s.as_ptr()).to_string_lossy().to_string()
}
