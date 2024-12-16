use bitfields::bitfield;

fn main() {
    #[bitfield(u32)]
    pub struct Bitfield {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    let bitfield =
        BitfieldBuilder::new().with_a(0x12).with_b(0x34).with_c(0x56).with_d(0x78).build();

    assert_eq!(bitfield.into_bits(), 0x78563412);
}
