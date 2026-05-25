use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = 0xFFFFFFFFF)]
    a: u32,
}

#[bitfield(u16)]
pub struct BitfieldB {
    #[bits(default = 0xFFFFFFFF)]
    a: u16,
    b: u16,
}

fn main() {}
