use bitfields::bitfield;

#[bitfield(u16)]
pub struct Bitfield {
    #[bits(default = -90000000)]
    a: i8,
    #[bits(8)]
    b: u8,
}

fn main() {}
