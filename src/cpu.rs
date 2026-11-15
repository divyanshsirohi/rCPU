use crate::memory::Memory;
use crate::mem_map;

struct Cpu{
    m_pc: u32, //program counter
    m_sp: u32, //stack pointer - where is the stack top
    m_regs: [u32; 16],
    m_flags: u8, // zero carry negative
    m_memory: Memory,
    m_running: bool, // is my cpu on?
}

impl Cpu{
    const FLAG_Z: u8 = 1<<0;
    const FLAG_C: u8 = 1<<1;
    const FLAG_N: u8 = 1<<2;
    fn new() -> Self {
        Self{
            m_pc: mem_map::ROM_START,
            m_sp: mem_map::STACK_START,
            m_regs: [0; 16],
            m_flags: 0,
            m_memory: Memory::new(),
            m_running: false,
        }
    }

    fn fetch(&self) -> u32 {
        self.m_memory.read32(self.m_pc as u16)
    }
}

