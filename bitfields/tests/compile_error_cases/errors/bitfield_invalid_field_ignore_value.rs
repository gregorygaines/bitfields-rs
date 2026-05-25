use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = 0x12, ignore = 43242)]
    a: u32,
}

fn main() {}
