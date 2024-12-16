use bitfields::bitfield;

#[bitfield(u64)]
pub struct Bitfield {
    #[bits(4, default = 0x1)]
    a: u8,
    #[bits(60, default = 0xFFFF_FFFF_FFFF)]
    _padding: u64
}

fn main() {
    let bitfield = Bitfield::new();
    assert_eq!(bitfield.a(), 0x1);
    assert_eq!(bitfield.into_bits(), 0xF_FFFF_FFFF_FFF1);
    assert_eq!(bitfield._padding(), 0x1);
}
