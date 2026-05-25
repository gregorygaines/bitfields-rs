use bitfields::bitfield;

#[bitfield(u32, set_get_bit_ops = 123)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

#[bitfield(u32, set_get_bit_ops = invalid)]
pub struct Bitfield2 {
    #[bits(default = 0x12)]
    a: u32,
}

fn main() {}

