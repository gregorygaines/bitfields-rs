#[cfg(test)]
mod tests {
    use bitfields::bitfield;

    #[test]
    fn array_field_single_byte_backing() {
        #[bitfield([u8; 1])]
        pub struct Bitfield {
            a: [u8; 1],
        }

        let mut bf = Bitfield::new();
        bf.set_a([0xAB]);
        assert_eq!(bf.a(), [0xAB]);
        assert_eq!(bf.into_bytes(), [0xAB]);
    }

    #[test]
    fn array_field_two_byte_backing() {
        #[bitfield([u8; 2])]
        pub struct Bitfield {
            a: [u8; 2],
        }

        let mut bf = Bitfield::new();
        bf.set_a([0x11, 0x22]);
        assert_eq!(bf.a(), [0x11, 0x22]);
        assert_eq!(bf.into_bytes(), [0x22, 0x11]);
    }

    #[test]
    fn array_field_four_byte_backing() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_a([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.a(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.into_bytes(), [0x44, 0x33, 0x22, 0x11]);
    }

    #[test]
    fn array_field_eight_byte_backing() {
        #[bitfield([u8; 8])]
        pub struct Bitfield {
            a: [u8; 8],
        }

        let mut bf = Bitfield::new();
        bf.set_a([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        assert_eq!(bf.a(), [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
        assert_eq!(bf.into_bytes(), [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]);
    }

    #[test]
    fn array_field_getter() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = Bitfield::from_bytes([0x44, 0x33, 0x22, 0x11]);
        assert_eq!(bf.logo(), [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn array_field_setter() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_logo([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.logo(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.into_bytes(), [0x44, 0x33, 0x22, 0x11]);
    }

    #[test]
    fn array_field_checked_setter_ok() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let mut bf = Bitfield::new();
        assert!(bf.checked_set_logo([0xDE, 0xAD, 0xBE, 0xEF]).is_ok());
        assert_eq!(bf.logo(), [0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn array_field_mixed_scalar_before() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 3],
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_logo([0x34, 0x56, 0x78]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56, 0x78]);
        assert_eq!(bf.into_bytes(), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn array_field_mixed_scalar_after() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 3],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_logo([0x12, 0x34, 0x56]);
        bf.set_b(0x78);
        assert_eq!(bf.logo(), [0x12, 0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
        assert_eq!(bf.into_bytes(), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn array_field_mixed_scalar_both_sides() {
        #[bitfield([u8; 4])]
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
        assert_eq!(bf.into_bytes(), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn array_field_multiple_array_fields() {
        #[bitfield([u8; 8])]
        pub struct Bitfield {
            first: [u8; 4],
            second: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_first([0x11, 0x22, 0x33, 0x44]);
        bf.set_second([0x55, 0x66, 0x77, 0x88]);
        assert_eq!(bf.first(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.second(), [0x55, 0x66, 0x77, 0x88]);
        assert_eq!(bf.into_bytes(), [0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]);
    }

    #[test]
    fn array_field_mixed_with_bool_fields() {
        #[bitfield([u8; 4])]
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
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = Bitfield::new();
        assert_eq!(bf.logo(), [0x00, 0x00, 0x00, 0x00]);
        assert_eq!(bf.into_bytes(), [0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn array_field_default_trait() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = Bitfield::default();
        assert_eq!(bf.logo(), [0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn array_field_new_without_defaults() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::new_without_defaults();
        assert_eq!(bf.a(), 0);
        assert_eq!(bf.logo(), [0, 0]);
        assert_eq!(bf.b(), 0);
        assert_eq!(bf.into_bytes(), [0, 0, 0, 0]);
    }

    #[test]
    fn array_field_builder_new_without_defaults() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = BitfieldBuilder::new_without_defaults().build();
        assert_eq!(bf.a(), 0);
        assert_eq!(bf.logo(), [0, 0]);
        assert_eq!(bf.b(), 0);
    }

    #[test]
    fn array_field_from_bytes_full_field() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = Bitfield::from_bytes([0x44, 0x33, 0x22, 0x11]);
        assert_eq!(bf.logo(), [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn array_field_from_bytes_mixed() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::from_bytes([0x78, 0x56, 0x34, 0x12]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_into_bytes_full_field() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_logo([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.into_bytes(), [0x44, 0x33, 0x22, 0x11]);
    }

    #[test]
    fn array_field_round_trip_bytes() {
        #[bitfield([u8; 8])]
        pub struct Bitfield {
            header: [u8; 2],
            payload: [u8; 4],
            footer: [u8; 2],
        }

        let mut bf1 = Bitfield::new();
        bf1.set_header([0xAA, 0xBB]);
        bf1.set_payload([0x11, 0x22, 0x33, 0x44]);
        bf1.set_footer([0xCC, 0xDD]);

        let bytes = bf1.into_bytes();
        let bf2 = Bitfield::from_bytes(bytes);

        assert_eq!(bf2.header(), [0xAA, 0xBB]);
        assert_eq!(bf2.payload(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf2.footer(), [0xCC, 0xDD]);
        assert_eq!(bf2.into_bytes(), bytes);
    }

    #[test]
    fn array_field_from_bytes_with_defaults() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            #[bits(access = ro)]
            ro_logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::from_bytes([0x78, 0x56, 0x34, 0x12]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.ro_logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_write_bytes() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_bytes([0x12, 0x34, 0x56, 0x78]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
        assert_eq!(bf.into_bytes(), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn array_field_write_bytes_full_array_field() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.write_bytes([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.logo(), [0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn array_field_clear_bytes() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::from_bytes([0x78, 0x56, 0x34, 0x12]);
        bf.clear_bytes();
        assert_eq!(bf.a(), 0);
        assert_eq!(bf.logo(), [0, 0]);
        assert_eq!(bf.b(), 0);
        assert_eq!(bf.into_bytes(), [0, 0, 0, 0]);
    }

    #[test]
    fn array_field_write_bytes_with_defaults() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            logo: [u8; 2],
            #[bits(default = 0x78)]
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_bytes_with_defaults([0x99, 0x34, 0x56, 0x99]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
        assert_eq!(bf.into_bytes(), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn array_field_clear_bytes_with_defaults() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            logo: [u8; 2],
            #[bits(default = 0x78)]
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.clear_bytes_with_defaults();
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0, 0]);
        assert_eq!(bf.b(), 0x78);
        assert_eq!(bf.into_bytes(), [0x78, 0x00, 0x00, 0x12]);
    }

    #[test]
    fn array_field_write_le_bytes() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_le_bytes([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.a(), 0x44);
        assert_eq!(bf.logo(), [0x33, 0x22]);
        assert_eq!(bf.b(), 0x11);
    }

    #[test]
    fn array_field_write_be_bytes() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_be_bytes([0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.a(), 0x11);
        assert_eq!(bf.logo(), [0x22, 0x33]);
        assert_eq!(bf.b(), 0x44);
    }

    #[test]
    fn array_field_from_slice() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::from_slice(&[0x78, 0x56, 0x34, 0x12]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_lsb_order() {
        #[bitfield([u8; 4], order = lsb)]
        pub struct Bitfield {
            low: [u8; 2],
            high: [u8; 2],
        }

        let mut bf = Bitfield::new();
        bf.set_low([0x11, 0x22]);
        bf.set_high([0x33, 0x44]);
        assert_eq!(bf.low(), [0x11, 0x22]);
        assert_eq!(bf.high(), [0x33, 0x44]);
        assert_eq!(bf.into_bytes(), [0x44, 0x33, 0x22, 0x11]);
    }

    #[test]
    fn array_field_msb_order() {
        #[bitfield([u8; 4], order = msb)]
        pub struct Bitfield {
            high: [u8; 2],
            low: [u8; 2],
        }

        let mut bf = Bitfield::new();
        bf.set_high([0x11, 0x22]);
        bf.set_low([0x33, 0x44]);
        assert_eq!(bf.high(), [0x11, 0x22]);
        assert_eq!(bf.low(), [0x33, 0x44]);
    }

    #[test]
    fn array_field_access_read_write() {
        #[bitfield([u8; 4])]
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
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            #[bits(access = ro)]
            data: [u8; 4],
        }

        let bf = BitfieldBuilder::new().with_data([0xAA, 0xBB, 0xCC, 0xDD]).build();
        assert_eq!(bf.data(), [0xAA, 0xBB, 0xCC, 0xDD]);
    }

    #[test]
    fn array_field_access_write_only_can_write() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            #[bits(access = wo)]
            data: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn array_field_access_no_access() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            #[bits(access = na)]
            data: [u8; 4],
        }

        let _ = Bitfield::new();
    }

    #[test]
    fn array_field_from_bytes_sets_read_only() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            rw: u8,
            #[bits(access = ro)]
            ro_logo: [u8; 2],
            other: u8,
        }

        let bf = Bitfield::from_bytes([0x78, 0x56, 0x34, 0x12]);
        assert_eq!(bf.rw(), 0x12);
        assert_eq!(bf.ro_logo(), [0x34, 0x56]);
        assert_eq!(bf.other(), 0x78);
    }

    #[test]
    fn array_field_write_bytes_respects_read_only() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            rw: u8,
            #[bits(access = ro)]
            ro_logo: [u8; 2],
            other: u8,
        }

        let mut bf = BitfieldBuilder::new().with_ro_logo([0x34, 0x56]).build();
        bf.write_bytes([0xFF, 0xFF, 0xFF, 0xFF]);
        assert_eq!(bf.rw(), 0xFF);
        assert_eq!(bf.ro_logo(), [0x34, 0x56]);
        assert_eq!(bf.other(), 0xFF);
    }

    #[test]
    fn array_field_reserved() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            _reserved: [u8; 3],
        }

        let bf = Bitfield::new();
        assert_eq!(bf.a(), 0);
        assert_eq!(bf.into_bytes(), [0, 0, 0, 0]);
    }

    #[test]
    fn array_field_reserved_with_regular_fields() {
        #[bitfield([u8; 8])]
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
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = BitfieldBuilder::new().build();
        assert_eq!(bf.logo(), [0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn array_field_builder_with_field() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            logo: [u8; 4],
        }

        let bf = BitfieldBuilder::new().with_logo([0x11, 0x22, 0x33, 0x44]).build();
        assert_eq!(bf.logo(), [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(bf.into_bytes(), [0x44, 0x33, 0x22, 0x11]);
    }

    #[test]
    fn array_field_builder_mixed_fields() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = BitfieldBuilder::new().with_a(0x12).with_logo([0x34, 0x56]).with_b(0x78).build();
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
        assert_eq!(bf.into_bytes(), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn array_field_inverted_getter() {
        #[bitfield([u8; 2])]
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
        #[bitfield([u8; 1])]
        pub struct Bitfield {
            data: [u8; 1],
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x55]);
        assert_eq!(bf.data_inverted(), [!0x55]);
    }

    #[test]
    fn array_field_debug() {
        #[bitfield([u8; 4])]
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
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            data: [u8; 4],
        }

        let mut bf = Bitfield::new();
        bf.set_data([0x11, 0x22, 0x33, 0x44]);
        let bf2 = bf;
        assert_eq!(bf.data(), bf2.data());
    }

    #[test]
    fn array_field_from_array_trait() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::from([0x78, 0x56, 0x34, 0x12]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_into_array_trait() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_logo([0x34, 0x56]);
        bf.set_b(0x78);
        let val: [u8; 4] = bf.into();
        assert_eq!(val, [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn array_field_from_bytes_little_endian() {
        #[bitfield([u8; 4], from_endian = little)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::from_bytes([0x12, 0x34, 0x56, 0x78]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_from_bytes_big_endian() {
        #[bitfield([u8; 4], from_endian = big)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let bf = Bitfield::from_bytes([0x78, 0x56, 0x34, 0x12]);
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0x34, 0x56]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn array_field_into_bytes_little_endian() {
        #[bitfield([u8; 4], into_endian = little)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_logo([0x34, 0x56]);
        bf.set_b(0x78);
        assert_eq!(bf.into_bytes(), [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn array_field_into_bytes_big_endian() {
        #[bitfield([u8; 4], into_endian = big)]
        pub struct Bitfield {
            a: u8,
            logo: [u8; 2],
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x12);
        bf.set_logo([0x34, 0x56]);
        bf.set_b(0x78);
        assert_eq!(bf.into_bytes(), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn array_field_with_ignored_field() {
        #[bitfield([u8; 4])]
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
    fn array_field_bitfield_with_bit_ops() {
        #[bitfield([u8; 4], bit_ops = true)]
        pub struct Bitfield {
            data: [u8; 4],
        }

        let bf = Bitfield::from_bytes([0x00, 0x00, 0x00, 0x01]);
        assert!(bf.get_bit(0));
        assert!(!bf.get_bit(1));
    }

    #[test]
    fn large_array_field_192_bits() {
        #[bitfield([u8; 24])]
        pub struct Bitfield {
            data: [u8; 24],
        }

        let mut bf = Bitfield::new();
        let mut input = [0u8; 24];
        for (i, b) in input.iter_mut().enumerate() {
            *b = (i + 1) as u8;
        }
        bf.set_data(input);
        assert_eq!(bf.data(), input);
    }

    #[test]
    fn large_array_field_with_mixed_fields() {
        #[bitfield([u8; 64])]
        pub struct Header {
            #[bits(8)]
            entry_point: u8,
            #[bits(384)]
            nintendo_logo: [u8; 48],
            #[bits(120)]
            _padding: [u8; 15],
        }

        let mut header = Header::new();
        let mut logo = [0u8; 48];
        for (i, b) in logo.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(1);
        }
        header.set_entry_point(0xAA);
        header.set_nintendo_logo(logo);
        assert_eq!(header.entry_point(), 0xAA);
        assert_eq!(header.nintendo_logo(), logo);
    }

    #[test]
    fn large_array_field_256_bit_mixed() {
        #[bitfield([u8; 36])]
        pub struct Packet {
            src_ip: [u8; 4],
            dst_ip: [u8; 4],
            payload: [u8; 28],
        }

        let mut pkt = Packet::new();
        pkt.set_src_ip([192, 168, 1, 1]);
        pkt.set_dst_ip([10, 0, 0, 1]);
        let mut payload = [0u8; 28];
        for (i, b) in payload.iter_mut().enumerate() {
            *b = i as u8;
        }
        pkt.set_payload(payload);

        assert_eq!(pkt.src_ip(), [192, 168, 1, 1]);
        assert_eq!(pkt.dst_ip(), [10, 0, 0, 1]);
        assert_eq!(pkt.payload(), payload);
    }

    #[test]
    fn array_field_write_defaults() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            logo: [u8; 2],
            #[bits(default = 0x78)]
            b: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_bytes([0xFF, 0xFF, 0xFF, 0xFF]);
        bf.write_defaults();
        assert_eq!(bf.a(), 0x12);
        assert_eq!(bf.logo(), [0xFF, 0xFF]);
        assert_eq!(bf.b(), 0x78);
    }

    #[test]
    fn reserved_array_field_respects_default_on_write_and_clear() {
        #[bitfield([u8; 4])]
        pub struct Bitfield {
            a: u8,
            b: u8,
            _reserved: u8,
            c: u8,
        }

        let mut bf = Bitfield::new();
        bf.write_bytes([0x44, 0x33, 0x22, 0x11]);
        assert_eq!(bf.a(), 0x44);
        assert_eq!(bf.b(), 0x33);
        assert_eq!(bf.c(), 0x11);
        assert_eq!(bf.into_bytes(), [0x11, 0x00, 0x33, 0x44]);

        bf.clear_bytes();
        assert_eq!(bf.a(), 0);
        assert_eq!(bf.b(), 0);
        assert_eq!(bf.c(), 0);
    }
}
