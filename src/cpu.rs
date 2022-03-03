use std::cell::Cell;

use crate::memory::Memory;

mod opcodes;

const HI: u16 = 0xFF00;
const LO: u16 = 0x00FF;

pub struct Cpu {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    memory: Cell<Option<&'static Memory>>,
}

impl Cpu {
    pub fn new(memory: Cell<Option<&'static Memory>>) -> Cpu {
        Cpu {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0,
            memory,
        }
    }

    pub fn connect_memory(&self, memory: &'static Memory) {
        self.memory.set(Some(memory));
    }

    pub fn cycle(&mut self) {
        let instruction = self.read_byte(self.pc);
        self.execute_instruction(instruction);
    }

    fn read_byte(&mut self, pc: u16) -> u8 {
        self.increment_pc();
        match self.memory.get() {
            Some(memory) => memory.read(pc),
            None => 0,
        }
    }

    fn execute_instruction(&mut self, instruction: u8) {
        match instruction {
            0xC3 => self.jp(), // Jump
            _ => panic!("Invalid instruction read: 0x{:X}", instruction),
        }
    }

    fn get_register(&self, register: char) -> u8 {
        match register {
            'A' => return ((HI & self.af) >> 4) as u8,
            'F' => return (LO & self.af) as u8,
            'B' => return ((HI & self.bc) >> 4) as u8,
            'C' => return (LO & self.bc) as u8,
            'D' => return ((HI & self.de) >> 4) as u8,
            'E' => return (LO & self.de) as u8,
            'H' => return ((HI & self.hl) >> 4) as u8,
            'L' => return (LO & self.hl) as u8,
            _ => panic!("Not a valid register: {}", register),
        }
    }

    fn set_register(&mut self, register: char, value: u8) {
        match register {
            'A' => self.af = (value as u16) << 4 | self.get_register('F') as u16,
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
