use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    a: [i8; 4],
}

fn main() {}

