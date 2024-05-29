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

// deserializing:
// 1) from_str => Aws => Command
// 2) from_bytes => Awb => Command

// serializing
// 1) as_str => Aws => String
// 2) as_vec => Awb => Vec<u8>

struct Command{
    instance_id: InstanceId,
    payload: Payload
}
struct InstanceId{
    id: u16
}
enum Payload{
    AudioPump(AudioPump),
    AudioStop(AudioStop),
    SetValue(SetValue),
    GetValue(GetValue),
    SetValues(SetValues),
    GetValues(GetValues)
}
struct AudioPump{}
struct AudioStop{}
struct SetValue{
    module: Module,
    variable: Variable,
    value: Value
}
struct Module{
    id: u32,
    name: String
}
struct Variable{
    mask: u32,
    offset: u32,
    name: String
}
enum Value{
    Uint(u32),
    Int(i32),
    Float(f32)
}
impl Command{
    fn as_str(&mut self) -> Result<String>{
        let aws = Aws::from(self)
        aws.deserialize()
    }
}
impl 
#[cfg(test)]
mod tests;