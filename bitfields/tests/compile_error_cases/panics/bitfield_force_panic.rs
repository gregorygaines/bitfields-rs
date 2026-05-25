use bitfields::bitfield;

#[bitfield(u8, force_panic = true)]
pub struct Bitfield {
    a: u8,
}

fn main() {}

