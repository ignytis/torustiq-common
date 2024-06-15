use crate::ffi::types::module::ModuleInfo;

use super::{module::ModuleInitStepArgs, std_types};

pub type ModuleGetInfoFn = extern fn() -> ModuleInfo;
pub type ModuleInitStepFn = extern fn(ModuleInitStepArgs);
pub type ModuleTerminationHandlerFn = extern "C" fn(std_types::Uint);