use bitfields::bitfield;

const SIZE: usize = 4;

#[bitfield([u8; SIZE])]
pub struct Bitfield {
    a: u32,
}

fn main() {}

