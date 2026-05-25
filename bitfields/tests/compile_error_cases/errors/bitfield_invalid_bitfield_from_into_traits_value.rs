use bitfields::bitfield;

#[bitfield(u32, from_traits = 123)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

#[bitfield(u32, from_traits = invalid)]
pub struct Bitfield2 {
    #[bits(default = 0x12)]
    a: u32,
}

fn main() {}

