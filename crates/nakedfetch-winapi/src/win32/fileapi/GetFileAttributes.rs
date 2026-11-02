use path::Path;

use crate::{
    ntdll,
    ntstatus::NtStatus,
    types::{FileBasicInfo, ObjectAttributes},
    unicode::UnicodeString,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    Dir,
    File,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemAttributes {
    pub is_readonly: bool,
    pub is_hidden: bool,
    pub is_system: bool,
    pub typ: ItemType,
}

impl ItemAttributes {
    pub const DIRECTORY: u32 = 0b10000;
    pub const HIDDEN: u32 = 0b00010;
    pub const READONLY: u32 = 0b00001;
    pub const SYSTEM: u32 = 0b00100;
}

#[derive(Debug, Clone, Copy)]
pub enum FileAttributesError {
    UnicodePathAllocate,
    QueryAttributes(NtStatus),
}

/// # Errors
///
/// On error returns `FileAttributesError`, which is either a Unicode string
/// allocation error or a `NtQueryAttributesFile` call error (Ntstatus).
pub fn GetFileAttributes<T: AsRef<Path>>(path: T) -> Result<ItemAttributes, FileAttributesError> {
    get_file_attributes_inner(path.as_ref())
}

fn get_file_attributes_inner(path: &Path) -> Result<ItemAttributes, FileAttributesError> {
    let path =
        UnicodeString::from_path(path).map_err(|_| FileAttributesError::UnicodePathAllocate)?;

    let mut attr = ObjectAttributes {
        length: size_of::<ObjectAttributes>() as u32,
        object_name: path.as_inner_ptr(),
        ..Default::default()
    };
    let mut info = FileBasicInfo::default();

    // SAFETY: We pass valid pointers, `OBJECT_ATTRIBUTES` stores a valid
    // pointer to `UNICODE_STRING`
    let ret = unsafe { ntdll::NtQueryAttributesFile(&raw mut attr, &raw mut info) };
    if !ret.is_success() {
        return Err(FileAttributesError::QueryAttributes(ret));
    }

    let raw = info.file_attributes;
    let has = |flag: u32| raw & flag != 0;

    Ok(ItemAttributes {
        is_readonly: has(ItemAttributes::READONLY),
        is_hidden: has(ItemAttributes::HIDDEN),
        is_system: has(ItemAttributes::SYSTEM),
        typ: if has(ItemAttributes::DIRECTORY) {
            ItemType::Dir
        } else {
            ItemType::File
        },
    })
}
