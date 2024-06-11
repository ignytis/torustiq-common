use crate::{
    ffi::{types::std_types, utils::strings},
    types::module::{Module as RustModule, IoKind as RustIoKind},
};

#[repr(C)]
pub enum IoKind {
    Batch,
    /// Defines a module as source if set to input
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
    pub id: std_types::ConstCharPtr,
    pub name: std_types::ConstCharPtr,
    pub input_kind: IoKind,
    pub output_kind: IoKind,

    pub init: extern "C" fn(extern "C" fn()),
}

impl Into<RustModule> for Module {
    fn into(self) -> RustModule {
        RustModule {
            id: strings::cchar_to_string(self.id),
            name: strings::cchar_to_string(self.name),
            input_kind: self.input_kind.into(),
            output_kind: self.output_kind.into(),

            init: self.init,
        }
    }
}