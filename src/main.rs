// use gameboy_emulator::disassembler::Disassembler;
use gameboy_emulator::gameboy::Gameboy;

fn main() {
    let rom = "test/tetris.gb";
    let mut gameboy = Gameboy::new(rom);

    // Disassemble and decode rom
    // let mut disassembler = Disassembler::new();
    // let rom_data = gameboy.get_rom();
    // disassembler.decode_rom(rom_data);

    // Main loop
    gameboy.cycle();
}
