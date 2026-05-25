use bitfields::bitfield;

#[bitfield(u8)]
struct Bitfield {
    #[bits(access = ro)]
    _reserved: u8,
}

fn main() {}
