pub mod rom;
mod vram;
mod wram;

use rom::Rom;
use vram::Vram;
use wram::Wram;

pub struct Bus {
    rom: Rom,
    vram: Vram,
    wram: Wram,
}

impl Bus {
    pub fn new(rom: Rom) -> Bus {
        Bus {
            rom,
            vram: Vram::new(),
            wram: Wram::new(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom.read(address),
            0x8000..=0x9FFF => self.vram.read(address),
            0xC000..=0xCFFF => self.wram.read(address),
            _ => panic!("Invalid address range."),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x3FFF => self.rom.write(address, data),
            0x8000..=0x9FFF => self.vram.write(address, data),
            0xC000..=0xCFFF => self.wram.write(address, data),
            _ => panic!("Invalid address range."),
        }
    }

    pub fn get_rom(&self) -> &Rom {
        &self.rom
    }
}
