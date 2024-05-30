use crate::ffi::types::{
    std::ConstCharPtr,
    module::Module
};

pub type GetIdFn = unsafe extern fn() -> ConstCharPtr;
pub type LoadFn = unsafe extern fn() -> Module;