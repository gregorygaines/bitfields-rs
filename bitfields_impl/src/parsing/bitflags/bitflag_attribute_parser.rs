//! Module for parsing the entire `#[bitflag]` attribute.

use getset::{CloneGetters, Getters};
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};

use crate::parsing::bitflags::bitflag_arguments::BitflagArguments;
use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::spanned_data_type::{DataType, SpannedDataTypeToken};
use crate::parsing::common::type_parse_error::TypeParsingError;

/// Represents the `#[bitflag]` attribute.
#[derive(Clone, Debug, Getters, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct BitflagAttribute {
    /// The type of the bitflag.
    spanned_data_type_token: SpannedDataTypeToken,

    /// The arguments of the bitflag.
    arguments: BitflagArguments,
}

impl Parse for BitflagAttribute {
    /// Parse the `Bitflag` from a token stream.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let spanned_data_type_token = Self::parse_bitflag_type(input)?;
        Self::check_supported_bitflag_type(&spanned_data_type_token)?;
        let arguments = BitflagArguments::parse(input)?;
        Ok(Self {
            spanned_data_type_token,
            arguments,
        })
    }
}

const BITFLAG_ATTRIBUTE_NON_UNSIGNED_INTEGER_FIRST_ARGUMENT_ERROR_MESSAGE: &str =
    "The bitflag attribute must have an unsigned integer type as its first argument";
const BITFLAG_ATTRIBUTE_FLOAT_FIRST_ARGUMENT_ERROR_MESSAGE: &str =
    "The bitflag attribute must have an unsigned integer type as its first argument, floats are \
     unsupported.";

impl BitflagAttribute {
    /// Parse the bitflag type from the attribute input.
    fn parse_bitflag_type(input: ParseStream) -> syn::Result<SpannedDataTypeToken> {
        match Self::parse_type(input) {
            Ok(spanned_data_type_token) => Ok(spanned_data_type_token),
            Err(err) => match err {
                TypeParsingError::NonType(ty) => {
                    Err(Self::create_unsupported_type_compiler_error(&ty, input.span()))
                },
                TypeParsingError::UnexpectedFloat => Err(create_user_parsing_compiler_error(
                    input.span(),
                    BITFLAG_ATTRIBUTE_FLOAT_FIRST_ARGUMENT_ERROR_MESSAGE,
                )),
                _ => Err(create_user_parsing_compiler_error(
                    input.span(),
                    format!(
                        "{BITFLAG_ATTRIBUTE_NON_UNSIGNED_INTEGER_FIRST_ARGUMENT_ERROR_MESSAGE}."
                    ),
                )),
            },
        }
    }

    /// Parses a type from an input parse stream.
    pub fn parse_type(input: ParseStream) -> Result<SpannedDataTypeToken, TypeParsingError> {
        if input.is_empty() {
            return Err(TypeParsingError::UnexpectedEndOfInput);
        }

        match input.parse::<syn::Type>() {
            Ok(ty) => SpannedDataTypeToken::new(&ty),
            Err(err) => Err(TypeParsingError::NonType(err.to_string())),
        }
    }

    /// Ensure the parsed type is a supported bitfield type.
    fn check_supported_bitflag_type(
        spanned_data_type_token: &SpannedDataTypeToken,
    ) -> syn::Result<()> {
        let data_type = spanned_data_type_token.data_type();
        if matches!(data_type, DataType::Custom) || !data_type.unsigned() {
            return Err(Self::create_unsupported_type_compiler_error(
                &spanned_data_type_token.to_string(),
                spanned_data_type_token.span(),
            ));
        }

        Ok(())
    }

    /// Create a user-facing parse compiler error for unsupported bitflag types.
    fn create_unsupported_type_compiler_error(type_string_repr: &str, span: Span) -> syn::Error {
        create_user_parsing_compiler_error(
            span,
            format!(
                "{BITFLAG_ATTRIBUTE_NON_UNSIGNED_INTEGER_FIRST_ARGUMENT_ERROR_MESSAGE}, \
                 '{type_string_repr}' is unsupported."
            ),
        )
    }
}
