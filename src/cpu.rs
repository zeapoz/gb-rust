use crate::memory::Memory;

mod opcodes;

const HI: u16 = 0xFF;
const LO: u16 = 0x00FF;

pub struct Cpu {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub fn cycle(&mut self, memory: &Memory) {
        let instruction = self.read_instruction(self.pc, memory);
        self.execute_instruction(instruction, memory);
    }

    fn read_instruction(&self, pc: u16, memory: &Memory) -> u8 {
        memory.read(pc)
    }

    fn execute_instruction(&mut self, instruction: u8, memory: &Memory) {
        match instruction {
            0xC3 => self.jp(memory), // Jump
            _ => panic!("Invalid instruction read: 0x{:X}", instruction),
        }
    }

    fn get_register(&self, register: char) -> u16 {
        match register {
            'A' => return HI & self.af,
            'B' => return HI & self.bc,
            'C' => return LO & self.bc,
            'D' => return HI & self.de,
            'E' => return LO & self.de,
            'H' => return HI & self.hl,
            'L' => return LO & self.hl,
            _ => panic!("Not a valid register: {}", register),
        }
    }

    fn get_flag(&self, flag: char) -> bool {
        let f = LO & self.af;
        match flag {
            'z' => return (0b0001 & f) != 0,
            'n' => return (0b0010 & f) != 0,
            'h' => return (0b0100 & f) != 0,
            'c' => return (0b1000 & f) != 0,
            _ => panic!("Not a valid flag: {}", flag),
        }
    }

    fn increment_pc(&mut self) {
        self.pc += 1;
    }
}
