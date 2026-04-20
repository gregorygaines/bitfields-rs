use bitfields::bitfield;

#[bitfield(u16)]
pub struct Bitfield {
    #[bits(6)]
    a: u16,
    #[bits(10)]
    b: u8,
}

fn main() {}
