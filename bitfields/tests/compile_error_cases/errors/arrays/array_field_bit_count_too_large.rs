use bitfields::bitfield;

#[bitfield([u8; 1])]
pub struct Bitfield {
    #[bits(4294967296)]
    a: u8,
}

fn main() {}

