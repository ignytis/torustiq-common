use crate::ffi::types::std_types;

use super::{buffer::ByteBuffer, functions::{
    ModuleOnDataReceivedFn, ModuleTerminationHandlerFn
}};

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
    pub step_handle: std_types::Uint,
    pub termination_handler: ModuleTerminationHandlerFn,

    pub on_data_received_fn: ModuleOnDataReceivedFn,
}

/// A single piece of data to transmit. Contains the data itself + metadata
/// TODO: add metadata
#[repr(C)]
pub struct Record {
    pub content: ByteBuffer,
}