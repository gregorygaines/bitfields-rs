use bitfields::bitfield;

#[bitfield(u8)]
pub struct Bitfield {
    #[bits(4, default = 9)]
    a: i8,
    #[bits(4)]
    _reserved: u8,
}

fn main() {}
