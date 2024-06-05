use crate::{
    ffi::{types::std, utils::strings},
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
    pub id: std::ConstCharPtr,
    pub name: std::ConstCharPtr,
    pub input_kind: IoKind,
    pub output_kind: IoKind,

    pub init: extern "C" fn(),
}

impl Into<RustModule> for Module {
    fn into(self) -> RustModule {
        (self.init)();
        let m = RustModule {
            id: strings::cchar_to_string(self.id),
            name: strings::cchar_to_string(self.name),
            input_kind: self.input_kind.into(),
            output_kind: self.output_kind.into(),

            init: self.init,
        };
        (m.init)();
        m
    }
}