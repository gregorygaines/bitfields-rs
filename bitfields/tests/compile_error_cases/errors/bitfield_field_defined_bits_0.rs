use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(0)]
    a: u8,
    #[bits(7)]
    b: u8,
    #[bits(5)]
    c: u8,
    #[bits(9)]
    d: u16,
}

fn main() {}
