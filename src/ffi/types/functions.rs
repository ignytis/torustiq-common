use crate::ffi::types::module::ModuleInfo;

use super::{
    module::{
        ModuleProcessRecordFnResult, ModuleStepConfigureArgs, ModuleStepConfigureFnResult,
        ModuleStepHandle, ModuleStepStartFnResult, Record}, std_types::{self, ConstCharPtr}
    };

// The following functions are expected to be exported by libraries
pub type ModuleGetInfoFn = extern fn() -> ModuleInfo;
/// Initialization of general module internals.
/// Runs once per module i.e. if pipeline has several steps attached to one module, this function will still run once
pub type ModuleInitFn = extern fn();
/// Passes a configuration to step
pub type ModuleStepConfigureFn = extern fn(ModuleStepConfigureArgs) -> ModuleStepConfigureFnResult;
/// Starts the routines inside step (opens HTTP connections, generator threads, message broker consumers, etc)
/// After calling this function the step is ready to process the data
pub type ModuleStepStartFn = extern fn(ModuleStepHandle) -> ModuleStepStartFnResult;
/// Sets a param for module step. Typicaly param is passed from step definition
pub type ModuleStepSetParamFn = extern fn(ModuleStepHandle, std_types::ConstCharPtr, std_types::ConstCharPtr);
/// Signals the module step to shut down
pub type ModuleStepShutdownFn = extern fn(ModuleStepHandle);
pub type ModuleProcessRecordFn = extern fn (Record, ModuleStepHandle) -> ModuleProcessRecordFnResult;

// These are callback functions

/// A callback for received data processed by main app. Arguments are:
/// 1. A record: payload + metadata
/// 2. Step handle to identity the source
pub type ModuleOnDataReceivedFn = extern fn(Record, ModuleStepHandle);
pub type ModuleTerminationHandlerFn = extern fn(std_types::Uint);


pub type ModuleFreeRecordFn = extern fn(Record);
pub type ModuleFreeCharPtrFn = extern fn(ConstCharPtr);