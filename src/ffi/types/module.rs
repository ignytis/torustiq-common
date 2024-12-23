use std::collections::HashMap;

use crate::ffi::{
    types::std_types,
    utils::strings::{cchar_to_string, string_to_cchar}
};

use super::{buffer::ByteBuffer, collections::Array, functions::{
    ModuleOnDataReceiveCb, ModuleTerminationHandlerFn
}, traits::ShallowCopy};

#[derive(Clone)]
#[repr(C)]
pub enum ModuleKind {
    /// A pipeline module. Extracts, transforms, loads the data.
    Pipeline,
    /// An event listener module. Reacts to application events.
    Listener,
}

/// Module information
#[derive(Clone)]
#[repr(C)]
pub struct ModuleInfo {
    pub api_version: std_types::Uint,
    pub id: std_types::ConstCharPtr,
    pub kind: ModuleKind,
    pub name: std_types::ConstCharPtr,
}

/// Specifies the position of step in pipeline
#[derive(Clone, PartialEq)]
#[repr(C)]
pub enum PipelineModuleKind {
    /// Source: produces the data itself, no input from other steps is expected.
    Source,
    /// Transformation: gets the data from the previous step (source or transformation)
    /// and sends the processed result to the next step
    Transformation,
    /// Destination: a final point in the pipeline. Receives the data, but doesn't send it
    /// to any further step
    Destination,
}

/// Arguments passed to init function for listener module
#[repr(C)]
#[derive(Clone)]
pub struct ModuleListenerConfigureArgs {
    pub module_handle: ModuleHandle,
}

/// Arguments passed to init function for pipeline module
#[repr(C)]
#[derive(Clone)]
pub struct ModulePipelineConfigureArgs {
    pub kind: PipelineModuleKind,
    pub module_handle: ModuleHandle,
    pub on_step_terminate_cb: ModuleTerminationHandlerFn,
    pub on_data_receive_cb: ModuleOnDataReceiveCb,
}

/// Record metadata. Each item is a key-value pair + a reference to the next record
#[repr(C)]
#[derive(Clone)]
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
#[derive(Clone)]
pub struct Record {
    pub content: ByteBuffer,
    pub metadata: Array<RecordMetadata>,
}

unsafe impl Send for Record {}

impl ShallowCopy for Record {
    fn shallow_copy(&self) -> Self {
        Self {
            content: self.content.shallow_copy(),
            metadata: self.metadata.shallow_copy(),
        }
    }
}

impl Record {
    /// Creates a record from standard types: content (vector of bytes)
    /// and metadata (string hashmap)
    pub fn from_std_types(content: Vec<u8>, metadata: HashMap<String, String>) -> Self {
        let metadata_vec: Vec<RecordMetadata> = metadata
            .into_iter()
            .map(|kv| kv.into()).collect();
        Record {
            content: ByteBuffer::from(content),
            metadata: Array::from_vec(metadata_vec),
        }
    }

    /// Returns metadata as hashmap of string key-value pairs
    pub fn get_metadata_as_hashmap(&self) -> HashMap<String, String> {
        let mtd_len = self.metadata.len as usize;
        let metadata: Vec<RecordMetadata> = unsafe { Vec::from_raw_parts(self.metadata.data, mtd_len, mtd_len) };
        metadata.into_iter()
            .map(|record| (cchar_to_string(record.name), cchar_to_string(record.value)))
            .collect::<HashMap<String, String>>()
    }

    pub fn get_content_len(&self) -> usize {
        self.content.len
    }
}

pub type ModuleHandle = std_types::Uint;

/// Returns the status of listener module configuration
#[repr(C)]
pub enum ModuleListenerConfigureFnResult {
    /// Configuration succeeded
    Ok,
    /// Other kind of error occurred. More details in text message
    ErrorMisc(std_types::ConstCharPtr),
}

/// Returns the status of pipeline module configuration
#[repr(C)]
pub enum ModulePipelineConfigureFnResult {
    /// Configuration succeeded
    Ok,
    /// The provided kind (source, transformation, destination) is not supported by module.
    /// Modules don't necessarily can handle all kinds of steps
    ErrorKindNotSupported,
    /// Module can be used in one step only.
    /// Some modules can have issues with having initialized for multiple steps
    /// Argument is a handle of previously initialized module which caused a conflict
    ErrorMultipleStepsNotSupported(ModuleHandle),
    /// Other kind of error occurred. More details in text message
    ErrorMisc(std_types::ConstCharPtr),
}

/// Returns the status of module step start
#[repr(C)]
pub enum StepStartFnResult {
    /// Started successfully
    Ok,
    /// Other kind of error occurred. More details in text message
    ErrorMisc(std_types::ConstCharPtr),
}

/// A result of sending a record to further processing
#[repr(C)]
pub enum ModulePipelineProcessRecordFnResult {
    /// Processing succeeded. No immediate error occurred
    Ok,
    /// Cannot proces record due to error
    Err(std_types::ConstCharPtr),
}