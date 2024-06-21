use crate::ffi::types::module::ModuleInfo;

use super::{module::{ModuleInitStepArgs, ModuleStepHandle, ModuleProcessRecordFnResult, Record}, std_types};

// The following functions are expected to be exported by libraries
pub type ModuleGetInfoFn = extern fn() -> ModuleInfo;
pub type ModuleInitStepFn = extern fn(ModuleInitStepArgs);
pub type ModuleProcessRecordFn = extern fn (Record, ModuleStepHandle) -> ModuleProcessRecordFnResult;

// These are callback functions

/// A callback for received data processed by main app. Arguments are:
/// 1. A record: payload + metadata
/// 2. Step handle to identity the source
pub type ModuleOnDataReceivedFn = extern fn(Record, ModuleStepHandle);
pub type ModuleTerminationHandlerFn = extern fn(std_types::Uint);