mod generation;
mod parsing;

use std::cmp::Ordering;
use std::fmt;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, ExprUnary, Fields, Lit, LitInt, Meta, Type, Visibility};

use crate::generation::bit_operations::{generate_get_bit_tokens, generate_set_bit_tokens};
use crate::generation::builder_struct::generate_builder_tokens;
use crate::generation::common::PANIC_ERROR_MESSAGE;
use crate::generation::debug_impl::generate_debug_implementation;
use crate::generation::default_impl::generate_default_implementation_tokens;
use crate::generation::field_const_getter_setter::{
    generate_field_constants_tokens, generate_field_getters_functions_tokens,
    generate_field_setters_functions_tokens,
};
use crate::generation::from_into_bits_conversions::{
    generate_from_bits_function_tokens, generate_from_bits_with_defaults_function_tokens,
    generate_into_bits_function_tokens,
};
use crate::generation::from_types_impl::{
    generate_from_bitfield_for_bitfield_type_implementation_tokens,
    generate_from_bitfield_type_for_bitfield_implementation_tokens,
};
use crate::generation::new_impl::{
    generate_new_function_tokens, generate_new_without_defaults_function_tokens,
};
use crate::generation::tuple_struct::{
    generate_struct_with_fields_tokens, generate_tuple_struct_tokens,
};
use crate::parsing::bitfield_attribute::{BitOrder, BitfieldAttribute};
use crate::parsing::bitfield_field::{BitfieldField, BitsAttribute, FieldAccess, FieldType};
use crate::parsing::number_parser::{NumberParseError, ParsedNumber, parse_number_string};
use crate::parsing::types::{
    IntegerType, get_bits_from_type, get_integer_suffix_from_integer_type,
    get_integer_type_from_type, get_type_ident, is_custom_field_type, is_size_type,
    is_supported_field_type, is_unsigned_integer_type,
};

/// The `#[bit]` attribute name.
pub(crate) const BIT_ATTRIBUTE_NAME: &str = "bits";

/// The ident prefix for padding fields.
pub(crate) const PADDING_FIELD_NAME_PREFIX: &str = "_";

/// Creates a bitfield for the attributed struct.
///
/// ## Example
///
/// ```ignore
/// use bitfields::bitfield;
///
/// /// All fields in the bitfield must sum up to the number of bits of the bitfield type.
/// #[bitfield(u64)]
/// pub struct Bitfield {
///     /// Fields without bits specified default to the size of the field type.
///     /// 8 bits.
///     u8int: u8,
///     /// A field can have specified bits, but the bits must be greater than zero
///     /// and fit in the field type.
///     #[bitfield(4)] // u8 is 8 bits, so 4 bits is valid.
///     small_u8int: u8,
///     /// A field that is signed, will be sign-extended by the most significant
///     /// bit of its type.
///     signed_int: i8,
///     /// If you specify bits, the field will be sign-extended by the most significant
///     /// bit of the specified bits. In this case, the most significant bit of 4 bits.
///     #[bits(4)]
///     small_signed_int: i8,
///     /// A field can be a bool type.
///     bool_field: bool,
///     /// A field can have a default value, which must fit in the field type.
///     #[bits(default = 0x1F)]
///     field_with_default: u8,
///     /// A field can have a default value and specified bits. The default value
///     /// must fit in the specified bits or a compile error will occur.
///     #[bits(4, default = 0xF)] // Default fits in 4 bits.
///     field_with_bits_default: u8,
///     /// By default, all functions share the same visibility as the bitfield struct.
///     /// Fields can have their getters and setters visibility overridden by specifying
///     /// the visibility of the field.
///     pub pub_field: u8, // Getter and setter are public.
///     /// Nested bitfields are supported, but must have their bits specified.
///     #[bits(3)]
///     nested_field: NestedBitfield,
///     /// Custom types are supported, but must have their bits specified and
///     /// implement the `from_bits` and `into_bits` functions.
///     #[bits(3)]
///     custom_type: CustomType,
///     /// Fields can have their access restricted. `ro` means read-only, meaning
///     /// the field can be read but not written.
///     #[bits(5, access = ro)] // Read-only field, no setter.
///     read_only: u8,
///     /// Fields prefixed with "_" are padding fields, which are inaccessible.
///     #[bits(4, default = 0x3)]
///     _padding: u8,
///     /// Fields with the ignore attribute are ignored.
///     #[bits(99, ignore = true)]
///     ignore_me: u128,
/// }
///
/// #[bitfield(u8)]
/// struct NestedBitfield {
///     field: u8
/// }
///
/// /// Custom types must have 2 const functions, `from_bits` and `into_bits` to convert
/// /// the type to and from bits functions.
/// #[derive(Default)]
/// struct CustomType {
///     a: u8,
/// }
///
/// impl CustomType {
///     /// Make sure the parameter type can fit the specified number of bits. Also,
///     /// must be const, we need that extra compile time safety.
///     const fn from_bits(bits: u8) -> Self {
///         Self {
///             a: bits,
///         }
///     }
///
///     /// Make sure the return type can fit the specified number of bits. Also,
///     /// must be const, we need that extra compile time safety.
///     const fn into_bits(self) -> u8 {
///         self.a
///     }
/// }
///
/// // Usage:
/// // Creates a new bitfield using a builder pattern, unset fields default to 0
/// // or their provided default value.
/// let mut bitfield = BitfieldBuilder::new()
///     .with_u8int(5)
///     .with_small_u8int(0xF)
///     .with_custom_type(CustomType::from_bits(0x3))
///     // .with_custom_type(CustomType::default()) // Can pass a [`CustomType`] instance.
///     // .with_read_only(0x3) // Compile error, read-only field can't be set.
///     // .with__padding(0x3) // Compile error, padding fields are inaccessible.
///     .with_signed_int(-5)
///     .with_small_signed_int(0xF)
///     .build();
///
/// // let bitfield = Bitfield::new(); // Bitfield with default values.
/// // let bitfield = Bitfield::new_without_defaults(); // Bitfield without default values.
/// // let bitfield = BitfieldBuilder::new_without_defaults(); // Builder without defaults.
///
/// // Accessing fields:
/// let u8int = bitfield.u8int(); // Getters
/// let small_u8int = bitfield.small_u8int(); // Signed-types are sign-extended.
/// bitfield.ignore_me; // Ignored fields can be accessed directly.
/// // Setting fields:
/// bitfield.set_u8int(0x3); // Setters
/// bitfield.checked_set_small_u8int(0xF); // Checked setter, error if value overflow bits.
///
/// // Converting to bits:
/// let bits = bitfield.into_bits();
///
/// // Converting from bits:
/// let bitfield = Bitfield::from_bits(0x3); // Converts from bits
/// // let bitfield = Bitfield::from_bits_with_defaults(0x3); // Converts, respects defaults.
///
/// // Constants:
/// assert_eq!(Bitfield::U8INT_BITS, 8); // Number of bits of the field.
/// assert_eq!(Bitfield::U8INT_OFFSET, 0); // The offset of the field in the bitfield.
/// ```
/// ## Features
///
/// ### Bitfield Types
///
/// A bitfield can represent unsigned types (`u8`, `u16`, `u32`, `u64`, `u128`)
/// up to 128-bits, because Rust was weak and stopped at `u128`. The field bits
/// of a bitfield must add up to the number of bits of the bitfield type.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u8)]
/// struct BitFieldU8 {
///     a: u8,
/// }
///
/// #[bitfield(u32)]
/// struct BitFieldU32 {
///     a: u32,
/// }
///
/// #[bitfield(u128)]
/// struct BitFieldU128 {
///     a: u128,
/// }
/// ```
///
/// ### Bitfield Field Types
///
/// A bitfield field can be any unsigned (`u8`, `u16`, `u32`, `u64`, `u128`),
/// signed type (`i8`, `i16`, `i32`, `i64`, `i128`), or a custom type that
/// implements the const functions `from_bits` and `into_bits`. A default value
/// can also be a const variable or a const function. Just be aware that const
/// function and variables defaults lose their compile-time field bits checking.
///
/// Signed types are treated as 2's complement data types, meaning the most
/// significant represents the sign bit. For example, if you had a field with 5
/// bits, the value range would be `-16` to `15`. The more bits you include, the
/// larger the value range.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// const CONST_VAR: u8 = 0x2;
///
/// const fn provide_val() -> u8 {
///     0x1
/// }
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0xFF)]
///     a: u8,
///     #[bits(default = -127)]
///     b: i8,
///     /// Sign-extended by the most significant bit of 4 bits. Also treated as 2's
///     /// complement, meaning this field with 4 bits has the value range of
///     /// `-8` to `7`. You can add more bits to increase this range!
///     #[bits(4, default = 9)]
///     c_sign_extended: i8,
///     #[bits(2, default = CONST_VAR)] // No compile time checks for const variables.
///     const_var_default: u8,
///     #[bits(2, default = provide_val())] // No compile time checks for const functions.
///     const_fn_default: u8, // No compile time checks for const functions.
///    #[bits(8, default = CustomType::C)]
///    custom_type: CustomType
/// }
///
/// #[derive(Debug, PartialEq)]
/// enum CustomType {
///     A = 0,
///     B = 1,
///     C = 2,
/// }
///
/// impl CustomType {
///   const fn from_bits(bits: u8) -> Self {
///       match bits {
///           0 => Self::A,
///           1 => Self::B,
///           2 => Self::C,
///           _ => unreachable!(),
///       }
///    }
///
///    const fn into_bits(self) -> u8 {
///        self as u8
///    }
/// }
///
/// let bitfield = Bitfield::new();
/// assert_eq!(bitfield.a(), 0xFF);
/// assert_eq!(bitfield.b(), -127);
/// assert_eq!(bitfield.c_sign_extended(), -7);
/// assert_eq!(bitfield.const_var_default(), 0x2);
/// assert_eq!(bitfield.const_fn_default(), 0x1);
/// assert_eq!(bitfield.custom_type(), CustomType::C);
/// ```
///
/// ### Constructing a Bitfield
///
/// A bitfield can be constructed using the `new` and `new_without_defaults`
/// constructors. The former initializes the bitfield with default values, while
/// the latter initializes the bitfield without default values, except for
/// padding fields which always keep their default value or 0.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     _d: u8,
/// }
///
/// let bitfield = Bitfield::new();
/// assert_eq!(bitfield.a(), 0x12);
/// assert_eq!(bitfield.b(), 0x34);
/// assert_eq!(bitfield.c(), 0x56);
/// assert_eq!(bitfield.into_bits(), 0x78563412);
///
/// let bitfield_without_defaults = Bitfield::new_without_defaults();
/// assert_eq!(bitfield_without_defaults.a(), 0);
/// assert_eq!(bitfield_without_defaults.b(), 0);
/// assert_eq!(bitfield_without_defaults.c(), 0);
/// assert_eq!(bitfield_without_defaults.into_bits(), 0x78000000);
/// ```
///
/// ### Bitfield Conversions
///
/// A bitfield can be converted from bits using the `from_bits` or
/// `from_bits_with_defaults` functions. The former ignores default values,
/// while the latter respects them. Padding fields are always 0 or their default
/// value. The bitfield can also be converted to bits using the `into_bits`
/// function. The `From` trait is also implemented between the bitfield and the
/// bitfield type and operates the same as `from_bits`.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// #[derive(Copy, Clone)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(8)]
///     b: CustomType,
///     c: u8,
///     #[bits(default = 0x78)]
///     _d: u8,
/// }
///
/// #[derive(Debug, PartialEq)]
/// enum CustomType {
///     A = 0,
///     B = 1,
///     C = 2,
/// }
///
/// impl CustomType {
///     const fn from_bits(bits: u8) -> Self {
///         match bits {
///             1 => Self::A,
///             2 => Self::B,
///             3 => Self::C,
///             _ => Self::A,
///         }
///     }
///
///     const fn into_bits(self) -> u8 {
///         self as u8
///     }
/// }
///
/// let bitfield = Bitfield::from_bits(0x11223344);
/// assert_eq!(bitfield.a(), 0x44);
/// assert_eq!(bitfield.b(), CustomType::A);
/// assert_eq!(bitfield.c(), 0x22);
/// let val = bitfield.into_bits();
/// assert_eq!(val, 0x78220044);
///
/// let bitfield_respect_defaults = Bitfield::from_bits_with_defaults(0x11223344);
/// assert_eq!(bitfield_respect_defaults.a(), 0x12); // Default value respected
/// assert_eq!(bitfield_respect_defaults.b(), CustomType::A);
/// assert_eq!(bitfield_respect_defaults.c(), 0x22);
/// let val = bitfield_respect_defaults.into_bits();
/// assert_eq!(val, 0x78220012);
///
/// // From trait
/// let val: u32 = bitfield.into();
/// assert_eq!(val, 0x78220044);
/// let bitfield: Bitfield = val.into();
/// assert_eq!(bitfield.into_bits(), 0x78220044);
/// ```
///
/// ### Conversion Endianess
///
/// Sometimes the outside world is outside our control, like how systems store
/// or expect data endian. Luckily, the endian of the bitfield conversions can
/// be controlled by specifying the `#[bitfield(from_endian = x, into_endian =
/// x)]` args. The possible endians are `little` or `big`. By default, the
/// endian of both is `big`.
///
/// ````ignore
/// use bitfields::bitfield;
///
/// // We are working with a system that stores data in little-endian, we
/// // set the from_endian to little for the proper representation.
/// //
/// // The system expects the data it stores in big-endian, we set the
/// // into_endian to big-endian for converting into the proper representation.
/// #[bitfield(u32, from_endian = little, into_endian = big)]
/// pub struct Bitfield {
///     a: u8,
///     b: u8,
///     c: u8,
///     d: u8,
/// }
///
/// // The host device stored the data 0x12345678 in little-endian memory
/// // as [0x78, 0x56, 0x34, 0x12].
/// let bitfield = Bitfield::from_bits(0x78563412);
///
/// assert_eq!(bitfield.a(), 0x78);
/// assert_eq!(bitfield.b(), 0x56);
/// assert_eq!(bitfield.c(), 0x34);
/// assert_eq!(bitfield.d(), 0x12);
/// assert_eq!(bitfield.into_bits(), 0x12345678);
/// ````
///
/// ### Field Order
///
/// By default, fields are ordered from the least significant bit (lsb) to the
/// most significant bit (msb). The order can be changed by specifying the
/// `#[bitfield(order = x)]` arg on the bitfield struct. There are two field
/// orderings, `lsb` and `msb`.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u32, order = msb)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     d: u8,
/// }
///
/// let bitfield = Bitfield::new();
/// assert_eq!(bitfield.a(), 0x12);
/// assert_eq!(bitfield.b(), 0x34);
/// assert_eq!(bitfield.c(), 0x56);
/// assert_eq!(bitfield.d(), 0x78);
/// let val = bitfield.into_bits();
///
/// //                .- a
/// //                |    .- b
/// //                |    | .- c
/// //                |    | |  .- d
/// assert_eq!(val, 0x12_34_56_78);
/// assert_eq!(Bitfield::A_OFFSET, 24); // Offset of the a field in the bitfield.
/// ```
///
/// ### Field Access
///
/// Field access can be controlled by specifying the `#[bits(access = x)]` arg
/// on a field. There are four accesses:
/// - `rw` - Read and write access (default)
/// - `ro` - Read-only access.
/// - `wo` - Write-only access.
/// - `none` - No access.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     read_write: u8,
///     #[bits(access = ro)]
///     read_only: u8,
///     #[bits(access = wo)]
///     write_only: u8,
///     #[bits(default = 0xFF, access = none)]
///     none: u8,
/// }
///
/// let mut bitfield = BitfieldBuilder::new()
///     .with_read_write(0x12)
///     // .with_read_only(0x34) // Compile error, read-only field can't be set.
///     .with_write_only(0x56)
///     // .with_none(0x78) // Compile error, none field can't be set.
///     .build();
/// bitfield.set_read_write(0x12);
/// // bitfield.set_read_only(0x34); // Compile error, read-only field can't be set.
/// bitfield.set_write_only(0x56);
/// // bitfield.set_none(0x78); // Compile error, none field can't be set.
///
/// assert_eq!(bitfield.read_write(), 0x12);
/// assert_eq!(bitfield.read_only(), 0);
/// // assert_eq!(bitfield.write_only(), 0x56); // Compile error, write-only can't be read.
/// // assert_eq!(bitfield.none(), 0xFF); // Compile error, none field can't be accessed.
/// assert_eq!(bitfield.into_bits(), 0xFF560012); // All fields exposed when converted to bits.
/// ```
///
/// ### Checked Setters
///
/// Normally, when a field is set, the value is truncated to the number of bits
/// of the field. Fields also have checked setters that returns an error if the
/// value overflows the number of bits of the field.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// struct Bitfield {
///     a: u8,
///     #[bits(4)]
///     b: u8,
///     #[bits(4)]
///     _padding: u8,
/// }
///
/// let mut bitfield = Bitfield::new();
/// bitfield.set_a(0xFF);
/// bitfield.set_b(0x12); // Truncated to 4 bits.
/// assert_eq!(bitfield.a(), 0xFF);
/// assert_eq!(bitfield.b(), 0x2);
///
/// let res = bitfield.checked_set_b(0x12); // Error, value overflows bits.
/// assert!(res.is_err());
/// ```
///
/// ### Bit Operations
///
/// Individual bits can be get or set using the `get_bit` and `set_bit`
/// functions. They can be enabled using the bitfield attribute arg For
/// `get_bit`, if the bit is  out-of-bounds or the field doesn't have write
/// access, `false` is returned. There is a checked version `checked_get_bit`
/// that return an error instead. Similarly, for `set_bit`, if the bit is
/// out-of-bounds or the  field doesn't have write access, the operation is
/// no-op. There is a checked version `checked_set_bit` that returns an error
/// instead.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u8, bit_ops = true)]
/// #[derive(Copy, Clone)]
/// pub struct Bitfield {
///     #[bits(2, default = 0b11)]
///     a: u8,
///     #[bits(2, default = 0b00)]
///     b: u8,
///     #[bits(2, default = 0b10, access = wo)]
///     c: u8,
///     #[bits(2, default = 0b01)]
///     _d: u8,
/// }
///
/// let bitfield = Bitfield::new();
///
/// assert!(bitfield.get_bit(0));
/// assert!(bitfield.get_bit(1));
/// assert!(!bitfield.get_bit(2));
/// assert!(!bitfield.get_bit(3));
/// assert!(bitfield.get_bit(4)); // No write access, false is returned.
/// assert!(bitfield.get_bit(5)); // No write access, false is returned.
/// assert!(bitfield.checked_get_bit(4).is_err()); // No write access, err.
/// assert!(bitfield.checked_get_bit(5).is_err()); // No write access, err.
/// assert!(bitfield.get_bit(6));
/// assert!(!bitfield.get_bit(7));
/// assert!(bitfield.get_bit(50)); // Out-of-bounds, false is returned.
/// assert!(bitfield.checked_get_bit(50).is_err()); // Out-of-bounds, err.
/// ```
///
/// ```ignore
/// #[bitfield(u8, bit_ops = true)]
/// #[derive(Copy, Clone)]
/// pub struct Bitfield {
///     #[bits(2)]
///     a: u8,
///     #[bits(2, default = 0b11)]
///     b: u8,
///     #[bits(2, default = 0b11, access = ro)]
///     c: u8,
///     #[bits(2, default = 0b00)]
///     _d: u8,
/// }
///
/// let mut bitfield = Bitfield::new();
///
/// bitfield.set_bit(0, true);
/// bitfield.set_bit(1, true);
/// bitfield.set_bit(2, false);
/// bitfield.set_bit(3, false);
/// bitfield.set_bit(4, false); // No-op, no write access.
/// bitfield.set_bit(5, false); // No-op, no write access.
/// assert!(bitfield.checked_set_bit(4, false).is_err()); // Error, no write access.
/// assert!(bitfield.checked_set_bit(5, false).is_err()); // Error, no write access.
/// bitfield.set_bit(6, true); // No-op, padding.
/// bitfield.set_bit(7, true); // No-op, padding.
/// assert!(bitfield.checked_set_bit(4, false).is_err()); // Error, padding.
/// assert!(bitfield.checked_set_bit(5, false).is_err()); // Error, padding..
/// assert_eq!(bitfield.into_bits(), 0b110011);
/// ```
///
/// ### Padding Fields
///
/// Fields prefixed with an underscore `_` are padding fields, which are
/// inaccessible. Meaning the field is always 0/false or a default value. They
/// are useful for padding the bits of the bitfield.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// struct Bitfield {
///     a: u8,
///     #[bits(default = 0xFF)]
///     _padding: u8, // Fills the remaining bits of the u16.
/// }
///
/// let bitfield = Bitfield::new();
/// assert_eq!(bitfield.a(), 0);
/// // assert_eq!(bitfield._padding(), 0xFF00); // Compile error, padding inaccessible.
/// // bitfield.set__padding(0xFF); // Compile error, padding fields are inaccessible.
/// assert_eq!(bitfield.into_bits(), 0xFF00); // All fields exposed when converted to bits.
/// ```
///
/// ### Ignored Fields
///
/// Fields with the `#[bits(ignore = true)` attribute are ignored and not
/// included in the bitfield. This is useful for when you are building a custom
/// bitfield, but want to include certain fields that aren't a part of the
/// bitfield without wrapping having to wrap bitfield is a parent struct. All
/// ignored fields must implement the `Default` trait. Ignored fields
/// are accessible directly like normal struct fields.
///
///```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// struct Bitfield {
///    a: u8,
///    b: u8,
///    #[bits(ignore = true)] // Ignored field.
///    field_id: u8,
///    #[bits(ignore = true)] // Ignored field.
///    field_custom: CustomType,
/// }
///
/// #[derive(Debug, Default, PartialEq)]
/// enum CustomType {
///    #[default]
///    A,
///    B,
/// }
///
/// let bitfield = Bitfield::new();
///
/// assert_eq!(bitfield.field_id, 0); // Ignored fields can be accessed directly.
/// assert_eq!(bitfield.field_custom, CustomType::A); // Ignored fields can be accessed directly.
/// ```
///
/// ### Field Constants
///
/// Fields with read or write access have constants generated for their number
/// of bits and offset in the bitfield.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     d: u8,
/// }
///
/// assert_eq!(Bitfield::A_BITS, 8); // Number of bits of the  afield.
/// assert_eq!(Bitfield::A_OFFSET, 0); // The offset of the a field in the bitfield.
/// assert_eq!(Bitfield::B_BITS, 8); // Number of bits of the b field.
/// assert_eq!(Bitfield::B_OFFSET, 8); // The offset of the b field in the bitfield.
/// assert_eq!(Bitfield::C_BITS, 8); // Number of bits of c the field.
/// assert_eq!(Bitfield::C_OFFSET, 16); // The offset of the c field in the bitfield.
/// assert_eq!(Bitfield::D_BITS, 8); // Number of bits of the d field.
/// assert_eq!(Bitfield::D_OFFSET, 24); // The offset of the d field in the bitfield.
/// ```
///
/// ### Debug Implementation
///
/// A debug implementation is generated for the bitfield, which prints the
/// fields and their values.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     d: u8,
/// }
///
/// let bitfield = Bitfield::new();
///
/// assert_eq!(format!("{:?}", bitfield), "Bitfield { d: 120, c: 86, b: 52, a: 18 }");
/// ```
///
/// ### Passing Attributes
///
/// Attributes below the `#[bitfield]` attribute are passed to the generated
/// struct.
///
/// ```ignore
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// #[derive(Copy, Clone)]
/// struct Bitfield {
///     a: u32,
/// }
/// ```
///
/// ### Complete Generation Control
///
/// You have complete control over what gets generated by the bitfield macro.
/// When your deploying to a resource-constrained environment, you can generate
/// only the necessary functions or implementations. You can disable generation
/// by passing `false` to its attribute arg.
///
/// The `#[bitfield]` args that control generation are:
///
/// - `#[bitfield(new = true)]` - Generates the `new` and `new_without_defaults`
///   constructor.
/// - `#[bitfield(from_bits = true)]` - Generates the `from_bits` and
///   `from_bits_with_defaults` functions.
/// - `#[bitfield(into_bits = true)]` - Generates the `into_bits` function.
/// - `#[bitfield(from = true)]` - Generates the `From` trait implementation.
/// - `#[bitfield(debug = true)]` - Generates the `Debug` trait implementation.
/// - `#[bitfield(default = true)]` - Generates the `Default` trait
///   implementation
/// - `#[bitfield(builder = true)]` - Generates the builder implementation.
/// - `#[bitfield(bit_ops = true)]` - Generates the bit operations
///   implementation.
#[proc_macro_attribute]
pub fn bitfield(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match parse_bitfield(args.into(), input.into()) {
        Ok(res) => res.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

/// Parses the bitfield attribute, struct, and fields.
fn parse_bitfield(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    // Parse the struct tokens
    let struct_tokens = syn::parse2::<syn::ItemStruct>(input.clone())?;

    // Parse the arguments of the '#[bitfield(arg, arg)]' attribute
    let bitfield_attribute: BitfieldAttribute = match syn::parse2(args) {
        Ok(bitfield_attribute) => bitfield_attribute,
        Err(err) => {
            return Err(create_syn_error(input.span(), err.to_string()));
        }
    };

    // Check if the bitfield type can contain the fields.
    let all_fields = parse_fields(&bitfield_attribute, &struct_tokens)?;
    let fields = all_fields.0;
    let ignored_fields = all_fields.1;
    check_bitfield_type_contain_field_bits(&bitfield_attribute, &fields)?;
    check_bitfield_names_unique(&fields)?;

    // Generate the bitfield functions.
    generate_functions(&bitfield_attribute, &fields, &ignored_fields, &struct_tokens)
}

/// Check if the bitfield type can contain the field bits.
fn check_bitfield_type_contain_field_bits(
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
) -> syn::Result<()> {
    let total_field_bits = fields.iter().map(|field| field.bits).sum::<u8>();

    match total_field_bits.cmp(&bitfield_attribute.bits) {
        Ordering::Greater => Err(create_syn_error(
            bitfield_attribute.ty.span(),
            format!(
                "The total number of bits of the fields ({} bits) is greater than the number of bits of the bitfield type '{}' ({} bits).",
                total_field_bits,
                get_type_ident(&bitfield_attribute.ty).unwrap(),
                bitfield_attribute.bits
            ),
        )),
        Ordering::Less => {
            let remaining_bits = bitfield_attribute.bits - total_field_bits;
            Err(create_syn_error(
                bitfield_attribute.ty.span(),
                format!(
                    "The total number of bits of the fields ({} bits) is less than the number of bits of the bitfield type '{}' ({} bits), you can add a padding field (prefixed with '_') to fill the remaining '{} bits'.",
                    total_field_bits,
                    get_type_ident(&bitfield_attribute.ty).unwrap(),
                    bitfield_attribute.bits,
                    remaining_bits,
                ),
            ))
        }
        Ordering::Equal => {
            // The total number of bits of all fields is equal to the number of bits, we're
            // good.
            Ok(())
        }
    }
}

fn check_bitfield_names_unique(fields: &[BitfieldField]) -> syn::Result<()> {
    let mut field_names = Vec::new();
    for field in fields {
        if field_names.contains(&field.name) {
            return Err(create_syn_error(
                field.name.span(),
                format!(
                    "The field name '{}' is duplicated, each field must have a unique name.",
                    field.name
                ),
            ));
        }
        if !field.padding {
            field_names.push(field.name.clone());
        }
    }

    Ok(())
}

/// Parses all the fields into a list of [`BitfieldField`]s.
fn parse_fields(
    bitfield_attribute: &BitfieldAttribute,
    struct_tokens: &syn::ItemStruct,
) -> syn::Result<(Vec<BitfieldField>, Vec<BitfieldField>)> {
    let fields_tokens = match &struct_tokens.fields {
        Fields::Named(named_files) => named_files,
        _ => {
            return Err(create_syn_error(
                struct_tokens.span(),
                "Non-named fields are not supported.",
            ));
        }
    };

    let mut fields = Vec::new();
    let mut ignored_fields = Vec::new();
    for field_token in fields_tokens.named.clone() {
        let field = do_parse_field(bitfield_attribute, field_token, &fields)?;
        if field.ignore {
            ignored_fields.push(field);
        } else {
            fields.push(field);
        }
    }

    Ok((fields, ignored_fields))
}

/// Internal implementation of [`parse_fields`] to parse a single field.
fn do_parse_field(
    bitfield_attribute: &BitfieldAttribute,
    field_tokens: syn::Field,
    prev_fields: &[BitfieldField],
) -> syn::Result<BitfieldField> {
    // Parse field attribute, a field could have multiple attributes, but we only
    // care about our 'bits' attribute.
    let field_bit_attribute = field_tokens.attrs.iter().find(|attr| {
        attr.path().is_ident(BIT_ATTRIBUTE_NAME) && attr.style == syn::AttrStyle::Outer
    });

    let visibility = match field_tokens.vis {
        // Pass the visibility to the field.
        Visibility::Public(_) | Visibility::Restricted(_) => Some(field_tokens.vis.clone()),
        // Use the visibility of the struct
        Visibility::Inherited => None,
    };

    let field_type = if is_custom_field_type(&field_tokens.ty) {
        FieldType::CustomFieldType
    } else {
        FieldType::IntegerFieldType
    };

    let padding =
        field_tokens.ident.clone().unwrap().to_string().starts_with(PADDING_FIELD_NAME_PREFIX);

    let bitfield = if field_bit_attribute.is_none() {
        if !is_supported_field_type(&field_tokens.ty) {
            return Err(create_syn_error(
                field_tokens.span(),
                format!(
                    "The field type {:?} is not supported.",
                    get_type_ident(&field_tokens.ty).unwrap()
                ),
            ));
        }

        // We have to determine the number of bits from the field type since there's no
        // '#[bits]' attribute.
        if is_size_type(&field_tokens.ty) {
            return Err(create_syn_error(
                field_tokens.span(),
                "The types isize and usize require a bit size, otherwise we can't determine the size of the field.",
            ));
        }

        if field_type != FieldType::IntegerFieldType {
            return Err(create_syn_error(
                field_tokens.span(),
                "Custom and nested field types require a defined bit size, otherwise we can't determine the size of the field.",
            ));
        }

        let bits = get_bits_from_type(&field_tokens.ty)?;
        let offset = calculate_field_offset(bits, bitfield_attribute, prev_fields)?;
        let access = if padding { FieldAccess::None } else { FieldAccess::ReadWrite };

        // Create a bitfield field with default values since we don't have one to
        // parse.
        BitfieldField {
            name: field_tokens.ident.unwrap(),
            ty: field_tokens.ty.clone(),
            vis: visibility,
            bits,
            offset,
            default_value_tokens: None,
            unsigned: true,
            padding,
            access,
            field_type: FieldType::IntegerFieldType,
            ignore: false,
        }
    } else {
        let bit_attribute_tokens = match &field_bit_attribute.unwrap().meta {
            Meta::List(list) => list,
            _ => {
                return Err(create_syn_error(
                    field_tokens.span(),
                    "The '#[bits]' attribute must be a list.",
                ));
            }
        };

        let bits_attribute: BitsAttribute = syn::parse2(bit_attribute_tokens.tokens.clone())?;

        if bits_attribute.ignore {
            return Ok(BitfieldField {
                ty: field_tokens.ty.clone(),
                vis: Some(field_tokens.vis),
                bits: 0,
                offset: 0,
                default_value_tokens: None,
                unsigned: false,
                padding,
                access: FieldAccess::ReadOnly,
                name: field_tokens.ident.unwrap(),
                ignore: true,
                field_type,
            });
        }

        if !is_supported_field_type(&field_tokens.ty) {
            return Err(create_syn_error(
                field_tokens.span(),
                format!(
                    "The field type {:?} is not supported.",
                    get_type_ident(&field_tokens.ty).unwrap()
                ),
            ));
        }

        let bits = match bits_attribute.bits {
            Some(bits) => {
                // Make sure the type of the field can contain the specified number of bits if
                // not a custom type.
                if field_type == FieldType::IntegerFieldType
                    && bits > get_bits_from_type(&field_tokens.ty)?
                {
                    return Err(create_syn_error(
                        bit_attribute_tokens.span(),
                        format!(
                            "The field type {:?} ({} bits) is too small to hold the specified '{} bits'.",
                            get_type_ident(&field_tokens.ty).unwrap(),
                            get_bits_from_type(&field_tokens.ty)?,
                            bits
                        ),
                    ));
                }

                bits
            }
            None => {
                if field_type != FieldType::IntegerFieldType {
                    return Err(create_syn_error(
                        field_tokens.span(),
                        "Custom and nested field types require a defined bit size, otherwise we can't determine the size of the field.",
                    ));
                }

                get_bits_from_type(&field_tokens.ty)?
            }
        };

        // Make sure the field bits are greater than 0.
        if bits == 0 {
            return Err(create_syn_error(
                bit_attribute_tokens.span(),
                "The field bits must be greater than 0.",
            ));
        }

        // Make sure the default value is within the field bits. If a number was unable
        // to be parsed, let's take a chance and see if the user is trying to
        // use a const variable or a const function.
        let parsed_number = if field_type == FieldType::IntegerFieldType
            && bits_attribute.clone().default_value_expr.is_some()
        {
            check_default_value_fit_in_field(
                &bits_attribute.clone().default_value_expr.unwrap(),
                bits,
                field_tokens.ty.clone(),
            )?
        } else {
            None
        };

        let unsigned =
            field_type != FieldType::IntegerFieldType || is_unsigned_integer_type(&field_tokens.ty);
        let access = if padding {
            if bits_attribute.access.is_some() {
                return Err(create_syn_error(
                    bit_attribute_tokens.span(),
                    "Padding fields can't have a specified access.",
                ));
            }

            FieldAccess::None
        } else {
            bits_attribute.access.unwrap_or(FieldAccess::ReadWrite)
        };
        let offset = calculate_field_offset(bits, bitfield_attribute, prev_fields)?;

        let default_value_tokens = match bits_attribute.default_value_expr {
            None => None,
            Some(ref expr) => {
                // We want to add integer literals to default values expressions if the
                // expression is a negative number without a suffix. We do alot of casting
                // so what happens is, if there is the default value expr `-125`, when we
                // try to cast later like `-125 as u8`, Rust will complain that the number
                // is too large for the type. Adding the integer suffix will fix this since
                // Rust will know the type of the number and will cast it.
                if unsigned
                    || field_type != FieldType::IntegerFieldType
                    || parsed_number.is_none()
                    || parsed_number.unwrap().has_integer_suffix
                {
                    Some(quote! {
                        #expr
                    })
                } else {
                    let tokens =
                        add_integer_literals_to_expr(&expr.clone(), field_tokens.ty.clone())?;

                    Some(quote! {
                        #tokens
                    })
                }
            }
        };

        BitfieldField {
            name: field_tokens.ident.unwrap(),
            ty: field_tokens.ty.clone(),
            vis: visibility,
            bits,
            offset,
            default_value_tokens,
            unsigned,
            padding,
            access,
            field_type,
            ignore: false,
        }
    };

    Ok(bitfield)
}

/// Checks if the default value can fit in the field bits.
fn check_default_value_fit_in_field(
    default_value_expr: &Expr,
    bits: u8,
    field_type: Type,
) -> syn::Result<Option<ParsedNumber>> {
    let default_value_str = &quote!(#default_value_expr).to_string();

    let parsed_number = match parse_number_string(default_value_str) {
        Ok(number) => number,
        Err(err) => {
            return match err {
                NumberParseError::FloatNotSupported => Err(create_syn_error(
                    default_value_expr.span(),
                    "Floats are not supported as default values.".to_string(),
                )),
                // Maybe the user is trying to use a const variable or a const
                // function call as a default.
                NumberParseError::InvalidNumberString => Ok(None),
            };
        }
    };

    let bits_max_value = 1 << bits as u128;
    if parsed_number.number >= bits_max_value {
        if parsed_number.negative {
            return Err(create_syn_error(
                default_value_expr.span(),
                format!(
                    "The default value -'{}' is too large to fit into the specified '{} bits'.",
                    parsed_number.number, bits,
                ),
            ));
        }
        return Err(create_syn_error(
            default_value_expr.span(),
            format!(
                "The default value '{}' is too large to fit into the specified '{} bits'.",
                parsed_number.number, bits,
            ),
        ));
    }

    let default_value_too_big_for_type = match get_integer_type_from_type(&field_type) {
        IntegerType::Bool => parsed_number.number > 1,
        IntegerType::U8 => parsed_number.number > u8::MAX as u128,
        IntegerType::U16 => parsed_number.number > u16::MAX as u128,
        IntegerType::U32 => parsed_number.number > u32::MAX as u128,
        IntegerType::U64 => parsed_number.number > u64::MAX as u128,
        IntegerType::U128 => {
            // Unable to happen, this is Rust's max unsigned type value.
            false
        }
        IntegerType::Usize => parsed_number.number > usize::MAX as u128,
        IntegerType::Isize => {
            if parsed_number.negative {
                parsed_number.number > isize::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > isize::MAX as u128
            }
        }
        IntegerType::I8 => {
            if parsed_number.negative {
                parsed_number.number > i8::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > i8::MAX as u128
            }
        }
        IntegerType::I16 => {
            if parsed_number.negative {
                parsed_number.number > i16::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > i16::MAX as u128
            }
        }
        IntegerType::I32 => {
            if parsed_number.negative {
                parsed_number.number > i32::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > i32::MAX as u128
            }
        }
        IntegerType::I64 => {
            if parsed_number.negative {
                parsed_number.number > i64::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > i64::MAX as u128
            }
        }
        IntegerType::I128 => {
            if parsed_number.negative {
                parsed_number.number > i128::MIN.unsigned_abs()
            } else {
                parsed_number.number > i128::MAX as u128
            }
        }
        _ => Err(create_syn_error(default_value_expr.span(), PANIC_ERROR_MESSAGE))?,
    };

    if default_value_too_big_for_type {
        let negative_str = if parsed_number.negative { "-" } else { "" };
        return Err(create_syn_error(
            default_value_expr.span(),
            format!(
                "The default value '{}{}' is too large to fit into the field type '{}'.",
                negative_str,
                parsed_number.number,
                get_type_ident(&field_type).unwrap()
            ),
        ));
    }

    Ok(Some(parsed_number))
}

/// Calculate the offset of a field based on previous fields.
fn calculate_field_offset(
    bits: u8,
    bitfield_attribute: &BitfieldAttribute,
    prev_fields: &[BitfieldField],
) -> syn::Result<u8> {
    let offset = prev_fields.iter().map(|field| field.bits).sum::<u8>();

    match bitfield_attribute.bit_order {
        BitOrder::Lsb => Ok(offset),
        BitOrder::Msb => {
            let bitfield_type_bits = get_bits_from_type(&bitfield_attribute.ty)?;
            // We calculate offset starting from the left. There's a chance that
            // the total bits of all fields is greater than the number of bits
            // of the bitfield type. We will catch it later so
            // we can ignore for now.
            if offset + bits < bitfield_type_bits {
                Ok(bitfield_type_bits - bits - offset)
            } else {
                // We've underflow the bitfield type, this will be caught later.
                Ok(0)
            }
        }
    }
}

/// Adds the field type integer literal suffix to the expression.
///
/// For example, if the expression is '-1' and the field type is 'i8', the
/// expression will be updated to '1i8'.
fn add_integer_literals_to_expr(expr: &Expr, field_type: Type) -> syn::Result<TokenStream> {
    let updated_expr = if let Expr::Unary(unary) = expr {
        let attrs = unary.attrs.clone();
        let op = unary.op;

        let updated_expr = if let Expr::Lit(expr_lit) = *unary.expr.clone() {
            let new_lit = create_expr_lit_with_integer_suffix(&expr_lit, field_type)?;

            Expr::Lit(ExprLit { attrs: expr_lit.attrs, lit: new_lit.lit })
        } else {
            Err(create_syn_error(expr.span(), PANIC_ERROR_MESSAGE))?
        };

        Expr::Unary(ExprUnary { attrs, op, expr: Box::new(updated_expr) })
    } else if let Expr::Lit(expr_lit) = expr {
        let new_lit = create_expr_lit_with_integer_suffix(expr_lit, field_type)?;

        Expr::Lit(ExprLit { attrs: expr_lit.clone().attrs, lit: new_lit.lit })
    } else {
        Err(create_syn_error(expr.span(), PANIC_ERROR_MESSAGE))?
    };

    Ok(quote! {
        #updated_expr
    })
}

/// Helper for creating an integer literal with the integer suffix.
fn create_expr_lit_with_integer_suffix(lit: &ExprLit, field_type: Type) -> syn::Result<ExprLit> {
    let integer_type = get_integer_type_from_type(&field_type);
    let integer_suffix = get_integer_suffix_from_integer_type(integer_type)?;

    let new_lit = match lit.lit.clone() {
        Lit::Int(lit_int) => {
            let new_lit_int =
                LitInt::new(&format!("{}{}", lit_int.token(), integer_suffix), lit_int.span());
            ExprLit { attrs: lit.attrs.clone(), lit: Lit::Int(new_lit_int) }
        }
        _ => Err(create_syn_error(lit.span(), PANIC_ERROR_MESSAGE))?,
    };

    Ok(new_lit)
}

/// Generate the bitfield functions.
fn generate_functions(
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    struct_tokens: &syn::ItemStruct,
) -> syn::Result<TokenStream> {
    let struct_attributes: TokenStream =
        struct_tokens.attrs.iter().map(ToTokens::to_token_stream).collect();
    let struct_name = &struct_tokens.ident;

    let bitfield_struct = if !ignored_fields.is_empty() {
        generate_struct_with_fields_tokens(
            struct_name.clone(),
            struct_tokens.vis.clone(),
            bitfield_attribute.ty.clone(),
            ignored_fields,
        )
    } else {
        generate_tuple_struct_tokens(
            struct_name.clone(),
            struct_tokens.vis.clone(),
            bitfield_attribute.ty.clone(),
        )
    };
    let new_function = bitfield_attribute.generate_new_func.then(|| {
        generate_new_function_tokens(
            struct_tokens.vis.clone(),
            fields,
            ignored_fields,
            &bitfield_attribute.ty,
        )
    });
    let new_without_defaults_function = bitfield_attribute.generate_new_func.then(|| {
        generate_new_without_defaults_function_tokens(
            struct_tokens.vis.clone(),
            fields,
            ignored_fields,
            &bitfield_attribute.ty,
        )
    });
    let from_bits_function = bitfield_attribute.generate_from_bits_func.then(|| {
        generate_from_bits_function_tokens(
            struct_tokens.vis.clone(),
            fields,
            ignored_fields,
            &bitfield_attribute.ty,
            bitfield_attribute,
        )
    });
    let from_bits_with_defaults_function = bitfield_attribute.generate_from_bits_func.then(|| {
        generate_from_bits_with_defaults_function_tokens(
            struct_tokens.vis.clone(),
            fields,
            &bitfield_attribute.ty,
            bitfield_attribute,
            !ignored_fields.is_empty(),
        )
    });
    let generate_into_bits_function = bitfield_attribute.generate_into_bits_func.then(|| {
        generate_into_bits_function_tokens(
            struct_tokens.vis.clone(),
            bitfield_attribute,
            !ignored_fields.is_empty(),
        )
    });
    let field_consts_tokens = generate_field_constants_tokens(struct_tokens.vis.clone(), fields);
    let field_getters_tokens = generate_field_getters_functions_tokens(
        struct_tokens.vis.clone(),
        &bitfield_attribute.ty,
        fields,
        !ignored_fields.is_empty(),
    )?;
    let field_setters_tokens = generate_field_setters_functions_tokens(
        struct_tokens.vis.clone(),
        &bitfield_attribute.ty,
        fields,
        !ignored_fields.is_empty(),
    );
    let default_function = bitfield_attribute.generate_default_impl.then(|| {
        generate_default_implementation_tokens(
            struct_name.clone(),
            &bitfield_attribute.ty,
            fields,
            ignored_fields,
        )
    });
    let builder_tokens = bitfield_attribute.generate_builder.then(|| {
        generate_builder_tokens(
            struct_tokens.vis.clone(),
            &bitfield_attribute.ty,
            struct_name.clone(),
            fields,
            ignored_fields,
        )
    });

    let from_bitfield_type_for_bitfield_function_tokens =
        bitfield_attribute.generate_from_trait_funcs.then(|| {
            generate_from_bitfield_type_for_bitfield_implementation_tokens(
                struct_name.clone(),
                fields,
                ignored_fields,
                &bitfield_attribute.ty,
            )
        });
    let from_bitfield_for_bitfield_type_function_tokens =
        bitfield_attribute.generate_from_trait_funcs.then(|| {
            generate_from_bitfield_for_bitfield_type_implementation_tokens(
                struct_name.clone(),
                bitfield_attribute,
                !ignored_fields.is_empty(),
            )
        });
    let debug_impl = bitfield_attribute.generate_debug_impl.then(|| {
        generate_debug_implementation(
            struct_name.clone(),
            bitfield_attribute,
            fields,
            !ignored_fields.is_empty(),
        )
    });
    let get_bit_operations = bitfield_attribute.generate_bit_ops.then(|| {
        generate_get_bit_tokens(
            struct_tokens.vis.clone(),
            &bitfield_attribute.ty,
            fields,
            !ignored_fields.is_empty(),
        )
    });
    let set_bit_operations = bitfield_attribute.generate_bit_ops.then(|| {
        generate_set_bit_tokens(
            struct_tokens.vis.clone(),
            &bitfield_attribute.ty,
            fields,
            !ignored_fields.is_empty(),
        )
    });
    let default_attrs = if ignored_fields.is_empty() {
        quote! {
            #[repr(transparent)]
        }
    } else {
        quote! {
            #[repr(C)]
        }
    };

    Ok(quote! {
        #struct_attributes
        #default_attrs
        #bitfield_struct

        impl #struct_name {
            #new_function
            #new_without_defaults_function

            #from_bits_function
            #from_bits_with_defaults_function

            #generate_into_bits_function

            #field_consts_tokens
            #field_getters_tokens
            #field_setters_tokens

            #get_bit_operations
            #set_bit_operations
        }

        #default_function

        #builder_tokens

        #from_bitfield_type_for_bitfield_function_tokens
        #from_bitfield_for_bitfield_type_function_tokens

        #debug_impl
    })
}

/// Creates a syn error with the specified message that occurred at the
/// specified span.
pub(crate) fn create_syn_error(span: proc_macro2::Span, msg: impl fmt::Display) -> syn::Error {
    syn::Error::new(span, msg)
}
