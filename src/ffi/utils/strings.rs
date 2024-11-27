use std::ffi::{c_char, CStr, CString};

use crate::ffi::types::std_types::{ConstBytePtr, ConstCharPtr};

///Converts a *char C type into Rust string.
/// ```
/// use torustiq_common::ffi::utils::strings;
/// assert_eq!(strings::cchar_to_string(c"Hello, World!".as_ptr()),
///            String::from("Hello, World!"));
/// ```
pub fn cchar_to_string(c: ConstCharPtr) -> String {
    unsafe { CStr::from_ptr(c).to_string_lossy().to_string() }
}

///Converts a C-style byte array (=a pointer to unsigned small integers) into Rust string,
/// considering the provided array length
/// ```
/// use torustiq_common::ffi::utils::strings;
/// assert_eq!(strings::bytes_to_string_safe(c"Hello, World!".as_ptr() as *const u8, 13),
///            String::from("Hello, World!"));
/// ```
pub fn bytes_to_string_safe(src: ConstBytePtr, len: usize) -> String {
    let mut dst: Vec<u8> = Vec::with_capacity(len);
    unsafe {
        std::ptr::copy(src, dst.as_mut_ptr(), len);
        // NB: set_len is needed here; setting the capacity is not enough
        dst.set_len(len);
    };
    String::from_utf8_lossy(dst.as_slice()).to_string()
}

/// NB: the output of this function must be deallocated later using 'cchar_const_deallocate' function
pub fn str_to_cchar(s: &str) -> ConstCharPtr {
    CString::new(s)
        .expect("Failed to convert String to c_char")
        .into_raw()
}

/// Converts String to char* C type.
/// NB: the output of this function must be deallocated later using 'cchar_const_deallocate' function
pub fn string_to_cchar<S: Into<String>>(s: S) -> ConstCharPtr {
    CString::new(s.into())
        .expect("Failed to convert String to c_char")
        .into_raw()
}

/// Deallocates memory for C string.
/// This function should be called for strings created by string_to_cchar functions to avoid memory leaks.
pub fn cchar_const_deallocate(c: ConstCharPtr) {
    let c = c as *mut c_char;
    let _ = unsafe { CString::from_raw(c) };
}