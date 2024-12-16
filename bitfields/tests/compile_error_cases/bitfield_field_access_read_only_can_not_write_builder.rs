use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = 0x12, access = ro)]
    a: u32,
}

fn main() {
    let mut bitfield = BitfieldBuilder::new();
    bitfield.set_a(0x34);
}
