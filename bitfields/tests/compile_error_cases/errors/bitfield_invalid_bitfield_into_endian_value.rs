use bitfields::bitfield;

#[bitfield(u32, into_endian = 123)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

#[bitfield(u32, into_endian = invalid)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

fn main() {}
