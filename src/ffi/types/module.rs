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
pub struct ModuleStepInitArgs {
    pub step_handle: ModuleStepHandle,
    pub termination_handler: ModuleTerminationHandlerFn,

    pub on_data_received_fn: ModuleOnDataReceivedFn,
}

/// A single piece of data to transmit. Contains the data itself + metadata
/// TODO: add metadata
#[repr(C)]
pub struct Record {
    pub content: ByteBuffer,
}

unsafe impl Send for Record {}

pub type ModuleStepHandle = std_types::Uint;

// TODO: review if any result is needed to return at all
#[repr(C)]
pub enum ModuleProcessRecordFnResult {
    /// No output. Typical for destination module
    None,
    /// Has some output
    Some(Record),
}