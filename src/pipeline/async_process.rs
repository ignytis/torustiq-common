use std::{collections::HashMap, sync::{mpsc::{Receiver, Sender, channel}, Mutex}};
use once_cell::sync::Lazy;
use crate::ffi::types::module::{ModuleStepHandle, ModuleProcessRecordFnResult, Record};

pub static RECORD_SENDERS: Lazy<Mutex<HashMap<ModuleStepHandle, Sender<Record>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub static RECORD_RECEIVERS: Lazy<Mutex<HashMap<ModuleStepHandle, Receiver<Record>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});


#[no_mangle]
extern "C" fn torustiq_module_process_record(in_record: Record, step_handle: ModuleStepHandle) -> ModuleProcessRecordFnResult {
    let mutex = RECORD_SENDERS.lock().unwrap();
    let sender = match mutex.get(&step_handle) {
        Some(s) => s,
        None => return ModuleProcessRecordFnResult::Ok,
    };
    // Cloning the record because the original record will be unallocated in main app
    sender.send(in_record.clone()).unwrap();
    ModuleProcessRecordFnResult::Ok
}

/// Extracts a receiver object from the map and returns it
pub fn get_receiver_owned(handle: ModuleStepHandle) -> Option<Receiver<Record>> {
    match RECORD_RECEIVERS.lock().unwrap().remove(&handle) {
        Some(r) => Some(r),
        None => None,
    }
}

/// Creates a sender and a receiver; stores them inside module maps
pub fn create_sender_and_receiver(step_handle: ModuleStepHandle) {
    let (sender, receiver) = channel::<Record>();
    RECORD_RECEIVERS.lock().unwrap().insert(step_handle, receiver);
    RECORD_SENDERS.lock().unwrap().insert(step_handle, sender);
}