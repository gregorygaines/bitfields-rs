// TODO: There is a linting issue with clippy that causes false positives in this file.
#![allow(clippy::all)]

use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u8,
    #[bits(default = 0x34, access = ro)]
    b: u8,
    #[bits(default = 0x56)]
    c: u8,
    #[bits(default = 0x78)]
    d: u8,
}

fn main() {
    let bitfield = Bitfield::from_bits(0x11_22_33_44);

    assert_eq!(bitfield.a(), 0x44);
    assert_eq!(bitfield.b(), 0x33);
    assert_eq!(bitfield.c(), 0x22);
    assert_eq!(bitfield.d(), 0x11);
    assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
}
