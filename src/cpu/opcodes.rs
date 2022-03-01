use super::Cpu;
use crate::memory::Memory;

impl Cpu {
    pub fn jp(&mut self, memory: &Memory) {
        self.increment_pc();
        // Read next jump destination
        let hi = memory.read(self.pc) as u16;
        self.increment_pc();
        let lo = memory.read(self.pc) as u16;
        self.pc = (hi << 4) | lo;
    }

    pub fn ret() {}
}
