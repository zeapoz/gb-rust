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
    stack: [u8; 65536],
    halted: bool,
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
            stack: [0; 65536],
            halted: false,
        }
    }

    pub fn cycle(&mut self, bus: &mut Bus) {
        if self.halted {
            return;
        }
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
        }
    }

    fn push_stack(&mut self, address: u16) {
        let lo = (address & 0x00FF) as u8;
        let hi = (address >> 8) as u8;
        self.sp = self.sp.wrapping_sub(1);
        self.stack[self.sp as usize] = hi;

        self.sp = self.sp.wrapping_sub(1);
        self.stack[(self.sp + 1) as usize] = lo;
    }

    fn pop_stack(&mut self) -> u16 {
        let lo = self.stack[self.sp as usize] as u16;
        self.sp = self.sp.wrapping_add(1);

        let hi = self.stack[self.sp as usize] as u16;
        self.sp = self.sp.wrapping_add(1);
        (hi << 8) | lo
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

    fn check_c(&mut self, a: u8, b: u8) {
        if (a as u16) + (b as u16) > u8::MAX as u16 {
            self.set_flag(Flag::C);
        } else {
            self.unset_flag(Flag::C);
        }
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
}
