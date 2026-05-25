use bitfields::bitfield;

#[bitfield(f64)]
pub struct Bitfield {
    a: u64,
}

#[bitfield(0f32)]
pub struct Bitfield {
    a: u64,
}

fn main() {}
