use crate::ffi::types::module::LibInfo;

use crate::ffi::types::{
    module as module_types,
    std_types,
};

// Pipeline library functions
pub type LibGetInfoFn = extern fn() -> LibInfo;
pub type LibPipelineInitFn = extern fn(module_types::LibPipelineInitArgs);

// Listener library functions

// Listener module routines
pub type ModuleListenerConfigureFn = extern fn(module_types::ModuleListenerConfigureArgs) -> module_types::ModuleListenerConfigureFnResult;
pub type LibListenerInitFn = extern fn(module_types::LibListenerInitArgs);
pub type ModuleListenerRecordRcvFn = extern fn(module_types::ModuleHandle, *const module_types::Record);
pub type ModuleListenerRecordSendSuccessFn = extern fn(module_types::ModuleHandle, *const module_types::Record);
pub type ModuleListenerRecordSendFailureFn = extern fn(module_types::ModuleHandle, *const module_types::Record);

/// Passes a configuration to step
pub type ModulePipelineConfigureFn = extern fn(module_types::ModulePipelineConfigureArgs) -> module_types::ModulePipelineConfigureFnResult;
pub type ModulePipelineProcessRecordFn = extern fn (module_types::ModuleHandle, module_types::Record) -> module_types::ModulePipelineProcessRecordFnResult;
/// Starts the routines inside step (opens HTTP connections, generator threads, message broker consumers, etc)
/// After calling this function the step is ready to process the data
pub type StepStartFn = extern fn(module_types::ModuleHandle) -> module_types::StepStartFnResult;
/// Sets a param for module step. Typicaly param is passed from step definition
pub type StepSetParamFn = extern fn(module_types::ModuleHandle, std_types::ConstCharPtr, std_types::ConstCharPtr);
/// Signals the module step to shut down
pub type ModuleStepShutdownFn = extern fn(module_types::ModuleHandle);

// These are callback functions

/// A callback for received data processed by main app. Arguments are:
/// 1. Step handle to identity the source
/// 2. A record: payload + metadata
pub type ModuleOnDataReceiveCb = extern fn(module_types::ModuleHandle, module_types::Record);
pub type ModuleTerminationHandlerFn = extern fn(std_types::Uint);

// These functions are called from host app

pub type ModuleFreeRecordFn = extern fn(module_types::Record);
pub type ModuleFreeCharPtrFn = extern fn(std_types::ConstCharPtr);