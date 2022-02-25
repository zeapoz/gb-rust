use serde_json;
use std::fs;

use crate::memory::rom::Rom;

pub struct Disassembler {
    lookup_table: serde_json::Value,
    pc: u16,
}

impl Disassembler {
    pub fn new() -> Disassembler {
        // Open and parse JSON file of opcodes
        let json = fs::read_to_string("./opcodes.json").unwrap();
        let lookup_table: serde_json::Value = serde_json::from_str(&json).unwrap();

        Disassembler {
            lookup_table,
            pc: 0x0100,
        }
    }

    pub fn decode_rom(&mut self, rom: &Rom) {
        for _ in 0..32 {
            let byte = rom.read(self.pc);
            // println!("HI: {:X}, LO: {:X}", hi, lo);
            self.read_byte(byte);
            self.increment_pc();
        }
    }

    fn read_byte(&self, byte: u8) {
        let mnemonic = self.lookup_opcode(byte);
        println!("{:X}: {:#X}, {}", self.pc, byte, mnemonic);
    }

    fn lookup_opcode(&self, byte: u8) -> String {
        let table = &self.lookup_table["unprefixed"];
        let value = &table[format!("{:#X}", byte)]["mnemonic"];
        value.to_string()
    }

    fn increment_pc(&mut self) {
        self.pc += 1;
    }
}
