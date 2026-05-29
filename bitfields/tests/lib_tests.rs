#[cfg(test)]
mod tests {
    use bitfields::bitfield;
    use bitfields::bitflag;

    #[test]
    fn bitfield_arguments() {
        #[bitfield(
            u8,
            order = lsb,
            from_endian = little,
            into_endian = little,
            write_endian = little,
            new = true,
            from_into_bits = true,
            from_traits = true,
            default = true,
            debug = true,
            copy = true,
            bit_ops = true,
            write_bit_ops = true,
            clear_bit_ops = true,
            set_get_bit_ops = true,
            invert_bit_ops = true,
            toggle_bit_ops = true,
            builder = true,
        )]
        struct Bitfield {
            a: u8,
        }

        let _ = Bitfield::new();
        let _ = BitfieldBuilder::new().build();
    }

    #[test]
    fn bitfield_arguments_disabled() {
        #[bitfield(
            u8,
            order = lsb,
            from_endian = little,
            into_endian = little,
            write_endian = little,
            new = true,
            from_into_bits = false,
            from_traits = false,
            default = false,
            debug = false,
            copy = false,
            bit_ops = false,
            write_bit_ops = false,
            clear_bit_ops = false,
            set_get_bit_ops = false,
            invert_bit_ops = false,
            toggle_bit_ops = false,
            builder = false,
        )]
        struct Bitfield {
            a: u8,
        }

        let _ = Bitfield::new();
    }

    #[test]
    fn bitfield_u8() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: u8,
        }
    }

    #[test]
    fn bitfield_u16() {
        #[bitfield(u16)]
        pub struct Bitfield {
            a: u16,
        }
    }

    #[test]
    fn bitfield_u32() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u32,
        }
    }

    #[test]
    fn bitfield_u64() {
        #[bitfield(u64)]
        pub struct Bitfield {
            a: u64,
        }
    }

    #[test]
    fn bitfield_u128() {
        #[bitfield(u128)]
        pub struct Bitfield {
            a: u128,
        }
    }

    #[test]
    fn bitfield_i8() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: i8,
        }
    }

    #[test]
    fn bitfield_i8_default_value() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = -1)]
            a: i8,
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.a(), -1);
    }

    #[test]
    fn bitfield_i8_default_value_binary() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = -0b101)]
            a: i8,
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.a(), -5);
    }

    #[test]
    fn bitfield_i8_default_value_hex() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = -0x5)]
            a: i8,
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.a(), -5);
    }

    #[test]
    fn bitfield_i8_default_value_octal() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = -0o5)]
            a: i8,
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.a(), -5);
    }

    #[test]
    fn bitfield_field_type_bits_sum_to_type_size() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }
    }

    #[test]
    fn bitfield_field_defined_bits_sum_to_type_size() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            #[bits(7)]
            b: u8,
            #[bits(5)]
            c: u8,
            #[bits(12)]
            d: u16,
        }
    }

    #[test]
    fn bitfield_fields_default_value() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0)]
            a: u8,
            #[bits(default = 1)]
            b: u8,
            #[bits(default = 2)]
            c: u8,
            #[bits(default = 3)]
            d: u8,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0);
        assert_eq!(bitfield.b(), 1);
        assert_eq!(bitfield.c(), 2);
        assert_eq!(bitfield.d(), 3);
    }

    #[test]
    fn bitfield_field_const_path_default_value() {
        const MY_DEFAULT_A: u8 = 7;
        const MY_DEFAULT_B: u8 = 42;

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(default = MY_DEFAULT_A)]
            a: u8,
            #[bits(default = MY_DEFAULT_B)]
            b: u8,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), MY_DEFAULT_A);
        assert_eq!(bitfield.b(), MY_DEFAULT_B);
    }

    #[test]
    fn bitfield_fields_integer_identifiers_default_value() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(5, default = 10u8)]
            a: u8,
            #[bits(5, default = 20u16)]
            b: u16,
            #[bits(5, default = 30u32)]
            c: u32,
            #[bits(6, default = 40u64)]
            d: u64,
            #[bits(6, default = 50u128)]
            e: u128,
            #[bits(5, default = 10i8)]
            f: i8,
            #[bits(8, default = 20i16)]
            g: i16,
            #[bits(8, default = 30i32)]
            h: i32,
            #[bits(8, default = 40i64)]
            i: i64,
            #[bits(8, default = 50i128)]
            j: i128,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 10);
        assert_eq!(bitfield.b(), 20);
        assert_eq!(bitfield.c(), 30);
        assert_eq!(bitfield.d(), 40);
        assert_eq!(bitfield.e(), 50);
        assert_eq!(bitfield.f(), 10);
        assert_eq!(bitfield.g(), 20);
        assert_eq!(bitfield.h(), 30);
        assert_eq!(bitfield.i(), 40);
        assert_eq!(bitfield.j(), 50);
    }

    #[test]
    fn bitfield_field_boolean_default_value() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = true)]
            a: bool,
            #[bits(default = false)]
            b: bool,
            #[bits(default = true)]
            c: bool,
            #[bits(default = false)]
            d: bool,
            #[bits(default = true)]
            e: bool,
            #[bits(default = false)]
            f: bool,
            #[bits(default = true)]
            g: bool,
            #[bits(default = false)]
            h: bool,
        }

        let bitfield = Bitfield::new();
        assert!(bitfield.a());
        assert!(!bitfield.b());
        assert!(bitfield.c());
        assert!(!bitfield.d());
        assert!(bitfield.e());
        assert!(!bitfield.f());
        assert!(bitfield.g());
        assert!(!bitfield.h());
    }

    #[test]
    fn bitfield_field_hex_default_value() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(5, default = 0x1)]
            a: u8,
            #[bits(5, default = 0x2)]
            b: u16,
            #[bits(5, default = 0x3)]
            c: u32,
            #[bits(6, default = 0x4)]
            d: u64,
            #[bits(6, default = 0x5)]
            e: u128,
            #[bits(5, default = 0x10)]
            f: i8,
            #[bits(8, default = 0xF_F)]
            g: i16,
            #[bits(8, default = 0x23)]
            h: i32,
            #[bits(8, default = 0x7_F)]
            i: i64,
            #[bits(8, default = 0x3F)]
            j: i128,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0x1);
        assert_eq!(bitfield.b(), 0x2);
        assert_eq!(bitfield.c(), 0x3);
        assert_eq!(bitfield.d(), 0x4);
        assert_eq!(bitfield.e(), 0x5);
        assert_eq!(bitfield.f(), -16);
        assert_eq!(bitfield.g(), -1);
        assert_eq!(bitfield.h(), 0x23);
        assert_eq!(bitfield.i(), 0x7F);
        assert_eq!(bitfield.j(), 0x3F);
    }

    #[test]
    fn bitfield_field_hex_has_float_identifier_default_value() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(default = 0xF32)]
            a: u32,
            #[bits(default = 0xF64)]
            b: u32,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0xF32);
        assert_eq!(bitfield.b(), 0xF64);
    }

    #[test]
    fn bitfield_field_octal_default_value() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(5, default = 0o1)]
            a: u8,
            #[bits(5, default = 0o2)]
            b: u16,
            #[bits(5, default = 0o3)]
            c: u32,
            #[bits(6, default = 0o4)]
            d: u64,
            #[bits(6, default = 0o5)]
            e: u128,
            #[bits(5, default = 0o6)]
            f: i8,
            #[bits(8, default = 0o11)]
            g: i16,
            #[bits(8, default = 0o12)]
            h: i32,
            #[bits(8, default = 0o1_3)]
            i: i64,
            #[bits(8, default = 0o1_4)]
            j: i128,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 1);
        assert_eq!(bitfield.b(), 2);
        assert_eq!(bitfield.c(), 3);
        assert_eq!(bitfield.d(), 4);
        assert_eq!(bitfield.e(), 5);
        assert_eq!(bitfield.f(), 6);
        assert_eq!(bitfield.g(), 9);
        assert_eq!(bitfield.h(), 10);
        assert_eq!(bitfield.i(), 11);
        assert_eq!(bitfield.j(), 12);
    }

    #[test]
    fn bitfield_field_binary_default_value() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(5, default = 0b01)]
            a: u8,
            #[bits(5, default = 0b10)]
            b: u16,
            #[bits(5, default = 0b11)]
            c: u32,
            #[bits(6, default = 0b100)]
            d: u64,
            #[bits(6, default = 0b101)]
            e: u128,
            #[bits(5, default = 0b110)]
            f: i8,
            #[bits(8, default = 0b111)]
            g: i16,
            #[bits(8, default = 0b1000)]
            h: i32,
            #[bits(8, default = 0b10_01)]
            i: i64,
            #[bits(8, default = 0b10_10)]
            j: i128,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 1);
        assert_eq!(bitfield.b(), 2);
        assert_eq!(bitfield.c(), 3);
        assert_eq!(bitfield.d(), 4);
        assert_eq!(bitfield.e(), 5);
        assert_eq!(bitfield.f(), 6);
        assert_eq!(bitfield.g(), 7);
        assert_eq!(bitfield.h(), 8);
        assert_eq!(bitfield.i(), 9);
        assert_eq!(bitfield.j(), 10);
    }

    #[test]
    fn bitfield_field_duplicate_reserved_name() {
        #[bitfield(u16)]
        pub struct Bitfield {
            a: u8,
            #[bits(4)]
            __: u8,
            #[bits(4)]
            __: u8,
        }
    }

    #[test]
    fn bitfield_field_reserved() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(4, default = 0x1)]
            a: u8,
            #[bits(60)]
            _reserved: u64,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0x1);
    }

    #[test]
    fn bitfield_field_reserved_default_value() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(4, default = 0x1)]
            a: u8,
            #[bits(60, default = 0xFFFF_FFFF_FFFF)]
            _reserved: u64,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0x1);
        assert_eq!(bitfield.into_bits(), 0xF_FFFF_FFFF_FFF1);
    }

    #[test]
    fn bitfield_field_multiple_attributes() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(4, default = 0x1)]
            a: u8,
            #[bits(60, default = 0xFFFF_FFFF_FFFF)]
            _reserved: u64,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0x1);
        assert_eq!(bitfield.into_bits(), 0xF_FFFF_FFFF_FFF1);
    }

    #[test]
    fn bitfield_getters() {
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
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78_56_34_12);
    }

    #[test]
    fn bitfield_signed_values_getters() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = -127)]
            a: i8,
            #[bits(default = 0x7F)]
            b: i8,
            #[bits(5, default = 0xF)]
            c: i8,
            #[bits(5, default = 0x1F)]
            d: i8,
            #[bits(6)]
            _reserved: u16,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), -127);
        assert_eq!(bitfield.b(), 127);
        assert_eq!(bitfield.c(), 15);
        assert_eq!(bitfield.d(), -1);
        assert_eq!(bitfield.into_bits(), 0x3EF7F81);
    }

    #[test]
    fn bitfield_setters() {
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

        let mut bitfield = Bitfield::new();
        bitfield.set_a(0x11);
        bitfield.set_b(0x22);
        bitfield.set_c(0x33);
        bitfield.set_d(0x44);
        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.into_bits(), 0x44332211);
    }

    #[test]
    fn bitfield_signed_setters() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: i8,
            #[bits(default = 0x34)]
            b: i8,
            #[bits(default = 0x56)]
            c: i8,
            #[bits(default = 0x78)]
            d: i8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_a(-1);
        bitfield.set_b(-127);
        bitfield.set_c(-15);
        bitfield.set_d(-44);
        assert_eq!(bitfield.a(), -1);
        assert_eq!(bitfield.b(), -127);
        assert_eq!(bitfield.c(), -15);
        assert_eq!(bitfield.d(), -44);
        assert_eq!(bitfield.into_bits(), 0xD4F181FF);
    }

    #[test]
    fn bitfield_checked_setters() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: i8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(4, default = 12)]
            c: u8,
            #[bits(12, default = 0x78)]
            d: i16,
        }

        let mut bitfield = Bitfield::new();
        let a_ok = bitfield.checked_set_a(-1);
        let b_ok = bitfield.checked_set_b(0xFF);
        let c_ok = bitfield.checked_set_c(0xFF);
        let d_ok = bitfield.checked_set_d(0x1FFF);

        assert!(a_ok.is_ok());
        assert!(b_ok.is_ok());
        assert!(c_ok.is_err());
        assert!(c_ok.unwrap_err().contains("Value is too big to fit within the field bits."));
        assert!(d_ok.is_err());
        assert!(d_ok.unwrap_err().contains("Value is too big to fit within the field bits."));
        assert_eq!(bitfield.a(), -1);
        assert_eq!(bitfield.b(), 255);
        assert_eq!(bitfield.c(), 12);
        assert_eq!(bitfield.d(), 120);
        assert_eq!(bitfield.into_bits(), 0x78CFFFF);
    }

    #[test]
    fn bitfield_field_nested_bitfield() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(8, default = NestedBitfield::new())]
            nested_field: NestedBitfield,
        }

        #[bitfield(u8)]
        pub struct NestedBitfield {
            #[bits(4, default = 0x3)]
            a: u8,
            #[bits(4, default = 0x4)]
            b: u16,
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.into_bits(), 0x4312)
    }

    #[test]
    fn bitfield_field_enum_custom_type() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(8, default = CustomType::A)]
            nested_field: CustomType,
        }

        pub enum CustomType {
            A = 0x3,
            B = 0x4,
        }

        impl CustomType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0x3 => Self::A,
                    0x4 => Self::B,
                    _ => unreachable!(),
                }
            }

            const fn into_bits(self) -> u8 {
                match self {
                    Self::A => 0x3,
                    Self::B => 0x4,
                }
            }
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.into_bits(), 0x312)
    }

    #[allow(dead_code)]
    #[test]
    fn bitfield_field_struct_custom_type() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(8, default = CustomType::new())]
            nested_field: CustomType,
        }

        pub struct CustomType {
            a: i8,
            b: u8,
        }

        impl CustomType {
            const fn new() -> Self {
                Self {
                    a: 3,
                    b: 4,
                }
            }

            const fn from_bits(bits: u8) -> Self {
                Self {
                    a: bits as i8,
                    b: bits,
                }
            }

            const fn into_bits(self) -> u8 {
                self.b
            }
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.into_bits(), 0x412)
    }

    #[allow(dead_code)]
    #[test]
    fn bitfield_field_reserved_struct_custom_type() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(8, default = CustomType::new())]
            _nested_field: CustomType,
        }

        pub struct CustomType {
            a: i8,
            b: u8,
        }

        impl CustomType {
            const fn new() -> Self {
                Self {
                    a: 3,
                    b: 4,
                }
            }

            const fn from_bits(bits: u8) -> Self {
                Self {
                    a: bits as i8,
                    b: bits,
                }
            }

            const fn into_bits(self) -> u8 {
                self.b
            }
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.into_bits(), 0x412)
    }

    #[test]
    fn bitfield_default() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(8, default = CustomType::B)]
            d: CustomType,
        }

        #[derive(Debug, PartialEq)]
        pub enum CustomType {
            A = 0x3,
            B = 0x4,
        }

        impl CustomType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0x3 => Self::A,
                    0x4 => Self::B,
                    _ => Self::A,
                }
            }

            const fn into_bits(self) -> u8 {
                match self {
                    Self::A => 0x3,
                    Self::B => 0x4,
                }
            }
        }

        let bitfield = Bitfield::default();
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), CustomType::B);
        assert_eq!(bitfield.into_bits(), 0x4563412);
    }

    #[test]
    fn bitfield_new() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(8, default = CustomType::B)]
            d: CustomType,
        }

        #[derive(Debug, PartialEq)]
        pub enum CustomType {
            A = 0x3,
            B = 0x4,
        }

        impl CustomType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0x3 => Self::A,
                    0x4 => Self::B,
                    _ => Self::A,
                }
            }

            const fn into_bits(self) -> u8 {
                match self {
                    Self::A => 0x3,
                    Self::B => 0x4,
                }
            }
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), CustomType::B);
        assert_eq!(bitfield.into_bits(), 0x4563412);
    }

    #[test]
    fn bitfield_new_without_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(8, default = CustomType::B)]
            d: CustomType,
        }

        #[derive(Debug, PartialEq)]
        pub enum CustomType {
            A = 0x3,
            B = 0x4,
        }

        impl CustomType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0x3 => Self::A,
                    0x4 => Self::B,
                    _ => Self::A,
                }
            }

            const fn into_bits(self) -> u8 {
                match self {
                    Self::A => 0x3,
                    Self::B => 0x4,
                }
            }
        }

        let bitfield = Bitfield::new_without_defaults();
        assert_eq!(bitfield.a(), 0);
        assert_eq!(bitfield.b(), 0);
        assert_eq!(bitfield.c(), 0);
        assert_eq!(bitfield.d(), CustomType::A);
        assert_eq!(bitfield.into_bits(), 0x3000000);
    }

    #[test]
    fn bitfield_builder_new_default_values() {
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

        let bitfield = BitfieldBuilder::new().build();
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78_56_34_12);
    }

    #[test]
    fn bitfield_builder_new() {
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

        let bitfield =
            BitfieldBuilder::new().with_a(0x11).with_b(0x22).with_c(0x33).with_d(0x44).build();
        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.into_bits(), 0x44332211);
    }

    #[test]
    fn bitfield_builder_new_without_defaults_default_values() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(8)]
            d: CustomType,
        }

        #[derive(Debug, PartialEq)]
        pub enum CustomType {
            A = 0x3,
            B = 0x4,
        }

        impl CustomType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0x3 => Self::A,
                    0x4 => Self::B,
                    _ => Self::A,
                }
            }

            const fn into_bits(self) -> u8 {
                match self {
                    Self::A => 0x3,
                    Self::B => 0x4,
                }
            }
        }

        let bitfield = BitfieldBuilder::new_without_defaults().build();
        assert_eq!(bitfield.a(), 0);
        assert_eq!(bitfield.b(), 0);
        assert_eq!(bitfield.c(), 0);
        assert_eq!(bitfield.d(), CustomType::A);
        assert_eq!(bitfield.into_bits(), 0x3000000);
    }

    #[test]
    fn bitfield_new_disabled_builder_new() {
        #[bitfield(u32, new = false)]
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

        let bitfield =
            BitfieldBuilder::new().with_a(0x11).with_b(0x22).with_c(0x33).with_d(0x44).build();
        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.into_bits(), 0x44332211);
    }

    #[test]
    fn bitfield_builder_checked_setters_unsigned_overflow() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(4)]
            a: u8,
            #[bits(12)]
            b: u16,
            #[bits(8)]
            c: u8,
            #[bits(8)]
            d: u8,
        }

        let result = BitfieldBuilder::new().checked_with_a(0x10);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Value is too big to fit within the field bits."));
        let result = BitfieldBuilder::new().checked_with_b(0x1000);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Value is too big to fit within the field bits."));
        let result = BitfieldBuilder::new()
            .checked_with_a(0xF)
            .and_then(|b| b.checked_with_b(0xFFF))
            .and_then(|b| b.checked_with_c(0xFF))
            .and_then(|b| b.checked_with_d(0xAB));
        assert!(result.is_ok());
        let bitfield = result.unwrap().build();
        assert_eq!(bitfield.a(), 0xF);
        assert_eq!(bitfield.b(), 0xFFF);
        assert_eq!(bitfield.c(), 0xFF);
        assert_eq!(bitfield.d(), 0xAB);
    }

    #[test]
    fn bitfield_builder_checked_setters_signed_overflow() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(4)]
            a: i8,
            #[bits(12)]
            b: i16,
            #[bits(16)]
            c: i16,
        }

        let result = BitfieldBuilder::new().checked_with_a(8);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Value is too big to fit within the field bits."));
        let result = BitfieldBuilder::new().checked_with_a(-9);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Value is too big to fit within the field bits."));
        let result = BitfieldBuilder::new().checked_with_b(2048);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Value is too big to fit within the field bits."));
        let result = BitfieldBuilder::new()
            .checked_with_a(7)
            .and_then(|b| b.checked_with_b(-2048))
            .and_then(|b| b.checked_with_c(-1));
        assert!(result.is_ok());
        let bitfield = result.unwrap().build();
        assert_eq!(bitfield.a(), 7);
        assert_eq!(bitfield.b(), -2048);
        assert_eq!(bitfield.c(), -1);
    }

    #[test]
    fn bitfield_builder_unchecked_setters_truncate() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(4)]
            a: u8,
            #[bits(4)]
            b: u8,
            #[bits(8)]
            c: u8,
        }

        let bitfield = BitfieldBuilder::new().with_a(0x1F).with_b(0x0).with_c(0x00).build();
        assert_eq!(bitfield.a(), 0xF);
    }

    #[test]
    fn bitfield_from_bits() {
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

        let bitfield = Bitfield::from_bits(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
    }

    #[test]
    fn bitfield_from_bits_with_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let bitfield = Bitfield::from_bits_with_defaults(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x78);
    }

    #[test]
    fn bitfield_from_bits_with_defaults_all_fields_have_defaults() {
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

        let bitfield = Bitfield::from_bits_with_defaults(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
    }

    #[test]
    fn bitfield_from_bits_booleans() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: bool,
            #[bits(default = false)]
            b: bool,
            c: bool,
            d: bool,
            #[bits(default = false)]
            e: bool,
            f: bool,
            #[bits(default = true)]
            g: bool,
            h: bool,
        }

        let bitfield = Bitfield::from_bits(0xFF);

        assert!(bitfield.a());
        assert!(bitfield.b());
        assert!(bitfield.c());
        assert!(bitfield.d());
        assert!(bitfield.e());
        assert!(bitfield.f());
        assert!(bitfield.g());
        assert!(bitfield.h());
    }

    #[test]
    fn bitfield_from_bits_with_defaults_booleans() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: bool,
            #[bits(default = false)]
            b: bool,
            c: bool,
            d: bool,
            #[bits(default = false)]
            e: bool,
            f: bool,
            #[bits(default = true)]
            g: bool,
            h: bool,
        }

        let bitfield = Bitfield::from_bits_with_defaults(0xFF);

        assert!(bitfield.a());
        assert!(!bitfield.b());
        assert!(bitfield.c());
        assert!(bitfield.d());
        assert!(!bitfield.e());
        assert!(bitfield.f());
        assert!(bitfield.g());
        assert!(bitfield.h());
    }

    #[test]
    fn bitfield_lsb() {
        #[bitfield(u32, order = Lsb)]
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

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78_56_34_12);
    }

    #[test]
    fn bitfield_msb() {
        #[bitfield(u32, order = Msb)]
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

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x12_34_56_78);
    }

    #[test]
    fn bitfield_lsb_by_default() {
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

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78_56_34_12);
    }

    #[test]
    fn bitfield_debug() {
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
        let debug_str = format!("{:?}", bitfield);

        assert_eq!(debug_str, "Bitfield { a: 18, b: 52, c: 86, d: 120 }");
    }

    #[test]
    fn bitfield_debug_msb() {
        #[bitfield(u32, order = Msb)]
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
        let debug_str = format!("{:?}", bitfield);

        assert_eq!(debug_str, "Bitfield { a: 18, b: 52, c: 86, d: 120 }");
    }

    #[test]
    fn bitfield_field_access_write_only_can_write() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12, access = wo)]
            a: u32,
        }

        Bitfield::new().set_a(0x34);
        BitfieldBuilder::new().with_a(0x34).build();
    }

    #[test]
    fn bitfield_field_access_read_only_can_read() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12, access = ro)]
            a: u32,
        }

        assert_eq!(Bitfield::new().a(), 0x12);
        assert_eq!(BitfieldBuilder::new().build().a(), 0x12)
    }

    #[test]
    fn bitfield_from_bits_field_access_read_only() {
        #[bitfield(u8)]
        struct Bitfield {
            #[bits(4)]
            rw: u8,
            #[bits(4, access = ro)]
            ro: u8,
        }

        let bar1 = Bitfield::from_bits(0xFF);
        assert_eq!(0xF, bar1.rw());
        assert_eq!(0xF, bar1.ro());

        let bar2 = BitfieldBuilder::new().with_ro(0xF).build();
        assert_eq!(0xF, bar2.ro());
    }

    #[test]
    fn bitfield_from_bits_sets_read_only_fields() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            rw_field: u8,
            #[bits(default = 0x34, access = ro)]
            ro_field: u8,
            no_default_field: u8,
            #[bits(default = 0x78)]
            another_field: u8,
        }

        let bitfield = Bitfield::from_bits(0x11_22_33_44);
        assert_eq!(bitfield.rw_field(), 0x44);
        assert_eq!(bitfield.ro_field(), 0x33);
        assert_eq!(bitfield.no_default_field(), 0x22);
        assert_eq!(bitfield.another_field(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_from_bits_sets_read_only_bool_fields() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(4)]
            a: u8,
            #[bits(access = ro)]
            ro_bool: bool,
            #[bits(3)]
            b: u8,
            #[bits(8)]
            c: u8,
        }

        let bitfield_true = Bitfield::from_bits(0xFFFF);
        assert!(bitfield_true.ro_bool());
        let bitfield_false = Bitfield::from_bits(0xFFEF);
        assert!(!bitfield_false.ro_bool());
    }

    #[test]
    fn bitfield_field_access_na() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12, access = na)]
            a: u32,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.into_bits(), 0x12);
    }

    #[test]
    fn bitfield_from_type() {
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

        let bitfield = Bitfield::from(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_into_type() {
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
        let val: u32 = bitfield.into();

        assert_eq!(val, 0x78_56_34_12);
    }

    #[test]
    fn bitfield_from_bits_little_endian() {
        #[bitfield(u32, from_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x44_33_22_11);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_from_bits_little_endian_msb_field_order() {
        #[bitfield(u32, from_endian = little, order = msb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x44_33_22_11);

        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_from_bits_big_endian() {
        #[bitfield(u32, from_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_from_bits_big_endian_msb_order() {
        #[bitfield(u32, from_endian = big, order = msb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x11_22_33_44);

        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_into_bits_little_endian() {
        #[bitfield(u32, into_endian = little)]
        pub struct Bitfield {
            #[bits(default = 0x44)]
            a: u8,
            #[bits(default = 0x33)]
            b: u8,
            #[bits(default = 0x22)]
            c: u8,
            #[bits(default = 0x11)]
            d: u8,
        }

        let bitfield = Bitfield::default();

        assert_eq!(bitfield.into_bits(), 0x44332211);
    }

    #[test]
    fn bitfield_into_bits_big_endian() {
        #[bitfield(u32, into_endian = big)]
        pub struct Bitfield {
            #[bits(default = 0x44)]
            a: u8,
            #[bits(default = 0x33)]
            b: u8,
            #[bits(default = 0x22)]
            c: u8,
            #[bits(default = 0x11)]
            d: u8,
        }

        let bitfield = Bitfield::default();

        assert_eq!(bitfield.into_bits(), 0x11223344);
    }

    #[test]
    fn bitfield_field_visibility() {
        #[bitfield(u32)]
        pub(crate) struct Bitfield {
            #[bits(default = 0x12)]
            pub a: u8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(default = 0x56)]
            pub c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.c(), 0x56);
    }

    #[test]
    fn bitfield_const_default_value() {
        const A_VAL: u8 = 0x12;

        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = A_VAL)]
            a: u8,
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.a(), 0x12);
    }

    #[test]
    fn bitfield_const_func_default_value() {
        const fn generate_val() -> u8 {
            0xFF
        }
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = generate_val())]
            a: u8,
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.a(), 0xFF);
    }

    #[test]
    fn bitfield_get_bit() {
        #[bitfield(u8, bit_ops = true)]
        pub struct Bitfield {
            #[bits(2, default = 0b11)]
            a: u8,
            #[bits(2, default = 0b00)]
            b: u8,
            #[bits(2, default = 0b10, access = wo)]
            c: u8,
            #[bits(2, default = 0b01)]
            _d: u8,
        }

        let bitfield = Bitfield::new();

        assert!(bitfield.get_bit(0));
        assert!(bitfield.get_bit(1));
        assert!(!bitfield.get_bit(2));
        assert!(!bitfield.get_bit(3));
        assert!(!bitfield.get_bit(4));
        assert!(!bitfield.get_bit(5));
        assert!(bitfield.get_bit(6));
        assert!(!bitfield.get_bit(7));
    }

    #[test]
    fn bitfield_get_bit_out_of_bounds() {
        #[bitfield(u8, bit_ops = true)]
        pub struct Bitfield {
            #[bits(2, default = 0b11)]
            a: u8,
            #[bits(2, default = 0b00)]
            b: u8,
            #[bits(2, default = 0b10, access = wo)]
            c: u8,
            #[bits(2, default = 0b01)]
            _d: u8,
        }

        let bitfield = Bitfield::new();

        assert!(!bitfield.get_bit(8));
        assert!(!bitfield.get_bit(50));
    }

    #[test]
    fn bitfield_checked_get_bit() {
        #[bitfield(u8, bit_ops = true)]
        pub struct Bitfield {
            #[bits(2, default = 0b11)]
            a: u8,
            #[bits(2, default = 0b00)]
            b: u8,
            #[bits(2, default = 0b10, access = wo)]
            c: u8,
            #[bits(2, default = 0b01)]
            _d: u8,
        }

        let bitfield = Bitfield::new();

        assert!(bitfield.checked_get_bit(0).unwrap());
        assert!(bitfield.checked_get_bit(1).unwrap());
        assert!(!bitfield.checked_get_bit(2).unwrap());
        assert!(!bitfield.checked_get_bit(3).unwrap());
        assert!(bitfield.checked_get_bit(4).is_err());
        assert!(bitfield.checked_get_bit(5).is_err());
        assert!(bitfield.get_bit(6));
        assert!(!bitfield.get_bit(7));
    }

    #[test]
    fn bitfield_checked_get_bit_none_access() {
        #[bitfield(u8, bit_ops = true)]
        pub struct Bitfield {
            #[bits(2, default = 0b11)]
            a: u8,
            #[bits(2, default = 0b00)]
            b: u8,
            #[bits(2, default = 0b10, access = na)]
            c: u8,
            #[bits(2, default = 0b01)]
            _d: u8,
        }

        let bitfield = Bitfield::new();

        assert!(bitfield.checked_get_bit(0).unwrap());
        assert!(bitfield.checked_get_bit(1).unwrap());
        assert!(!bitfield.checked_get_bit(2).unwrap());
        assert!(!bitfield.checked_get_bit(3).unwrap());
        assert!(bitfield.checked_get_bit(4).is_err());
        assert!(bitfield.checked_get_bit(5).is_err());
        assert!(bitfield.get_bit(6));
        assert!(!bitfield.get_bit(7));
    }

    #[test]
    fn bitfield_checked_get_bit_rw_access() {
        #[bitfield(u8, bit_ops = true)]
        pub struct Bitfield {
            #[bits(2, default = 0b11)]
            a: u8,
            #[bits(2, default = 0b00)]
            b: u8,
            #[bits(2, default = 0b10, access = rw)]
            c: u8,
            #[bits(2, default = 0b01)]
            _d: u8,
        }

        let bitfield = Bitfield::new();

        assert!(bitfield.checked_get_bit(0).unwrap());
        assert!(bitfield.checked_get_bit(1).unwrap());
        assert!(!bitfield.checked_get_bit(2).unwrap());
        assert!(!bitfield.checked_get_bit(3).unwrap());
        assert!(!bitfield.checked_get_bit(4).unwrap());
        assert!(bitfield.checked_get_bit(5).unwrap());
        assert!(bitfield.get_bit(6));
        assert!(!bitfield.get_bit(7));
    }

    #[test]
    fn bitfield_full_field_size_u32() {
        #[bitfield(u32)]
        pub struct Bitfield {
            data: u32,
        }

        let bitfield = Bitfield::from_bits(0xaabbccdd);
        assert_eq!(bitfield.data(), 0xaabbccdd);
    }

    #[test]
    fn bitfield_full_field_size_u64() {
        #[bitfield(u64)]
        pub struct Bitfield {
            data: u64,
        }

        let bitfield = Bitfield::from_bits(0x11223344aabbccdd);
        assert_eq!(bitfield.data(), 0x11223344aabbccdd);
    }

    #[test]
    fn bitfield_set_bit() {
        #[bitfield(u8, bit_ops = true)]
        pub struct Bitfield {
            #[bits(2)]
            a: u8,
            #[bits(2, default = 0b11)]
            b: u8,
            #[bits(2, default = 0b11, access = ro)]
            c: u8,
            #[bits(2, default = 0b00)]
            _d: u8,
        }

        let mut bitfield = Bitfield::new();

        bitfield.set_bit(0, true);
        bitfield.set_bit(1, true);
        bitfield.set_bit(2, false);
        bitfield.set_bit(3, false);
        bitfield.set_bit(4, false);
        bitfield.set_bit(5, false);
        bitfield.set_bit(6, true);
        bitfield.set_bit(7, true);
        assert_eq!(bitfield.into_bits(), 0b110011);
    }

    #[test]
    fn bitfield_checked_set_bit() {
        #[bitfield(u8, bit_ops = true)]
        pub struct Bitfield {
            #[bits(2)]
            a: u8,
            #[bits(2, default = 0b11)]
            b: u8,
            #[bits(2, default = 0b11, access = ro)]
            c: u8,
            #[bits(2, default = 0b00)]
            _d: u8,
        }

        let mut bitfield = Bitfield::new();

        assert!(bitfield.checked_set_bit(0, true).is_ok());
        assert!(bitfield.checked_set_bit(1, true).is_ok());
        assert!(bitfield.checked_set_bit(2, false).is_ok());
        assert!(bitfield.checked_set_bit(3, false).is_ok());
        assert!(bitfield.checked_set_bit(4, false).is_err());
        assert!(bitfield.checked_set_bit(5, false).is_err());
        assert!(bitfield.checked_set_bit(6, true).is_err());
        assert!(bitfield.checked_set_bit(7, true).is_err());
        assert_eq!(bitfield.into_bits(), 0b110011);
    }

    #[test]
    fn bitfield_ignored_field() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(2, ignore = true)]
            ignored: char,
            #[bits(99, ignore = true)]
            ignored1: Custom,
            #[bits(4, default = 0b11)]
            b: u8,
            #[bits(2, default = 0b11, access = ro)]
            c: u8,
            #[bits(2, default = 0b00)]
            _d: u8,
        }

        #[derive(Debug, Default, PartialEq, Copy, Clone)]
        enum Custom {
            #[default]
            A = 0,
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.ignored1, Custom::A);
    }

    #[test]
    fn bitfield_write_bits() {
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

        let mut bitfield = Bitfield::new();

        bitfield.write_bits(0x11223344);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11223344);
    }

    #[test]
    fn bitfield_write_bits_non_writable() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = ro)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(default = 0x78)]
            _d: u8,
        }

        let mut bitfield = Bitfield::new();

        bitfield.write_bits(0x11223344);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.into_bits(), 0x78223444);
    }

    #[test]
    fn bitfield_write_bits_with_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            b: u8,
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let mut bitfield = Bitfield::new();

        bitfield.write_bits_with_defaults(0x11223344);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78223312);
    }

    #[test]
    fn bitfield_clear_bits() {
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

        let mut bitfield = Bitfield::new();

        bitfield.clear_bits();

        assert_eq!(bitfield.a(), 0);
        assert_eq!(bitfield.b(), 0);
        assert_eq!(bitfield.c(), 0);
        assert_eq!(bitfield.d(), 0);
        assert_eq!(bitfield.into_bits(), 0);
    }

    #[test]
    fn bitfield_clear_bits_with_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            b: u8,
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let mut bitfield = Bitfield::new();

        bitfield.clear_bits_with_defaults();

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0);
        assert_eq!(bitfield.c(), 0);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78000012);
    }

    #[test]
    fn bitfield_field_access() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(access = ro)]
            a: u8,
            #[bits(access = wo)]
            b: u8,
            #[bits(access = rw)]
            c: u8,
            #[bits(access = na)]
            d: u8,
        }
    }

    #[test]
    fn bitfield_field_access_read_only_can_build() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12, access = ro)]
            a: u32,
        }

        assert_eq!(Bitfield::new().a(), 0x12);
        assert_eq!(BitfieldBuilder::new().with_a(0x22).build().a(), 0x22);
    }

    #[test]
    fn bitfield_bool_fields() {
        #[bitfield(u8)]
        pub struct PxiCnt {
            #[bits(access = ro)]
            a: bool,
            b: bool,
            c: bool,
            d: bool,
            #[bits(4)]
            __: u32,
        }
    }

    #[test]
    fn bitfield_neg_inverts_bits() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(5, default = 0xC)]
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(9, default = 0x78)]
            d: u16,
            #[bits(default = true)]
            e: bool,
            #[bits(default = false)]
            f: bool,
        }

        let builder = Bitfield::new();

        assert_eq!(builder.a(), 0xC);
        assert_eq!(builder.b(), 0x34);
        assert_eq!(builder.c(), 0x56);
        assert_eq!(builder.d(), 0x78);
        assert!(builder.e());
        assert_eq!(builder.a_inverted(), 0x13);
        assert_eq!(builder.b_inverted(), 0xCB);
        assert_eq!(builder.c_inverted(), 0xA9);
        assert_eq!(builder.d_inverted(), 0x187);
        assert!(!builder.e_inverted());
        assert!(builder.f_inverted());
    }

    #[test]
    fn bitfield_neg_inverts_bits_custom_type() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(4, default = CustomType::A)]
            a: CustomType,
            #[bits(4, default = CustomType::B)]
            b: CustomType,
        }

        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(u8)]
        pub enum CustomType {
            A = 0x3,
            B = 0x4,
            C(u8),
        }

        impl CustomType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0x3 => Self::A,
                    0x4 => Self::B,
                    _ => Self::C(bits),
                }
            }

            const fn into_bits(self) -> u8 {
                match self {
                    Self::A => 0x3,
                    Self::B => 0x4,
                    CustomType::C(bits) => bits,
                }
            }
        }

        let builder = Bitfield::new();

        assert_eq!(builder.a(), CustomType::A);
        assert_eq!(builder.b(), CustomType::B);
        assert_eq!(builder.a_inverted(), CustomType::C(0xC));
        assert_eq!(builder.b_inverted(), CustomType::C(0xB));
    }

    #[test]
    fn bitfield_custom_type_setter() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(4)]
            a: CustomType,
            #[bits(4)]
            b: CustomType,
        }

        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(u8)]
        pub enum CustomType {
            A = 0x3,
            B = 0x4,
            C(u8),
        }

        impl CustomType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0x3 => Self::A,
                    0x4 => Self::B,
                    _ => Self::C(bits),
                }
            }

            const fn into_bits(self) -> u8 {
                match self {
                    Self::A => 0x3,
                    Self::B => 0x4,
                    CustomType::C(bits) => bits,
                }
            }
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_a(CustomType::C(0x9));
        bitfield.set_b(CustomType::C(0x5));

        assert_eq!(bitfield.a(), CustomType::C(0x9));
        assert_eq!(bitfield.b(), CustomType::C(0x5));
    }

    #[test]
    fn bitfield_custom_type_checked_setter() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(4)]
            a: CustomType,
            #[bits(4)]
            b: CustomType,
        }

        #[derive(Clone, Copy, Debug, PartialEq)]
        #[repr(u8)]
        pub enum CustomType {
            A = 0x3,
            B = 0x4,
            C(u8),
        }

        impl CustomType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0x3 => Self::A,
                    0x4 => Self::B,
                    _ => Self::C(bits),
                }
            }

            const fn into_bits(self) -> u8 {
                match self {
                    Self::A => 0x3,
                    Self::B => 0x4,
                    CustomType::C(bits) => bits,
                }
            }
        }

        let mut bitfield = Bitfield::new();
        let set_a_results = bitfield.checked_set_a(CustomType::C(0x9));
        let set_b_results = bitfield.checked_set_b(CustomType::C(0x5));

        assert!(set_a_results.is_ok());
        assert!(set_b_results.is_ok());
        assert_eq!(bitfield.a(), CustomType::C(0x9));
        assert_eq!(bitfield.b(), CustomType::C(0x5));
    }

    #[test]
    fn bitfield_read_only_custom_field() {
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Colour {
            White,
            Black,
        }

        impl Colour {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0 => Self::White,
                    1 => Self::Black,
                    _ => unreachable!(),
                }
            }

            const fn into_bits(self) -> u8 {
                self as u8
            }
        }

        #[derive(Clone, Copy, Debug, PartialEq)]
        enum PieceType {
            King,
            Queen,
            Rook,
            Bishop,
            Knight,
            Pawn,
        }

        impl PieceType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0 => Self::King,
                    1 => Self::Queen,
                    2 => Self::Rook,
                    3 => Self::Bishop,
                    4 => Self::Knight,
                    5 => Self::Pawn,
                    _ => unreachable!(),
                }
            }

            const fn into_bits(self) -> u8 {
                self as u8
            }
        }

        #[bitfield(u8)]
        struct Piece {
            #[bits(3, access = ro)]
            piece: PieceType,
            #[bits(1, access = ro)]
            colour: Colour,
            #[bits(4, default = 0)]
            _reserved: u8,
        }

        let val =
            PieceBuilder::new().with_piece(PieceType::King).with_colour(Colour::White).build();

        assert_eq!(val.into_bits(), 0b00000000);
        assert_eq!(val.colour(), Colour::White);
    }

    #[test]
    fn bitfield_builder_checked_with() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(2, access = ro)]
            a: u8,
            #[bits(30)]
            b: u32,
        }

        let result = BitfieldBuilder::new().checked_with_a(0x11);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Value is too big to fit within the field bits.");
    }

    #[test]
    fn bitfield_sign_extended_bit() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(1)]
            a: i8,
            #[bits(7)]
            _reserved: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_a(1);

        assert_eq!(bitfield.a(), -1);
    }

    #[test]
    fn bitfield_suffix() {
        #[bitfield(u8)]
        pub struct BitfieldI8 {
            #[bits(8, default = -1)]
            a: i8,
        }

        #[bitfield(u16)]
        pub struct BitfieldI16 {
            #[bits(16, default = -1)]
            a: i16,
        }

        #[bitfield(u32)]
        pub struct BitfieldI32 {
            #[bits(32, default = -1)]
            a: i32,
        }

        #[bitfield(u64)]
        pub struct BitfieldI64 {
            #[bits(64, default = -1)]
            a: i64,
        }

        #[bitfield(u128)]
        pub struct BitfieldI128 {
            #[bits(128, default = -1)]
            a: i128,
        }

        assert_eq!(BitfieldI8::new().a(), -1i8);
        assert_eq!(BitfieldI16::new().a(), -1i16);
        assert_eq!(BitfieldI32::new().a(), -1i32);
        assert_eq!(BitfieldI64::new().a(), -1i64);
        assert_eq!(BitfieldI128::new().a(), -1i128);
    }

    #[test]
    fn bitfield_write_bits_arg_with_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            b: u8,
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let mut bitfield = Bitfield::new();

        bitfield.write_bits_with_defaults(0x11223344);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78223312);
    }

    #[test]
    fn bitfield_clear_bits_arg() {
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

        let mut bitfield = Bitfield::new();
        bitfield.clear_bits();

        assert_eq!(bitfield.a(), 0);
        assert_eq!(bitfield.b(), 0);
        assert_eq!(bitfield.c(), 0);
        assert_eq!(bitfield.d(), 0);
        assert_eq!(bitfield.into_bits(), 0);
    }

    #[test]
    fn bitfield_write_bits_with_defaults_all_defaults_ignored_fields() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 1)]
            a: u8,
            #[bits(default = 2)]
            b: u8,
            #[bits(default = 3)]
            c: u8,
            #[bits(default = 4)]
            d: u8,
            #[bits(ignore = true)]
            cache: u32,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_bits_with_defaults(0x11223344);

        assert_eq!(bitfield.a(), 1);
        assert_eq!(bitfield.b(), 2);
        assert_eq!(bitfield.c(), 3);
        assert_eq!(bitfield.d(), 4);
        assert_eq!(bitfield.into_bits(), 0x4030201);
    }

    #[test]
    fn bitfield_write_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            b: u8,
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_bits(0x11_22_33_44);
        bitfield.write_defaults();

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78_22_33_12);
    }

    #[test]
    fn bitfield_write_defaults_all_fields_have_defaults() {
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

        let mut bitfield = Bitfield::new();
        bitfield.write_bits(0x11_22_33_44);
        bitfield.write_defaults();

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78_56_34_12);
    }

    #[test]
    fn bitfield_write_defaults_no_fields_have_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_bits(0x11_22_33_44);
        bitfield.write_defaults();

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_write_defaults_booleans() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: bool,
            #[bits(default = false)]
            b: bool,
            c: bool,
            d: bool,
            #[bits(default = false)]
            e: bool,
            f: bool,
            #[bits(default = true)]
            g: bool,
            h: bool,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_bits(0xFF);
        bitfield.write_defaults();

        assert!(bitfield.a());
        assert!(!bitfield.b());
        assert!(bitfield.c());
        assert!(bitfield.d());
        assert!(!bitfield.e());
        assert!(bitfield.f());
        assert!(bitfield.g());
        assert!(bitfield.h());
    }

    #[test]
    fn bitfield_write_le_bits() {
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

        let mut bitfield = Bitfield::new();
        bitfield.write_le_bits(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.into_bits(), 0x44_33_22_11);
    }

    #[test]
    fn bitfield_write_le_bits_with_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            b: u8,
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_le_bits_with_defaults(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78_33_22_12);
    }

    #[test]
    fn bitfield_write_le_bits_with_defaults_all_fields_have_defaults() {
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

        let mut bitfield = Bitfield::new();
        bitfield.write_le_bits_with_defaults(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
    }

    #[test]
    fn bitfield_write_le_bits_booleans() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: bool,
            #[bits(default = false)]
            b: bool,
            c: bool,
            d: bool,
            #[bits(default = false)]
            e: bool,
            f: bool,
            #[bits(default = true)]
            g: bool,
            h: bool,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_le_bits(0xFF);

        assert!(bitfield.a());
        assert!(bitfield.b());
        assert!(bitfield.c());
        assert!(bitfield.d());
        assert!(bitfield.e());
        assert!(bitfield.f());
        assert!(bitfield.g());
        assert!(bitfield.h());
    }

    #[test]
    fn bitfield_write_le_bits_with_defaults_booleans() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: bool,
            #[bits(default = false)]
            b: bool,
            c: bool,
            d: bool,
            #[bits(default = false)]
            e: bool,
            f: bool,
            #[bits(default = true)]
            g: bool,
            h: bool,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_le_bits_with_defaults(0xFF);

        assert!(bitfield.a());
        assert!(!bitfield.b());
        assert!(bitfield.c());
        assert!(bitfield.d());
        assert!(!bitfield.e());
        assert!(bitfield.f());
        assert!(bitfield.g());
        assert!(bitfield.h());
    }

    #[test]
    fn bitfield_write_le_bits_field_access_read_only() {
        #[bitfield(u8)]
        struct Bitfield {
            #[bits(4)]
            rw: u8,
            #[bits(4, access = ro)]
            ro: u8,
        }

        let mut bitfield = BitfieldBuilder::new().with_ro(0xA).build();
        bitfield.write_le_bits(0xFF);

        assert_eq!(0xF, bitfield.rw());
        assert_eq!(0xA, bitfield.ro());
    }

    #[test]
    fn bitfield_write_le_bits_does_not_set_read_only_fields() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            rw_field: u8,
            #[bits(default = 0x34, access = ro)]
            ro_field: u8,
            no_default_field: u8,
            #[bits(default = 0x78)]
            another_field: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_le_bits(0x44_33_22_11);

        assert_eq!(bitfield.rw_field(), 0x44);
        assert_eq!(bitfield.ro_field(), 0x34);
        assert_eq!(bitfield.no_default_field(), 0x22);
        assert_eq!(bitfield.another_field(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_34_44);
    }

    #[test]
    fn bitfield_write_le_bits_does_not_set_read_only_bool_fields() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(4)]
            a: u8,
            #[bits(access = ro)]
            ro_bool: bool,
            #[bits(3)]
            b: u8,
            #[bits(8)]
            c: u8,
        }

        let mut bitfield_with_ro_true = BitfieldBuilder::new().with_ro_bool(true).build();
        bitfield_with_ro_true.write_le_bits(0x0000);
        assert!(bitfield_with_ro_true.ro_bool());

        let mut bitfield_with_ro_false = Bitfield::new();
        bitfield_with_ro_false.write_le_bits(0xFFFF);
        assert!(!bitfield_with_ro_false.ro_bool());
    }

    #[test]
    fn bitfield_write_be_bits() {
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

        let mut bitfield = Bitfield::new();
        bitfield.write_be_bits(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_write_be_bits_with_defaults() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            b: u8,
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_be_bits_with_defaults(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78_22_33_12);
    }

    #[test]
    fn bitfield_write_be_bits_with_defaults_all_fields_have_defaults() {
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

        let mut bitfield = Bitfield::new();
        bitfield.write_be_bits_with_defaults(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
    }

    #[test]
    fn bitfield_write_be_bits_booleans() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: bool,
            #[bits(default = false)]
            b: bool,
            c: bool,
            d: bool,
            #[bits(default = false)]
            e: bool,
            f: bool,
            #[bits(default = true)]
            g: bool,
            h: bool,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_be_bits(0xFF);

        assert!(bitfield.a());
        assert!(bitfield.b());
        assert!(bitfield.c());
        assert!(bitfield.d());
        assert!(bitfield.e());
        assert!(bitfield.f());
        assert!(bitfield.g());
        assert!(bitfield.h());
    }

    #[test]
    fn bitfield_write_be_bits_with_defaults_booleans() {
        #[bitfield(u8)]
        pub struct Bitfield {
            a: bool,
            #[bits(default = false)]
            b: bool,
            c: bool,
            d: bool,
            #[bits(default = false)]
            e: bool,
            f: bool,
            #[bits(default = true)]
            g: bool,
            h: bool,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_be_bits_with_defaults(0xFF);

        assert!(bitfield.a());
        assert!(!bitfield.b());
        assert!(bitfield.c());
        assert!(bitfield.d());
        assert!(!bitfield.e());
        assert!(bitfield.f());
        assert!(bitfield.g());
        assert!(bitfield.h());
    }

    #[test]
    fn bitfield_write_be_bits_field_access_read_only() {
        #[bitfield(u8)]
        struct Bitfield {
            #[bits(4)]
            rw: u8,
            #[bits(4, access = ro)]
            ro: u8,
        }

        let mut bitfield = BitfieldBuilder::new().with_ro(0xA).build();
        bitfield.write_be_bits(0xFF);

        assert_eq!(0xF, bitfield.rw());
        assert_eq!(0xA, bitfield.ro());
    }

    #[test]
    fn bitfield_write_be_bits_does_not_set_read_only_fields() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            rw_field: u8,
            #[bits(default = 0x34, access = ro)]
            ro_field: u8,
            no_default_field: u8,
            #[bits(default = 0x78)]
            another_field: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_be_bits(0x11_22_33_44);

        assert_eq!(bitfield.rw_field(), 0x44);
        assert_eq!(bitfield.ro_field(), 0x34);
        assert_eq!(bitfield.no_default_field(), 0x22);
        assert_eq!(bitfield.another_field(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_34_44);
    }

    #[test]
    fn bitfield_write_be_bits_does_not_set_read_only_bool_fields() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(4)]
            a: u8,
            #[bits(access = ro)]
            ro_bool: bool,
            #[bits(3)]
            b: u8,
            #[bits(8)]
            c: u8,
        }

        let mut bitfield_with_ro_true = BitfieldBuilder::new().with_ro_bool(true).build();
        bitfield_with_ro_true.write_be_bits(0x0000);
        assert!(bitfield_with_ro_true.ro_bool());

        let mut bitfield_with_ro_false = Bitfield::new();
        bitfield_with_ro_false.write_be_bits(0xFFFF);
        assert!(!bitfield_with_ro_false.ro_bool());
    }

    #[test]
    fn bitfield_custom_field_no_write_access() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(8, default = CustomType::A, access = ro)]
            nested_field: CustomType,
        }

        pub enum CustomType {
            A,
            B,
        }

        impl CustomType {
            const fn from_bits(bits: u8) -> Self {
                match bits {
                    0x3 => Self::A,
                    0x4 => Self::B,
                    _ => Self::A,
                }
            }

            const fn into_bits(self) -> u8 {
                match self {
                    Self::A => 0x3,
                    Self::B => 0x4,
                }
            }
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.into_bits(), 0x312)
    }

    #[test]
    fn bitfield_from_traits_msb_from_little_into_little() {
        #[bitfield(u32, order = msb, from_endian = little, into_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x78563412);

        let bits: u32 = bitfield.into();
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bits, 0x78563412);
    }

    #[test]
    fn bitfield_from_traits_lsb_from_little_into_little() {
        #[bitfield(u32, order = lsb, from_endian = little, into_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x78563412);

        let bits: u32 = bitfield.into();
        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bits, 0x78563412);
    }

    #[test]
    fn bitfield_from_traits_msb_from_big_into_little() {
        #[bitfield(u32, order = msb, from_endian = big, into_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x78563412);

        let bits: u32 = bitfield.into();
        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bits, 0x12345678);
    }

    #[test]
    fn bitfield_from_traits_lsb_from_big_into_little() {
        #[bitfield(u32, order = lsb, from_endian = big, into_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x78563412);

        let bits: u32 = bitfield.into();
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bits, 0x12345678);
    }

    #[test]
    fn bitfield_from_traits_msb_from_little_into_big() {
        #[bitfield(u32, order = msb, from_endian = little, into_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x78563412);

        let bits: u32 = bitfield.into();
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bits, 0x12345678);
    }

    #[test]
    fn bitfield_from_traits_lsb_from_little_into_big() {
        #[bitfield(u32, order = lsb, from_endian = little, into_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x78563412);

        let bits: u32 = bitfield.into();
        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bits, 0x12345678);
    }

    #[test]
    fn bitfield_from_traits_msb_from_big_into_big() {
        #[bitfield(u32, order = msb, from_endian = big, into_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x78563412);

        let bits: u32 = bitfield.into();
        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bits, 0x78563412);
    }

    #[test]
    fn bitfield_from_traits_lsb_from_big_into_big() {
        #[bitfield(u32, order = lsb, from_endian = big, into_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x78563412);

        let bits: u32 = bitfield.into();
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bits, 0x78563412);
    }

    #[test]
    fn bitfield_from_into_bits_msb_from_little_into_little() {
        #[bitfield(u32, order = msb, from_endian = little, into_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_from_into_bits_lsb_from_little_into_little() {
        #[bitfield(u32, order = lsb, from_endian = little, into_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bitfield.into_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_from_into_bits_msb_from_big_into_little() {
        #[bitfield(u32, order = msb, from_endian = big, into_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bitfield.into_bits(), 0x12345678);
    }

    #[test]
    fn bitfield_from_into_bits_lsb_from_big_into_little() {
        #[bitfield(u32, order = lsb, from_endian = big, into_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x12345678);
    }

    #[test]
    fn bitfield_from_into_bits_msb_from_little_into_big() {
        #[bitfield(u32, order = msb, from_endian = little, into_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x12345678);
    }

    #[test]
    fn bitfield_from_into_bits_lsb_from_little_into_big() {
        #[bitfield(u32, order = lsb, from_endian = little, into_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bitfield.into_bits(), 0x12345678);
    }

    #[test]
    fn bitfield_from_into_bits_msb_from_big_into_big() {
        #[bitfield(u32, order = msb, from_endian = big, into_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bitfield.into_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_from_into_bits_lsb_from_big_into_big() {
        #[bitfield(u32, order = lsb, from_endian = big, into_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_msb_from_le_bits_into_le_bits() {
        #[bitfield(u32, order = msb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_le_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_le_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_lsb_from_le_bits_into_le_bits() {
        #[bitfield(u32, order = lsb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_le_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bitfield.into_le_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_msb_from_be_bits_into_le_bits() {
        #[bitfield(u32, order = msb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_be_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bitfield.into_le_bits(), 0x12345678);
    }

    #[test]
    fn bitfield_lsb_from_be_bits_into_le_bits() {
        #[bitfield(u32, order = lsb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_be_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_le_bits(), 0x12345678);
    }

    #[test]
    fn bitfield_msb_from_le_bits_into_be_bits() {
        #[bitfield(u32, order = msb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_le_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_be_bits(), 0x12345678);
    }

    #[test]
    fn bitfield_lsb_from_le_bits_into_be_bits() {
        #[bitfield(u32, order = lsb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_le_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bitfield.into_be_bits(), 0x12345678);
    }

    #[test]
    fn bitfield_msb_from_be_bits_into_be_bits() {
        #[bitfield(u32, order = msb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_be_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x78);
        assert_eq!(bitfield.b(), 0x56);
        assert_eq!(bitfield.c(), 0x34);
        assert_eq!(bitfield.d(), 0x12);
        assert_eq!(bitfield.into_be_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_signed_checked_setters() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: i8,
            #[bits(12)]
            b: i16,
            #[bits(4)]
            _reserved: u8,
            #[bits(8)]
            _reserved2: u8,
        }

        let mut bf = Bitfield::new_without_defaults();

        assert!(bf.checked_set_a(-1i8).is_ok());
        assert!(bf.checked_set_a(-128i8).is_ok());
        assert!(bf.checked_set_a(127i8).is_ok());

        assert!(bf.checked_set_b(0i16).is_ok());
        assert!(bf.checked_set_b(-1i16).is_ok());
        assert!(bf.checked_set_b(2047i16).is_ok());
        assert!(bf.checked_set_b(-2048i16).is_ok());
        assert!(bf.checked_set_b(2048i16).is_err());
        assert!(bf.checked_set_b(-2049i16).is_err());
    }

    #[test]
    fn bitfield_lsb_from_be_bits_into_be_bits() {
        #[bitfield(u32, order = lsb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_be_bits(0x78563412);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_be_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_custom_field_ignored_field() {
        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            b: u8,
            #[bits(8, default = CustomType::A, ignore = true)]
            nested_field: CustomType,
        }

        #[derive(Copy, Clone, Default)]
        pub enum CustomType {
            #[default]
            A,
        }
    }

    #[test]
    fn bitfield_user_attributes() {
        #[bitfield(u8)]
        #[derive(PartialEq)]
        pub struct Bitfield {
            a: u8,
        }
    }

    #[test]
    fn bitfield_reserved_no_bits() {
        #[bitfield(u16)]
        pub struct Bitfield {
            a: u8,
            _reserved: u8,
        }
    }

    #[test]
    fn bitfield_reserved_bits() {
        #[bitfield(u16)]
        pub struct Bitfield {
            a: u8,
            #[bits(8)]
            _reserved: u8,
        }
    }

    #[test]
    fn bitfield_reserved_default_value() {
        #[bitfield(u16)]
        pub struct Bitfield {
            a: u8,
            #[bits(8)]
            _reserved: u8,
        }
    }

    #[test]
    fn bitfield_bits_leading_comma() {
        #[bitfield(u16)]
        pub struct Bitfield {
            a: u8,
            #[bits(,8)]
            _reserved: u8,
        }
    }

    #[test]
    fn bitfield_set_bits_range_non_writable() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = ro)]
            b: u8,
            #[bits(default = 0x56, access = rw)]
            c: u8,
            #[bits(default = 0x78)]
            _d: u8,
        }

        let mut bitfield = Bitfield::new();

        bitfield.set_bits_range(0, 32, 0xFFFFFFFF);

        assert_eq!(bitfield.a(), 0xFF);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0xFF);
        assert_eq!(bitfield.into_bits(), 0x78FF34FF);
    }

    #[test]
    fn bitfield_get_bits_range_non_readable() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = wo)]
            b: u8,
            #[bits(default = 0x56, access = rw)]
            c: u8,
            #[bits(default = 0x78)]
            _d: u8,
        }

        let bitfield = Bitfield::new();

        bitfield.get_bits_range(0, 32);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.into_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_invert_non_writable() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = ro)]
            b: u8,
            #[bits(default = 0x56, access = rw)]
            c: u8,
            #[bits(default = 0x78)]
            _d: u8,
        }

        let mut bitfield = Bitfield::new();

        bitfield.invert_bits();

        assert_eq!(bitfield.a(), 0xED);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0xA9);
        assert_eq!(bitfield.into_bits(), 0x78A934ED);
    }

    #[test]
    fn bitfield_field_constants() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(7, default = 0x34)]
            b: u8,
            #[bits(5, default = 0x1F)]
            c: u8,
            #[bits(12, default = 0x78)]
            d: u16,
        }

        assert_eq!(Bitfield::A_BITS, 8);
        assert_eq!(Bitfield::A_OFFSET, 0);
        assert_eq!(Bitfield::B_BITS, 7);
        assert_eq!(Bitfield::B_OFFSET, 8);
        assert_eq!(Bitfield::C_BITS, 5);
        assert_eq!(Bitfield::C_OFFSET, 15);
        assert_eq!(Bitfield::D_BITS, 12);
        assert_eq!(Bitfield::D_OFFSET, 20);
    }

    #[test]
    fn bitfield_field_constants_msb() {
        #[bitfield(u32, order = msb)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        assert_eq!(Bitfield::A_BITS, 8);
        assert_eq!(Bitfield::A_OFFSET, 24);
        assert_eq!(Bitfield::B_BITS, 8);
        assert_eq!(Bitfield::B_OFFSET, 16);
        assert_eq!(Bitfield::C_BITS, 8);
        assert_eq!(Bitfield::C_OFFSET, 8);
        assert_eq!(Bitfield::D_BITS, 8);
        assert_eq!(Bitfield::D_OFFSET, 0);
    }

    #[test]
    fn bitfield_clear_field() {
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

        let mut bitfield = Bitfield::new();
        bitfield.clear_a();
        assert_eq!(bitfield.a(), 0);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78563400);
    }

    #[test]
    fn bitfield_clear_field_to_default() {
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

        let mut bitfield = Bitfield::new();
        bitfield.set_a(0xFF);
        bitfield.set_b(0xFF);
        bitfield.clear_a_to_default();
        bitfield.clear_b_to_default();

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78563412);
    }

    #[test]
    fn bitfield_clear_field_multiple() {
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

        let mut bitfield = Bitfield::new();
        bitfield.clear_a();
        bitfield.clear_b();
        bitfield.clear_c();
        bitfield.clear_d();
        assert_eq!(bitfield.into_bits(), 0);
    }

    #[test]
    fn bitfield_clear_field_write_only_not_generated_for_read_only() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12, access = ro)]
            a: u8,
            #[bits(default = 0x34, access = wo)]
            b: u8,
            #[bits(default = 0x56, access = rw)]
            c: u8,
            #[bits(default = 0x78)]
            _d: u8,
        }

        let mut bitfield = Bitfield::new();

        bitfield.clear_b();
        bitfield.clear_c();

        assert_eq!(bitfield.into_bits() & 0x0000_FF00, 0);
        assert_eq!(bitfield.c(), 0);

        assert_eq!(bitfield.a(), 0x12);
    }

    #[test]
    fn bitfield_invert_field() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(5, default = 0xC)]
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(9, default = 0x78)]
            d: u16,
            #[bits(default = true)]
            e: bool,
            #[bits(default = false)]
            f: bool,
        }

        let mut bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0xC);
        bitfield.invert_a();
        assert_eq!(bitfield.a(), 0x13);
        bitfield.invert_b();
        assert_eq!(bitfield.b(), 0xCB);
        bitfield.invert_c();
        assert_eq!(bitfield.c(), 0xA9);
        bitfield.invert_d();
        assert_eq!(bitfield.d(), 0x187);
        assert!(bitfield.e());
        bitfield.invert_e();
        assert!(!bitfield.e());
        assert!(!bitfield.f());
        bitfield.invert_f();
        assert!(bitfield.f());
    }

    #[test]
    fn bitfield_invert_field_bool() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(default = true)]
            a: bool,
            #[bits(default = false)]
            b: bool,
            #[bits(6)]
            _reserved: u8,
        }

        let mut bitfield = Bitfield::new();
        assert!(bitfield.a());
        assert!(!bitfield.b());
        bitfield.invert_a();
        bitfield.invert_b();
        assert!(!bitfield.a());
        assert!(bitfield.b());
    }

    #[test]
    fn bitfield_checked_set_bits_range() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let mut bitfield = Bitfield::new();
        assert!(bitfield.checked_set_bits_range(0, 8, 0xAB).is_ok());
        assert_eq!(bitfield.a(), 0xAB);
        assert!(bitfield.checked_set_bits_range(8, 8, 0xCD).is_ok());
        assert_eq!(bitfield.b(), 0xCD);
        assert!(bitfield.checked_set_bits_range(0, 40, 0xFF).is_err());
        assert!(bitfield.checked_set_bits_range(50, 8, 0xFF).is_err());
    }

    #[test]
    fn bitfield_checked_get_bits_range() {
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
        assert_eq!(bitfield.checked_get_bits_range(0, 8).unwrap(), 0x12);
        assert_eq!(bitfield.checked_get_bits_range(8, 8).unwrap(), 0x34);
        assert_eq!(bitfield.checked_get_bits_range(16, 8).unwrap(), 0x56);
        assert_eq!(bitfield.checked_get_bits_range(24, 8).unwrap(), 0x78);
        assert!(bitfield.checked_get_bits_range(0, 40).is_err());
        assert!(bitfield.checked_get_bits_range(50, 8).is_err());
    }

    #[test]
    fn bitfield_checked_set_bits_range_read_only() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = ro)]
            b: u8,
            #[bits(default = 0x56, access = rw)]
            c: u8,
            #[bits(default = 0x78)]
            _d: u8,
        }

        let mut bitfield = Bitfield::new();

        assert!(bitfield.checked_set_bits_range(0, 8, 0xAB).is_ok());
        assert_eq!(bitfield.a(), 0xAB);

        assert!(bitfield.checked_set_bits_range(8, 8, 0xFF).is_err());

        assert_eq!(bitfield.b(), 0x34);

        assert!(bitfield.checked_set_bits_range(0, 16, 0xFFFF).is_err());
        assert_eq!(bitfield.b(), 0x34);

        assert!(bitfield.checked_set_bits_range(16, 8, 0xEF).is_ok());
        assert_eq!(bitfield.c(), 0xEF);
    }

    #[test]
    fn bitfield_checked_get_bits_range_write_only() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = wo)]
            b: u8,
            #[bits(default = 0x56, access = rw)]
            c: u8,
            #[bits(default = 0x78)]
            _d: u8,
        }

        let bitfield = Bitfield::new();

        assert_eq!(bitfield.checked_get_bits_range(0, 8).unwrap(), 0x12);

        assert!(bitfield.checked_get_bits_range(8, 8).is_err());

        assert!(bitfield.checked_get_bits_range(0, 16).is_err());

        assert_eq!(bitfield.checked_get_bits_range(16, 8).unwrap(), 0x56);
    }

    #[test]
    fn bitfield_set_bits_range_basic() {
        #[bitfield(u32)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_bits_range(0, 8, 0xAB);
        bitfield.set_bits_range(8, 8, 0xCD);
        bitfield.set_bits_range(16, 8, 0xEF);

        assert_eq!(bitfield.a(), 0xAB);
        assert_eq!(bitfield.b(), 0xCD);
        assert_eq!(bitfield.c(), 0xEF);
        assert_eq!(bitfield.into_bits(), 0x00EFCDAB);
    }

    #[test]
    fn bitfield_get_bits_range_basic() {
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
        assert_eq!(bitfield.get_bits_range(0, 8), 0x12);
        assert_eq!(bitfield.get_bits_range(8, 8), 0x34);
        assert_eq!(bitfield.get_bits_range(16, 8), 0x56);
        assert_eq!(bitfield.get_bits_range(24, 8), 0x78);
    }

    #[test]
    fn bitfield_get_bits_range_out_of_bounds() {
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
        assert_eq!(bitfield.get_bits_range(50, 8), 0);
        assert_eq!(bitfield.get_bits_range(0, 40), 0);
    }

    #[test]
    fn bitfield_write_bits_write_endian_little() {
        #[bitfield(u32, write_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_bits(0x11_22_33_44);
        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.into_bits(), 0x44_33_22_11);
    }

    #[test]
    fn bitfield_write_bits_write_endian_big() {
        #[bitfield(u32, write_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.write_bits(0x11_22_33_44);
        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_builder_default() {
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

        let bitfield = BitfieldBuilder::default().build();
        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
        assert_eq!(bitfield.into_bits(), 0x78_56_34_12);
    }

    #[test]
    fn bitfield_builder_checked_with_success() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(4, access = ro)]
            a: u8,
            #[bits(28)]
            b: u32,
        }

        let result = BitfieldBuilder::new().checked_with_a(0xF);
        assert!(result.is_ok());
        let bitfield = result.unwrap().build();
        assert_eq!(bitfield.a(), 0xF);
    }

    #[test]
    fn bitfield_builder_checked_with_failure() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(4, access = ro)]
            a: u8,
            #[bits(28)]
            b: u32,
        }

        let result = BitfieldBuilder::new().checked_with_a(0xFF);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Value is too big to fit within the field bits.");
    }

    #[test]
    fn bitfield_debug_write_only_field() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = wo)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let bitfield = Bitfield::new();
        let debug_str = format!("{:?}", bitfield);
        assert_eq!(debug_str, "Bitfield { a: 18, b: 52, c: 86, d: 120 }");
    }

    #[test]
    fn bitfield_debug_na_field() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = na)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let bitfield = Bitfield::new();
        let debug_str = format!("{:?}", bitfield);
        assert_eq!(debug_str, "Bitfield { a: 18, b: 52, c: 86, d: 120 }");
    }

    #[test]
    fn bitfield_from_bits_with_defaults_msb() {
        #[bitfield(u32, order = msb)]
        pub struct Bitfield {
            a: u8,
            #[bits(default = 0x34)]
            b: u8,
            c: u8,
            #[bits(default = 0x78)]
            d: u8,
        }

        let bitfield = Bitfield::from_bits_with_defaults(0x11_22_33_44);
        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.d(), 0x78);
    }

    #[test]
    fn bitfield_invert_bits_na_access() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = na)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(default = 0x78)]
            _d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.invert_bits();
        assert_eq!(bitfield.a(), 0xED);
        assert_eq!(bitfield.c(), 0xA9);
        assert_eq!(bitfield.into_bits(), 0x78A934ED);
    }

    #[test]
    fn bitfield_invert_bits_write_only_field() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12)]
            a: u8,
            #[bits(default = 0x34, access = wo)]
            b: u8,
            #[bits(default = 0x56)]
            c: u8,
            #[bits(default = 0x78)]
            _d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.invert_bits();
        assert_eq!(bitfield.a(), 0xED);
        assert_eq!(bitfield.c(), 0xA9);
        assert_eq!(bitfield.into_bits(), 0x78A9CBED);
    }

    #[test]
    fn from_trait_data_type_to_bitfield_big_endian_lsb() {
        #[bitfield(u32, from_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x11_22_33_44_u32);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
    }

    #[test]
    fn from_trait_data_type_to_bitfield_big_endian_msb() {
        #[bitfield(u32, order = msb, from_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x11_22_33_44_u32);

        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.d(), 0x44);
    }

    #[test]
    fn from_trait_data_type_to_bitfield_little_endian_lsb() {
        #[bitfield(u32, from_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x44_33_22_11_u32);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
    }

    #[test]
    fn from_trait_data_type_to_bitfield_little_endian_msb() {
        #[bitfield(u32, order = msb, from_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x78_56_34_12_u32);

        assert_eq!(bitfield.a(), 0x12);
        assert_eq!(bitfield.b(), 0x34);
        assert_eq!(bitfield.c(), 0x56);
        assert_eq!(bitfield.d(), 0x78);
    }

    #[test]
    fn from_trait_bitfield_to_data_type_big_endian_lsb() {
        #[bitfield(u32, into_endian = big)]
        pub struct Bitfield {
            #[bits(default = 0x44)]
            a: u8,
            #[bits(default = 0x33)]
            b: u8,
            #[bits(default = 0x22)]
            c: u8,
            #[bits(default = 0x11)]
            d: u8,
        }

        let bitfield = Bitfield::new();
        let val = u32::from(bitfield);

        assert_eq!(val, 0x11_22_33_44);
    }

    #[test]
    fn from_trait_bitfield_to_data_type_big_endian_msb() {
        #[bitfield(u32, order = msb, into_endian = big)]
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

        let bitfield = Bitfield::new();
        let val = u32::from(bitfield);

        assert_eq!(val, 0x11_22_33_44);
    }

    #[test]
    fn from_trait_bitfield_to_data_type_little_endian_lsb() {
        #[bitfield(u32, into_endian = little)]
        pub struct Bitfield {
            #[bits(default = 0x44)]
            a: u8,
            #[bits(default = 0x33)]
            b: u8,
            #[bits(default = 0x22)]
            c: u8,
            #[bits(default = 0x11)]
            d: u8,
        }

        let bitfield = Bitfield::new();

        let val = u32::from(bitfield);

        assert_eq!(val, 0x44_33_22_11);
    }

    #[test]
    fn from_trait_bitfield_to_data_type_little_endian_msb() {
        #[bitfield(u32, order = msb, into_endian = little)]
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

        let bitfield = Bitfield::new();

        let val = u32::from(bitfield);

        assert_eq!(val, 0x44_33_22_11);
    }

    #[test]
    fn from_trait_round_trip_big_endian() {
        #[bitfield(u32, from_endian = big, into_endian = big)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let original: u32 = 0xDE_AD_BE_EF;
        let bitfield = Bitfield::from(original);
        let recovered = u32::from(bitfield);

        assert_eq!(recovered, original);
    }

    #[test]
    fn from_trait_round_trip_little_endian() {
        #[bitfield(u32, from_endian = little, into_endian = little)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let original: u32 = 0xDE_AD_BE_EF;
        let bitfield = Bitfield::from(original);
        let recovered = u32::from(bitfield);

        assert_eq!(recovered, original);
    }

    #[test]
    fn from_trait_u8_data_type() {
        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(4)]
            lo: u8,
            #[bits(4)]
            hi: u8,
        }

        let bitfield = Bitfield::from(0xAB_u8);
        assert_eq!(bitfield.lo(), 0xB);
        assert_eq!(bitfield.hi(), 0xA);
        assert_eq!(u8::from(bitfield), 0xAB);
    }

    #[test]
    fn from_trait_u64_data_type() {
        #[bitfield(u64)]
        pub struct Bitfield {
            a: u32,
            b: u32,
        }

        let bitfield = Bitfield::from(0x11_22_33_44_AA_BB_CC_DD_u64);
        assert_eq!(bitfield.a(), 0xAA_BB_CC_DD);
        assert_eq!(bitfield.b(), 0x11_22_33_44);
        assert_eq!(u64::from(bitfield), 0x11_22_33_44_AA_BB_CC_DD);
    }

    #[test]
    fn from_trait_u128_data_type() {
        #[bitfield(u128)]
        pub struct Bitfield {
            a: u64,
            b: u64,
        }

        let val: u128 = 0x0102_0304_0506_0708_090A_0B0C_0D0E_0F10;
        let bitfield = Bitfield::from(val);
        assert_eq!(u128::from(bitfield), val);
    }

    #[test]
    fn from_trait_disabled_does_not_generate_from_data_type() {
        #[bitfield(u32, from_traits = true)]
        pub struct Bitfield {
            a: u8,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from(0x11_22_33_44_u32);
        assert_eq!(u32::from(bitfield), 0x11_22_33_44);
    }

    #[test]
    fn custom_numeric_type_newtype_getter() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(8)]
            voltage: Millivolts,
        }

        let bitfield = Bitfield::from_bits(0x42);
        assert_eq!(bitfield.voltage(), Millivolts(0x42));
    }

    #[test]
    fn custom_numeric_type_newtype_setter() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(8)]
            voltage: Millivolts,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_voltage(Millivolts(0x55));
        assert_eq!(bitfield.into_bits(), 0x55);
    }

    #[test]
    fn custom_numeric_type_newtype_bitfield_into_bits() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(8)]
            a: Millivolts,
            b: u8,
            #[bits(8)]
            c: Millivolts,
            d: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_a(Millivolts(0x11));
        bitfield.set_b(0x22);
        bitfield.set_c(Millivolts(0x33));
        bitfield.set_d(0x44);
        assert_eq!(bitfield.into_bits(), 0x44_33_22_11);
    }

    #[test]
    fn custom_numeric_type_newtype_bitfield_from_bits() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(8)]
            a: Millivolts,
            b: u8,
            #[bits(8)]
            c: Millivolts,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x44_33_22_11);
        assert_eq!(bitfield.a(), Millivolts(0x11));
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.c(), Millivolts(0x33));
        assert_eq!(bitfield.d(), 0x44);
    }

    #[test]
    fn custom_numeric_type_newtype_round_trip() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u16);
        impl Millivolts {
            const fn from_bits(bits: u16) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u16 {
                self.0
            }
        }

        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(16)]
            lo: Millivolts,
            #[bits(16)]
            hi: Millivolts,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_lo(Millivolts(0x1234));
        bitfield.set_hi(Millivolts(0x5678));
        assert_eq!(bitfield.lo(), Millivolts(0x1234));
        assert_eq!(bitfield.hi(), Millivolts(0x5678));
        assert_eq!(bitfield.into_bits(), 0x5678_1234);
    }

    #[test]
    fn custom_numeric_type_non_trivial_transformation_inverted_bits() {
        #[derive(Debug, PartialEq)]
        struct InvertedBits(u8);
        impl InvertedBits {
            const fn from_bits(bits: u8) -> Self {
                Self(!bits)
            }
            const fn into_bits(self) -> u8 {
                !self.0
            }
        }

        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(8)]
            value: InvertedBits,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_value(InvertedBits(0xAA));
        assert_eq!(bitfield.into_bits(), 0x55);

        assert_eq!(bitfield.value(), InvertedBits(0xAA));

        let bitfield2 = Bitfield::from_bits(0x55);
        assert_eq!(bitfield2.value(), InvertedBits(0xAA));
    }

    #[test]
    fn custom_numeric_type_non_trivial_transformation_half_resolution() {
        #[derive(Debug, PartialEq)]
        struct HalfRes(u8);
        impl HalfRes {
            const fn from_bits(bits: u8) -> Self {
                Self(bits.wrapping_mul(2))
            }
            const fn into_bits(self) -> u8 {
                self.0 / 2
            }
        }

        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(8)]
            value: HalfRes,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_value(HalfRes(10));
        assert_eq!(bitfield.into_bits(), 5);

        assert_eq!(bitfield.value(), HalfRes(10));

        let bitfield2 = Bitfield::from_bits(5);
        assert_eq!(bitfield2.value(), HalfRes(10));
    }

    #[test]
    fn custom_numeric_type_default_value() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(8, default = Millivolts(0xAB))]
            voltage: Millivolts,
            #[bits(8)]
            other: u8,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.voltage(), Millivolts(0xAB));
        assert_eq!(bitfield.into_bits(), 0x00AB);
    }

    #[test]
    fn custom_numeric_type_new_without_defaults() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(8, default = Millivolts(0xAB))]
            voltage: Millivolts,
            #[bits(8)]
            other: u8,
        }

        let bitfield = Bitfield::new_without_defaults();
        assert_eq!(bitfield.voltage(), Millivolts(0));
        assert_eq!(bitfield.into_bits(), 0x0000);
    }

    #[test]
    fn custom_numeric_type_builder() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(8)]
            voltage: Millivolts,
            status: u8,
        }

        let bitfield =
            BitfieldBuilder::new().with_voltage(Millivolts(0xC0)).with_status(0xFF).build();
        assert_eq!(bitfield.voltage(), Millivolts(0xC0));
        assert_eq!(bitfield.status(), 0xFF);
        assert_eq!(bitfield.into_bits(), 0xFF_C0);
    }

    #[test]
    fn custom_numeric_type_checked_setter() {
        #[derive(Debug, PartialEq)]
        struct SmallValue(u8);
        impl SmallValue {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u8)]
        pub struct Bitfield {
            #[bits(4)]
            nibble: SmallValue,
            #[bits(4)]
            other: u8,
        }

        let mut bitfield = Bitfield::new();

        assert!(bitfield.checked_set_nibble(SmallValue(0xF)).is_ok());
        assert_eq!(bitfield.nibble(), SmallValue(0xF));

        assert!(bitfield.checked_set_nibble(SmallValue(0x10)).is_err());
        assert_eq!(bitfield.nibble(), SmallValue(0xF));
    }

    #[test]
    fn custom_numeric_type_read_only_field() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(8, access = ro)]
            voltage: Millivolts,
            #[bits(8)]
            status: u8,
        }

        let bitfield = Bitfield::from_bits(0x55_AA);
        assert_eq!(bitfield.voltage(), Millivolts(0xAA));
        assert_eq!(bitfield.status(), 0x55);
    }

    #[test]
    fn custom_numeric_type_write_only_field() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(8, access = wo)]
            voltage: Millivolts,
            #[bits(8)]
            status: u8,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_voltage(Millivolts(0xBE));
        assert_eq!(bitfield.into_bits() & 0xFF, 0xBE);
    }

    #[test]
    fn custom_numeric_type_msb_order() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u32, order = msb)]
        pub struct Bitfield {
            #[bits(8)]
            a: Millivolts,
            b: u8,
            c: u8,
            d: u8,
        }

        let bitfield = Bitfield::from_bits(0x11_22_33_44);
        assert_eq!(bitfield.a(), Millivolts(0x11));
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn custom_numeric_type_from_traits() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(8)]
            voltage: Millivolts,
            status: u8,
        }

        let bitfield = Bitfield::from(0x55_AA_u16);
        assert_eq!(bitfield.voltage(), Millivolts(0xAA));
        assert_eq!(bitfield.status(), 0x55);

        let raw: u16 = bitfield.into();
        assert_eq!(raw, 0x55_AA);
    }

    #[test]
    fn custom_numeric_type_multiple_custom_fields() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[derive(Debug, PartialEq)]
        struct MilliAmps(u8);
        impl MilliAmps {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(8)]
            voltage: Millivolts,
            #[bits(8)]
            current: MilliAmps,
            #[bits(16)]
            reserved: u16,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_voltage(Millivolts(0x12));
        bitfield.set_current(MilliAmps(0x34));

        assert_eq!(bitfield.voltage(), Millivolts(0x12));
        assert_eq!(bitfield.current(), MilliAmps(0x34));
        assert_eq!(bitfield.into_bits(), 0x0000_3412);
    }

    #[test]
    fn custom_numeric_type_mixed_with_integer_fields() {
        #[derive(Debug, PartialEq)]
        struct Celsius(i8);
        impl Celsius {
            const fn from_bits(bits: u8) -> Self {
                Self(bits as i8)
            }
            const fn into_bits(self) -> u8 {
                self.0 as u8
            }
        }

        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(8)]
            temperature: Celsius,
            #[bits(8)]
            humidity: u8,
            #[bits(16)]
            pressure: u16,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_temperature(Celsius(-10));
        bitfield.set_humidity(75);
        bitfield.set_pressure(1013);

        assert_eq!(bitfield.temperature(), Celsius(-10));
        assert_eq!(bitfield.humidity(), 75);
        assert_eq!(bitfield.pressure(), 1013);

        let raw = bitfield.into_bits();
        assert_eq!((raw & 0xFF) as u8, (-10i8) as u8);
    }

    #[test]
    fn custom_numeric_type_from_bits_with_defaults() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u16)]
        pub struct Bitfield {
            #[bits(8, default = Millivolts(0xAB))]
            voltage: Millivolts,
            #[bits(8)]
            status: u8,
        }

        let bitfield = Bitfield::from_bits_with_defaults(0x55_FF);
        assert_eq!(bitfield.voltage(), Millivolts(0xAB));
        assert_eq!(bitfield.status(), 0x55);
    }

    #[test]
    fn custom_numeric_type_full_integration() {
        #[derive(Debug, PartialEq)]
        struct Millivolts(u8);
        impl Millivolts {
            const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }
            const fn into_bits(self) -> u8 {
                self.0
            }
        }

        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(8)]
            a: Millivolts,
            b: u8,
            #[bits(8)]
            c: Millivolts,
            d: u8,
        }

        let original = BitfieldBuilder::new()
            .with_a(Millivolts(0x11))
            .with_b(0x22)
            .with_c(Millivolts(0x33))
            .with_d(0x44)
            .build();

        let raw = original.into_bits();
        assert_eq!(raw, 0x44_33_22_11);

        let recovered = Bitfield::from_bits(raw);
        assert_eq!(recovered.a(), Millivolts(0x11));
        assert_eq!(recovered.b(), 0x22);
        assert_eq!(recovered.c(), Millivolts(0x33));
        assert_eq!(recovered.d(), 0x44);
    }

    #[test]
    fn custom_numeric_type_clear_bits_uses_from_bits() {
        #[derive(Debug, PartialEq)]
        struct InvertedBits(u8);
        impl InvertedBits {
            const fn from_bits(bits: u8) -> Self {
                Self(!bits)
            }
            const fn into_bits(self) -> u8 {
                !self.0
            }
        }

        #[bitfield(u8, bit_ops = true)]
        pub struct Bitfield {
            #[bits(8)]
            value: InvertedBits,
        }

        let mut bitfield = Bitfield::new();
        bitfield.set_value(InvertedBits(0xAA));
        assert_eq!(bitfield.value(), InvertedBits(0xAA));

        bitfield.clear_bits();
        assert_eq!(bitfield.value(), InvertedBits(0xFF));
    }

    #[test]
    fn custom_numeric_type_write_bits_uses_from_bits() {
        #[derive(Debug, PartialEq)]
        struct InvertedBits(u8);
        impl InvertedBits {
            const fn from_bits(bits: u8) -> Self {
                Self(!bits)
            }
            const fn into_bits(self) -> u8 {
                !self.0
            }
        }

        #[bitfield(u8, bit_ops = true)]
        pub struct Bitfield {
            #[bits(8)]
            value: InvertedBits,
        }

        let mut bitfield = Bitfield::new();

        bitfield.write_bits(0x55u8);
        assert_eq!(bitfield.value(), InvertedBits(0xAA));

        assert_eq!(bitfield.into_bits(), 0x55u8);
    }

    #[test]
    fn bitflag_base_variant_from_bits_known_values() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1), Flags::A);
        assert_eq!(Flags::from_bits(2), Flags::B);
    }

    #[test]
    fn bitflag_base_variant_from_bits_unknown_value_maps_to_base() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(0), Flags::Unknown);
        assert_eq!(Flags::from_bits(99), Flags::Unknown);
    }

    #[test]
    fn bitflag_base_variant_not_included_in_exact_match_arms() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Default = 99,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1), Flags::A);
        assert_eq!(Flags::from_bits(2), Flags::B);
        assert_eq!(Flags::from_bits(99), Flags::Default);
        assert_eq!(Flags::from_bits(50), Flags::Default);
    }

    #[test]
    fn bitflag_default_variant_from_bits_known_values() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq, Default)]
        enum Flags {
            #[default]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(0), Flags::Unknown);
        assert_eq!(Flags::from_bits(1), Flags::A);
        assert_eq!(Flags::from_bits(2), Flags::B);
    }

    #[test]
    fn bitflag_default_variant_from_bits_unknown_value_maps_to_default() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq, Default)]
        enum Flags {
            #[default]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(99), Flags::Unknown);
    }

    #[test]
    fn bitflag_into_bits_returns_correct_values() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::Unknown.into_bits(), 0_u8);
        assert_eq!(Flags::A.into_bits(), 1_u8);
        assert_eq!(Flags::B.into_bits(), 2_u8);
    }

    #[test]
    fn bitflag_u16_type() {
        #[bitflag(u16)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1_u16), Flags::A);
        assert_eq!(Flags::from_bits(2_u16), Flags::B);
        assert_eq!(Flags::from_bits(99_u16), Flags::Unknown);
        assert_eq!(Flags::A.into_bits(), 1_u16);
        assert_eq!(Flags::B.into_bits(), 2_u16);
    }

    #[test]
    fn bitflag_u32_type() {
        #[bitflag(u32)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1_u32), Flags::A);
        assert_eq!(Flags::from_bits(2_u32), Flags::B);
        assert_eq!(Flags::from_bits(99_u32), Flags::Unknown);
        assert_eq!(Flags::A.into_bits(), 1_u32);
        assert_eq!(Flags::B.into_bits(), 2_u32);
    }

    #[test]
    fn bitflag_u32_from_endian_little() {
        #[bitflag(u32, from_endian = little)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 0x01000000,
            B = 0x02000000,
        }
        assert_eq!(Flags::from_bits(0x00000001_u32), Flags::A);
        assert_eq!(Flags::from_bits(0x00000002_u32), Flags::B);
        assert_eq!(Flags::from_bits(0x000000FF_u32), Flags::Unknown);
    }

    #[test]
    fn bitflag_u32_into_endian_little() {
        #[bitflag(u32, into_endian = little)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::A.into_bits(), 0x01000000_u32);
        assert_eq!(Flags::B.into_bits(), 0x02000000_u32);
        assert_eq!(Flags::Unknown.into_bits(), 0_u32);
    }

    #[test]
    fn bitflag_u32_from_and_into_endian_little() {
        #[bitflag(u32, from_endian = little, into_endian = little)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 0x01000000,
            B = 0x02000000,
        }
        assert_eq!(Flags::from_bits(0x00000001_u32), Flags::A);
        assert_eq!(Flags::from_bits(0x00000002_u32), Flags::B);
        assert_eq!(Flags::A.into_bits(), 0x00000001_u32);
        assert_eq!(Flags::B.into_bits(), 0x00000002_u32);
    }

    #[test]
    fn bitflag_u128_type() {
        #[bitflag(u128)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1_u128), Flags::A);
        assert_eq!(Flags::from_bits(2_u128), Flags::B);
        assert_eq!(Flags::from_bits(99_u128), Flags::Unknown);
        assert_eq!(Flags::A.into_bits(), 1_u128);
        assert_eq!(Flags::B.into_bits(), 2_u128);
    }

    #[test]
    fn bitflag_u128_from_endian_little() {
        #[bitflag(u128, from_endian = little)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1_u128.swap_bytes()), Flags::A);
        assert_eq!(Flags::from_bits(2_u128.swap_bytes()), Flags::B);
        assert_eq!(Flags::from_bits(99_u128.swap_bytes()), Flags::Unknown);
    }

    #[test]
    fn bitflag_u128_into_endian_little() {
        #[bitflag(u128, into_endian = little)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::A.into_bits(), 1_u128.swap_bytes());
        assert_eq!(Flags::B.into_bits(), 2_u128.swap_bytes());
        assert_eq!(Flags::Unknown.into_bits(), 0_u128);
    }

    #[test]
    fn bitflag_u128_from_and_into_endian_little() {
        #[bitflag(u128, from_endian = little, into_endian = little)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1_u128.swap_bytes()), Flags::A);
        assert_eq!(Flags::from_bits(2_u128.swap_bytes()), Flags::B);
        assert_eq!(Flags::A.into_bits(), 1_u128.swap_bytes());
        assert_eq!(Flags::B.into_bits(), 2_u128.swap_bytes());
    }

    #[test]
    fn bitflag_from_endian_little_swaps_bytes_on_from_bits() {
        #[bitflag(u16, from_endian = little)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 0x0100,
            B = 0x0200,
        }
        assert_eq!(Flags::from_bits(0x0001_u16), Flags::A);
        assert_eq!(Flags::from_bits(0x0002_u16), Flags::B);
        assert_eq!(Flags::from_bits(0x00FF_u16), Flags::Unknown);
    }

    #[test]
    fn bitflag_from_endian_big_no_swap_on_from_bits() {
        #[bitflag(u16, from_endian = big)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1_u16), Flags::A);
        assert_eq!(Flags::from_bits(2_u16), Flags::B);
        assert_eq!(Flags::from_bits(99_u16), Flags::Unknown);
    }

    #[test]
    fn bitflag_into_endian_little_swaps_bytes_on_into_bits() {
        #[bitflag(u16, into_endian = little)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::A.into_bits(), 0x0100_u16);
        assert_eq!(Flags::B.into_bits(), 0x0200_u16);
        assert_eq!(Flags::Unknown.into_bits(), 0_u16);
    }

    #[test]
    fn bitflag_into_endian_big_no_swap_on_into_bits() {
        #[bitflag(u16, into_endian = big)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::A.into_bits(), 1_u16);
        assert_eq!(Flags::B.into_bits(), 2_u16);
        assert_eq!(Flags::Unknown.into_bits(), 0_u16);
    }

    #[test]
    fn bitflag_from_and_into_endian_little() {
        #[bitflag(u16, from_endian = little, into_endian = little)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 0x0100,
            B = 0x0200,
        }
        assert_eq!(Flags::from_bits(0x0001_u16), Flags::A);
        assert_eq!(Flags::from_bits(0x0002_u16), Flags::B);
        assert_eq!(Flags::A.into_bits(), 0x0001_u16);
        assert_eq!(Flags::B.into_bits(), 0x0002_u16);
    }

    #[test]
    fn bitflag_pub_visibility() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        pub enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1), Flags::A);
        assert_eq!(Flags::from_bits(0), Flags::Unknown);
        assert_eq!(Flags::A.into_bits(), 1_u8);
    }

    #[test]
    fn bitflag_hex_and_binary_variant_values() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0x00,
            A = 0x01,
            B = 0b0000_0010,
            C = 0x04,
        }
        assert_eq!(Flags::from_bits(0x01), Flags::A);
        assert_eq!(Flags::from_bits(0b0000_0010), Flags::B);
        assert_eq!(Flags::from_bits(0x04), Flags::C);
        assert_eq!(Flags::from_bits(0x08), Flags::Unknown);
        assert_eq!(Flags::A.into_bits(), 0x01_u8);
        assert_eq!(Flags::B.into_bits(), 0x02_u8);
        assert_eq!(Flags::C.into_bits(), 0x04_u8);
    }

    #[test]
    fn bitflag_user_attributes_on_variants() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            #[allow(dead_code)]
            Unknown = 0,
            #[doc = "Variant A"]
            A = 1,
            B = 2,
        }
        assert_eq!(Flags::from_bits(1), Flags::A);
        assert_eq!(Flags::from_bits(2), Flags::B);
        assert_eq!(Flags::A.into_bits(), 1_u8);
    }

    #[test]
    fn bitflag_user_attributes_on_enum() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Flags {
            #[base]
            Unknown = 0,
            A = 1,
            B = 2,
        }
        let a = Flags::A;
        let b = a;
        assert_eq!(a, b);
        assert_eq!(Flags::from_bits(1), Flags::A);
        assert_eq!(Flags::A.into_bits(), 1_u8);
    }

    #[test]
    fn bitfield_with_bitflag_field_getter_setter() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Status {
            #[base]
            Idle = 0,
            Active = 1,
            Paused = 2,
        }

        #[bitfield(u16)]
        struct Packet {
            #[bits(8)]
            status: Status,
            flags: u8,
        }

        let mut packet = Packet::new();
        packet.set_status(Status::Active);
        packet.set_flags(0xAB);
        assert_eq!(packet.status(), Status::Active);
        assert_eq!(packet.flags(), 0xAB);
    }

    #[test]
    fn bitfield_with_bitflag_field_from_into_bits() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Mode {
            #[base]
            Off = 0,
            Read = 1,
            Write = 2,
            ReadWrite = 3,
        }

        #[bitfield(u16)]
        struct Register {
            #[bits(8)]
            mode: Mode,
            value: u8,
        }

        let raw =
            RegisterBuilder::new().with_mode(Mode::Write).with_value(0x42).build().into_bits();

        let recovered = Register::from_bits(raw);
        assert_eq!(recovered.mode(), Mode::Write);
        assert_eq!(recovered.value(), 0x42);
    }

    #[test]
    fn bitfield_with_multiple_bitflag_fields() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Direction {
            #[base]
            None = 0,
            North = 1,
            South = 2,
            East = 3,
            West = 4,
        }

        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Speed {
            #[base]
            Stop = 0,
            Slow = 1,
            Fast = 2,
        }

        #[bitfield(u16)]
        struct Command {
            #[bits(8)]
            direction: Direction,
            #[bits(8)]
            speed: Speed,
        }

        let cmd =
            CommandBuilder::new().with_direction(Direction::East).with_speed(Speed::Fast).build();

        assert_eq!(cmd.direction(), Direction::East);
        assert_eq!(cmd.speed(), Speed::Fast);

        let raw = cmd.into_bits();
        let recovered = Command::from_bits(raw);
        assert_eq!(recovered.direction(), Direction::East);
        assert_eq!(recovered.speed(), Speed::Fast);
    }

    #[test]
    fn bitfield_with_bitflag_field_default_value() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum State {
            #[base]
            Unknown = 0,
            Ready = 1,
            Busy = 2,
        }

        #[bitfield(u16)]
        struct Control {
            #[bits(8, default = State::Ready)]
            state: State,
            config: u8,
        }

        let ctrl = Control::new();
        assert_eq!(ctrl.state(), State::Ready);
        assert_eq!(ctrl.config(), 0);
    }

    #[test]
    fn bitfield_with_bitflag_field_partial_bits() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Flag {
            #[base]
            Off = 0,
            On = 1,
            Pending = 2,
        }

        #[bitfield(u8)]
        struct Packed {
            #[bits(2)]
            a: Flag,
            #[bits(2)]
            b: Flag,
            #[bits(4)]
            rest: u8,
        }

        let p = PackedBuilder::new().with_a(Flag::On).with_b(Flag::Pending).with_rest(0xF).build();

        assert_eq!(p.a(), Flag::On);
        assert_eq!(p.b(), Flag::Pending);
        assert_eq!(p.rest(), 0xF);

        let raw = p.into_bits();
        let recovered = Packed::from_bits(raw);
        assert_eq!(recovered.a(), Flag::On);
        assert_eq!(recovered.b(), Flag::Pending);
        assert_eq!(recovered.rest(), 0xF);
    }

    #[test]
    fn bitfield_with_bitflag_field_unknown_bits_map_to_base() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Priority {
            #[base]
            Low = 0,
            Medium = 1,
            High = 2,
        }

        #[bitfield(u16)]
        struct Task {
            #[bits(8)]
            priority: Priority,
            id: u8,
        }

        let raw = TaskBuilder::new().with_id(0x10).build().into_bits() | 0xFFu16;

        let task = Task::from_bits(raw);
        assert_eq!(task.priority(), Priority::Low);
        assert_eq!(task.id(), 0x10);
    }

    #[test]
    fn bitfield_with_bitflag_and_primitive_fields_round_trip() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Protocol {
            #[base]
            Unknown = 0,
            Uart = 1,
            Spi = 2,
            I2c = 3,
        }

        #[bitfield(u32)]
        struct Frame {
            #[bits(8)]
            protocol: Protocol,
            address: u8,
            length: u8,
            checksum: u8,
        }

        let frame = FrameBuilder::new()
            .with_protocol(Protocol::Spi)
            .with_address(0x12)
            .with_length(0x08)
            .with_checksum(0xAB)
            .build();

        assert_eq!(frame.protocol(), Protocol::Spi);
        assert_eq!(frame.address(), 0x12);
        assert_eq!(frame.length(), 0x08);
        assert_eq!(frame.checksum(), 0xAB);

        let raw = frame.into_bits();
        let recovered = Frame::from_bits(raw);
        assert_eq!(recovered.protocol(), Protocol::Spi);
        assert_eq!(recovered.address(), 0x12);
        assert_eq!(recovered.length(), 0x08);
        assert_eq!(recovered.checksum(), 0xAB);
    }

    #[test]
    fn bitfield_with_bitflag_field_from_traits() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Channel {
            #[base]
            None = 0,
            Ch1 = 1,
            Ch2 = 2,
            Ch3 = 3,
        }

        #[bitfield(u16, from_traits = true)]
        struct Config {
            #[bits(8)]
            channel: Channel,
            gain: u8,
        }

        let cfg = Config::from(0x03_01_u16);
        assert_eq!(cfg.channel(), Channel::Ch1);
        assert_eq!(cfg.gain(), 0x03);

        let raw: u16 =
            ConfigBuilder::new().with_channel(Channel::Ch3).with_gain(0x07).build().into();
        assert_eq!(raw, 0x07_03_u16);
    }

    #[test]
    fn bitfield_force_panic_disabled() {
        #[bitfield(u8, force_panic = false)]
        struct Bitfield {
            a: u8,
        }

        let _ = Bitfield::new();
    }

    #[test]
    fn bitfield_field_bits_leading_comma() {
        #[bitfield(u16)]
        struct Bitfield {
            #[bits(, 7)]
            a: u8,
            #[bits(9)]
            b: u16,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x7F);
        assert_eq!(bf.a(), 0x7F);
    }

    #[test]
    fn bitfield_field_bits_trailing_comma() {
        #[bitfield(u8)]
        struct Bitfield {
            #[bits(7)]
            a: u8,
            b: bool,
        }

        let mut bf = Bitfield::new();
        bf.set_a(0x7F);
        assert_eq!(bf.a(), 0x7F);
    }

    #[test]
    fn bitflag_derive_copy_auto_derives_copy_and_clone() {
        #[bitflag(u8)]
        #[derive(Debug, PartialEq)]
        enum Status {
            #[base]
            Unknown = 0,
            Active = 1,
            Paused = 2,
        }

        let a = Status::Active;
        let b = a;
        assert_eq!(a, b);

        let c = a;
        assert_eq!(a, c);
    }

    #[test]
    fn bitflag_derive_copy_inline_opt_out() {
        #[bitflag(u8, copy = false)]
        #[derive(Debug, PartialEq)]
        enum NonCopyFlag {
            #[base]
            Unknown = 0,
            A = 1,
        }

        let a = NonCopyFlag::A;
        assert_eq!(NonCopyFlag::from_bits(1), NonCopyFlag::A);
        assert_eq!(a.into_bits(), 1_u8);
    }

    #[test]
    fn bitflag_derive_copy_inline_opt_in() {
        #[bitflag(u8, copy = true)]
        #[derive(Debug, PartialEq)]
        enum CopyFlag {
            #[base]
            Unknown = 0,
            On = 1,
            Off = 2,
        }

        let a = CopyFlag::On;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn compile_error_cases() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/errors/*.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn compile_panics_cases() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/panics/*.rs");
    }

    #[test]
    fn reserved_fields_always_respect_defaults_when_creating_without_defaults() {
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

        let bf = Bitfield::new_without_defaults();
        assert_eq!(bf.a(), 0, "new_without_defaults: a should be 0");
        assert_eq!(bf.b(), 0, "new_without_defaults: b should be 0");
        assert_eq!(bf.c(), 0, "new_without_defaults: c should be 0");
        assert_eq!(
            bf.into_bits(),
            0x78_00_00_00,
            "new_without_defaults: _reserved must retain its default 0x78"
        );

        let bf = BitfieldBuilder::new_without_defaults().build();
        assert_eq!(bf.a(), 0, "builder new_without_defaults: a should be 0");
        assert_eq!(bf.b(), 0, "builder new_without_defaults: b should be 0");
        assert_eq!(bf.c(), 0, "builder new_without_defaults: c should be 0");
        assert_eq!(
            bf.into_bits(),
            0x78_00_00_00,
            "builder new_without_defaults: _reserved must retain its default 0x78"
        );

        let mut bf = Bitfield::new();
        bf.write_bits(0x11_22_33_44);
        assert_eq!(bf.a(), 0x44, "write_bits: a should be 0x44");
        assert_eq!(bf.b(), 0x33, "write_bits: b should be 0x33");
        assert_eq!(bf.c(), 0x22, "write_bits: c should be 0x22");
        assert_eq!(
            bf.into_bits(),
            0x78_22_33_44,
            "write_bits: _reserved must retain its default 0x78"
        );

        let mut bf = Bitfield::new();
        bf.write_le_bits(0x11_22_33_44);
        assert_eq!(bf.a(), 0x11, "write_le_bits: a should be 0x11");
        assert_eq!(bf.b(), 0x22, "write_le_bits: b should be 0x22");
        assert_eq!(bf.c(), 0x33, "write_le_bits: c should be 0x33");
        assert_eq!(
            bf.into_bits(),
            0x78_33_22_11,
            "write_le_bits: _reserved must retain its default 0x78"
        );

        let mut bf = Bitfield::new();
        bf.write_be_bits(0x11_22_33_44);
        assert_eq!(bf.a(), 0x44, "write_be_bits: a should be 0x44");
        assert_eq!(bf.b(), 0x33, "write_be_bits: b should be 0x33");
        assert_eq!(bf.c(), 0x22, "write_be_bits: c should be 0x22");
        assert_eq!(
            bf.into_bits(),
            0x78_22_33_44,
            "write_be_bits: _reserved must retain its default 0x78"
        );

        let mut bf = Bitfield::new();
        bf.clear_bits();
        assert_eq!(bf.a(), 0, "clear_bits: a should be 0 (default not re-applied)");
        assert_eq!(bf.b(), 0, "clear_bits: b should be 0 (default not re-applied)");
        assert_eq!(bf.c(), 0, "clear_bits: c should be 0 (default not re-applied)");
        assert_eq!(
            bf.into_bits(),
            0x78_00_00_00,
            "clear_bits: _reserved must retain its default 0x78"
        );
    }

    #[test]
    fn bitfields_into_bits_unnecessary_cast() {
        #[bitfield(u8)]
        struct TimerControl {
            #[bits(5)]
            __: u8,
            #[bits(1)]
            timer_stop: TimerStop,
            #[bits(2)]
            clock_select: ClockSelect,
        }

        #[bitflag(u8)]
        enum TimerStop {
            #[base]
            Stop = 0x0,
            Start = 0x1,
        }

        #[bitflag(u8)]
        enum ClockSelect {
            #[base]
            _4096 = 0x00,
            _262144 = 0x01,
            _65536 = 0x02,
            _16384 = 0x03,
        }
    }
}
