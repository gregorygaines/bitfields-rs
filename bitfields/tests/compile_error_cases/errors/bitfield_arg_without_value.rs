use bitfields::bitfield;

#[bitfield(u32, order)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

fn main() {}
