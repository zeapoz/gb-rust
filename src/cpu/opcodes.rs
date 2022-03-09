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
                self.check_h(b, -1);

                b = b.wrapping_sub(1);

                self.set_register(Register::B, b);

                self.check_z(b);
                self.set_flag(Flag::N);
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
            // Low power standby mode
            0x10 => {
                // TODO
            }
            // Increment D
            0x14 => {
                let mut d = self.get_register(Register::D);
                self.check_h(d, 1);

                d = d.wrapping_add(1);

                self.set_register(Register::D, d);

                self.check_z(d);
                self.unset_flag(Flag::N);
            }
            // Decrement D
            0x15 => {
                let mut d = self.get_register(Register::D);
                self.check_h(d, -1);

                d = d.wrapping_sub(1);

                self.set_register(Register::D, d);

                self.check_z(d);
                self.set_flag(Flag::N);
            }
            // Load H with d8
            0x16 => {
                let byte = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::H, byte);
            }
            // Rotate A right through carry
            0x1F => {
                let mut a = self.get_register(Register::A);
                // Mask check if last digit will be shifted out
                let bit = 1 & a;
                // Shift a and carry right
                a >>= 1;
                a |= (self.get_flag(Flag::C) as u8) << 7;
                self.set_register(Register::A, a);
                // Set carry flag if bit was 1
                if bit == 1 {
                    self.set_flag(Flag::C);
                } else {
                    self.unset_flag(Flag::C);
                }

                self.unset_flag(Flag::Z);
                self.unset_flag(Flag::N);
                self.unset_flag(Flag::H);
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
            // Decrement H
            0x25 => {
                let mut h = self.get_register(Register::H);
                self.check_h(h, -1);

                h = h.wrapping_sub(1);

                self.set_register(Register::H, h);

                self.check_z(h);
                self.set_flag(Flag::N);
            }
            // Load HL with A, decrement HL
            0x32 => {
                self.hl = self.get_register(Register::A) as u16;
                self.hl = self.hl.wrapping_sub(1);
            }
            // Load A with E
            0x7B => {
                let e = self.get_register(Register::E);
                self.set_register(Register::A, e);
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
            // OR B
            0xB0 => {
                let a = self.get_register(Register::A);
                let b = self.get_register(Register::B);
                let value = a | b;
                self.set_register(Register::A, value);

                self.check_z(value);
                self.unset_flag(Flag::N);
                self.unset_flag(Flag::H);
                self.unset_flag(Flag::C);
            }
            // Compare A
            0xBF => {
                let a = self.get_register(Register::A);
                self.check_h(a, -(a as i8));
                self.check_z(0);
                self.set_flag(Flag::N);
                // TODO Check carry
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
