use crate::ffi::types::{
    std_types::ConstCharPtr,
    module::ModuleInfo,
};

pub type ModuleGetInfoFn = unsafe extern fn() -> ModuleInfo;
// pub type LoadFn = unsafe extern fn() -> ModuleInfo;