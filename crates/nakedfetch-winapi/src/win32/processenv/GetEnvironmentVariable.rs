use alloc::vec;

use wtf8::{Wtf8, Wtf8Buf};

use crate::{ntdll, ntstatus::NtStatus, types::RawUnicodeString, unicode::UnicodeString};

const STATUS_BUFFER_TOO_SMALL: NtStatus = NtStatus::new(0xC000_0023);

/// # Errors
///
/// If the system failed to find or correctly retrieve the env variable from the
/// PEB.
pub fn GetEnvironmentVariable<T: AsRef<Wtf8>>(name: T) -> Result<Wtf8Buf, NtStatus> {
    get_env_var_inner(name.as_ref())
}

fn get_env_var_inner(name: &Wtf8) -> Result<Wtf8Buf, NtStatus> {
    let env = ntdll::RtlGetCurrentPeb().process_parameters.environment;
    let name_unicode = UnicodeString::from_wtf8(name);
    let mut raw_value = RawUnicodeString::default();

    // Get size
    // SAFETY: Pass a valid pointer to Env; according to the documentation, an
    // empty `UNICODE_STRING` can be passed to get the output string size
    let ret = unsafe {
        ntdll::RtlQueryEnvironmentVariable_U(
            env.cast(),
            name_unicode.as_inner_ptr(),
            &raw mut raw_value,
        )
    };

    if ret != STATUS_BUFFER_TOO_SMALL {
        return Err(ret);
    }

    let len = raw_value.length;
    let mut buffer = vec![0u16; len as usize + 2];

    let mut value = RawUnicodeString {
        maximum_length: (buffer.len() + 2) as u16,
        buffer: buffer.as_mut_ptr(),
        ..Default::default()
    };

    // Get value
    // SAFETY: The size of the allocated buffer equals the requirement of the
    // same function
    let ret = unsafe {
        ntdll::RtlQueryEnvironmentVariable_U(
            env.cast(),
            name_unicode.as_inner_ptr(),
            &raw mut value,
        )
    };

    if ret.is_success() {
        let chars = value.length as usize / size_of::<u16>();
        buffer.truncate(chars);
        Ok(Wtf8Buf::from_utf16(buffer.into_iter()))
    } else {
        Err(ret)
    }
}
