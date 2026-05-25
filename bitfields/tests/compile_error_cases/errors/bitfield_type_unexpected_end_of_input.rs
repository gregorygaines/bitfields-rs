use bitfields::bitfield;

#[bitfield()]
pub struct Bitfield {
    a: u64,
}

#[bitfield]
pub struct Bitfield {
    a: u64,
}


fn main() {}
