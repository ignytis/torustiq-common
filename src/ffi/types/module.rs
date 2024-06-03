// use crate::types::module::{Module as RustModule, ModuleType as RustModuleType};
use crate::types::module::{Module as RustModule, IoKind as RustIoKind};

// #[repr(C)]
// pub enum ModuleType {
//     Destination,
//     Source,
//     Transofmation,
// }


// impl Into<RustModuleType> for ModuleType {
//     fn into(self) -> RustModuleType {
//         match self {
//             Self::Destination => RustModuleType::Destination,
//             Self::Source => RustModuleType::Source,
//             Self::Transofmation => RustModuleType::Transofmation,
//         }
//     }
// }

#[repr(C)]
pub enum IoKind {
    Batch,
    /// Defines a module as source if set to input.
    /// Defines a module as destination if set to output.
    External,
    Stream,
}

impl Into<RustIoKind> for IoKind {
    fn into(self) -> RustIoKind {
        match self {
            Self::Batch => RustIoKind::Batch,
            Self::External => RustIoKind::External,
            Self::Stream => RustIoKind::Stream,
        }
    }
}

/// A module returned by dynamic link library
#[repr(C)]
pub struct Module {
    // /// Is it source, destination, or transformation in between
    // pub module_type: ModuleType,

    pub input_kind: IoKind,
    pub output_kind: IoKind,
}

impl Into<RustModule> for Module {
    fn into(self) -> RustModule {
        RustModule {
            input_kind: self.input_kind.into(),
            output_kind: self.output_kind.into(),
        }
    }
}