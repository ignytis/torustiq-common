/// Routines which are re-usable in modules

use std::{collections::HashMap, sync::Mutex};

use crate::
    ffi::{
        types::{module::ModuleStepHandle, std_types::ConstCharPtr},
        utils::strings::cchar_to_string,
    };

use once_cell::sync::Lazy;

static MODULE_PARAMS: Lazy<Mutex<HashMap<ModuleStepHandle, HashMap<String, String>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// Sets a parameter for step
#[no_mangle]
pub extern "C" fn torustiq_module_step_set_param(h: ModuleStepHandle, k: ConstCharPtr, v: ConstCharPtr) {
    let mut module_params_container = MODULE_PARAMS.lock().unwrap();
    if !module_params_container.contains_key(&h) {
        module_params_container.insert(h, HashMap::new());
    }
    let step_cfg = module_params_container.get_mut(&h).unwrap();
    step_cfg.insert(cchar_to_string(k), cchar_to_string(v));
}