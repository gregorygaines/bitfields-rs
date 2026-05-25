use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    a: [u16; 2],
}

fn main() {}

