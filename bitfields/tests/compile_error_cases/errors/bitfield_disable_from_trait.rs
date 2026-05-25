use bitfields::bitfield;

#[bitfield(u64, from_traits = false)]
pub struct Bitfield {
    #[bits(4, default = 0x1)]
    a: u8,
    #[bits(60, default = 0xFFFF_FFFF_FFFF)]
    _reserved: u64
}

fn main() {
    let _b: Bitfield = 0u64.into();
}
