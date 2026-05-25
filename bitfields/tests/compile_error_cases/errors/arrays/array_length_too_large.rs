use bitfields::bitfield;

#[bitfield([u8; 4294967296])]
pub struct Bitfield {
    a: u8,
}

fn main() {}

