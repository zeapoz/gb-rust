use std::fs;
use std::io::Result;

#[derive(Copy, Clone)]
pub struct Rom {
    data: [u8; 32768],
}

impl Rom {
    pub fn new() -> Rom {
        Rom { data: [0; 32768] }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<()> {
        let buffer: Vec<u8> = fs::read(path)?;

        for i in 0..buffer.len() {
            let address = i as u16;
            self.write(address, buffer[i]);
        }
        Ok(())
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
}
