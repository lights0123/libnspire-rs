//! Start with [`Handle::new`]

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::ptr::{null_mut, NonNull};

use rusb::{DeviceHandle, UsbContext};

use dir::{DirItem, DirList};
pub use error::*;
use info::Info;
use libnspire_sys::{
    free, nspire_attr, nspire_device_info, nspire_devinfo, nspire_dir_create, nspire_dirlist,
    nspire_file_copy, nspire_file_delete, nspire_file_move, nspire_file_read, nspire_file_write,
    nspire_free, nspire_handle, nspire_image, nspire_init, nspire_os_send, nspire_screenshot,
};

pub mod dir;
mod error;
pub mod info;

pub struct Handle<T: UsbContext> {
    handle: NonNull<nspire_handle>,
    _device: DeviceHandle<T>,
}

impl<T: UsbContext> Handle<T> {
    pub fn new(device: DeviceHandle<T>) -> Result<Self> {
        let mut handle: *mut nspire_handle = null_mut();
        err(unsafe { nspire_init(&mut handle, device.as_raw() as _) })?;
        Ok(Handle {
            handle: NonNull::new(handle).ok_or(Error::NoDevice)?,
            _device: device,
        })
    }

    pub fn info(&self) -> Result<Info> {
        unsafe {
            let mut info: nspire_devinfo = mem::zeroed();
            err(nspire_device_info(self.handle.as_ptr(), &mut info))?;
            Ok(info.into())
        }
    }

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
                bbp,
                data,
            })
        }
    }

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

    pub fn file_attr(&self, src: &str) -> Result<DirItem> {
        let src = CString::new(src)?;
        unsafe {
            let mut item = mem::zeroed();
            err(nspire_attr(self.handle.as_ptr(), src.as_ptr(), &mut item))?;
            Ok(item.into())
        }
    }

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

    pub fn delete_file(&self, path: &str) -> Result<()> {
        let path = CString::new(path)?;
        unsafe { err(nspire_file_delete(self.handle.as_ptr(), path.as_ptr())) }
    }

    pub fn read_file(&self, path: &str, buf: &mut [u8]) -> Result<usize> {
        let path = CString::new(path)?;
        let mut bytes = 0;
        unsafe {
            err(nspire_file_read(
                self.handle.as_ptr(),
                path.as_ptr(),
                buf.as_mut_ptr() as _,
                buf.len() as _,
                &mut bytes,
            ))?;
        }
        Ok(bytes as usize)
    }

    pub fn write_file(&self, path: &str, buf: &[u8]) -> Result<()> {
        let path = CString::new(path)?;
        unsafe {
            err(nspire_file_write(
                self.handle.as_ptr(),
                path.as_ptr(),
                buf.as_ptr() as _,
                buf.len() as _,
            ))
        }
    }

    pub fn send_os(&self, buf: &[u8]) -> Result<()> {
        unsafe {
            err(nspire_os_send(
                self.handle.as_ptr(),
                buf.as_ptr() as _,
                buf.len() as _,
            ))
        }
    }

    pub fn create_dir(&self, path: &str) -> Result<()> {
        let path = CString::new(path)?;
        unsafe { err(nspire_dir_create(self.handle.as_ptr(), path.as_ptr())) }
    }

    pub fn delete_dir(&self, path: &str) -> Result<()> {
        let path = CString::new(path)?;
        unsafe { err(nspire_dir_create(self.handle.as_ptr(), path.as_ptr())) }
    }

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

impl<T: UsbContext> Drop for Handle<T> {
    fn drop(&mut self) {
        unsafe { nspire_free(self.handle.as_ptr()) }
    }
}

pub struct Image {
    pub width: u16,
    pub height: u16,
    pub bbp: u8,
    pub data: Vec<u8>,
}

unsafe fn c_str(s: &[c_char]) -> String {
    CStr::from_ptr(s.as_ptr()).to_string_lossy().to_string()
}
