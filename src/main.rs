use gameboy_emulator::Gameboy;

fn main() {
    let rom = "test/tetris.gb";
    let gameboy = Gameboy::new(rom);
}
