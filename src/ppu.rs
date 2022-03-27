use crate::bus::Bus;

#[derive(Copy, Clone)]
enum ColorId {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[ColorId; 8]; 8];

pub struct Ppu {
    tile_set: [Tile; 384],
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            tile_set: [[[ColorId::Zero; 8]; 8]; 384],
        }
    }

    pub fn cycle(bus: &mut Bus) {}
}
