pub mod rom;
mod wram;

use rom::Rom;
use wram::Wram;

pub struct Bus {
    rom: Rom,
    wram: Wram,
}

impl Bus {
    pub fn new(rom: Rom) -> Bus {
        let wram = Wram::new();
        Bus { rom, wram }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom.read(address),
            0xC000..=0xCFFF => self.wram.read(address),
            _ => panic!("Invalid address range."),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {}

    pub fn get_rom(&self) -> &Rom {
        &self.rom
    }
}
