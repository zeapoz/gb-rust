use std::cell::Cell;

use crate::cpu::Cpu;
use crate::memory::rom::Rom;
use crate::memory::Memory;

pub struct Gameboy {
    memory: Memory,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(path: &str) -> Gameboy {
        let mut rom = Rom::new();
        rom.load_rom(path).unwrap();

        let memory = Memory::new(rom);
        let gb = Gameboy {
            memory,
            cpu: Cpu::new(Cell::new(None)),
        };
        gb
    }

    pub fn connect_bus(&'static self) {
        self.cpu.connect_memory(&self.memory);
    }

    pub fn cycle(&mut self) {
        loop {
            self.cpu.cycle();
        }
    }

    pub fn get_rom(&self) -> &Rom {
        self.memory.get_rom()
    }
}
