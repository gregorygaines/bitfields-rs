use bitfields::bitfield;

#[bitfield(u32, +==32 = hello)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u32,
}

fn main() {}
