pub enum ModuleType {
    Source,
    Transofmation,
    Destination,
}

pub struct Module {
    pub module_type: ModuleType,
}