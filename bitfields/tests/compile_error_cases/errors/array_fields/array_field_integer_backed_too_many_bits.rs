use bitfields::bitfield;

/// Array field bits exceed the integer-backed bitfield size.
#[bitfield(u16)]
pub struct Bitfield {
    a: [u8; 4],
}

fn main() {}

