use crate::ffi::types::std_types;

#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub enum IoKind {
    Batch,
    /// Defines a module as source if set to input
    /// Defines a module as destination if set to output.
    External,
    Stream,
}

/// Module information
#[derive(Clone)]
#[repr(C)]
pub struct ModuleInfo {
    pub id: std_types::ConstCharPtr,
    pub name: std_types::ConstCharPtr,
    pub input_kind: IoKind,
    pub output_kind: IoKind,
}

/// Arguments passed to init function
#[repr(C)]
pub struct ModuleInitStepArgs {
    pub termination_handler: extern "C" fn(),
}
