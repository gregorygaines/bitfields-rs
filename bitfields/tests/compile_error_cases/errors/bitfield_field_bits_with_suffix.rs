use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(8u8)]
    a: u32,
}

fn main() {}
