use core::ffi::c_char;

#[repr(C)]
pub struct Module {
    /// Human-readable name
    pub name: *const c_char,
    /// ID to use as a reference in pipeline config
    pub ref_id: *const c_char,
}