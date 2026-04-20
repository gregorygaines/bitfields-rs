use bitfields::bitfield;

#[bitfield(u16)]
pub struct Bitfield {
    #[bits(default = 0xFFFFFFFF)]
    a: u8,
    #[bits(8)]
    b: u8,
}

fn main() {}
