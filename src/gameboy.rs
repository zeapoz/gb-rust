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

        Gameboy {
            memory: Memory::new(rom),
            cpu: Cpu::new(),
        }
    }

    pub fn cycle(&self) {
        self.cpu.cycle();
    }

    pub fn get_rom(&self) -> &Rom {
        self.memory.get_rom()
    }
}
