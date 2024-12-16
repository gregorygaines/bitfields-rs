#![allow(clippy::unnecessary_cast)]

#[cfg(test)]
mod tests {
    use bitfields::bitfield;

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_attribute_non_struct_compile_fail() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_attribute_non_struct.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_std_compile_pass() {
        let t = trybuild::TestCases::new();
        t.pass("tests/compile_error_cases/bitfield_no_std.rs");
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_unsupported_type() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_unsupported_type.rs");
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_types_bits_less_than_bitfield_type_size() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_types_bits_less_than_bitfield_type_size.rs",
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_types_bits_more_than_bitfield_type_size() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_types_bits_more_than_bitfield_type_size.rs",
        );
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_defined_bits_less_than_bitfield_type_size() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_defined_bits_less_than_bitfield_type_size.rs",
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_defined_bits_more_than_bitfield_type_size() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_defined_bits_more_than_bitfield_type_size.rs",
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_defined_bits_0() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_field_defined_bits_0.rs");
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_float_default_value() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_field_float_default_value.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_float32_identifier_default_value() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_float32_identifier_default_value.rs",
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_float64_identifier_default_value() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_float64_identifier_default_value.rs",
        );
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_duplicate_name() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_field_duplicate_name.rs");
    }

    #[test]
    fn bitfield_field_duplicate_padding_name() {
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
    fn bitfield_field_padding() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(4, default = 0x1)]
            a: u8,
            #[bits(60)]
            _padding: u64,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0x1);
    }

    #[test]
    fn bitfield_field_padding_default_value() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[bits(4, default = 0x1)]
            a: u8,
            #[bits(60, default = 0xFFFF_FFFF_FFFF)]
            _padding: u64,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0x1);
        assert_eq!(bitfield.into_bits(), 0xF_FFFF_FFFF_FFF1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_access_padding_value() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_field_access_padding_value.rs");
    }

    #[test]
    fn bitfield_field_multiple_attributes() {
        #[bitfield(u64)]
        pub struct Bitfield {
            #[serde(skip)]
            #[bits(4, default = 0x1)]
            a: u8,
            #[bits(60, default = 0xFFFF_FFFF_FFFF)]
            _padding: u64,
        }

        let bitfield = Bitfield::new();
        assert_eq!(bitfield.a(), 0x1);
        assert_eq!(bitfield.into_bits(), 0xF_FFFF_FFFF_FFF1);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_too_small_for_default_value() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_field_too_small_for_default_value.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_defined_bits_too_small_for_default_value() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_defined_bits_too_small_for_default_value.rs",
        );
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
            _padding: u16,
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_nested_bitfield_without_defined_bits() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_nested_bitfield_without_defined_bits.rs",
        );
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
                Self { a: 3, b: 4 }
            }

            const fn from_bits(bits: u8) -> Self {
                Self { a: bits as i8, b: bits }
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
    fn bitfield_field_padding_struct_custom_type() {
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
                Self { a: 3, b: 4 }
            }

            const fn from_bits(bits: u8) -> Self {
                Self { a: bits as i8, b: bits }
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

        assert_eq!(debug_str, "Bitfield { d: 120, c: 86, b: 52, a: 18 }");
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_access_write_only() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_field_access_write_only.rs");
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_access_write_only_builder() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_field_access_write_only_builder.rs");
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_access_read_only_can_not_write() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_access_read_only_can_not_write.rs",
        );
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_field_access_read_only_can_not_write_builder() {
        let t = trybuild::TestCases::new();
        t.compile_fail(
            "tests/compile_error_cases/bitfield_field_access_read_only_can_not_write_builder.rs",
        );
    }

    #[test]
    fn bitfield_field_access_none() {
        #[bitfield(u32)]
        pub struct Bitfield {
            #[bits(default = 0x12, access = none)]
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
        // The data 0x11223344 is stored in little-endian memory as [0x44, 0x33, 0x22,
        // 0x11].
        #[bitfield(u32, from_endian = little)]
        pub struct Bitfield {
            // Lsb
            a: u8,
            b: u8,
            c: u8,
            d: u8,
            // Msb
        }

        // Raw data from memory.
        let bitfield = Bitfield::from_bits(0x44_33_22_11);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_from_bits_little_endian_msb_field_order() {
        // The data 0x11223344 is stored in little-endian memory as [0x44, 0x33, 0x22,
        // 0x11].
        #[bitfield(u32, from_endian = little, order = msb)]
        pub struct Bitfield {
            // Msb
            a: u8,
            b: u8,
            c: u8,
            d: u8,
            // Lsb
        }

        // Raw data from memory.
        let bitfield = Bitfield::from_bits(0x44_33_22_11);

        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_from_bits_big_endian() {
        // The data 0x11223344 is stored in big-endian memory as [0x11, 0x22, 0x33,
        // 0x44].
        #[bitfield(u32, from_endian = big)]
        pub struct Bitfield {
            // Lsb
            a: u8,
            b: u8,
            c: u8,
            d: u8,
            // Msb
        }

        // Raw data from memory.
        let bitfield = Bitfield::from_bits(0x11_22_33_44);

        assert_eq!(bitfield.a(), 0x44);
        assert_eq!(bitfield.b(), 0x33);
        assert_eq!(bitfield.c(), 0x22);
        assert_eq!(bitfield.d(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_from_bits_big_endian_msb_order() {
        // The data 0x11223344 is stored in big-endian memory as [0x11, 0x22, 0x33,
        // 0x44].
        #[bitfield(u32, from_endian = big, order = msb)]
        pub struct Bitfield {
            // Msb
            a: u8,
            b: u8,
            c: u8,
            d: u8,
            // Lsb
        }

        // Raw data from memory.
        let bitfield = Bitfield::from_bits(0x11_22_33_44);

        assert_eq!(bitfield.d(), 0x44);
        assert_eq!(bitfield.c(), 0x33);
        assert_eq!(bitfield.b(), 0x22);
        assert_eq!(bitfield.a(), 0x11);
        assert_eq!(bitfield.into_bits(), 0x11_22_33_44);
    }

    #[test]
    fn bitfield_into_bits_little_endian() {
        // Represents 0x11223344
        #[bitfield(u32, into_endian = little)]
        pub struct Bitfield {
            // Lsb
            #[bits(default = 0x44)]
            a: u8,
            #[bits(default = 0x33)]
            b: u8,
            #[bits(default = 0x22)]
            c: u8,
            #[bits(default = 0x11)]
            d: u8,
            // Msb
        }

        let bitfield = Bitfield::default();

        assert_eq!(bitfield.into_bits(), 0x44332211);
    }

    #[test]
    fn bitfield_into_bits_big_endian() {
        // Represents 0x11223344
        #[bitfield(u32, into_endian = big)]
        pub struct Bitfield {
            // Lsb
            #[bits(default = 0x44)]
            a: u8,
            #[bits(default = 0x33)]
            b: u8,
            #[bits(default = 0x22)]
            c: u8,
            #[bits(default = 0x11)]
            d: u8,
            // Msb
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
    #[cfg_attr(miri, ignore)]
    fn bitfield_disable_from_trait_compile_error() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_disable_from_trait.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_disable_builder_compile_error() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_disable_builder.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_disable_debug_compile_error() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_disable_debug.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_disable_default_compile_error() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_disable_default.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_disable_from_bits_compile_error() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_disable_from_bits.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_disable_into_bits_compile_error() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_disable_into_bits.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_disable_new_compile_error() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_disable_new.rs");
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    fn bitfield_padding_field_with_access_compile_error() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_error_cases/bitfield_padding_field_with_access.rs");
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
}
