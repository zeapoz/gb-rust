use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs;

use crate::memory::rom::Rom;

pub struct Disassembler {
    opcodes: OpcodeTable,
    pc: u16,
}

impl Disassembler {
    pub fn new() -> Disassembler {
        // Open and parse JSON file of opcodes
        let json = fs::read_to_string("./opcodes.json").unwrap();
        let opcodes: OpcodeTable = serde_json::from_str(&json).unwrap();

        Disassembler { opcodes, pc: 0x100 }
    }

    pub fn decode_rom(&mut self, rom: &Rom) {
        println!("ADDR    HEXA    INSTRUCTION    OPERANDS\n");
        for _ in 0..32 {
            let byte = rom.read(self.pc);
            // println!("HI: {:X}, LO: {:X}", hi, lo);
            self.read_instruction(byte, rom);
            self.pc += 1;
        }
    }

    fn read_instruction(&mut self, byte: u8, rom: &Rom) {
        let opcode = self.opcodes.lookup_opcode(byte);
        // Print opcode and associated address
        print!(
            "{:04X}    0x{:02X}    {:<11}    ",
            self.pc, byte, opcode.mnemonic
        );
        let count = opcode.operands.len();
        // If opcode has operands continue otherwise print mnemonic
        if count > 0 {
            // Read each operand and print
            for _ in 0..count {
                self.pc += 1;
                let byte = rom.read(self.pc);
                print!("0x{:02X}, ", byte);
            }
        }
        print!("\n");
    }
}

// Structs for deserializing JSON into structs
#[derive(Debug, Deserialize)]
struct OpcodeTable {
    unprefixed: HashMap<String, Opcode>,
    cbprefixed: HashMap<String, Opcode>,
}

impl OpcodeTable {
    fn lookup_opcode(&self, byte: u8) -> &Opcode {
        // Look up opcode name from tables
        let table = &self.unprefixed;
        let hex = format!("0x{:02X}", byte);
        let opcode = match table.get(&hex) {
            Some(value) => value,
            _ => panic!("No opcode like that exists"),
        };
        opcode
    }
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
