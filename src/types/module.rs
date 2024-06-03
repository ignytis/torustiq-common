// pub enum ModuleType {
//     Source,
//     Transofmation,
//     Destination,
// }

pub enum IoKind {
    Batch,
    /// Defines a module as source if set to input.
    /// Defines a module as destination if set to output.
    External,
    Stream,
}

pub struct Module {
    // pub module_type: ModuleType,
    pub input_kind: IoKind,
    pub output_kind: IoKind,
}