use bitfields::bitfield;

#[bitfield(u32, order = 123)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

#[bitfield(u32, order = invalid)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

fn main() {}
