use bitfields::bitfield;

#[bitfield(u32, new = 123)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

#[bitfield(u32, new = invalid)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}


fn main() {}
