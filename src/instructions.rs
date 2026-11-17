#[derive(Default)]
pub struct Instruction{
    pub opcode: u32,
    pub reg1: u8,
    pub reg2: u8,
    pub address: u16,
    pub immediate: u32, //for immediate values
    pub length: u8,
}
