use bitfields::bitfield;

#[bitfield(u32, default = 123)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

#[bitfield(u32, default = invalid)]
pub struct Bitfield2 {
    #[bits(default = 0x12)]
    a: u32,
}

fn main() {}

