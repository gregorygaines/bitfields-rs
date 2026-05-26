use bitfields::bitfield;

/// Array field bits exceed the array-backed bitfield size.
#[bitfield([u8; 2])]
pub struct Bitfield {
    a: [u8; 4294967296],
}

fn main() {}

