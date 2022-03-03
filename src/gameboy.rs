use crate::bus::rom::Rom;
use crate::bus::Bus;
use crate::cpu::Cpu;

pub struct Gameboy {
    bus: Bus,
    cpu: Cpu,
}

impl Gameboy {
    pub fn new(path: &str) -> Gameboy {
        let mut rom = Rom::new();
        rom.load_rom(path).unwrap();

        let bus = Bus::new(rom);
        let gb = Gameboy {
            bus,
            cpu: Cpu::new(),
        };
        gb
    }

    pub fn cycle(&mut self) {
        loop {
            self.cpu.cycle(&mut self.bus);
        }
    }

    pub fn get_rom(&self) -> &Rom {
        self.bus.get_rom()
    }
}
