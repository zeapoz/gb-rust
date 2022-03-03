use super::Cpu;

impl Cpu {
    pub fn jp(&mut self) {
        // Read next jump destination
        let hi = self.read_byte(self.pc) as u16;
        let lo = self.read_byte(self.pc) as u16;
        // Jump to read destination
        self.pc = (hi << 4) | lo;
    }

    pub fn jp_hl(&mut self) {
        self.pc = self.hl;
    }

    pub fn or_n(&mut self) {
        // Get value at register A
        let a = self.get_register('A');
        // Read next byte
        let byte = self.read_byte(self.pc);

        let value = a | byte;
        self.set_register('A', value);
    }

    pub fn ret() {}
}
