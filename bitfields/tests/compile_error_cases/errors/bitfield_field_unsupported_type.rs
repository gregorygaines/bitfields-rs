use bitfields::bitfield;

#[bitfield(u8)]
pub struct Bitfield {
    a: (u8, u8),
}

fn main() {}

