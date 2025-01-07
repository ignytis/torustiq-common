use std::{collections::HashMap, sync::{mpsc::{Receiver, Sender, channel}, Mutex}};
use once_cell::sync::Lazy;
use crate::ffi::types::module::{ModuleHandle, ModulePipelineProcessRecordFnResult, Record};

use log::debug;

pub static RECORD_SENDERS: Lazy<Mutex<HashMap<ModuleHandle, Sender<Record>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub static RECORD_RECEIVERS: Lazy<Mutex<HashMap<ModuleHandle, Receiver<Record>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});


#[no_mangle]
extern "C" fn torustiq_module_pipeline_process_record(module_handle: ModuleHandle, in_record: Record) -> ModulePipelineProcessRecordFnResult {
    let mutex = RECORD_SENDERS.lock().unwrap();
    let sender = match mutex.get(&module_handle) {
        Some(s) => s,
        None => return ModulePipelineProcessRecordFnResult::ErrWrongModuleHandle(module_handle, false),
    };
    sender.send(in_record).unwrap();
    ModulePipelineProcessRecordFnResult::Ok(true)
}

/// Extracts a receiver object from the map and returns it
pub fn get_receiver_owned(handle: ModuleHandle) -> Option<Receiver<Record>> {
    match RECORD_RECEIVERS.lock().unwrap().remove(&handle) {
        Some(r) => Some(r),
        None => None,
    }
}

/// Creates a sender and a receiver; stores them inside module maps
pub fn create_sender_and_receiver(module_handle: ModuleHandle) {
    let (sender, receiver) = channel::<Record>();
    RECORD_RECEIVERS.lock().unwrap().insert(module_handle, receiver);
    RECORD_SENDERS.lock().unwrap().insert(module_handle, sender);
}