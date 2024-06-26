use core::ffi::{c_char, c_uint};

pub type ConstBytePtr = *const u8;
pub type ConstCharPtr = *const c_char;
pub type ConstCStrPtr = *const i8;
pub type Char = c_char;
pub type Uint = c_uint;
