use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = 0x12, access = 123)]
    a: u32,
}

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = 0x12, access = invalid)]
    a: u32,
}

fn main() {}
