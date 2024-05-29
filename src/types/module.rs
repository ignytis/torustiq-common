/// Contains the general information about module
#[repr(C)]
pub struct ModuleInfo {
    /// Human-readable name
    pub name: String,
    /// ID to use as a reference in pipeline config
    pub ref_id: String,
}