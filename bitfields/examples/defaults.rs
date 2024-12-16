use bitfields::bitfield;

fn main() {
    #[bitfield(u32)]
    pub struct Bitfield {
        #[bits(default = 0x12)]
        a: u8,
        #[bits(default = 0x34)]
        b: u8,
        #[bits(default = 0x56)]
        c: u8,
        #[bits(default = 0x78)]
        d: u8,
    }

    let bitfield = Bitfield::new();

    assert_eq!(bitfield.into_bits(), 0x78563412);
}
