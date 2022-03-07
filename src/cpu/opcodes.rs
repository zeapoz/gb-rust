use crate::bus::Bus;

use super::{
    registers::{Flag, Register},
    Cpu,
};

pub enum AddressingMode {
    D8,
    D16,
    A8,
    A16,
    R8,
}

impl Cpu {
    pub fn execute_instruction(&mut self, instruction: u8, bus: &mut Bus) {
        println!("Executing instruction: 0x{:X}", instruction);
        match instruction {
            // NOP
            0x00 => return,
            // Decrement B
            0x05 => {
                let mut b = self.get_register(Register::B);
                b = b.wrapping_sub(1);
                self.set_register(Register::B, b);

                self.check_z(b);
                self.set_flag(Flag::N);
                // TODO check H flag
            }
            // Load B with d8
            0x06 => {
                let byte = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::B, byte);
            }
            // Load C with d8
            0x0E => {
                let byte = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::C, byte);
            }
            // Low power standy mode
            0x10 => {
                // TODO
            }
            // Load H with d8
            0x16 => {
                let byte = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::H, byte);
            }
            // Conditional relative jump if not Z
            0x20 => {
                if !self.get_flag(Flag::Z) {
                    let byte = self.fetch_data(bus, AddressingMode::R8);
                    self.pc = self.pc.wrapping_add(byte);
                }
            }
            // Load HL with d16
            0x21 => {
                let bytes = self.fetch_data(bus, AddressingMode::D16);
                self.hl = bytes;
            }
            // Load HL with A, decrement HL
            0x32 => {
                self.hl = self.get_register(Register::A) as u16;
                self.hl = self.hl.wrapping_sub(1);
            }
            // Set A = A XOR A
            0xAF => {
                let a = self.get_register(Register::A);
                let value = a ^ a;
                self.set_register(Register::A, value);

                self.check_z(value);
                self.unset_flag(Flag::N);
                self.unset_flag(Flag::H);
                self.unset_flag(Flag::C);
            }
            // Jump to nn
            0xC3 => {
                let address = self.fetch_data(bus, AddressingMode::A16);
                self.pc = address;
            }
            // Call to 18
            0xDF => {
                // TODO
            }
            // Jump to HL
            0xE9 => self.pc = self.hl,
            // Call to 38
            0xFF => {
                // TODO
            }
            _ => panic!(
                "Invalid instruction read: 0x{:X} at 0x{:X}",
                instruction,
                self.pc - 1
            ),
        }
    }
}
