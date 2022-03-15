use super::Cpu;

const HI: u16 = 0xFF00;
const LO: u16 = 0x00FF;

pub enum Register {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}

pub enum Flag {
    Z,
    N,
    H,
    C,
}

impl Cpu {
    pub fn get_register(&self, register: Register) -> u8 {
        match register {
            Register::A => return ((HI & self.af) >> 8) as u8,
            Register::F => return (LO & self.af) as u8,
            Register::B => return ((HI & self.bc) >> 8) as u8,
            Register::C => return (LO & self.bc) as u8,
            Register::D => return ((HI & self.de) >> 8) as u8,
            Register::E => return (LO & self.de) as u8,
            Register::H => return ((HI & self.hl) >> 8) as u8,
            Register::L => return (LO & self.hl) as u8,
            Register::DE => return self.de as u8,
            Register::HL => return self.hl as u8,
            _ => panic!("Not a valid register"),
        }
    }

    pub fn set_register(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.af = (value as u16) << 8 | self.get_register(Register::F) as u16,
            Register::F => self.af = (self.get_register(Register::F) as u16) << 8 | (value as u16),
            Register::B => self.af = (value as u16) << 8 | self.get_register(Register::C) as u16,
            Register::C => self.bc = (self.get_register(Register::B) as u16) << 8 | (value as u16),
            Register::D => self.de = (value as u16) << 8 | self.get_register(Register::E) as u16,
            Register::E => self.bc = (self.get_register(Register::D) as u16) << 8 | (value as u16),
            Register::H => self.hl = (value as u16) << 8 | self.get_register(Register::L) as u16,
            Register::L => self.bc = (self.get_register(Register::H) as u16) << 8 | (value as u16),
            Register::HL => self.hl = value as u16,
            _ => panic!("Not a valid register"),
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        let f = self.get_register(Register::F);
        match flag {
            Flag::Z => return (0b10000000 & f) != 0,
            Flag::N => return (0b01000000 & f) != 0,
            Flag::H => return (0b00100000 & f) != 0,
            Flag::C => return (0b00010000 & f) != 0,
            _ => panic!("Not a valid flag"),
        }
    }

    pub fn set_flag(&mut self, flag: Flag) {
        let f = self.get_register(Register::F);
        match flag {
            Flag::Z => self.set_register(Register::F, 0b10000000 | f),
            Flag::N => self.set_register(Register::F, 0b01000000 | f),
            Flag::H => self.set_register(Register::F, 0b00100000 | f),
            Flag::C => self.set_register(Register::F, 0b00010000 | f),
            _ => panic!("Not a valid flag"),
        };
    }

    pub fn unset_flag(&mut self, flag: Flag) {
        let f = self.get_register(Register::F);
        match flag {
            Flag::Z => self.set_register(Register::F, 0b01111111 & f),
            Flag::N => self.set_register(Register::F, 0b10111111 & f),
            Flag::H => self.set_register(Register::F, 0b11011111 & f),
            Flag::C => self.set_register(Register::F, 0b11101111 & f),
            _ => panic!("Not a valid flag"),
        };
    }
}
