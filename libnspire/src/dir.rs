//! Utilities related to files and directories.

use std::ffi::CStr;
use std::fmt;
use std::mem::transmute;
use std::ops::Deref;

use libnspire_sys::{nspire_dir_info, nspire_dir_item, nspire_dir_type, nspire_dirlist_free};

/// The type of entry: a file or directory.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum EntryType {
    File,
    Directory,
}

impl From<nspire_dir_type> for EntryType {
    fn from(hw_type: nspire_dir_type) -> Self {
        use libnspire_sys::*;
        #[forbid(unreachable_patterns)]
        #[allow(non_upper_case_globals)]
        match hw_type {
            nspire_dir_type_NSPIRE_FILE => EntryType::File,
            nspire_dir_type_NSPIRE_DIR => EntryType::Directory,
            v => unreachable!("Invalid file type {}", v),
        }
    }
}

/// A directory entry: either a file or directory.
#[repr(transparent)]
pub struct DirItem(nspire_dir_item);

impl DirItem {
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0.name.as_ptr()) }
    }
    pub fn size(&self) -> u64 {
        self.0.size
    }
    pub fn date(&self) -> u64 {
        self.0.date
    }
    /// Whether this is a file or directory.
    pub fn entry_type(&self) -> EntryType {
        self.0.type_.into()
    }
}

impl fmt::Debug for DirItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DirItem")
            .field("name", &self.name())
            .field("size", &self.size())
            .field("date", &self.date())
            .field("entry_type", &self.entry_type())
            .finish()
    }
}

impl From<nspire_dir_item> for DirItem {
    fn from(item: nspire_dir_item) -> Self {
        DirItem(item)
    }
}

/// A list of entries within a directory.
///
/// This struct implements [`Deref`] to `slice`, so you can simply access this
/// as if it was a slice, i.e. with `[index]` and `.iter()`.
#[repr(transparent)]
pub struct DirList(*mut nspire_dir_info);

impl DirList {
    pub(crate) fn from_raw(item: *mut nspire_dir_info) -> Self {
        DirList(item)
    }
}

impl fmt::Debug for DirList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl Deref for DirList {
    type Target = [DirItem];

    fn deref(&self) -> &Self::Target {
        unsafe { transmute((*self.0).items.as_slice((*self.0).num as usize)) }
    }
}

impl Drop for DirList {
    fn drop(&mut self) {
        unsafe {
            nspire_dirlist_free(self.0);
        }
    }
}
