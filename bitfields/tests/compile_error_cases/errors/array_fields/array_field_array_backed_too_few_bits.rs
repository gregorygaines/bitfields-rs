use bitfields::bitfield;

#[bitfield([u8; 4])]
pub struct Bitfield {
    a: [u8; 2],
}

fn main() {}

