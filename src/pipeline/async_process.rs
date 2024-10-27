#[cfg(feature="pipeline_module_async_process")]
use std::{collections::HashMap, sync::{mpsc::{Receiver, Sender}, Mutex}};
#[cfg(feature="pipeline_module_async_process")]
use crate::ffi::types::module::{ModuleStepHandle, ModuleProcessRecordFnResult, Record};
#[cfg(feature="pipeline_module_async_process")]
use once_cell::sync::Lazy;

#[cfg(feature="pipeline_module_async_process")]
pub static RECORD_SENDERS: Lazy<Mutex<HashMap<ModuleStepHandle, Sender<Record>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});
#[cfg(feature="pipeline_module_async_process")]
pub static RECORD_RECEIVERS: Lazy<Mutex<HashMap<ModuleStepHandle, Receiver<Record>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[cfg(feature="pipeline_module_async_process")]
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

#[cfg(feature="pipeline_module_async_process")]
pub fn get_receiver_owned(handle: ModuleStepHandle) -> Option<Receiver<Record>> {
    match RECORD_RECEIVERS.lock().unwrap().remove(&handle) {
        Some(r) => Some(r),
        None => None,
    }
}

#[cfg(feature="pipeline_module_async_process")]
pub fn add_receiver(handle: ModuleStepHandle, receiver: Receiver<Record>) {
    RECORD_RECEIVERS.lock().unwrap().insert(handle, receiver);
}