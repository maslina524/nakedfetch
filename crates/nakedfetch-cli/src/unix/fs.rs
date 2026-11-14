use alloc::{
    string::{FromUtf8Error, String},
    vec,
    vec::Vec,
};
use core::{
    ffi::{CStr, c_char, c_int},
    ptr,
};

use crate::unix::{
    error::{self, ErrorCode},
    libc::{
        Dir, FileHandle, LinuxDirent64, close, fclose, ferror, fopen, fread, fwrite, getdents64,
        mkdir, open, opendir, readdir, readlink, rewind,
    },
    path::Path,
};
const O_RDONLY: c_int = 0;

#[cfg(target_arch = "aarch64")]
const O_DIRECTORY: c_int = 0x4000;
#[cfg(not(target_arch = "aarch64"))]
const O_DIRECTORY: c_int = 0x10000;

const AT_FDCWD: c_int = -100;
const BUF_SIZE: usize = 4096 * 16;

const SEEK_END: c_int = 2;
const DT_REG: u8 = 8;
const DT_DIR: u8 = 4;
const DT_LNK: u8 = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    File,
    Dir,
    Link,
    Unknown,
}

impl From<u8> for ItemType {
    fn from(value: u8) -> Self {
        match value {
            DT_REG => Self::File,
            DT_DIR => Self::Dir,
            DT_LNK => Self::Link,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    typ: ItemType,
    name: String,
}

impl Item {
    pub fn typ(&self) -> ItemType {
        self.typ.clone()
    }

    pub const fn name(&self) -> &String {
        &self.name
    }

    pub fn into_name(self) -> String {
        self.name
    }
}

pub struct ReadDirIter {
    dir: Dir,
}

impl Iterator for ReadDirIter {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let raw_item = readdir(&raw mut self.dir);
            if raw_item.is_null() {
                return None;
            }

            // SAFETY: We check if the pointer is null, safe
            let item = unsafe { raw_item.read() };

            // SAFETY: Libc is guaranteed to return a valid string
            let c_str_name = unsafe { CStr::from_ptr(item.d_name.as_ptr()) };
            if c_str_name == c"." || c_str_name == c".." {
                continue;
            }

            let name = c_str_name.to_string_lossy().into_owned();
            let typ = ItemType::from(item.d_type);

            return Some(Item { typ, name });
        }
    }
}

pub fn read_dir(path: impl Into<Path>) -> error::Result<ReadDirIter> {
    let path = path.into();
    let c_path = path.as_c_str();

    let dir = opendir(c_path.as_ptr());
    if dir.is_null() {
        return Err(ErrorCode::last());
    }

    Ok(ReadDirIter { dir })
}

pub fn read_dir_all(path: impl Into<Path>) -> error::Result<Vec<Item>> {
    let path = path.into();
    let c_path = path.as_c_str();

    let fd = open(c_path.as_ptr(), O_RDONLY | O_DIRECTORY, 0);
    // crate::println!("Open: Fd: {fd}, Path: {c_path:?}, Error: {}:{}",
    // ErrorCode::last(), ErrorCode::last().code());
    if fd < 0 {
        return Err(ErrorCode::last());
    }

    let mut items = Vec::with_capacity(20);
    let mut buf = vec![0u8; BUF_SIZE].into_boxed_slice();

    loop {
        let n = getdents64(fd, buf.as_mut_ptr().cast(), BUF_SIZE);
        if n < 0 {
            let err = ErrorCode::last();
            close(fd);
            return Err(err);
        }
        if n == 0 {
            break;
        }

        let mut pos = 0;
        while pos < n as usize {
            // SAFETY: Libc always returns a valid pointer
            let entry =
                unsafe { ptr::read_unaligned(buf.as_ptr().add(pos).cast::<LinuxDirent64>()) };
            let reclen = entry.d_reclen as usize;
            if reclen == 0 {
                break;
            }

            // SAFETY: Libc is guaranteed to return a valid string
            let name_cstr = unsafe { CStr::from_ptr(entry.d_name.as_ptr()) };
            let name = name_cstr.to_string_lossy().into_owned();
            if name != "." && name != ".." {
                let typ = ItemType::from(entry.d_type);
                items.push(Item { typ, name });
            }
            pos += reclen;
        }
    }

    close(fd);
    Ok(items)
}

pub fn read_link(path: impl Into<Path>, len: usize) -> Option<String> {
    let path = path.into();
    let c_path = path.as_c_str();
    let mut buf = vec![c_char::default(); len];

    let len = readlink(c_path.as_ptr(), buf.as_mut_ptr(), len - 1);
    if len == -1 {
        None
    } else {
        // SAFETY: Libs are guaranteed to store a valid cstr
        let c_str = unsafe { CStr::from_ptr(buf.as_ptr()) };
        let string = c_str.to_string_lossy().into_owned();
        Some(string)
    }
}
