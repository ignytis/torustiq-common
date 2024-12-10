use std::{collections::HashMap, sync::{mpsc::{Receiver, Sender, channel}, Mutex}};
use once_cell::sync::Lazy;
use crate::ffi::types::module::{ModuleHandle, ModulePipelineProcessRecordFnResult, Record};

pub static RECORD_SENDERS: Lazy<Mutex<HashMap<ModuleHandle, Sender<Record>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub static RECORD_RECEIVERS: Lazy<Mutex<HashMap<ModuleHandle, Receiver<Record>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});


#[no_mangle]
extern "C" fn torustiq_module_pipeline_process_record(in_record: Record, module_handle: ModuleHandle) -> ModulePipelineProcessRecordFnResult {
    let mutex = RECORD_SENDERS.lock().unwrap();
    let sender = match mutex.get(&module_handle) {
        Some(s) => s,
        None => return ModulePipelineProcessRecordFnResult::Ok,
    };
    // Cloning the record because the original record will be unallocated in main app
    sender.send(in_record.clone()).unwrap();
    ModulePipelineProcessRecordFnResult::Ok
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