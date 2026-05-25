//! Module for parsing only the arguments of the `#[bitfield]` attribute.

use std::collections::HashSet;
use std::str::FromStr;

use getset::CopyGetters;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use syn::parse::{Parse, ParseStream};

use crate::parsing::common::attribute_argument_parser::{
    parse_attribute_arguments, parse_boolean_attribute_argument,
};
use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::conversion_endian::{ConversionEndian, resolve_endian_feature};

const ORDER_LSB_FEATURE_ENABLED: bool = cfg!(feature = "order_lsb");
const ORDER_MSB_FEATURE_ENABLED: bool = cfg!(feature = "order_msb");

const FROM_ENDIAN_LITTLE_FEATURE_ENABLED: bool = cfg!(feature = "from_endian_little");
const FROM_ENDIAN_BIG_FEATURE_ENABLED: bool = cfg!(feature = "from_endian_big");

const INTO_ENDIAN_LITTLE_FEATURE_ENABLED: bool = cfg!(feature = "into_endian_little");
const INTO_ENDIAN_BIG_FEATURE_ENABLED: bool = cfg!(feature = "into_endian_big");

const WRITE_ENDIAN_LITTLE_FEATURE_ENABLED: bool = cfg!(feature = "write_endian_little");
const WRITE_ENDIAN_BIG_FEATURE_ENABLED: bool = cfg!(feature = "write_endian_big");

const GENERATE_NEW_FEATURE_ENABLED: bool = cfg!(feature = "generate_new");
const DISABLE_NEW_FEATURE_ENABLED: bool = cfg!(feature = "disable_new");

const GENERATE_FROM_INTO_BITS_FEATURE_ENABLED: bool = cfg!(feature = "generate_from_into_bits");
const DISABLE_FROM_INTO_BITS_FEATURE_ENABLED: bool = cfg!(feature = "disable_from_into_bits");

const GENERATE_FROM_TRAITS_FEATURE_ENABLED: bool = cfg!(feature = "generate_from_traits");
const DISABLE_FROM_TRAITS_FEATURE_ENABLED: bool = cfg!(feature = "disable_from_traits");

const GENERATE_DEFAULT_FEATURE_ENABLED: bool = cfg!(feature = "generate_default");
const DISABLE_DEFAULT_FEATURE_ENABLED: bool = cfg!(feature = "disable_default");

const GENERATE_DEBUG_FEATURE_ENABLED: bool = cfg!(feature = "generate_debug");
const DISABLE_DEBUG_FEATURE_ENABLED: bool = cfg!(feature = "disable_debug");

const DERIVE_COPY_FEATURE_ENABLED: bool = cfg!(feature = "derive_copy");
const DISABLE_COPY_FEATURE_ENABLED: bool = cfg!(feature = "disable_copy");

const GENERATE_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "generate_bit_ops");
const DISABLE_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "disable_bit_ops");

const GENERATE_WRITE_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "generate_write_bit_ops");
const DISABLE_WRITE_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "disable_write_bit_ops");

const GENERATE_CLEAR_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "generate_clear_bit_ops");
const DISABLE_CLEAR_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "disable_clear_bit_ops");

const GENERATE_SET_GET_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "generate_set_get_bit_ops");
const DISABLE_SET_GET_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "disable_set_get_bit_ops");

const GENERATE_INVERT_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "generate_invert_bit_ops");
const DISABLE_INVERT_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "disable_invert_bit_ops");

const GENERATE_TOGGLE_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "generate_toggle_bit_ops");
const DISABLE_TOGGLE_BIT_OPS_FEATURE_ENABLED: bool = cfg!(feature = "disable_toggle_bit_ops");

const GENERATE_BUILDER_FEATURE_ENABLED: bool = cfg!(feature = "generate_builder");
const DISABLE_BUILDER_FEATURE_ENABLED: bool = cfg!(feature = "disable_builder");

const ENABLE_ARRAY_HEAP_FEATURE_ENABLED: bool = cfg!(feature = "enable_array_heap");
const DISABLE_ARRAY_HEAP_FEATURE_ENABLED: bool = cfg!(feature = "disable_array_heap");

/// The order of the bits in the bitfield.
///
/// If the order is `Lsb`, the top most field will be the least significant bit
/// and the bottom most field is the most significant bit. If the order is
/// `Msb`, the top most field will be the most significant bit and the bottom
/// most field is the least significant bit.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BitOrder {
    /// The least significant bit order.
    Lsb,

    /// The most significant bit order.
    Msb,
}

impl FromStr for BitOrder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "lsb" => Ok(Self::Lsb),
            "msb" => Ok(Self::Msb),
            _ => Err(format!("Invalid order argument '{s}'. Valid values are 'lsb' or 'msb'.")),
        }
    }
}

/// Parsed arguments for the bitfield attribute.
#[derive(Clone, Copy, Debug, PartialEq, Eq, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct BitfieldArguments {
    /// The order of the bits in the bitfield.
    order: BitOrder,

    /// Whether to generate the new functions.
    generate_new: bool,

    /// Whether to generate the from and into traits and functions.
    generate_from_into_bits: bool,

    /// The endian for integers passed to from trait and functions.
    from_endian: ConversionEndian,

    /// The endian for integers passed to from trait and functions.
    into_endian: ConversionEndian,

    /// Whether to generate the [`From`] traits implementation.
    generate_from_traits: bool,

    /// Whether to generate a [`Default`] implementation.
    generate_default: bool,

    /// Whether to generate a [`std::fmt::Debug`] implementation.
    generate_debug: bool,

    /// Whether the bitfield should derive copy and clone.
    derive_copy: bool,

    /// Whether to generate all bit operations.
    generate_bit_ops: bool,

    /// Whether to generate write bit operations.
    generate_write_bit_ops: bool,

    /// Whether the user set generated write bit operations flag.
    user_set_generate_write_bit_ops: bool,

    /// The endian for integers passed to write functions.
    write_endian: ConversionEndian,

    /// Whether to generate clear bit operations.
    generate_clear_bit_ops: bool,

    /// Whether the user set clear bit ops.
    user_set_clear_bit_ops: bool,

    /// Whether to generate set and get bit(s) operations.
    generate_set_get_bit_ops: bool,

    /// Whether the user set generate set and get bit(s) operations.
    user_set_set_get_bit_ops: bool,

    /// Whether to generate invert bit operations.
    generate_invert_bit_ops: bool,

    /// Whether the user set generate invert bit operations.
    user_set_invert_bit_ops: bool,

    /// Whether to generate toggle bit operations.
    generate_toggle_bit_ops: bool,

    /// Whether the user set generate toggle bit operations.
    user_set_toggle_bit_ops: bool,

    /// Whether a bitfield builder should be generated.
    generate_builder: bool,

    /// Whether to allocate array-backed bitfield storage on the heap.
    ///
    /// Useful when the array would be too large to live on the stack and has no
    /// effect on integer-backed bitfields.
    array_heap: bool,

    /// Whether to force a panic during macro generation.
    force_panic: bool,
}

impl Default for BitfieldArguments {
    /// Default bitfield argument values used when none are supplied.
    fn default() -> Self {
        Self {
            order: resolve_bit_order_feature(ORDER_LSB_FEATURE_ENABLED, ORDER_MSB_FEATURE_ENABLED),
            from_endian: resolve_endian_feature(
                FROM_ENDIAN_LITTLE_FEATURE_ENABLED,
                FROM_ENDIAN_BIG_FEATURE_ENABLED,
                ConversionEndian::Big,
            ),
            into_endian: resolve_endian_feature(
                INTO_ENDIAN_LITTLE_FEATURE_ENABLED,
                INTO_ENDIAN_BIG_FEATURE_ENABLED,
                ConversionEndian::Big,
            ),
            write_endian: resolve_endian_feature(
                WRITE_ENDIAN_LITTLE_FEATURE_ENABLED,
                WRITE_ENDIAN_BIG_FEATURE_ENABLED,
                ConversionEndian::Big,
            ),
            generate_new: resolve_bool_feature(
                GENERATE_NEW_FEATURE_ENABLED,
                DISABLE_NEW_FEATURE_ENABLED,
            ),
            generate_from_into_bits: resolve_bool_feature(
                GENERATE_FROM_INTO_BITS_FEATURE_ENABLED,
                DISABLE_FROM_INTO_BITS_FEATURE_ENABLED,
            ),
            generate_from_traits: resolve_bool_feature(
                GENERATE_FROM_TRAITS_FEATURE_ENABLED,
                DISABLE_FROM_TRAITS_FEATURE_ENABLED,
            ),
            generate_default: resolve_bool_feature(
                GENERATE_DEFAULT_FEATURE_ENABLED,
                DISABLE_DEFAULT_FEATURE_ENABLED,
            ),
            generate_debug: resolve_bool_feature(
                GENERATE_DEBUG_FEATURE_ENABLED,
                DISABLE_DEBUG_FEATURE_ENABLED,
            ),
            derive_copy: resolve_bool_feature(
                DERIVE_COPY_FEATURE_ENABLED,
                DISABLE_COPY_FEATURE_ENABLED,
            ),
            generate_bit_ops: resolve_bool_feature(
                GENERATE_BIT_OPS_FEATURE_ENABLED,
                DISABLE_BIT_OPS_FEATURE_ENABLED,
            ),
            generate_write_bit_ops: resolve_bool_feature(
                GENERATE_WRITE_BIT_OPS_FEATURE_ENABLED,
                DISABLE_WRITE_BIT_OPS_FEATURE_ENABLED,
            ),
            generate_clear_bit_ops: resolve_bool_feature(
                GENERATE_CLEAR_BIT_OPS_FEATURE_ENABLED,
                DISABLE_CLEAR_BIT_OPS_FEATURE_ENABLED,
            ),
            user_set_clear_bit_ops: false,
            generate_set_get_bit_ops: resolve_bool_feature(
                GENERATE_SET_GET_BIT_OPS_FEATURE_ENABLED,
                DISABLE_SET_GET_BIT_OPS_FEATURE_ENABLED,
            ),
            user_set_set_get_bit_ops: false,
            generate_invert_bit_ops: resolve_bool_feature(
                GENERATE_INVERT_BIT_OPS_FEATURE_ENABLED,
                DISABLE_INVERT_BIT_OPS_FEATURE_ENABLED,
            ),
            user_set_invert_bit_ops: false,
            generate_toggle_bit_ops: resolve_bool_feature(
                GENERATE_TOGGLE_BIT_OPS_FEATURE_ENABLED,
                DISABLE_TOGGLE_BIT_OPS_FEATURE_ENABLED,
            ),
            user_set_toggle_bit_ops: false,
            generate_builder: resolve_bool_feature(
                GENERATE_BUILDER_FEATURE_ENABLED,
                DISABLE_BUILDER_FEATURE_ENABLED,
            ),
            array_heap: ENABLE_ARRAY_HEAP_FEATURE_ENABLED && !DISABLE_ARRAY_HEAP_FEATURE_ENABLED,
            user_set_generate_write_bit_ops: false,
            force_panic: false,
        }
    }
}

/// Resolves a boolean feature flag.
///
/// Returns `true` if `enabled` is set, `false` if `disabled` is set,
/// or `true` when neither flag is active (default behavior preserved).
const fn resolve_bool_feature(enabled: bool, disabled: bool) -> bool {
    if enabled { true } else { !disabled }
}

/// Resolves a [`BitOrder`] from two mutually exclusive feature flags.
///
/// Returns `Lsb` if `lsb` is set, `Msb` if `msb` is set, or `Lsb`
/// when neither flag is active.
const fn resolve_bit_order_feature(lsb: bool, msb: bool) -> BitOrder {
    if lsb {
        BitOrder::Lsb
    } else if msb {
        BitOrder::Msb
    } else {
        BitOrder::Lsb
    }
}

#[derive(Display, EnumString, EnumIter, PartialEq, Eq)]
enum BitfieldArgumentKey {
    #[strum(serialize = "order")]
    Order,

    #[strum(serialize = "from_endian")]
    FromEndian,

    #[strum(serialize = "into_endian")]
    IntoEndian,

    #[strum(serialize = "write_endian")]
    WriteEndian,

    #[strum(serialize = "new")]
    New,

    #[strum(serialize = "from_into_bits")]
    FromIntoBits,

    #[strum(serialize = "from_traits")]
    FromIntoTraits,

    #[strum(serialize = "default")]
    Default,

    #[strum(serialize = "debug")]
    Debug,

    #[strum(serialize = "copy")]
    Copy,

    #[strum(serialize = "bit_ops")]
    BitOps,

    #[strum(serialize = "write_bit_ops")]
    WriteBitOps,

    #[strum(serialize = "clear_bit_ops")]
    ClearBitOps,

    #[strum(serialize = "set_get_bit_ops")]
    SetGetBitOps,

    #[strum(serialize = "invert_bit_ops")]
    InvertBitOps,

    #[strum(serialize = "toggle_bit_ops")]
    ToggleBitOps,

    #[strum(serialize = "builder")]
    Builder,

    #[strum(serialize = "array_heap")]
    ArrayHeap,

    #[strum(serialize = "force_panic")]
    ForcePanic,
}

impl Parse for BitfieldArguments {
    /// Parses bitfield attribute arguments from the given input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let valid_keys = BitfieldArgumentKey::iter()
            .filter(|argument_key| *argument_key != BitfieldArgumentKey::ForcePanic)
            .map(|k| k.to_string())
            .collect();
        let internal_keys = HashSet::from([BitfieldArgumentKey::ForcePanic.to_string()]);
        let attribute_arguments = parse_attribute_arguments(input, valid_keys, internal_keys)?;
        let mut bitfield_arguments = Self::default();

        for argument in attribute_arguments {
            match BitfieldArgumentKey::from_str(argument.key().token().as_str())
                .expect("This should be caught by the known keys check")
            {
                BitfieldArgumentKey::Order => {
                    bitfield_arguments.order =
                        BitOrder::from_str(argument.value().token().as_str()).map_err(|err| {
                            create_user_parsing_compiler_error(argument.value().span(), err)
                        })?;
                },
                BitfieldArgumentKey::FromEndian => {
                    bitfield_arguments.from_endian =
                        ConversionEndian::from_str(argument.value().token().as_str()).map_err(
                            |err| create_user_parsing_compiler_error(argument.value().span(), err),
                        )?;
                },
                BitfieldArgumentKey::IntoEndian => {
                    bitfield_arguments.into_endian =
                        ConversionEndian::from_str(argument.value().token().as_str()).map_err(
                            |err| create_user_parsing_compiler_error(argument.value().span(), err),
                        )?;
                },
                BitfieldArgumentKey::WriteEndian => {
                    bitfield_arguments.write_endian =
                        ConversionEndian::from_str(argument.value().token().as_str()).map_err(
                            |err| create_user_parsing_compiler_error(argument.value().span(), err),
                        )?;
                },
                BitfieldArgumentKey::New => {
                    bitfield_arguments.generate_new = parse_boolean_attribute_argument(argument)?;
                },
                BitfieldArgumentKey::FromIntoBits => {
                    bitfield_arguments.generate_from_into_bits =
                        parse_boolean_attribute_argument(argument)?;
                },
                BitfieldArgumentKey::FromIntoTraits => {
                    bitfield_arguments.generate_from_traits =
                        parse_boolean_attribute_argument(argument)?;
                },
                BitfieldArgumentKey::Default => {
                    bitfield_arguments.generate_default =
                        parse_boolean_attribute_argument(argument)?;
                },
                BitfieldArgumentKey::Debug => {
                    bitfield_arguments.generate_debug = parse_boolean_attribute_argument(argument)?;
                },
                BitfieldArgumentKey::BitOps => {
                    bitfield_arguments.generate_bit_ops =
                        parse_boolean_attribute_argument(argument)?;
                },
                BitfieldArgumentKey::WriteBitOps => {
                    bitfield_arguments.generate_write_bit_ops =
                        parse_boolean_attribute_argument(argument)?;
                    bitfield_arguments.user_set_generate_write_bit_ops = true;
                },
                BitfieldArgumentKey::ClearBitOps => {
                    bitfield_arguments.generate_clear_bit_ops =
                        parse_boolean_attribute_argument(argument)?;
                    bitfield_arguments.user_set_clear_bit_ops = true;
                },
                BitfieldArgumentKey::SetGetBitOps => {
                    bitfield_arguments.generate_set_get_bit_ops =
                        parse_boolean_attribute_argument(argument)?;
                    bitfield_arguments.user_set_set_get_bit_ops = true;
                },
                BitfieldArgumentKey::InvertBitOps => {
                    bitfield_arguments.generate_invert_bit_ops =
                        parse_boolean_attribute_argument(argument)?;
                    bitfield_arguments.user_set_invert_bit_ops = true;
                },
                BitfieldArgumentKey::ToggleBitOps => {
                    bitfield_arguments.generate_toggle_bit_ops =
                        parse_boolean_attribute_argument(argument)?;
                    bitfield_arguments.user_set_toggle_bit_ops = true;
                },
                BitfieldArgumentKey::Builder => {
                    bitfield_arguments.generate_builder =
                        parse_boolean_attribute_argument(argument)?;
                },
                BitfieldArgumentKey::ArrayHeap => {
                    bitfield_arguments.array_heap = parse_boolean_attribute_argument(argument)?;
                },
                BitfieldArgumentKey::Copy => {
                    bitfield_arguments.derive_copy = parse_boolean_attribute_argument(argument)?;
                },
                BitfieldArgumentKey::ForcePanic => {
                    bitfield_arguments.force_panic = parse_boolean_attribute_argument(argument)?;
                },
            }
        }

        Ok(bitfield_arguments)
    }
}
