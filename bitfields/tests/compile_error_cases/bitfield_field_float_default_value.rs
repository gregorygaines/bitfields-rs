use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = 3.1)]
    a: u8,
    #[bits(default = 1)]
    b: u8,
    #[bits(default = 2)]
    c: u8,
    #[bits(default = 3)]
    d: u8,
}

fn main() {}
