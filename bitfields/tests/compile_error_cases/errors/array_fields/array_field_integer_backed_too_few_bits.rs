use bitfields::bitfield;

/// Array field bits are fewer than the integer-backed bitfield size.
#[bitfield(u32)]
pub struct Bitfield {
    a: [u8; 2],
}

fn main() {}

