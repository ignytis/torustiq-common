use core::ffi::c_char;
use crate::{ffi::utils::strings::cchar_to_string,
    types::module::ModuleInfo as RustModuleInfo};

/// Contains the general information about module
#[repr(C)]
pub struct ModuleInfo {
    /// Human-readable name
    pub name: *const c_char,
    /// ID to use as a reference in pipeline config
    pub ref_id: *const c_char,
}

impl Into<RustModuleInfo> for ModuleInfo {
    fn into(self) -> RustModuleInfo {
        RustModuleInfo {
            name: cchar_to_string(self.name),
            ref_id: cchar_to_string(self.ref_id),
        }
    }
}