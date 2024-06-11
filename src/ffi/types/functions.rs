use crate::ffi::types::{
    std_types::ConstCharPtr,
    module::Module
};

pub type GetIdFn = unsafe extern fn() -> ConstCharPtr;
pub type LoadFn = unsafe extern fn() -> Module;