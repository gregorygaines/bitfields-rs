use bitfields::bitfield;

#[bitfield(u16)]
pub struct Bitfield {
    #[bits(3, default = 0xFFF)]
    a: u8,
    #[bits(13, default = 1)]
    b: u16,
}

fn main() {}
