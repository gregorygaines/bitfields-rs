use bitfields::bitfield;

fn main() {
    #[bitfield(u32)]
    pub struct Bitfield {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    let bitfield = Bitfield::new();

    assert_eq!(bitfield.into_bits(), 0);
}
