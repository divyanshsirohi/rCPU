use crate::memory::Memory;
use crate::mem_map;
use crate::instructions::Instruction;

pub struct Cpu{
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
    pub fn new() -> Self {
        Self{
            m_pc: mem_map::ROM_START,
            m_sp: mem_map::STACK_START,
            m_regs: [0; 16],
            m_flags: 0,
            m_memory: Memory::new(),
            m_running: false,
        }
    }

    pub fn fetch(&self) -> u32 {
        self.m_memory.read32(self.m_pc as u16)
    }

    pub fn decode(&self, opcode: u32) -> Instruction {
        let mut instruction = Instruction{
            opcode,
            ..Default::default()
        };

        let safe_reg = |offset: u32| -> u8 {
            (self.m_memory.read32((self.m_pc + offset) as u16) & 0x0F) as u8 //keeps only the lower 4 bits
        };

        match opcode{
            0x10 => { // LOAD R1, imm
                instruction.reg1 = safe_reg(4);
                instruction.immediate = self.m_memory.read32((self.m_pc + 8) as u16);
                instruction.length = 12;
            }

            0x11 => { //mov r1,r2 
                instruction.reg1 = safe_reg(4);
                instruction.reg2 = safe_reg(8);
                instruction.length = 12;
            }
            0x30 | 0x31 => { //add r1,r2
                instruction.reg1 = safe_reg(4);
                instruction.reg2 = safe_reg(8);
                instruction.length = 12;
            }
            0x01 => { 
                instruction.length = 4;
            }
            _ => {
                eprintln!("Unknown opcode: {:#04X}", opcode);
                instruction.opcode = 0x01;
                instruction.length = 4;
            }
        }
        instruction
    }
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode{
            0x10 => { //load
                self.m_regs[instruction.reg1 as usize] = instruction.immediate;
            }
            0x11 => {
                //mov r1,r2 => r1 = r2
                self.m_regs[instruction.reg1 as usize] = self.m_regs[instruction.reg2 as usize]
            }
            0x30 => {
                let a: u32;
                let b: u32;
                let result: u32;
                a = self.m_regs[instruction.reg1 as usize];
                b = self.m_regs[instruction.reg2 as usize];
                
            }
            _ => {
                eprintln!("unknown opcode!");
            }
        }
    }
}

