use crate::ffi::types::module::ModuleInfo;

use super::{module::{ModuleInitStepArgs, Record}, std_types};

// The following functions are expected to be exported by libraries
pub type ModuleGetInfoFn = extern fn() -> ModuleInfo;
pub type ModuleInitStepFn = extern fn(ModuleInitStepArgs);

// These are callback functions
pub type ModuleOnDataReceivedFn = extern fn(Record);
pub type ModuleTerminationHandlerFn = extern fn(std_types::Uint);