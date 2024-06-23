use crate::ffi::types::module::ModuleInfo;

use super::{module::{ModuleStepInitArgs, ModuleStepHandle, ModuleProcessRecordFnResult, Record}, std_types};

// The following functions are expected to be exported by libraries
pub type ModuleGetInfoFn = extern fn() -> ModuleInfo;
/// Initialization of general module internals.
/// Runs once per module i.e. if pipeline has several steps attached to one module, this function will still run once
pub type ModuleInitFn = extern fn();
pub type ModuleStepInitFn = extern fn(ModuleStepInitArgs);
/// Sets a param for module step. Typicaly param is passed from step definition
pub type ModuleStepSetParamFn = extern fn(ModuleStepHandle, std_types::ConstCharPtr, std_types::ConstCharPtr);
pub type ModuleProcessRecordFn = extern fn (Record, ModuleStepHandle) -> ModuleProcessRecordFnResult;

// These are callback functions

/// A callback for received data processed by main app. Arguments are:
/// 1. A record: payload + metadata
/// 2. Step handle to identity the source
pub type ModuleOnDataReceivedFn = extern fn(Record, ModuleStepHandle);
pub type ModuleTerminationHandlerFn = extern fn(std_types::Uint);