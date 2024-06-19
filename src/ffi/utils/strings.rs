use core::ffi::c_char;
use std::ffi::{CStr, CString};

pub fn cchar_to_string(c: *const c_char) -> String {
    unsafe { CStr::from_ptr(c).to_string_lossy().to_string() }
}

pub fn str_to_cchar(s: &str) -> *const c_char {
    CString::new(s)
        .expect("Failed to convert String to c_char")
        .into_raw()
}