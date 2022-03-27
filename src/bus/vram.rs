pub struct Vram {
    data: [u8; 8192],
}

impl Vram {
    pub fn new() -> Vram {
        Vram { data: [0; 8192] }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
}
