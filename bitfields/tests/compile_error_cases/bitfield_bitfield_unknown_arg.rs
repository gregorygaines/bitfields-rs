use bitfields::bitfield;

#[bitfield(u32, deez = what)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

fn main() {}
