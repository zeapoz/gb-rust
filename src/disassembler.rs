use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs;

use crate::memory::rom::Rom;

pub struct Disassembler {
    lookup_table: OpcodeTable,
    pc: u16,
}

impl Disassembler {
    pub fn new() -> Disassembler {
        // Open and parse JSON file of opcodes
        let json = fs::read_to_string("./opcodes.json").unwrap();
        let lookup_table: OpcodeTable = serde_json::from_str(&json).unwrap();

        Disassembler {
            lookup_table,
            pc: 0x0100,
        }
    }

    pub fn decode_rom(&mut self, rom: &Rom) {
        println!("ADDR    HEXA    INSTRUCTION\n");
        for _ in 0..32 {
            let byte = rom.read(self.pc);
            // println!("HI: {:X}, LO: {:X}", hi, lo);
            self.read_byte(byte);
            self.increment_pc();
        }
    }

    fn read_byte(&self, byte: u8) {
        let mnemonic = self.lookup_opcode(byte);
        // If opcode has operands continue otherwise print mnemonic
        println!("{:04X}    0x{:02X}    {}", self.pc, byte, mnemonic);
    }

    fn lookup_opcode(&self, byte: u8) -> String {
        // Look up opcode name from tables
        let table = &self.lookup_table.unprefixed;
        let hex = format!("0x{:02X}", byte);
        let code = match table.get(&hex) {
            Some(value) => value,
            _ => return String::from(""),
        };
        code.mnemonic.to_string()
    }

    fn increment_pc(&mut self) {
        self.pc += 1;
    }
}

// Structs for deserializing JSON into structs
#[derive(Debug, Deserialize)]
struct OpcodeTable {
    unprefixed: HashMap<String, Opcode>,
    cbprefixed: HashMap<String, Opcode>,
}

#[derive(Debug, Deserialize)]
struct Opcode {
    mnemonic: String,
    bytes: u8,
    cycles: Vec<u8>,
    operands: Vec<Operand>,
    immediate: bool,
    flags: HashMap<char, char>,
}

#[derive(Debug, Deserialize)]
struct Operand {
    name: String,
    bytes: Option<u8>,
    immediate: bool,
}
