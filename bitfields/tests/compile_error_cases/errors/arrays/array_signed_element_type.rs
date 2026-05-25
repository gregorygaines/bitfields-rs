use bitfields::bitfield;

#[bitfield([i8; 4])]
pub struct Bitfield {
    a: u32,
}

fn main() {}

