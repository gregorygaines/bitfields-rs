use bitfields::bitfield;

#[bitfield(u64, debug = false)]
pub struct Bitfield {
    #[bits(4, default = 0x1)]
    a: u8,
    #[bits(60, default = 0xFFFF_FFFF_FFFF)]
    _padding: u64
}

fn main() {
    let bitfield = Bitfield::new();
    println!("{:?}", bitfield);
}
