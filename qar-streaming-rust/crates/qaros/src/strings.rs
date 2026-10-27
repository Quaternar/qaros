use std::ffi::{CStr, CString, c_char};
use std::path::Path;

use crate::error::{Error, Result, StatusCode};

/// Buffer for the C API's string getters, which take a caller buffer but
/// report neither the required size nor truncation.
pub(crate) const GETTER_BUFFER_SIZE: usize = 4096;

pub(crate) fn to_c_string(value: &str, what: &str) -> Result<CString> {
    CString::new(value).map_err(|_| Error::InvalidArgument(format!("{what} contains a NUL byte")))
}

pub(crate) fn optional_c_string(value: Option<&str>, what: &str) -> Result<Option<CString>> {
    value.map(|text| to_c_string(text, what)).transpose()
}

pub(crate) fn path_to_c_string(path: &Path, what: &str) -> Result<CString> {
    let text = path
        .to_str()
        .ok_or_else(|| Error::InvalidArgument(format!("{what} is not valid UTF-8")))?;
    to_c_string(text, what)
}

pub(crate) fn pointer_or_null(value: Option<&CString>) -> *const c_char {
    value.map_or(std::ptr::null(), |text| text.as_ptr())
}

/// Text up to the first NUL, replacing invalid UTF-8.
pub(crate) fn from_buffer(buffer: &[c_char]) -> String {
    let bytes: Vec<u8> = buffer
        .iter()
        .take_while(|&&character| character != 0)
        .map(|&character| character.to_ne_bytes()[0])
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Calls a C string getter with a [`GETTER_BUFFER_SIZE`] buffer.
pub(crate) fn read_string(getter: impl FnOnce(*mut c_char, usize) -> Result<()>) -> Result<String> {
    let mut buffer = vec![0 as c_char; GETTER_BUFFER_SIZE];
    getter(buffer.as_mut_ptr(), buffer.len())?;
    Ok(from_buffer(&buffer))
}

/// Copies a NUL-terminated string the C API returned from `function`,
/// replacing invalid UTF-8; a null pointer after a success result is an API
/// error.
///
/// # Safety
///
/// `pointer` is null or points to a NUL-terminated string valid for the
/// duration of the call.
pub(crate) unsafe fn from_returned_pointer(
    pointer: *const c_char,
    function: &'static str,
    what: &str,
) -> Result<String> {
    if pointer.is_null() {
        return Err(Error::Api {
            function,
            code: StatusCode::UNCLASSIFIED,
            message: format!("returned success without {what}"),
            causes: Vec::new(),
        });
    }
    // SAFETY: non-null and NUL-terminated per the caller's contract.
    let text = unsafe { CStr::from_ptr(pointer) };
    Ok(text.to_string_lossy().into_owned())
}
