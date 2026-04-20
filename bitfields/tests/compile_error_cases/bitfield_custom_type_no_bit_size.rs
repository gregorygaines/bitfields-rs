use bitfields::bitfield;

#[bitfield(u16)]
pub struct Bitfield {
    a: u8,
    b: CustomType,
}

enum CustomType {

}

fn main() {}
