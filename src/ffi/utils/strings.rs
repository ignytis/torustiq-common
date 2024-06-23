use std::ffi::{CStr, CString};

use crate::ffi::types::std_types::{ConstBytePtr, ConstCharPtr};

pub fn cchar_to_string(c: ConstCharPtr) -> String {
    unsafe { CStr::from_ptr(c).to_string_lossy().to_string() }
}

pub fn cchar_to_string_safe(src: ConstCharPtr, len: usize) -> String {
    let src_u8: ConstBytePtr = unsafe { std::mem::transmute(src) };
    let mut dst: Vec<u8> = Vec::with_capacity(len);
    unsafe { std::ptr::copy(src_u8, dst.as_mut_ptr(), len) };
    String::from_utf8_lossy(dst.as_slice()).to_string()
}

pub fn bytes_to_string_safe(src: ConstBytePtr, len: usize) -> String {
    let mut dst: Vec<u8> = Vec::with_capacity(len);
    unsafe {
        std::ptr::copy(src, dst.as_mut_ptr(), len);
        dst.set_len(len);
    };
    String::from_utf8_lossy(dst.as_slice()).to_string()
}

pub fn str_to_cchar(s: &str) -> ConstCharPtr {
    CString::new(s)
        .expect("Failed to convert String to c_char")
        .into_raw()
}

pub fn string_to_cchar<S: Into<String>>(s: S) -> ConstCharPtr {
    CString::new(s.into())
        .expect("Failed to convert String to c_char")
        .into_raw()
}