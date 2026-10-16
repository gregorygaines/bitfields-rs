#![deny(conflicting_repr_hints)]

use bitfields::bitflag;
use zerocopy::{IntoBytes, TryFromBytes};

#[test]
fn explicit_type_matches_repr_with_derives_before_bitflag() {
    #[derive(
        Debug, PartialEq, zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable,
    )]
    #[bitflag(u8)]
    #[repr(u8)]
    enum Flags {
        #[base]
        Zero = 0,
        One = 1,
        Two = 2,
    }

    assert_eq!(Flags::try_read_from_bytes(&[1]).unwrap(), Flags::One);
    assert!(Flags::try_read_from_bytes(&[3]).is_err());
    assert_eq!(Flags::Two.as_bytes(), &[2]);
    assert_eq!(Flags::from_bits(1), Flags::One);
    assert_eq!(Flags::Two.into_bits(), 2_u8);
}

#[test]
fn infer_type_from_repr_with_derives_after_bitflag() {
    #[bitflag()]
    #[derive(
        Debug, PartialEq, zerocopy::TryFromBytes, zerocopy::IntoBytes, zerocopy::Immutable,
    )]
    #[repr(u16)]
    enum Flags {
        #[base]
        Zero = 0,
        Value = 0x1234,
    }

    assert_eq!(Flags::try_read_from_bytes(&0x1234_u16.to_ne_bytes()).unwrap(), Flags::Value);
    assert_eq!(Flags::Value.as_bytes(), &0x1234_u16.to_ne_bytes());
    assert_eq!(Flags::from_bits(0x1234_u16), Flags::Value);
    assert_eq!(Flags::Value.into_bits(), 0x1234_u16);
    assert_eq!(core::mem::size_of::<Flags>(), 2);
}

#[test]
fn infer_type_from_repr_with_bare_attribute() {
    #[bitflag]
    #[repr(u32)]
    #[derive(Debug, PartialEq)]
    enum Flags {
        #[base]
        Zero = 0,
        Value = 0x12345678,
    }

    assert_eq!(Flags::from_bits(0x12345678_u32), Flags::Value);
    assert_eq!(Flags::Value.into_bits(), 0x12345678_u32);
}

#[test]
fn infer_type_from_repr_with_configuration() {
    #[repr(u16)]
    #[bitflag(from_endian = little, into_endian = little, copy = false)]
    #[derive(Debug, PartialEq)]
    enum Flags {
        #[base]
        Zero = 0,
        Value = 0x1234,
    }

    assert_eq!(Flags::from_bits(0x3412_u16), Flags::Value);
    assert_eq!(Flags::Value.into_bits(), 0x3412_u16);
}

#[test]
fn infer_type_from_large_integer_reprs() {
    #[bitflag]
    #[repr(u64)]
    enum Flags64 {
        #[base]
        Zero = 0,
        Value = 0x8000_0000_0000_0000,
    }

    #[bitflag]
    #[repr(u128)]
    enum Flags128 {
        #[base]
        Zero = 0,
        Value = 0x8000_0000_0000_0000_0000_0000_0000_0000,
    }

    assert_eq!(Flags64::from_bits(1_u64 << 63).into_bits(), 1_u64 << 63);
    assert_eq!(Flags128::from_bits(1_u128 << 127).into_bits(), 1_u128 << 127);
}

#[test]
fn preserve_repr_modifiers() {
    #[bitflag]
    #[repr(align(4), u16)]
    enum Combined {
        #[base]
        Zero = 0,
    }

    #[bitflag(u16)]
    #[repr(align(4))]
    #[repr(u16)]
    enum Separate {
        #[base]
        Zero = 0,
    }

    #[bitflag(u16)]
    #[repr(align(4))]
    enum AlignmentOnly {
        #[base]
        Zero = 0,
    }

    assert_eq!(core::mem::align_of::<Combined>(), 4);
    assert_eq!(core::mem::align_of::<Separate>(), 4);
    assert_eq!(core::mem::align_of::<AlignmentOnly>(), 4);
    assert_eq!(Combined::from_bits(0_u16).into_bits(), 0_u16);
    assert_eq!(Separate::from_bits(0_u16).into_bits(), 0_u16);
    assert_eq!(AlignmentOnly::from_bits(0_u16).into_bits(), 0_u16);
}
