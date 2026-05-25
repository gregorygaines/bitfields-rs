use bitfields::bitfield;

#[bitfield([u16; 4])]
pub struct Bitfield {
    a: u64,
}

fn main() {}

