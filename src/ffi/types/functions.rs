use crate::ffi::types::module::ModuleInfo;

use super::{module::ModuleInitStepArgs, std_types};

// TODO: add the payload size and use it in string initialization. 
// Null-terminated strings are most likely not safe, as we can receive the binary data
// Perhaps should use arrays of numbers instead?
pub type ModuleOnDataReceivedFn = extern fn(std_types::ConstCharPtr);

pub type ModuleGetInfoFn = extern fn() -> ModuleInfo;
pub type ModuleInitStepFn = extern fn(ModuleInitStepArgs);
pub type ModuleTerminationHandlerFn = extern fn(std_types::Uint);
