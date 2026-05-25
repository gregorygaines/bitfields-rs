use bitfields::bitfield;

#[bitfield([f32; 4])]
pub struct Bitfield {
    a: u32,
}

fn main() {}

