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
            // Load BC with d16
            0x01 => {
                let value = self.fetch_data(bus, AddressingMode::D16);
                self.bc = value;
            }
            // Load BC with A
            0x02 => {
                let a = self.get_register(&Register::A);
                self.bc = a as u16;
            }
            // Increment BC
            0x03 => {
                self.bc = self.bc.wrapping_add(1);
            }
            // INC DEC B
            0x04 => self.inc(Register::B),
            0x05 => self.dec(Register::B),
            // Load B with d8
            0x06 => {
                let byte = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::B, byte);
            }
            // Rotate A left
            0x07 => {
                let mut a = self.get_register(&Register::A);
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
            // Decrement BC
            0x0B => {
                self.bc = self.bc.wrapping_sub(1);
            }
            // INC DEC C
            0x0C => self.inc(Register::C),
            0x0D => self.dec(Register::C),
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
            0x12 => self.de = self.get_register(&Register::A) as u16,

            // Increment DE
            0x13 => self.de = self.de.wrapping_add(1),
            // INC DEC D
            0x14 => self.inc(Register::D),
            0x15 => self.dec(Register::D),
            // Load H with d8
            0x16 => {
                let byte = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::H, byte);
            }
            // Relative jump to r8
            0x18 => {
                let address = self.fetch_data(bus, AddressingMode::R8) as i8;
                self.pc = self.pc.wrapping_add(address as u16);
            }
            // Add DE to HL
            0x19 => {
                self.unset_flag(Flag::N);
                self.check_h(self.hl as u8, self.de as i8);
                // TODO check carry
            }
            // Load A with DE
            0x1A => self.load(Register::A, Register::DE),
            // Decrement DE
            0x1B => self.de = self.de.wrapping_sub(1),
            // INC DEC E
            0x1C => self.inc(Register::E),
            0x1D => self.dec(Register::E),
            // Load E with d8
            0x1E => {
                let data = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::E, data);
            }
            // Rotate A right through carry
            0x1F => {
                let mut a = self.get_register(&Register::A);
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
            // INC DEC H
            0x24 => self.inc(Register::H),
            0x25 => self.dec(Register::H),
            // Decimal adjust A
            0x27 => {
                // TODO
            }
            // Relative jump if Z
            0x28 => {
                if self.get_flag(Flag::Z) {
                    let address = self.fetch_data(bus, AddressingMode::R8) as i8;
                    self.pc = self.pc.wrapping_add(address as u16);
                }
            }
            // Add HL to HL
            0x29 => {
                self.hl = self.hl.wrapping_add(self.hl);

                self.unset_flag(Flag::N);
                self.check_h(self.hl as u8, self.hl as i8);
                // TODO check carry
            }
            // Load A with HL, increment HL
            0x2A => {
                self.load(Register::A, Register::HL);
                self.hl = self.hl.wrapping_add(1);
            }
            // INC DEC L
            0x2C => self.inc(Register::L),
            0x2D => self.dec(Register::L),
            // LD SP with d16
            0x31 => {
                let address = self.fetch_data(bus, AddressingMode::D16);
                self.sp = address;
            }
            // Load HL with A, decrement HL
            0x32 => {
                self.hl = self.get_register(&Register::A) as u16;
                self.hl = self.hl.wrapping_sub(1);
            }
            // INC DEC A
            0x3C => self.inc(Register::A),
            0x3D => self.dec(Register::A),
            // Load A with d8
            0x3E => {
                let byte = self.fetch_data(bus, AddressingMode::D8) as u8;
                self.set_register(Register::A, byte);
            }
            // B loads
            0x40 => self.load(Register::B, Register::B),
            0x41 => self.load(Register::B, Register::C),
            0x42 => self.load(Register::B, Register::D),
            0x43 => self.load(Register::B, Register::E),
            0x44 => self.load(Register::B, Register::H),
            0x45 => self.load(Register::B, Register::L),
            0x46 => self.load(Register::B, Register::HL),
            0x47 => self.load(Register::B, Register::A),
            // C loads
            0x48 => self.load(Register::C, Register::B),
            0x49 => self.load(Register::C, Register::C),
            0x4A => self.load(Register::C, Register::D),
            0x4B => self.load(Register::C, Register::E),
            0x4C => self.load(Register::C, Register::H),
            0x4D => self.load(Register::C, Register::L),
            0x4E => self.load(Register::C, Register::HL),
            0x4F => self.load(Register::C, Register::A),
            // D loads
            0x50 => self.load(Register::D, Register::B),
            0x51 => self.load(Register::D, Register::C),
            0x52 => self.load(Register::D, Register::D),
            0x53 => self.load(Register::D, Register::E),
            0x54 => self.load(Register::D, Register::H),
            0x55 => self.load(Register::D, Register::L),
            0x56 => self.load(Register::D, Register::HL),
            0x57 => self.load(Register::D, Register::A),
            // E loads
            0x58 => self.load(Register::E, Register::B),
            0x59 => self.load(Register::E, Register::C),
            0x5A => self.load(Register::E, Register::D),
            0x5B => self.load(Register::E, Register::E),
            0x5C => self.load(Register::E, Register::H),
            0x5D => self.load(Register::E, Register::L),
            0x5E => self.load(Register::E, Register::HL),
            0x5F => self.load(Register::E, Register::A),
            // H loads
            0x60 => self.load(Register::H, Register::B),
            0x61 => self.load(Register::H, Register::C),
            0x62 => self.load(Register::H, Register::D),
            0x63 => self.load(Register::H, Register::E),
            0x64 => self.load(Register::H, Register::H),
            0x65 => self.load(Register::H, Register::L),
            0x66 => self.load(Register::H, Register::HL),
            0x67 => self.load(Register::H, Register::A),
            // L loads
            0x68 => self.load(Register::L, Register::B),
            0x69 => self.load(Register::L, Register::C),
            0x6A => self.load(Register::L, Register::D),
            0x6B => self.load(Register::L, Register::E),
            0x6C => self.load(Register::L, Register::H),
            0x6D => self.load(Register::L, Register::L),
            0x6E => self.load(Register::L, Register::HL),
            0x6F => self.load(Register::L, Register::A),
            // HL loads
            0x70 => self.load(Register::HL, Register::B),
            0x71 => self.load(Register::HL, Register::C),
            0x72 => self.load(Register::HL, Register::D),
            0x73 => self.load(Register::HL, Register::E),
            0x74 => self.load(Register::HL, Register::H),
            0x75 => self.load(Register::HL, Register::L),
            // Halt instruction 0x76 inbetween
            0x76 => self.halted = true,

            0x77 => self.load(Register::HL, Register::A),
            // A loads
            0x78 => self.load(Register::A, Register::B),
            0x79 => self.load(Register::A, Register::C),
            0x7A => self.load(Register::A, Register::D),
            0x7B => self.load(Register::A, Register::E),
            0x7C => self.load(Register::A, Register::H),
            0x7D => self.load(Register::A, Register::L),
            0x7E => self.load(Register::A, Register::HL),
            0x7F => self.load(Register::A, Register::A),
            // ADD operations
            0x80 => self.add(Register::B),
            0x81 => self.add(Register::C),
            0x82 => self.add(Register::D),
            0x83 => self.add(Register::E),
            0x84 => self.add(Register::H),
            0x85 => self.add(Register::L),
            0x86 => self.add(Register::HL),
            0x87 => self.add(Register::A),
            // Add D to A with carry
            0x8A => {
                let d = self.get_register(&Register::D);
                let mut a = self.get_register(&Register::A);
                let carry = self.get_flag(Flag::C) as u8;

                a = a.wrapping_add(d + carry);
                self.set_register(Register::A, a);

                self.check_z(a as u16);
                self.unset_flag(Flag::N);
                self.check_h(a, d as i8);
                // TODO check carry
            }
            // SUB operations
            0x90 => self.sub(Register::B),
            0x91 => self.sub(Register::C),
            0x92 => self.sub(Register::D),
            0x93 => self.sub(Register::E),
            0x94 => self.sub(Register::H),
            0x95 => self.sub(Register::L),
            0x96 => self.sub(Register::HL),
            0x97 => self.sub(Register::A),
            // AND operations
            0xA0 => self.and(Register::B),
            0xA1 => self.and(Register::C),
            0xA2 => self.and(Register::D),
            0xA3 => self.and(Register::E),
            0xA4 => self.and(Register::H),
            0xA5 => self.and(Register::L),
            0xA6 => self.and(Register::HL),
            0xA7 => self.and(Register::A),
            // XOR operations
            0xA8 => self.xor(Register::B),
            0xA9 => self.xor(Register::C),
            0xAA => self.xor(Register::D),
            0xAB => self.xor(Register::E),
            0xAC => self.xor(Register::H),
            0xAD => self.xor(Register::L),
            0xAE => self.xor(Register::HL),
            0xAF => self.xor(Register::A),
            // OR operations
            0xB0 => self.or(Register::B),
            0xB1 => self.or(Register::C),
            0xB2 => self.or(Register::D),
            0xB3 => self.or(Register::E),
            0xB4 => self.or(Register::H),
            0xB5 => self.or(Register::L),
            0xB6 => self.or(Register::HL),
            0xB7 => self.or(Register::A),
            // CP operations
            0xB8 => self.cp(Register::B),
            0xB9 => self.cp(Register::C),
            0xBA => self.cp(Register::D),
            0xBB => self.cp(Register::E),
            0xBC => self.cp(Register::H),
            0xBD => self.cp(Register::L),
            0xBE => self.cp(Register::HL),
            0xBF => self.cp(Register::A),
            // Return if not Z
            0xC0 => {
                if !self.get_flag(Flag::Z) {
                    self.ret();
                }
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
            // Return if Z
            0xC8 => {
                if self.get_flag(Flag::Z) {
                    self.ret();
                }
            }
            // Return
            0xC9 => self.ret(),
            // Prefix CB
            0xCB => {
                // TODO lookup table
            }
            // Call to a16
            0xCD => self.call(bus),
            // Add d8 to A with carry
            0xCE => {
                let data = self.fetch_data(bus, AddressingMode::D8) as u8;
                let mut a = self.get_register(&Register::A);
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
            // Write to IO port n
            0xE0 => {
                // TODO
            }
            // Push HL
            0xE5 => {
                // TODO
            }
            // Jump to HL
            0xE9 => self.pc = self.hl,
            // Load a16 with A
            0xEA => {
                // TODO
            }
            // Read from IO port n
            0xF0 => {
                // TODO
            }
            // RST 30H
            0xF7 => {
                // TODO
            }
            // Call to 38
            0xFF => {
                // TODO
            }
            // BLANKS temporary catch
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                println!("Invalid read");
            }
            _ => panic!(
                "Invalid instruction read: 0x{:02X} at 0x{:02X}",
                instruction,
                self.pc - 1
            ),
        }
    }

    fn inc(&mut self, reg: Register) {
        let mut value = self.get_register(&reg);
        self.check_h(value, 1);

        value = value.wrapping_add(1);
        self.set_register(reg, value);

        self.check_z(value as u16);
        self.unset_flag(Flag::N);
    }

    fn dec(&mut self, reg: Register) {
        let mut value = self.get_register(&reg);
        self.check_h(value, -1);

        value = value.wrapping_sub(1);

        self.set_register(reg, value);

        self.check_z(value as u16);
        self.set_flag(Flag::N);
    }

    fn load(&mut self, reg1: Register, reg2: Register) {
        let r = self.get_register(&reg2);
        self.set_register(reg1, r)
    }

    fn add(&mut self, reg: Register) {
        let a = self.get_register(&Register::A);
        let r = self.get_register(&reg);
        let value = a.wrapping_add(r);
        self.set_register(Register::A, value);

        self.check_z(value as u16);
        self.unset_flag(Flag::N);
        self.check_h(a, r as i8);
        // TODO check carry
    }

    fn sub(&mut self, reg: Register) {
        let a = self.get_register(&Register::A);
        let r = self.get_register(&reg);
        let value = a.wrapping_sub(r);
        self.set_register(Register::A, value);

        self.check_z(value as u16);
        self.set_flag(Flag::N);
        self.check_h(a, r as i8);
        // TODO check carry
    }

    fn and(&mut self, reg: Register) {
        let a = self.get_register(&Register::A);
        let r = self.get_register(&reg);
        let value = a & r;
        self.set_register(Register::A, value);

        self.check_z(value as u16);
        self.unset_flag(Flag::N);
        self.set_flag(Flag::H);
        self.unset_flag(Flag::C);
    }

    fn xor(&mut self, reg: Register) {
        let a = self.get_register(&Register::A);
        let r = self.get_register(&reg);
        let value = a ^ r;
        self.set_register(Register::A, value);

        self.check_z(value as u16);
        self.unset_flag(Flag::N);
        self.unset_flag(Flag::H);
        self.unset_flag(Flag::C);
    }

    fn or(&mut self, reg: Register) {
        let a = self.get_register(&Register::A);
        let r = self.get_register(&reg);
        let value = a | r;
        self.set_register(Register::A, value);

        self.check_z(value as u16);
        self.unset_flag(Flag::N);
        self.unset_flag(Flag::H);
        self.unset_flag(Flag::C);
    }

    fn cp(&mut self, reg: Register) {
        let a = self.get_register(&Register::A);
        let r = self.get_register(&reg);
        self.check_h(a, -(r as i8));
        self.check_z(0);
        self.set_flag(Flag::N);
        // TODO Check carry
    }

    fn call(&mut self, bus: &mut Bus) {
        let address = self.fetch_data(bus, AddressingMode::A16);
        self.push_stack(self.pc);
        self.pc = address;
    }

    fn ret(&mut self) {
        let address = self.pop_stack();
        self.pc = address;
    }
}
