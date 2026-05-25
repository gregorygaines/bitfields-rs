use std::collections::HashSet;
use std::str::FromStr;

use getset::{CloneGetters, CopyGetters, Getters};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use syn::parse::{Parse, ParseStream};

use crate::parsing::common::attribute_argument_parser::{
    parse_attribute_arguments, parse_boolean_attribute_argument,
};
use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::conversion_endian::{ConversionEndian, resolve_endian_feature};

const FROM_ENDIAN_LITTLE_FEATURE_ENABLED: bool = cfg!(feature = "bitflag_from_endian_little");
const FROM_ENDIAN_BIG_FEATURE_ENABLED: bool = cfg!(feature = "bitflag_from_endian_big");

const INTO_ENDIAN_LITTLE_FEATURE_ENABLED: bool = cfg!(feature = "bitflag_into_endian_little");
const INTO_ENDIAN_BIG_FEATURE_ENABLED: bool = cfg!(feature = "bitflag_into_endian_big");

const BITFLAG_DERIVE_COPY_FEATURE_ENABLED: bool = cfg!(feature = "bitflag_derive_copy");
const BITFLAG_DISABLE_COPY_FEATURE_ENABLED: bool = cfg!(feature = "bitflag_disable_copy");

/// Resolves a boolean feature flag.
///
/// Returns `true` if `enabled` is set, `false` if `disabled` is set,
/// or `true` when neither flag is active (default behavior preserved).
const fn resolve_bool_feature(enabled: bool, disabled: bool) -> bool {
    if enabled { true } else { !disabled }
}

/// Represents the arguments of the `#[bitflag]` attribute.
#[derive(Clone, Debug, Getters, CopyGetters, CloneGetters)]
#[getset(get_copy = "pub")]
pub struct BitflagArguments {
    /// The endian for integers passed to from trait and functions.
    from_endian: ConversionEndian,

    /// The endian for integers passed to from trait and functions.
    into_endian: ConversionEndian,

    /// Whether the bitflag should derive Copy and Clone.
    derive_copy: bool,
}

impl Default for BitflagArguments {
    fn default() -> Self {
        Self {
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
            derive_copy: resolve_bool_feature(
                BITFLAG_DERIVE_COPY_FEATURE_ENABLED,
                BITFLAG_DISABLE_COPY_FEATURE_ENABLED,
            ),
        }
    }
}

#[derive(Display, EnumString, EnumIter)]
enum BitflagArgumentKey {
    #[strum(serialize = "from_endian")]
    FromEndian,

    #[strum(serialize = "into_endian")]
    IntoEndian,

    #[strum(serialize = "copy")]
    Copy,
}

impl Parse for BitflagArguments {
    /// Parses bits attribute arguments from the given input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let valid_keys = BitflagArgumentKey::iter().map(|k| k.to_string()).collect();
        let attribute_arguments = parse_attribute_arguments(
            input,
            valid_keys,
            /* internal_keys= */ HashSet::default(),
        )?;
        let mut bitflag_arguments = Self::default();

        for argument in attribute_arguments {
            match BitflagArgumentKey::from_str(argument.key().token().as_str())
                .expect("This should be caught by the known keys check")
            {
                BitflagArgumentKey::FromEndian => {
                    bitflag_arguments.from_endian =
                        ConversionEndian::from_str(argument.value().token().as_str()).map_err(
                            |err| create_user_parsing_compiler_error(argument.value().span(), err),
                        )?;
                },
                BitflagArgumentKey::IntoEndian => {
                    bitflag_arguments.into_endian =
                        ConversionEndian::from_str(argument.value().token().as_str()).map_err(
                            |err| create_user_parsing_compiler_error(argument.value().span(), err),
                        )?;
                },
                BitflagArgumentKey::Copy => {
                    bitflag_arguments.derive_copy = parse_boolean_attribute_argument(argument)?;
                },
            }
        }

        Ok(bitflag_arguments)
    }
}
