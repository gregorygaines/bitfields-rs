use bitfields::bitfield;

#[test]
fn test_const_methods() {
    #[bitfield(u32, bit_ops = true, neg = true)]
    pub struct Bitfield {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
    }

    #[rustversion::all(stable, before(1.83))]
    fn test_methods() {
        let _ = Bitfield::new_without_defaults();
        let mut bitfield = Bitfield::new();
        bitfield.set_a(1);
        bitfield.neg_b();
        let _ = bitfield.checked_set_c(3);
        bitfield.set_bit(24, true);
        let _ = bitfield.checked_set_bit(31, true);
        let bits = bitfield.into_bits();
        let _ = Bitfield::from_bits_with_defaults(bits);
        let _ = Bitfield::from_bits(bits);
    }

    #[rustversion::any(all(stable, since(1.83)), all(nightly, since(1.41)))]
    const fn test_methods() {
        let _ = Bitfield::new_without_defaults();
        let mut bitfield = Bitfield::new();
        bitfield.set_a(1);
        bitfield.a();
        bitfield.neg_b();
        let _ = bitfield.checked_set_c(3);
        bitfield.set_bit(24, true);
        let _ = bitfield.checked_set_bit(31, true);
        let bits = bitfield.into_bits();
        let _ = Bitfield::from_bits_with_defaults(bits);
        let _ = Bitfield::from_bits(bits);
    }

    test_methods();
}
