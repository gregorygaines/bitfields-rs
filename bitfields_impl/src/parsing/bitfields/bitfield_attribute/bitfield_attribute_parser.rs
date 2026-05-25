//! Module for parsing the entire `#[bitfield]` attribute.

use getset::{CloneGetters, Getters};
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};

use crate::parsing::bitfields::bitfield_attribute::bitfield_arguments::BitfieldArguments;
use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::spanned_data_type::{DataType, SpannedDataTypeToken};
use crate::parsing::common::type_parse_error::TypeParsingError;

/// Represents the `#[bitfield]` attribute.
#[derive(Clone, Debug, Getters, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct BitfieldAttribute {
    /// The type of the bitfield.
    spanned_data_type_token: SpannedDataTypeToken,

    /// The arguments of the bitfield.
    arguments: BitfieldArguments,
}

impl Parse for BitfieldAttribute {
    /// Parse the `BitfieldAttribute` from a token stream.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let spanned_data_type_token = Self::parse_bitfield_type(input)?;
        Self::check_supported_bitfield_type(&spanned_data_type_token)?;
        let arguments = BitfieldArguments::parse(input)?;
        Ok(Self {
            spanned_data_type_token,
            arguments,
        })
    }
}

const BITFIELD_ATTRIBUTE_NON_UNSIGNED_INTEGER_FIRST_ARGUMENT_ERROR_MESSAGE: &str =
    "The bitfield must have an unsigned integer type as its first argument";
const BITFIELD_ATTRIBUTE_FLOAT_FIRST_ARGUMENT_ERROR_MESSAGE: &str =
    "The bitfield must have an unsigned integer type as its first argument, floats are \
     unsupported.";

impl BitfieldAttribute {
    /// Parse the bitfield type (first argument) from the attribute input.
    fn parse_bitfield_type(input: ParseStream) -> syn::Result<SpannedDataTypeToken> {
        match Self::parse_type(input) {
            Ok(spanned_data_type_token) => Ok(spanned_data_type_token),
            Err(err) => match err {
                TypeParsingError::NonType(ty) => {
                    Err(Self::create_unsupported_type_compiler_error(&ty, input.span()))
                },
                TypeParsingError::Unexpected(_)
                | TypeParsingError::UnexpectedEndOfInput
                | TypeParsingError::SizeTypeNotSupported => {
                    Err(create_user_parsing_compiler_error(
                        input.span(),
                        format!("{BITFIELD_ATTRIBUTE_NON_UNSIGNED_INTEGER_FIRST_ARGUMENT_ERROR_MESSAGE}."),
                    ))
                },
                TypeParsingError::UnexpectedFloat => Err(create_user_parsing_compiler_error(
                    input.span(),
                    BITFIELD_ATTRIBUTE_FLOAT_FIRST_ARGUMENT_ERROR_MESSAGE,
                )),
                TypeParsingError::NonIntegerArrayType => Err(create_user_parsing_compiler_error(
                    input.span(),
                    "The bitfield array must have an unsigned `u8` integer type as its first argument.",
                )),
                TypeParsingError::ZeroArrayLength => Err(create_user_parsing_compiler_error(
                    input.span(),
                    "The bitfield array length must be greater than 0.",
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
    fn check_supported_bitfield_type(
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

    /// Create a user-facing parse compiler error for unsupported bitfield
    /// types.
    fn create_unsupported_type_compiler_error(type_string_repr: &str, span: Span) -> syn::Error {
        create_user_parsing_compiler_error(
            span,
            format!(
                "{BITFIELD_ATTRIBUTE_NON_UNSIGNED_INTEGER_FIRST_ARGUMENT_ERROR_MESSAGE}, \
                 '{type_string_repr}' is unsupported."
            ),
        )
    }
}
