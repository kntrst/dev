#[repr(transparent)]
pub struct Tsf{
    pub map: HashMap<SymbolId, ModuleName>
}

pub struct SymbolId{
    id: u32
}
pub struct ModuleName{
    name: String
}