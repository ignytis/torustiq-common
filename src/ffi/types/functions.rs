use crate::ffi::types::module::ModuleInfo;

use super::module::ModuleInitStepArgs;

pub type ModuleGetInfoFn = unsafe extern fn() -> ModuleInfo;
pub type ModuleInitStepFn = unsafe extern fn(ModuleInitStepArgs);