/// Routines which are re-usable in modules

use std::{collections::HashMap, sync:: Mutex};

use once_cell::sync::Lazy;

use crate::ffi::types::{
    buffer::free_buf,
    module::{ModuleStepHandle, ModulePipelineStepConfigureArgs, Record}
};

#[cfg(feature="export_type__cchar")]
use crate::ffi::types::std_types::ConstCharPtr;

#[cfg(feature="export_fn__step_set_param")]
use crate::ffi::utils::strings::cchar_to_string;

static MODULE_STEP_CONFIGURATION: Lazy<Mutex<HashMap<ModuleStepHandle, ModulePipelineStepConfigureArgs>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// A 2-dimensional hash map of parameters passed from configuration - like credentials, operating mode, etc
/// Dimension 1: key = module step handle
/// Dimension 2: key = parameter name
static MODULE_PARAMS: Lazy<Mutex<HashMap<ModuleStepHandle, HashMap<String, String>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// Sets a parameter for step
#[cfg(feature="export_fn__step_set_param")]
#[no_mangle]
pub extern "C" fn torustiq_step_set_param(h: ModuleStepHandle, k: ConstCharPtr, v: ConstCharPtr) {
    let mut module_params_container = MODULE_PARAMS.lock().unwrap();
    if !module_params_container.contains_key(&h) {
        module_params_container.insert(h, HashMap::new());
    }
    let step_cfg = module_params_container.get_mut(&h).unwrap();
    step_cfg.insert(cchar_to_string(k), cchar_to_string(v));
}

/// Called by main application to trigger the shutdown
#[cfg(feature="export_fn__step_shutdown")]
#[no_mangle]
pub extern "C" fn torustiq_module_step_shutdown(h: ModuleStepHandle) {
    // No action except forwarding the termination signal back to the main application.
    // Some modules might need additional action like graceful shutdown, exitting from loops etc
    use log::error;
    let cfg = match get_step_configuration(h) {
        Some(c) => c,
        None => {
            error!("An unregistered module handle provided: {}", h);
            return;
        }
    };
    (cfg.on_step_terminate_cb)(h);
}


/// Deallocates memory for a record
#[cfg(feature="export_fn__free_record")]
#[no_mangle]
pub extern "C" fn torustiq_module_free_record(r: Record) {
    do_free_record(r);
}

/// Deallocates memory for a record
#[cfg(feature="export_fn__free_char_ptr")]
#[no_mangle]
pub extern "C" fn torustiq_module_free_char_ptr(c: ConstCharPtr) {
    use super::utils::strings::cchar_const_deallocate;

    cchar_const_deallocate(c);
}


pub fn do_free_record(r: Record) {
    free_buf(r.content);
}

pub fn get_step_configuration(h: ModuleStepHandle) -> Option<ModulePipelineStepConfigureArgs> {
    let module_params_container = MODULE_STEP_CONFIGURATION.lock().unwrap();
    match module_params_container.get(&h) {
        Some(c) => Some(c.clone()),
        None => None,
    }
}

pub fn set_step_configuration(a: ModulePipelineStepConfigureArgs) {
    MODULE_STEP_CONFIGURATION.lock().unwrap().insert(a.step_handle, a);
}

pub fn get_params(h: ModuleStepHandle) -> Option<HashMap<String, String>> {
    let module_params_container = MODULE_PARAMS.lock().unwrap();
    match module_params_container.get(&h) {
        Some(params) => Some(params.clone()),
        None => None,
    }
}

pub fn get_param<S: Into<String>>(h: ModuleStepHandle, k: S) -> Option<String> {
    let module_params_container = MODULE_PARAMS.lock().unwrap();
    match module_params_container.get(&h) {
        Some(params) => match params.get(&(k.into())) {
            Some(s) => Some(s.clone()),
            None => None,
        },
        None => None,
    }
}