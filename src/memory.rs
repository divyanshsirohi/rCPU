pub struct Memory {
    cells: [u8; 65536],
}

impl Memory{
    pub fn new() -> Self {
        Self{
            cells: [0; 65536],
        }
    }
    
    //since the memory is byte addressable adding functionality to read and write 8, 16 and 32 bit
    //values
    pub fn read8 (&self, addr: u16) -> u8 { 0 }
    pub fn write8 (&self, addr: u16, data: u8) {}
    pub fn read16 (&self, addr: u16) -> u16 { 0 }
    pub fn write16 (&self, addr: u16, data: u16) {}
    pub fn read32 (&self, addr: u16) -> u32 { 0 }
    pub fn write32 (&self, addr: u16, data:u32) {}
}
