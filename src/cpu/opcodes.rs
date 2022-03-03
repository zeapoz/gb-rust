use crate::bus::Bus;

use super::{
    registers::{Flag, Register},
    Cpu,
};

impl Cpu {
    pub fn execute_instruction(&mut self, instruction: u8, bus: &mut Bus) {
        match instruction {
            // NOP
            0x00 => return,
            // Set A = A XOR A
            0xAF => {
                let a = self.get_register(Register::A);
                let value = a ^ a;
                self.set_register(Register::A, value);

                if value == 0 {
                    self.set_flag(Flag::Z);
                }
            }
            // Jump to nn
            0xC3 => {
                let lo = self.read_byte(bus) as u16;
                let hi = self.read_byte(bus) as u16;
                let address = (hi << 8) | lo;
                self.pc = address;
            }
            // Jump to HL
            0xE9 => self.pc = self.hl,
            _ => panic!(
                "Invalid instruction read: 0x{:X} at 0x{:X}",
                instruction,
                self.pc - 1
            ),
        }
    }
}
