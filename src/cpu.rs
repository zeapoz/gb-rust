use crate::bus::Bus;

use self::{
    opcodes::AddressingMode,
    registers::{Flag, Register},
};

mod opcodes;
mod registers;

const HI: u16 = 0xFF00;
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
            pc: 0x100,
        }
    }

    pub fn cycle(&mut self, bus: &mut Bus) {
        let instruction = self.fetch_byte(bus);
        self.execute_instruction(instruction, bus);
    }

    fn fetch_byte(&mut self, bus: &mut Bus) -> u8 {
        let data = bus.read(self.pc);
        self.increment_pc();
        data
    }

    fn fetch_data(&mut self, bus: &mut Bus, addressing_mode: AddressingMode) -> u16 {
        match addressing_mode {
            AddressingMode::D8 | AddressingMode::A8 | AddressingMode::R8 => {
                return bus.read(self.pc) as u16;
            }
            AddressingMode::A16 | AddressingMode::D16 => {
                let lo = self.fetch_byte(bus) as u16;
                let hi = self.fetch_byte(bus) as u16;
                return (hi << 8) | lo;
            }
            _ => panic!("No addressing mode specified"),
        }
    }

    fn get_register(&self, register: Register) -> u8 {
        match register {
            Register::A => return ((HI & self.af) >> 8) as u8,
            Register::F => return (LO & self.af) as u8,
            Register::B => return ((HI & self.bc) >> 8) as u8,
            Register::C => return (LO & self.bc) as u8,
            Register::D => return ((HI & self.de) >> 8) as u8,
            Register::E => return (LO & self.de) as u8,
            Register::H => return ((HI & self.hl) >> 8) as u8,
            Register::L => return (LO & self.hl) as u8,
            _ => panic!("Not a valid register"),
        }
    }

    fn set_register(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.af = (value as u16) << 8 | self.get_register(Register::F) as u16,
            Register::F => self.af = (self.get_register(Register::F) as u16) << 8 | (value as u16),
            Register::B => self.af = (value as u16) << 8 | self.get_register(Register::C) as u16,
            Register::C => self.bc = (self.get_register(Register::B) as u16) << 8 | (value as u16),
            Register::D => self.de = (value as u16) << 8 | self.get_register(Register::E) as u16,
            Register::E => self.bc = (self.get_register(Register::D) as u16) << 8 | (value as u16),
            Register::H => self.hl = (value as u16) << 8 | self.get_register(Register::L) as u16,
            Register::L => self.bc = (self.get_register(Register::H) as u16) << 8 | (value as u16),
            _ => panic!("Not a valid register"),
        }
    }

    fn get_flag(&self, flag: Flag) -> bool {
        let f = self.get_register(Register::F);
        match flag {
            Flag::Z => return (0b10000000 & f) != 0,
            Flag::N => return (0b01000000 & f) != 0,
            Flag::H => return (0b00100000 & f) != 0,
            Flag::C => return (0b00010000 & f) != 0,
            _ => panic!("Not a valid flag"),
        }
    }

    fn set_flag(&mut self, flag: Flag) {
        let f = self.get_register(Register::F);
        match flag {
            Flag::Z => self.set_register(Register::F, 0b10000000 | f),
            Flag::N => self.set_register(Register::F, 0b01000000 | f),
            Flag::H => self.set_register(Register::F, 0b00100000 | f),
            Flag::C => self.set_register(Register::F, 0b00010000 | f),
            _ => panic!("Not a valid flag"),
        };
    }

    fn unset_flag(&mut self, flag: Flag) {
        let f = self.get_register(Register::F);
        match flag {
            Flag::Z => self.set_register(Register::F, 0b01111111 & f),
            Flag::N => self.set_register(Register::F, 0b10111111 & f),
            Flag::H => self.set_register(Register::F, 0b11011111 & f),
            Flag::C => self.set_register(Register::F, 0b11101111 & f),
            _ => panic!("Not a valid flag"),
        };
    }

    fn check_z(&mut self, value: u16) {
        match value {
            0 => self.set_flag(Flag::Z),
            _ => self.unset_flag(Flag::Z),
        };
    }

    fn check_h(&mut self, a: u8, b: i8) {
        // Check to see if lower 4 bits will result in a carry
        if (((a & 0x0F) + (b & 0x0F) as u8) & 0x10) == 0x10 {
            self.set_flag(Flag::H);
        } else {
            self.unset_flag(Flag::H);
        }
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
}
