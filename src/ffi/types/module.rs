use crate::types::module::{Module as RustModule, ModuleType as RustModuleType};

#[repr(C)]
pub enum ModuleType {
    Destination,
    Source,
    Transofmation,
}

impl Into<RustModuleType> for ModuleType {
    fn into(self) -> RustModuleType {
        match self {
            Self::Destination => RustModuleType::Destination,
            Self::Source => RustModuleType::Source,
            Self::Transofmation => RustModuleType::Transofmation,
        }
    }
}

#[repr(C)]
pub struct Module {
    pub module_type: ModuleType,
}

impl Into<RustModule> for Module {
    fn into(self) -> RustModule {
        RustModule {
            module_type: self.module_type.into(),
        }
    }
}