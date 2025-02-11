use std::collections::HashMap;

use crate::ffi::{
    types::std_types,
    utils::strings::{cchar_to_string, string_to_cchar}
};

use super::{buffer::ByteBuffer, collections::Array};
use crate::ffi::types::functions as fn_defs;

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
pub struct LibInfo {
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

#[repr(C)]
#[derive(Clone)]
pub struct LibCommonInitArgs {
    pub on_step_terminate_cb: fn_defs::ModuleTerminationHandlerFn,
}

/// Arguments passed to initialization function of pipeline library
#[repr(C)]
#[derive(Clone)]
pub struct LibPipelineInitArgs {
    pub common: LibCommonInitArgs,
    pub on_data_receive_cb: fn_defs::ModuleOnDataReceiveCb,
}

/// Arguments passed to initialization function of listener library
#[repr(C)]
#[derive(Clone)]
pub struct LibListenerInitArgs {
    pub common: LibCommonInitArgs,
}


/// Arguments passed to init function for listener module
#[repr(C)]
#[derive(Clone)]
pub struct ModuleListenerConfigureArgs {
    pub module_handle: ModuleHandle,
}

/// Arguments passed to configuration function of module
#[repr(C)]
#[derive(Clone)]
pub struct ModulePipelineConfigureArgs {
    pub kind: PipelineModuleKind,
    pub module_handle: ModuleHandle,
}

/// Record metadata. Each item is a key-value pair + a reference to the next record
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct Record {
    pub content: ByteBuffer,
    pub metadata: Array<RecordMetadata>,
}

unsafe impl Send for Record {}

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

    pub fn free_contents(&mut self) {
        self.content.free_contents();
        self.metadata.free_contents();
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
/// For all options the last Boolean argument specifies if the record
/// was consumed (i.e. passed forward to other modules)
/// If the record wasn't consumed, it has to be deallocated
#[repr(C)]
pub enum ModulePipelineProcessRecordFnResult {
    /// Processing succeeded. No immediate error occurred
    Ok(bool),
    ///
    ErrWrongModuleHandle(ModuleHandle, bool),
    /// Cannot proces record due to error
    ErrMisc(std_types::ConstCharPtr, bool),
}