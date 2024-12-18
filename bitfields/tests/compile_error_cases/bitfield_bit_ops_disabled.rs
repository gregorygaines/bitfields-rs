use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

fn main() {
    let bitfield = Bitfield::new();

    bitfield.get_bit(0);
}
