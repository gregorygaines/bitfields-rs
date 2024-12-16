use bitfields::bitfield;

#[bitfield(u16)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u8,
    #[bits(default = NestedBitfield::new())]
    nested_field: NestedBitfield,
}

#[bitfield(u8)]
pub struct NestedBitfield {
    #[bits(4, default = 0x3)]
    a: u8,
    #[bits(4, default = 0x4)]
    b: u16,
}

fn main() {}
