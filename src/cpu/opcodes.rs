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
        println!(
            "0x{:02X}:  Executing instruction: 0x{:02X}",
            self.pc - 1,
            instruction
        );
        match instruction {
            // NOP
            0x00 => return,
            // Increment BC
            0x03 => {
                self.bc = self.bc.wrapping_add(1);
            }
            // Decrement B
            0x05 => {
                let mut b = self.get_register(Register::B);
                self.check_h(b, -1);

                b = b.wrapping_sub(1);

                self.set_register(Register::B, b);

                self.check_z(b as u16);
                self.set_flag(Flag::N);
            }
            // Load B with d8
            0x06 => {
                let byte = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::B, byte);
            }
            // Rotate A left
            0x07 => {
                let mut a = self.get_register(Register::A);
                // Mask check if last digit will be shifted out
                let bit = 0x80 & a;
                // Shift a left
                a <<= 1;
                a |= bit >> 7;
                self.set_register(Register::A, a);
                // Set carry flag if bit was 1
                if bit == 0x80 {
                    self.set_flag(Flag::C);
                } else {
                    self.unset_flag(Flag::C);
                }

                self.unset_flag(Flag::Z);
                self.unset_flag(Flag::N);
                self.unset_flag(Flag::H);
            }
            // LD SP with a16
            0x08 => {
                let address = self.fetch_data(bus, AddressingMode::A16);
                self.sp = address;
            }
            // Load A with BC
            0x0A => {
                self.set_register(Register::A, self.bc as u8);
            }
            // Increment C
            0x0C => {
                let mut c = self.get_register(Register::C);
                self.check_h(c, 1);

                c = c.wrapping_add(1);
                self.set_register(Register::C, c);

                self.check_z(c as u16);
                self.unset_flag(Flag::N);
            }
            // Decrement C
            0x0D => {
                let mut c = self.get_register(Register::C);
                self.check_h(c, -1);

                c = c.wrapping_sub(1);
                self.set_register(Register::C, c);

                self.check_z(c as u16);
                self.set_flag(Flag::N);
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
            // Load DE with d16
            0x11 => {
                let address = self.fetch_data(bus, AddressingMode::D16);
                self.de = address;
            }
            // Load DE with A
            0x12 => {
                self.de = self.get_register(Register::A) as u16;
            }
            // Increment DE
            0x13 => {
                self.de = self.de.wrapping_add(1);
            }
            // Increment D
            0x14 => {
                let mut d = self.get_register(Register::D);
                self.check_h(d, 1);

                d = d.wrapping_add(1);
                self.set_register(Register::D, d);

                self.check_z(d as u16);
                self.unset_flag(Flag::N);
            }
            // Decrement D
            0x15 => {
                let mut d = self.get_register(Register::D);
                self.check_h(d, -1);

                d = d.wrapping_sub(1);
                self.set_register(Register::D, d);

                self.check_z(d as u16);
                self.set_flag(Flag::N);
            }
            // Load H with d8
            0x16 => {
                let byte = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::H, byte);
            }
            // Add DE to HL
            0x19 => {
                self.hl = self.hl.wrapping_add(self.de);

                self.unset_flag(Flag::N);
                self.check_h(self.hl as u8, self.de as i8);
                // TODO check carry
            }
            // Decrement E
            0x1D => {
                let mut e = self.get_register(Register::E);
                self.check_h(e, -1);

                e = e.wrapping_sub(1);

                self.set_register(Register::E, e);

                self.check_z(e as u16);
                self.set_flag(Flag::N);
            }
            // Load E with d8
            0x1E => {
                let data = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::E, data);
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
            // Increment HL
            0x23 => {
                self.hl = self.hl.wrapping_add(1);
            }
            // Increment H
            0x24 => {
                let mut h = self.get_register(Register::H);
                self.check_h(h, 1);

                h = h.wrapping_add(1);
                self.set_register(Register::H, h);

                self.check_z(h as u16);
                self.unset_flag(Flag::N);
            }
            // Decrement H
            0x25 => {
                let mut h = self.get_register(Register::H);
                self.check_h(h, -1);

                h = h.wrapping_sub(1);
                self.set_register(Register::H, h);

                self.check_z(h as u16);
                self.set_flag(Flag::N);
            }
            // Add HL to HL
            0x29 => {
                self.hl = self.hl.wrapping_add(self.hl);

                self.unset_flag(Flag::N);
                self.check_h(self.hl as u8, self.hl as i8);
                // TODO check carry
            }
            // Increment L
            0x2C => {
                let mut l = self.get_register(Register::L);
                self.check_h(l, 1);

                l = l.wrapping_add(1);
                self.set_register(Register::L, l);

                self.check_z(l as u16);
                self.unset_flag(Flag::N);
            }
            // LD SP with d16
            0x31 => {
                let address = self.fetch_data(bus, AddressingMode::D16);
                self.sp = address;
            }

            // TODO Refactor load in seperate method

            // Load HL with A, decrement HL
            0x32 => {
                self.hl = self.get_register(Register::A) as u16;
                self.hl = self.hl.wrapping_sub(1);
            }
            // Load B with C
            0x41 => {
                let c = self.get_register(Register::C);
                self.set_register(Register::B, c);
            }
            // Load H with B
            0x60 => {
                let b = self.get_register(Register::B);
                self.set_register(Register::H, b);
            }
            // Load H with C
            0x61 => {
                let c = self.get_register(Register::C);
                self.set_register(Register::H, c);
            }
            // Load H with D
            0x62 => {
                let d = self.get_register(Register::D);
                self.set_register(Register::H, d);
            }
            // Load H with E
            0x63 => {
                let e = self.get_register(Register::E);
                self.set_register(Register::H, e);
            }
            // Load H with H
            0x64 => {
                let h = self.get_register(Register::H);
                self.set_register(Register::H, h);
            }
            // Load H with L
            0x65 => {
                let l = self.get_register(Register::L);
                self.set_register(Register::H, l);
            }
            // Load H with HL
            0x66 => {
                self.set_register(Register::H, self.hl as u8);
            }
            // Load H with A
            0x67 => {
                let a = self.get_register(Register::A);
                self.set_register(Register::H, a);
            }
            // Load L with B
            0x68 => {
                let b = self.get_register(Register::B);
                self.set_register(Register::L, b);
            }
            // Load L with C
            0x69 => {
                let c = self.get_register(Register::C);
                self.set_register(Register::L, c);
            }
            // Load HL with B
            0x70 => {
                let b = self.get_register(Register::B);
                self.hl = b as u16;
            }
            // Load HL with A
            0x77 => {
                let a = self.get_register(Register::A);
                self.hl = a as u16;
            }
            // Load A with D
            0x7A => {
                let d = self.get_register(Register::D);
                self.set_register(Register::A, d);
            }
            // Load A with E
            0x7B => {
                let e = self.get_register(Register::E);
                self.set_register(Register::A, e);
            }
            // Add B to A
            0x80 => {
                let b = self.get_register(Register::B);
                let mut a = self.get_register(Register::A);
                a = a.wrapping_add(b);
                self.set_register(Register::A, a);

                self.unset_flag(Flag::N);
                self.check_h(self.hl as u8, self.de as i8);
                // TODO check carry
            }
            // Add D to A with carry
            0x8A => {
                let d = self.get_register(Register::D);
                let mut a = self.get_register(Register::A);
                let carry = self.get_flag(Flag::C) as u8;

                a = a.wrapping_add(d + carry);
                self.set_register(Register::A, a);

                self.check_z(a as u16);
                self.unset_flag(Flag::N);
                self.check_h(a, d as i8);
                // TODO check carry
            }
            // Subtract E from A
            0x93 => {
                let e = self.get_register(Register::E);
                let mut a = self.get_register(Register::A);
                self.check_h(a, e as i8);

                a = a.wrapping_sub(e);
                self.set_register(Register::A, a);

                self.check_z(a as u16);
                self.set_flag(Flag::N);
                // TODO check carry
            }
            // Set A = A XOR A
            0xAF => {
                let a = self.get_register(Register::A);
                let value = a ^ a;
                self.set_register(Register::A, value);

                self.check_z(value as u16);
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

                self.check_z(value as u16);
                self.unset_flag(Flag::N);
                self.unset_flag(Flag::H);
                self.unset_flag(Flag::C);
            }
            // OR E
            0xB3 => {
                let a = self.get_register(Register::A);
                let e = self.get_register(Register::E);
                let value = a | e;
                self.set_register(Register::A, value);

                self.check_z(value as u16);
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
            // RST 00H
            0xC7 => {
                // TODO
            }
            // Add d8 to A with carry
            0xCE => {
                let data = self.fetch_data(bus, AddressingMode::D8) as u8;
                let mut a = self.get_register(Register::A);
                let carry = self.get_flag(Flag::C) as u8;

                a = a.wrapping_add(data + carry);
                self.set_register(Register::A, a);

                self.check_z(a as u16);
                self.unset_flag(Flag::N);
                self.check_h(a, data as i8);
                // TODO check carry
            }
            // Jump if C
            0xD2 => {
                if self.get_flag(Flag::C) {
                    let address = self.fetch_data(bus, AddressingMode::A16);
                    self.pc = address;
                }
            }
            // Call to 18
            0xDF => {
                // TODO
            }
            // Jump to HL
            0xE9 => self.pc = self.hl,
            // RST 30H
            0xF7 => {
                // TODO
            }
            // Call to 38
            0xFF => {
                // TODO
            }
            // BLANKS temporary catch
            0xFC | 0xEB => {
                return;
            }
            _ => panic!(
                "Invalid instruction read: 0x{:02X} at 0x{:02X}",
                instruction,
                self.pc - 1
            ),
        }
    }
}
