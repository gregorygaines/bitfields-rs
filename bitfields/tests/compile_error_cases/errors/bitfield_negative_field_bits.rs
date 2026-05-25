use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(-1)]
    a: u32,
}

fn main() {}
