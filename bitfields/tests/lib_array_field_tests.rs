#[cfg(test)]
mod tests {
    use bitfields::bitfield;

    #[test]
    fn array_field_u8_backing() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: [u8; 1],
        }

        let mut bf = Bitfield::new();
        bf.set_a([0xAB]);
        assert_eq!(bf.a(), [0xAB]);
        assert_eq!(bf.into_bits(), 0xAB_u8);
    }

    #[test]
    fn array_field_u16_backing() {
        #[bitfield(u16)]
        pub struct Bitfield {
            a: [u8; 2],
        }

        let mut bf = Bitfield::new();
        bf.set_a([0x11, 0x22]);
        assert_eq!(bf.a(), [0x11, 0x22]);
        assert_eq!(bf.into_bits(), 0x2211_u16);
    }

    #[test]
    fn array_field_u32_backing() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_a([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.a(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.into_bits(), 0x44332211_u32);
    }

    #[test]
    fn array_field_u64_backing() {
        #[bitfield(u64)]
        pub struct Bitfield {
            a: [u8; 8],
        }

        let mut bf = Bitfield::new();
        bf.set_a([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        assert_eq!(bf.a(), [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        assert_eq!(bf.into_bits(), 0x0807060504030201_u64);
    }

    #[test]
    fn array_field_u128_backing() {
        #[bitfield(u128)]
        pub struct Bitfield {
            a: [u8; 16],
        }

        let mut bf = Bitfield::new();
        let val = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];
        bf.set_a(val);
        assert_eq!(bf.a(), val);
    }

    #[test]
    fn array_field_getter() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = Bitfield::from_bits(0x44332211);
        assert_eq!(bf.logo(), [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn array_field_setter() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_logo([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.logo(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.into_bits(), 0x44332211);
    }

    #[test]
    fn array_field_checked_setter_ok() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let mut bf = Bitfield::new();
        assert!(bf.checked_set_logo([0xDE, 0xAD, 0xBE, 0xEF]).is_ok());
        assert_eq!(bf.logo(), [0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn array_field_multiple_same_size() {
        #[bitfield(u64)]
        pub struct Bitfield {
            first: [u8; 4],
            second: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_first([0x11, 0x22, 0x33, 0x44]);
        bf.set_second([0x55, 0x66, 0x77, 0x88]);
        assert_eq!(bf.first(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.second(), [0x55, 0x66, 0x77, 0x88]);
        assert_eq!(bf.into_bits(), 0x8877665544332211_u64);
    }

    #[test]
    fn array_field_multiple_different_size() {
        #[bitfield(u64)]
        pub struct Bitfield {
            small: [u8; 2],
            large: [u8; 6],
        }

        let mut bf = Bitfield::new();
        bf.set_small([0xAA, 0xBB]);
        bf.set_large([0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
        assert_eq!(bf.small(), [0xAA, 0xBB]);
        assert_eq!(bf.large(), [0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }

    #[test]
    fn array_field_mixed_scalar_before() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 3],
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_logo([0x34, 0x56, 0x78]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56, 0x78]);
        assert_eq!(bf.into_bits(), 0x78563412);
    }

    #[test]
    fn array_field_mixed_scalar_after() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 3],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_logo([0x12, 0x34, 0x56]);
        bf.set_b(0x78);
        assert_eq!(bf.logo(), [0x12, 0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
        assert_eq!(bf.into_bits(), 0x78563412);
    }

    #[test]
    fn array_field_mixed_scalar_both_sides() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_logo([0x34, 0x56]);
        bf.set_b(0x78);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
        assert_eq!(bf.into_bits(), 0x78563412);
    }

    #[test]
    fn array_field_mixed_with_bool_fields() {
        #[bitfield(u32)]
        pub struct Bitfield {
            flag_a: bool,
            flag_b: bool,
            #[bits(14)]
            _padding: u16,
            data: [u8; 2],
        }

        let mut bf = Bitfield::new();
        bf.set_flag_a(true);
        bf.set_flag_b(false);
        bf.set_data([0xAB, 0xCD]);
        assert!(bf.flag_a());
        assert!(!bf.flag_b());
        assert_eq!(bf.data(), [0xAB, 0xCD]);
    }

    #[test]
    fn array_field_new_zeroes() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = Bitfield::new();
        assert_eq!(bf.logo(), [0x00, 0x00, 0x00, 0x00]);
        assert_eq!(bf.into_bits(), 0);
    }

    #[test]
    fn array_field_default_trait() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = Bitfield::default();
        assert_eq!(bf.logo(), [0x00, 0x00, 0x00, 0x00]);
        assert_eq!(bf.into_bits(), 0);
    }

    #[test]
    fn array_field_from_bits_single_field() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = Bitfield::from_bits(0x44332211);
        assert_eq!(bf.logo(), [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn array_field_into_bits_single_field() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_logo([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.into_bits(), 0x44332211_u32);
    }

    #[test]
    fn array_field_from_bits_mixed() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::from_bits(0x78563412);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_into_bits_mixed() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_logo([0x34, 0x56]);
        bf.set_b(0x78);
        assert_eq!(bf.into_bits(), 0x78563412_u32);
    }

    #[test]
    fn array_field_round_trip() {
        #[bitfield(u64)]
        pub struct Bitfield {
            header: [u8; 2],
            payload: [u8; 4],
            footer: [u8; 2],
        }

        let bits = 0xBBBB_CCCC_CCCC_AAAA_u64;
        let bf = Bitfield::from_bits(bits);
        assert_eq!(bf.header(), [0xAA, 0xAA]);
        assert_eq!(bf.payload(), [0xCC, 0xCC, 0xCC, 0xCC]);
        assert_eq!(bf.footer(), [0xBB, 0xBB]);
        assert_eq!(bf.into_bits(), bits);
    }

    #[test]
    fn array_field_lsb_order() {
        #[bitfield(u32, order = lsb)]
        pub struct Bitfield {
            low: [u8; 2],
            high: [u8; 2],
        }

        let mut bf = Bitfield::new();
        bf.set_low([0x11, 0x22]);
        bf.set_high([0x33, 0x44]);
        assert_eq!(bf.low(), [0x11, 0x22]);
        assert_eq!(bf.high(), [0x33, 0x44]);
        assert_eq!(bf.into_bits(), 0x44332211_u32);
    }

    #[test]
    fn array_field_msb_order() {
        #[bitfield(u32, order = msb)]
        pub struct Bitfield {
            high: [u8; 2],
            low: [u8; 2],
        }

        let mut bf = Bitfield::new();
        bf.set_high([0x11, 0x22]);
        bf.set_low([0x33, 0x44]);
        assert_eq!(bf.into_bits(), 0x22114433_u32);
        assert_eq!(bf.high(), [0x11, 0x22]);
        assert_eq!(bf.low(), [0x33, 0x44]);
    }

    #[test]
    fn array_field_access_read_write() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(access = rw)]
            data: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.data(), [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn array_field_access_read_only() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(access = ro)]
            data: [u8; 4],
        }

        let bf = BitfieldBuilder::new().with_data([0xAA, 0xBB, 0xCC, 0xDD]).build();
        assert_eq!(bf.data(), [0xAA, 0xBB, 0xCC, 0xDD]);
    }

    #[test]
    fn array_field_access_write_only_can_write() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(access = wo)]
            data: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn array_field_access_no_access() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(access = na)]
            data: [u8; 4],
        }

        let _ = Bitfield::new();
    }

    #[test]
    fn array_field_from_bits_sets_read_only() {
        #[bitfield(u32)]
        pub struct Bitfield {
            rw: u8,
            #[bits(access = ro)]
            ro_data: [u8; 2],
            other: u8,
        }

        let bf = Bitfield::from_bits(0x44332211);
        assert_eq!(bf.rw(), 0x11);
        assert_eq!(bf.ro_data(), [0x22, 0x33]);
        assert_eq!(bf.other(), 0x44);
    }

    #[test]
    fn array_field_reserved() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            _reserved: [u8; 3],
        }

        let bf = Bitfield::new();
        assert_eq!(bf.a(), 0);
        assert_eq!(bf.into_bits(), 0);
    }

    #[test]
    fn array_field_reserved_with_regular_fields() {
        #[bitfield(u64)]
        pub struct Bitfield {
            a: u8,
            _reserved_logo: [u8; 6],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_b(0x78);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_builder_new_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = BitfieldBuilder::new().build();
        assert_eq!(bf.logo(), [0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn array_field_builder_with_field() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = BitfieldBuilder::new().with_logo([0x11, 0x22, 0x33, 0x44]).build();
        assert_eq!(bf.logo(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.into_bits(), 0x44332211_u32);
    }

    #[test]
    fn array_field_builder_mixed_fields() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = BitfieldBuilder::new().with_a(0x12).with_logo([0x34, 0x56]).with_b(0x78).build();
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
        assert_eq!(bf.into_bits(), 0x78563412_u32);
    }

    #[test]
    fn array_field_builder_checked_with() {
        #[bitfield(u32)]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let result = BitfieldBuilder::new().checked_with_logo([0xDE, 0xAD, 0xBE, 0xEF]);
        assert!(result.is_ok());
    }

    #[test]
    fn array_field_new_without_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::new_without_defaults();
        assert_eq!(bf.a(), 0);
        assert_eq!(bf.logo(), [0, 0]);
        assert_eq!(bf.b(), 0);
        assert_eq!(bf.into_bits(), 0);
    }

    #[test]
    fn array_field_builder_new_without_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = BitfieldBuilder::new_without_defaults().build();
        assert_eq!(bf.a(), 0);
        assert_eq!(bf.logo(), [0, 0]);
        assert_eq!(bf.b(), 0);
        assert_eq!(bf.into_bits(), 0);
    }

    #[test]
    fn array_field_write_bits() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_bits(0x78563412);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
        assert_eq!(bf.into_bits(), 0x78563412);
    }

    #[test]
    fn array_field_clear_bits() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::from_bits(0x78563412);
        bf.clear_bits();
        assert_eq!(bf.a(), 0);
        assert_eq!(bf.logo(), [0, 0]);
        assert_eq!(bf.b(), 0);
        assert_eq!(bf.into_bits(), 0);
    }

    #[test]
    fn array_field_write_le_bits() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_le_bits(0x12345678_u32);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_write_be_bits() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_be_bits(0x78563412_u32);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_inverted_getter() {
        #[bitfield(u16)]
        pub struct Bitfield {
            data: [u8; 2],
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x00, 0xFF]);
        let inv = bf.data_inverted();
        assert_eq!(inv, [0xFF, 0x00]);
    }

    #[test]
    fn array_field_inverted_getter_all_bits() {
        #[bitfield(u8)]
        pub struct Bitfield {
            data: [u8; 1],
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x55]);
        assert_eq!(bf.data_inverted(), [!0x55]);
    }

    #[test]
    fn array_field_debug() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_logo([0x34, 0x56]);
        bf.set_b(0x78);
        let s = format!("{:?}", bf);
        assert!(s.contains("Bitfield"));
        assert!(s.contains("a"));
        assert!(s.contains("logo"));
        assert!(s.contains("b"));
    }

    #[test]
    fn array_field_copy() {
        #[bitfield(u32)]
        pub struct Bitfield {
            data: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x11, 0x22, 0x33, 0x44]);
        let bf2 = bf;
        assert_eq!(bf.data(), bf2.data());
    }

    #[test]
    fn array_field_from_integer_trait() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::from(0x78563412_u32);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_into_integer_trait() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_logo([0x34, 0x56]);
        bf.set_b(0x78);
        let val: u32 = bf.into();
        assert_eq!(val, 0x78563412_u32);
    }

    #[test]
    fn array_field_partial_bits() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(8)]
            partial: [u8; 2],
            _rest: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_partial([0xAB, 0x00]);
        assert_eq!(bf.partial()[0], 0xAB);
    }

    #[test]
    fn array_field_bitfield_with_bit_ops() {
        #[bitfield(u32, bit_ops = true)]
        pub struct Bitfield {
            data: [u8; 4],
        }

        let bf = Bitfield::from_bits(0x00000001);
        assert!(bf.get_bit(0));
        assert!(!bf.get_bit(1));
    }

    #[test]
    fn array_field_with_ignored_field() {
        #[bitfield(u32)]
        pub struct Bitfield {
            data: [u8; 4],
            #[bits(ignore = true)]
            cache: u32,
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.data(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.cache, 0);
    }

    #[test]
    fn array_field_from_bits_little_endian() {
        #[bitfield(u32, from_endian = little)]
        pub struct Bitfield {
            data: [u8; 4],
        }

        let bf = Bitfield::from_bits(0x44332211);
        assert_eq!(bf.data(), [0x44, 0x33, 0x22, 0x11]);
    }

    #[test]
    fn array_field_into_bits_little_endian() {
        #[bitfield(u32, into_endian = little)]
        pub struct Bitfield {
            data: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.into_bits(), 0x11223344);
    }

    #[test]
    fn array_field_full_16_bytes_in_u128() {
        #[bitfield(u128)]
        pub struct Bitfield {
            data: [u8; 16],
        }

        let input = [
            0x01_u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];
        let mut bf = Bitfield::new();
        bf.set_data(input);
        assert_eq!(bf.data(), input);
    }

    #[test]
    fn array_field_write_bits_respects_read_only() {
        #[bitfield(u32)]
        pub struct Bitfield {
            rw: u8,
            #[bits(access = ro)]
            ro_data: [u8; 2],
            other: u8,
        }

        let mut bf = BitfieldBuilder::new().with_ro_data([0x34, 0x56]).build();
        bf.write_bits(0xFF_FF_FF_FF);
        assert_eq!(bf.rw(), 0xFF);
        assert_eq!(bf.ro_data(), [0x34, 0x56]);
        assert_eq!(bf.other(), 0xFF);
    }

    #[test]
    fn bitfield_large_array_field_384_bits() {
        #[bitfield([u8; 64])]
        pub struct GameHeader {
            #[bits(8)]
            entry_point: u8,
            #[bits(384)]
            logo: [u8; 48],
            #[bits(120)]
            _padding: [u8; 15],
        }

        let mut header = GameHeader::new();

        let mut logo = [0u8; 48];
        for (i, b) in logo.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(1);
        }

        header.set_logo(logo);
        assert_eq!(header.logo(), logo);
    }

    #[test]
    fn test_array_field_smaller_than_type_does_not_overflow() {
        #[bitfield(u32)]
        struct Packet {
            header: [u8; 2],
            #[bits(16)]
            payload: [u8; 16], /* [u8; 16] is 128-bit type, but we only have 16 bits allocated
                                * in the u32 bitfield. */
        }

        let mut packet = Packet::new();
        packet.set_header([0u8; 2]);
        packet.set_payload([0xFFu8; 16]);

        assert_eq!(packet.header(), [0u8; 2]);
        let mut expected = [0u8; 16];
        expected[0] = 0xFF;
        expected[1] = 0xFF;
        assert_eq!(packet.payload(), expected);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn compile_error_cases_array_fields() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/errors/array_fields/*.rs");
    }
}
