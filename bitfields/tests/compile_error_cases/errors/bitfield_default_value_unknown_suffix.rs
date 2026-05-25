use bitfields::bitfield;

#[bitfield(u8)]
pub struct Bitfield {
    #[bits(8, default = 9op)]
    a: u8,
}

fn main() {
    let bitfield = Bitfield::new();
}
