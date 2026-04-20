use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = 0x12, deez = what)]
    a: u32,
}

fn main() {}
