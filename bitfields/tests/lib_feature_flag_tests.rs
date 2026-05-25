#[cfg(test)]
mod feature_flag_tests {

    #[cfg(feature = "order_lsb")]
    #[test]
    fn feature_order_lsb_default_packing() {
        use bitfields::bitfield;

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(default = 0xAB)]
            low: u8,
            #[bits(default = 0xCD)]
            high: u8,
        }

        let bf = Bitfield::new();
        assert_eq!(bf.low(), 0xAB, "low byte should be in bits [7..=0]");
        assert_eq!(bf.high(), 0xCD, "high byte should be in bits [15..=8]");
        assert_eq!(
            bf.into_bits(),
            0xCDAB,
            "LSB order: low field in lower byte, high field in upper byte"
        );
    }

    #[cfg(all(feature = "order_msb", not(feature = "order_lsb")))]
    #[test]
    fn feature_order_msb_default_packing() {
        use bitfields::bitfield;

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(default = 0xAB)]
            high: u8,
            #[bits(default = 0xCD)]
            low: u8,
        }

        let bf = Bitfield::new();
        assert_eq!(bf.high(), 0xAB, "first field should be in most-significant byte");
        assert_eq!(bf.low(), 0xCD, "second field should be in least-significant byte");
        assert_eq!(
            bf.into_bits(),
            0xABCD,
            "MSB order: first field in upper byte, second field in lower byte"
        );
    }

    #[cfg(feature = "generate_new")]
    #[test]
    fn feature_generate_new_creates_with_defaults() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x11)]
            a: u8,
            #[bits(default = 0x22)]
            b: u8,
            #[bits(default = 0x33)]
            c: u8,
            #[bits(default = 0x44)]
            d: u8,
        }

        let bf = Bitfield::new();
        assert_eq!(bf.a(), 0x11);
        assert_eq!(bf.b(), 0x22);
        assert_eq!(bf.c(), 0x33);
        assert_eq!(bf.d(), 0x44);
        assert_eq!(bf.into_bits(), 0x44332211);
    }

    #[cfg(feature = "generate_new")]
    #[test]
    fn feature_generate_new_without_defaults_zeroes_non_reserved() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x11)]
            a: u8,
            #[bits(default = 0x22)]
            b: u8,
            #[bits(default = 0x33)]
            c: u8,

            #[bits(default = 0x44)]
            _reserved: u8,
        }

        let bf = Bitfield::new_without_defaults();
        assert_eq!(bf.a(), 0x00, "non-reserved field a should be zeroed");
        assert_eq!(bf.b(), 0x00, "non-reserved field b should be zeroed");
        assert_eq!(bf.c(), 0x00, "non-reserved field c should be zeroed");
        assert_eq!(bf.into_bits(), 0x44_00_00_00, "reserved field must keep its default 0x44");
    }

    #[cfg(feature = "generate_from_into_bits")]
    #[test]
    fn feature_generate_from_bits_round_trips() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let raw: u32 = 0xDEAD_BEEF;
        let bf = Bitfield::from_bits(raw);
        assert_eq!(bf.into_bits(), raw, "from_bits → into_bits must be the identity");
    }

    #[cfg(feature = "generate_from_into_bits")]
    #[test]
    fn feature_generate_from_bits_with_defaults_applies_defaults() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0xFF)]
            a: u8,
            b: u8,
            c: u8,
            #[bits(default = 0xFF)]
            d: u8,
        }

        let bf = Bitfield::from_bits_with_defaults(0x11_22_33_44);
        assert_eq!(bf.a(), 0xFF, "a should be replaced by its default 0xFF");
        assert_eq!(bf.b(), 0x33, "b has no default – raw bits[15:8] kept");
        assert_eq!(bf.c(), 0x22, "c has no default – raw bits[23:16] kept");
        assert_eq!(bf.d(), 0xFF, "d should be replaced by its default 0xFF");
        assert_eq!(bf.into_bits(), 0xFF_22_33_FF);
    }

    #[cfg(feature = "from_endian_big")]
    #[test]
    fn feature_from_endian_big_no_byte_swap() {
        use bitfields::bitfield;
        #[bitfield(u32, from_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bf = Bitfield::from_bits(0x11_22_33_44);
        assert_eq!(bf.a(), 0x44, "LSB-order: a = bits[7..=0] = 0x44");
        assert_eq!(bf.b(), 0x33);
        assert_eq!(bf.c(), 0x22);
        assert_eq!(bf.d(), 0x11);
    }

    #[cfg(feature = "from_endian_little")]
    #[test]
    fn feature_from_endian_little_byte_swap() {
        use bitfields::bitfield;
        #[bitfield(u32, from_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bf = Bitfield::from_bits(0x11_22_33_44);
        assert_eq!(bf.a(), 0x11, "LE from_bits: a should be 0x11 after byte swap");
        assert_eq!(bf.b(), 0x22);
        assert_eq!(bf.c(), 0x33);
        assert_eq!(bf.d(), 0x44);
    }

    #[cfg(feature = "into_endian_big")]
    #[test]
    fn feature_into_endian_big_no_byte_swap() {
        use bitfields::bitfield;
        #[bitfield(u32, from_endian = big, into_endian = big)]
        pub struct Bitfield {
            #[bits(default = 0x11)]
            a: u8,
            #[bits(default = 0x22)]
            b: u8,
            #[bits(default = 0x33)]
            c: u8,
            #[bits(default = 0x44)]
            d: u8,
        }

        let bf = Bitfield::new();

        assert_eq!(bf.into_bits(), 0x44_33_22_11, "BE into_bits should not byte-swap");
    }

    #[cfg(feature = "into_endian_little")]
    #[test]
    fn feature_into_endian_little_byte_swap() {
        use bitfields::bitfield;
        #[bitfield(u32, from_endian = big, into_endian = little)]
        pub struct Bitfield {
            #[bits(default = 0x11)]
            a: u8,
            #[bits(default = 0x22)]
            b: u8,
            #[bits(default = 0x33)]
            c: u8,
            #[bits(default = 0x44)]
            d: u8,
        }

        let bf = Bitfield::new();

        assert_eq!(bf.into_bits(), 0x11_22_33_44, "LE into_bits should byte-swap the result");
    }

    #[cfg(feature = "generate_from_traits")]
    #[test]
    fn feature_generate_from_traits_round_trips() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0xAA)]
            a: u8,
            #[bits(default = 0xBB)]
            b: u8,
            #[bits(default = 0xCC)]
            c: u8,
            #[bits(default = 0xDD)]
            d: u8,
        }

        let raw: u32 = 0xDDCCBBAA;

        let bf = Bitfield::from(raw);
        assert_eq!(bf.a(), 0xAA);
        assert_eq!(bf.b(), 0xBB);
        assert_eq!(bf.c(), 0xCC);
        assert_eq!(bf.d(), 0xDD);

        let recovered: u32 = bf.into();
        assert_eq!(recovered, raw, "From<Bitfield> for u32 should recover the raw value");
    }

    #[cfg(feature = "generate_default")]
    #[test]
    fn feature_generate_default_matches_new() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x10)]
            a: u8,
            #[bits(default = 0x20)]
            b: u8,
            #[bits(default = 0x30)]
            c: u8,
            #[bits(default = 0x40)]
            d: u8,
        }

        let via_new = Bitfield::new();
        let via_default = Bitfield::default();
        assert_eq!(
            via_new.into_bits(),
            via_default.into_bits(),
            "Default::default() and new() must produce identical bitfields"
        );
        assert_eq!(via_default.a(), 0x10);
        assert_eq!(via_default.b(), 0x20);
        assert_eq!(via_default.c(), 0x30);
        assert_eq!(via_default.d(), 0x40);
    }

    #[cfg(feature = "generate_debug")]
    #[test]
    fn feature_generate_debug_format_succeeds() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0xAB)]
            a: u8,
            #[bits(default = 0xCD)]
            b: u8,
            #[bits(8)]
            c: u8,
            #[bits(8)]
            d: u8,
        }

        let bf = Bitfield::new();
        let s = format!("{:?}", bf);
        assert!(s.contains("Bitfield"), "Debug output should include the struct name");
        assert!(s.contains("a"), "Debug output should include field 'a'");
        assert!(s.contains("b"), "Debug output should include field 'b'");
    }

    #[cfg(feature = "derive_copy")]
    #[test]
    fn feature_derive_copy_bitfield_is_copy() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0xAA)]
            a: u8,
            #[bits(default = 0xBB)]
            b: u8,
            #[bits(default = 0xCC)]
            c: u8,
            #[bits(default = 0xDD)]
            d: u8,
        }

        let original = Bitfield::new();
        let copy = original;

        assert_eq!(original.into_bits(), copy.into_bits());
    }

    #[cfg(feature = "generate_builder")]
    #[test]
    fn feature_generate_builder_applies_defaults() {
        use bitfields::bitfield;
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

        let bf = BitfieldBuilder::new().build();
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.b(), 0x34);
        assert_eq!(bf.c(), 0x56);
        assert_eq!(bf.d(), 0x78);
        assert_eq!(bf.into_bits(), 0x78563412);
    }

    #[cfg(feature = "generate_builder")]
    #[test]
    fn feature_generate_builder_with_field_overrides_default() {
        use bitfields::bitfield;
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

        let bf = BitfieldBuilder::new().with_a(0xFF).with_d(0x00).build();
        assert_eq!(bf.a(), 0xFF, "with_a(0xFF) should override default 0x12");
        assert_eq!(bf.b(), 0x34, "b should keep its default");
        assert_eq!(bf.c(), 0x56, "c should keep its default");
        assert_eq!(bf.d(), 0x00, "with_d(0x00) should override default 0x78");
    }

    #[cfg(feature = "generate_bit_ops")]
    #[test]
    fn feature_generate_bit_ops_get_and_set() {
        use bitfields::bitfield;
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = 0b1010_1010)]
            val: u8,
        }

        let mut bf = Bitfield::new();

        assert!(!bf.get_bit(0), "bit 0 should be 0");
        assert!(bf.get_bit(1), "bit 1 should be 1");
        assert!(!bf.get_bit(2), "bit 2 should be 0");
        assert!(bf.get_bit(3), "bit 3 should be 1");

        bf.set_bit(0, true);
        assert!(bf.get_bit(0), "bit 0 should be 1 after set_bit");

        bf.set_bit(1, false);
        assert!(!bf.get_bit(1), "bit 1 should be 0 after set_bit");
    }

    #[cfg(feature = "generate_write_bit_ops")]
    #[test]
    fn feature_generate_write_bit_ops_write_bits() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x11)]
            a: u8,
            #[bits(default = 0x22)]
            b: u8,
            #[bits(default = 0x33)]
            c: u8,
            #[bits(default = 0x44)]
            d: u8,
        }

        let mut bf = Bitfield::new();

        bf.write_bits(0xAA_BB_CC_DD);
        assert_eq!(bf.a(), 0xDD);
        assert_eq!(bf.b(), 0xCC);
        assert_eq!(bf.c(), 0xBB);
        assert_eq!(bf.d(), 0xAA);
    }

    #[cfg(feature = "generate_write_bit_ops")]
    #[test]
    fn feature_generate_write_bit_ops_preserves_read_only_fields() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,

            #[bits(default = 0x78)]
            _reserved: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_bits(0x11_22_33_44);
        assert_eq!(bf.a(), 0x44);
        assert_eq!(bf.b(), 0x33);
        assert_eq!(bf.c(), 0x22);
        assert_eq!(
            bf.into_bits(),
            0x78_22_33_44,
            "reserved field must be unchanged after write_bits"
        );
    }

    #[cfg(feature = "write_endian_big")]
    #[test]
    fn feature_write_endian_big_write_be_bits() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_be_bits(0x11_22_33_44);
        assert_eq!(bf.a(), 0x44);
        assert_eq!(bf.b(), 0x33);
        assert_eq!(bf.c(), 0x22);
        assert_eq!(bf.d(), 0x11);
    }

    #[cfg(feature = "write_endian_little")]
    #[test]
    fn feature_write_endian_little_write_le_bits() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_le_bits(0x11_22_33_44);
        assert_eq!(bf.a(), 0x11, "LE write: a should be 0x11");
        assert_eq!(bf.b(), 0x22);
        assert_eq!(bf.c(), 0x33);
        assert_eq!(bf.d(), 0x44);
    }

    #[cfg(feature = "generate_clear_bit_ops")]
    #[test]
    fn feature_generate_clear_bits_zeroes_all_fields() {
        use bitfields::bitfield;
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

        let mut bf = Bitfield::new();
        bf.clear_bits();
        assert_eq!(bf.a(), 0, "a should be zeroed");
        assert_eq!(bf.b(), 0, "b should be zeroed");
        assert_eq!(bf.c(), 0, "c should be zeroed");
        assert_eq!(bf.d(), 0, "d should be zeroed");
        assert_eq!(bf.into_bits(), 0);
    }

    #[cfg(feature = "generate_clear_bit_ops")]
    #[test]
    fn feature_generate_clear_bits_with_defaults_restores_defaults() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            b: u8,
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let mut bf = Bitfield::new();
        bf.clear_bits_with_defaults();
        assert_eq!(bf.a(), 0x12, "a must be restored to its default 0x12");
        assert_eq!(bf.b(), 0x00, "b has no default → stays zero");
        assert_eq!(bf.c(), 0x00, "c has no default → stays zero");
        assert_eq!(bf.d(), 0x78, "d must be restored to its default 0x78");
        assert_eq!(bf.into_bits(), 0x78_00_00_12);
    }

    #[cfg(feature = "generate_clear_bit_ops")]
    #[test]
    fn feature_generate_clear_bits_preserves_reserved_field() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(default = 0x78)]
            _reserved: u8,
        }

        let mut bf = Bitfield::new();
        bf.clear_bits();
        assert_eq!(bf.a(), 0, "a should be zero after clear_bits");
        assert_eq!(bf.b(), 0, "b should be zero after clear_bits");
        assert_eq!(bf.c(), 0, "c should be zero after clear_bits");
        assert_eq!(
            bf.into_bits(),
            0x78_00_00_00,
            "reserved field must retain its default 0x78 after clear_bits"
        );
    }

    #[cfg(feature = "generate_set_get_bit_ops")]
    #[test]
    fn feature_generate_get_bits_range_reads_correct_slice() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0xAB)]
            a: u8,
            #[bits(default = 0xCD)]
            b: u8,
            #[bits(default = 0xEF)]
            c: u8,
            #[bits(default = 0x12)]
            d: u8,
        }

        let bf = Bitfield::new();
        assert_eq!(bf.get_bits_range(0, 8), 0xAB, "bits[7..=0] should be 0xAB");
        assert_eq!(bf.get_bits_range(8, 8), 0xCD, "bits[15..=8] should be 0xCD");
        assert_eq!(bf.get_bits_range(16, 8), 0xEF, "bits[23..=16] should be 0xEF");
        assert_eq!(bf.get_bits_range(24, 8), 0x12, "bits[31..=24] should be 0x12");
    }

    #[cfg(feature = "generate_set_get_bit_ops")]
    #[test]
    fn feature_generate_set_bits_range_writes_correct_slice() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_bits_range(0, 8, 0x11);
        bf.set_bits_range(8, 8, 0x22);
        bf.set_bits_range(16, 8, 0x33);
        bf.set_bits_range(24, 8, 0x44);

        assert_eq!(bf.a(), 0x11);
        assert_eq!(bf.b(), 0x22);
        assert_eq!(bf.c(), 0x33);
        assert_eq!(bf.d(), 0x44);
        assert_eq!(bf.into_bits(), 0x44332211);
    }

    #[cfg(feature = "generate_invert_bit_ops")]
    #[test]
    fn feature_generate_invert_bit_ops_invert_field() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x0F)]
            a: u8,
            #[bits(default = 0xF0)]
            b: u8,
            #[bits(default = 0x00)]
            c: u8,
            #[bits(default = 0xFF)]
            d: u8,
        }

        let mut bf = Bitfield::new();
        bf.invert_a();
        assert_eq!(bf.a(), 0xF0, "invert_a: 0x0F → 0xF0");
        bf.invert_b();
        assert_eq!(bf.b(), 0x0F, "invert_b: 0xF0 → 0x0F");
        bf.invert_c();
        assert_eq!(bf.c(), 0xFF, "invert_c: 0x00 → 0xFF");
        bf.invert_d();
        assert_eq!(bf.d(), 0x00, "invert_d: 0xFF → 0x00");
    }

    #[cfg(feature = "generate_invert_bit_ops")]
    #[test]
    fn feature_generate_invert_bit_ops_invert_bits_whole() {
        use bitfields::bitfield;
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x00)]
            a: u8,
            #[bits(default = 0xFF)]
            b: u8,
            #[bits(default = 0x0F)]
            c: u8,
            #[bits(default = 0xF0)]
            d: u8,
        }

        let mut bf = Bitfield::new();
        bf.invert_bits();
        assert_eq!(bf.a(), 0xFF);
        assert_eq!(bf.b(), 0x00);
        assert_eq!(bf.c(), 0xF0);
        assert_eq!(bf.d(), 0x0F);
    }

    #[cfg(feature = "generate_invert_bit_ops")]
    #[test]
    fn feature_generate_invert_bit_ops_bool_field() {
        use bitfields::bitfield;
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = true)]
            flag_a: bool,
            #[bits(default = false)]
            flag_b: bool,
            #[bits(6)]
            _padding: u8,
        }

        let mut bf = Bitfield::new();
        assert!(bf.flag_a(), "flag_a should start true");
        assert!(!bf.flag_b(), "flag_b should start false");

        bf.invert_flag_a();
        bf.invert_flag_b();
        assert!(!bf.flag_a(), "flag_a should be false after invert");
        assert!(bf.flag_b(), "flag_b should be true after invert");
    }
}
