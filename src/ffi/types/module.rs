use crate::ffi::{types::std_types, utils::strings::string_to_cchar};

use super::{buffer::ByteBuffer, collections::Array, functions::{
    ModuleOnDataReceivedFn, ModuleTerminationHandlerFn
}};

/// Module information
#[derive(Clone)]
#[repr(C)]
pub struct ModuleInfo {
    pub id: std_types::ConstCharPtr,
    pub name: std_types::ConstCharPtr,
}

/// Specifies the position of step in pipeline
#[derive(Clone, PartialEq)]
#[repr(C)]
pub enum PipelineStepKind {
    /// Source: produces the data itself, no input from other steps is expected.
    Source,
    /// Transformation: gets the data from the previous step (source or transformation)
    /// and sends the processed result to the next step
    Transformation,
    /// Destination: a final point in the pipeline. Receives the data, but doesn't send it
    /// to any further step
    Destination,
}


/// Arguments passed to init function
#[repr(C)]
pub struct ModuleStepInitArgs {
    pub kind: PipelineStepKind,
    pub step_handle: ModuleStepHandle,
    pub termination_handler: ModuleTerminationHandlerFn,

    pub on_data_received_fn: ModuleOnDataReceivedFn,
}

/// Record metadata. Each item is a key-value pair + a reference to the next record
#[repr(C)]
pub struct RecordMetadata {
    pub name : std_types::ConstCharPtr,
    pub value: std_types::ConstCharPtr,
}

impl From<(String, String)> for RecordMetadata {
    fn from(value: (String, String)) -> Self {
        RecordMetadata {
            name: string_to_cchar(value.0),
            value: string_to_cchar(value.1),
        }
    }
}

/// A single piece of data to transmit. Contains the data itself + metadata
#[repr(C)]
pub struct Record {
    pub content: ByteBuffer,
    pub metadata: Array<RecordMetadata>,
}

unsafe impl Send for Record {}

pub type ModuleStepHandle = std_types::Uint;

/// Returns the status of module initialization
#[repr(C)]
pub enum ModuleStepInitFnResult {
    /// Initialization succeeded
    Ok,
    /// The provided kind (source, transformation, destination) is not supported by module.
    /// Modules don't necessarily can handle all kinds of steps
    ErrorKindNotSupported,
    /// Other kind of error occurred. More details in text message
    ErrorMisc(std_types::ConstCharPtr),
}

// TODO: review if any result is needed to return at all
// TODO2: probably just need to report if data transfer is succeeded
#[repr(C)]
pub enum ModuleProcessRecordFnResult {
    /// No output. Typical for destination module
    None,
    /// Has some output
    Some(Record),
}