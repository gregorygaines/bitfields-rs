use bitfields::bitfield;

fn main() {
    #[bitfield(u32)]
    pub struct Bitfield {
        #[bits(default = 0xFF)]
        a: u8,
        b: u8,
        c: u8,
        #[bits(default = 0xFF)]
        d: u8,
    }

    let bitfield = Bitfield::from_bits_with_defaults(0x78563412);

    assert_eq!(bitfield.into_bits(), 0xFF5634FF);
}
