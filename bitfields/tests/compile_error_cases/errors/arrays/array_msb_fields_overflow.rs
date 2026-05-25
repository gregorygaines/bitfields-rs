use bitfields::bitfield;

#[bitfield([u8; 2], order = msb)]
pub struct Bitfield {
    a: u8,
    b: u8,
    c: u8,
}

fn main() {}

