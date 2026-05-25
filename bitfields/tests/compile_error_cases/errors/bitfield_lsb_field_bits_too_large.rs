use bitfields::bitfield;

#[bitfield(u32, order = lsb)]
pub struct Bitfield {
    a: u32,
    b: u32
}

fn main() {}
