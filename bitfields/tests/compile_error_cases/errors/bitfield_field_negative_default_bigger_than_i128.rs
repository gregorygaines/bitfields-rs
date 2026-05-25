use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = -340282366920938463463374607431768211456)]
    a: i32,
}

fn main() {}
