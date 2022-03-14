use crate::bus::Bus;

use self::{opcodes::AddressingMode, registers::Flag};

mod opcodes;
mod registers;

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
