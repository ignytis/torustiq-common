/// Routines which are re-usable in modules

use std::{collections::HashMap, sync:: Mutex};

use once_cell::sync::Lazy;

use crate::ffi::types::{
    buffer::free_buf,
    module as module_types,
};

#[cfg(feature="export_type__cchar")]
use crate::ffi::types::std_types::ConstCharPtr;

#[cfg(feature="export_fn__step_set_param")]
use crate::ffi::utils::strings::cchar_to_string;

use super::types::module::ModuleListenerConfigureArgs;

static COMMON_LIB_CONFIGURATION: Lazy<Mutex<Option<module_types::LibCommonInitArgs>>> = Lazy::new(|| {
    Mutex::new(None)
});

static PIPELINE_LIB_CONFIGURATION: Lazy<Mutex<Option<module_types::LibPipelineInitArgs>>> = Lazy::new(|| {
    Mutex::new(None)
});

static LISTENER_LIB_CONFIGURATION: Lazy<Mutex<Option<module_types::LibListenerInitArgs>>> = Lazy::new(|| {
    Mutex::new(None)
});

static PIPELINE_MODULE_CONFIGURATION: Lazy<Mutex<HashMap<module_types::ModuleHandle, module_types::ModulePipelineConfigureArgs>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

static LISTENER_MODULE_CONFIGURATION: Lazy<Mutex<HashMap<module_types::ModuleHandle, ModuleListenerConfigureArgs>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// A 2-dimensional hash map of parameters passed from configuration - like credentials, operating mode, etc
/// Dimension 1: key = module step handle
/// Dimension 2: key = parameter name
static MODULE_PARAMS: Lazy<Mutex<HashMap<module_types::ModuleHandle, HashMap<String, String>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[cfg(feature="export_fn__lib_listener_init")]
#[no_mangle]
extern "C" fn torustiq_lib_listener_init(a: module_types::LibListenerInitArgs) {
    use crate::logging::init_logger;

    set_listener_lib_configuration(a);
    init_logger();
}

#[cfg(feature="export_fn__lib_pipeline_init")]
#[no_mangle]
extern "C" fn torustiq_lib_pipeline_init(a: module_types::LibPipelineInitArgs) {
    use crate::logging::init_logger;

    set_pipeline_lib_configuration(a);
    init_logger();
}

/// Sets a parameter for step
#[cfg(feature="export_fn__step_set_param")]
#[no_mangle]
pub extern "C" fn torustiq_module_common_set_param(h: module_types::ModuleHandle, k: ConstCharPtr, v: ConstCharPtr) {
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
pub extern "C" fn torustiq_module_common_shutdown(h: module_types::ModuleHandle) {
    // No action except forwarding the termination signal back to the main application.
    // Some modules might need additional action like graceful shutdown, exitting from loops etc
    use log::error;
    let cfg = match get_common_lib_configuration() {
        Some(c) => c,
        None => {
            error!("torustiq_module_common_shutdown: Failed to load the library configuration");
            return;
        }
    };
    (cfg.on_step_terminate_cb)(h);
}

/// Deallocates memory for a record
#[cfg(feature="export_fn__free_record")]
#[no_mangle]
pub extern "C" fn torustiq_module_pipeline_free_record(r: module_types::Record) {
    do_free_record(r);
}

/// Deallocates memory for a C-string
#[cfg(feature="export_fn__free_char_ptr")]
#[no_mangle]
pub extern "C" fn torustiq_module_common_free_char(c: ConstCharPtr) {
    use super::utils::strings::cchar_const_deallocate;

    cchar_const_deallocate(c);
}

pub fn do_free_record(r: module_types::Record) {
    free_buf(r.content);
}

pub fn set_listener_lib_configuration(a: module_types::LibListenerInitArgs) {
    *COMMON_LIB_CONFIGURATION.lock().unwrap() = Some(a.common.clone());
    *LISTENER_LIB_CONFIGURATION.lock().unwrap() = Some(a);
}

pub fn get_listener_lib_configuration() -> Option<module_types::LibListenerInitArgs> {
    LISTENER_LIB_CONFIGURATION.lock().unwrap().clone()
}

pub fn set_pipeline_lib_configuration(a: module_types::LibPipelineInitArgs) {
    *COMMON_LIB_CONFIGURATION.lock().unwrap() = Some(a.common.clone());
    *PIPELINE_LIB_CONFIGURATION.lock().unwrap() = Some(a);
}

pub fn get_pipeline_lib_configuration() -> Option<module_types::LibPipelineInitArgs> {
    PIPELINE_LIB_CONFIGURATION.lock().unwrap().clone()
}

pub fn get_common_lib_configuration() -> Option<module_types::LibCommonInitArgs> {
    COMMON_LIB_CONFIGURATION.lock().unwrap().clone()
}

pub fn set_pipeline_module_configuration(a: module_types::ModulePipelineConfigureArgs) {
    PIPELINE_MODULE_CONFIGURATION.lock().unwrap().insert(a.module_handle, a);
}

pub fn get_pipeline_module_configuration(h: module_types::ModuleHandle) -> Option<module_types::ModulePipelineConfigureArgs> {
    let module_params_container = PIPELINE_MODULE_CONFIGURATION.lock().unwrap();
    match module_params_container.get(&h) {
        Some(c) => Some(c.clone()),
        None => None,
    }
}

pub fn set_listener_module_configuration(a: ModuleListenerConfigureArgs) {
    LISTENER_MODULE_CONFIGURATION.lock().unwrap().insert(a.module_handle, a);
}

pub fn get_listener_module_configuration(h: module_types::ModuleHandle) -> Option<ModuleListenerConfigureArgs> {
    let module_params_container = LISTENER_MODULE_CONFIGURATION.lock().unwrap();
    match module_params_container.get(&h) {
        Some(c) => Some(c.clone()),
        None => None,
    }
}

pub fn get_params(h: module_types::ModuleHandle) -> Option<HashMap<String, String>> {
    let module_params_container = MODULE_PARAMS.lock().unwrap();
    match module_params_container.get(&h) {
        Some(params) => Some(params.clone()),
        None => None,
    }
}

pub fn get_param<S: Into<String>>(h: module_types::ModuleHandle, k: S) -> Option<String> {
    let module_params_container = MODULE_PARAMS.lock().unwrap();
    match module_params_container.get(&h) {
        Some(params) => match params.get(&(k.into())) {
            Some(s) => Some(s.clone()),
            None => None,
        },
        None => None,
    }
}