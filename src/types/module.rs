#[derive(Clone, PartialEq, Debug)]
pub enum IoKind {
    Batch,
    /// Defines a module as source if set to input.
    /// Defines a module as destination if set to output.
    External,
    Stream,
}

#[derive(Clone)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub input_kind: IoKind,
    pub output_kind: IoKind,

    pub init: extern fn(),
}