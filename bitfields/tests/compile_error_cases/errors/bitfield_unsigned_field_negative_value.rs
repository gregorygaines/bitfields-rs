use bitfields::bitfield;

#[bitfield(u8)]
pub struct Bitfield {
    #[bits(8, default = -0x1u8)]
    a: u8,
}

fn main() {}
