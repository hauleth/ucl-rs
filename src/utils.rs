use libc::c_char;

use std::str;
use std::ffi::{
    CString,
    CStr
};

pub fn to_c_str<T: AsRef<str>>(string: T) -> *const c_char {
    CString::new(string.as_ref()).unwrap().as_ptr()
}

pub fn to_str(cstring: *const c_char) -> Option<String> {
    if cstring.is_null() { return None }
    Some(str::from_utf8(unsafe { CStr::from_ptr(cstring).to_bytes() }).unwrap().to_string())
}
