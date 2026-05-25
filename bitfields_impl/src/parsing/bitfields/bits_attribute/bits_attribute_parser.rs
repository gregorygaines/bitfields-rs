//! Module for parsing the entire `#[bits]` attribute.

use getset::{CloneGetters, Getters};
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::{LitInt, Token};

use crate::parsing::bitfields::bits_attribute::bits_arguments::BitsArguments;
use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;

/// Represents the `#[bits]` attribute.
#[derive(Clone, Debug, Getters, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct BitsAttribute {
    /// The bits of the attribute.
    bits: Option<u32>,

    /// The arguments of the attribute.
    arguments: BitsArguments,

    /// The span of the bits attribute.
    span: Option<Span>,
}

impl Parse for BitsAttribute {
    /// Parse a `BitsAttribute` from the attribute token stream.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let bits_and_span = Self::parse_bits(input)?;
        let arguments = BitsArguments::parse(input)?;
        let bits = bits_and_span
            .is_some()
            .then(|| bits_and_span.expect("Expected a bits and span for bits attribute").0);
        let span = bits_and_span
            .is_some()
            .then(|| bits_and_span.expect("Expected a bits and span for bits attribute").1);
        Ok(Self {
            bits,
            arguments,
            span,
        })
    }
}

impl BitsAttribute {
    /// Parses an optional non-negative integer bit count from the input.
    ///
    /// Accepts either `<n>` or `<n>,` at the start of the attribute, where
    /// `<n>` is a non-negative integer literal (e.g. `8`, `4`). If no
    /// integer is present the bit count is `None`.
    fn parse_bits(input: ParseStream) -> syn::Result<Option<(u32, Span)>> {
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        if input.peek(Token![-]) {
            return Err(create_user_parsing_compiler_error(
                input.span(),
                "Invalid bit count, expected an unsigned integer.".to_string(),
            ));
        }

        if !input.peek(LitInt) {
            return Ok(None);
        }

        let lit: LitInt = input.parse()?;

        if !lit.suffix().is_empty() {
            return Err(create_user_parsing_compiler_error(
                lit.span(),
                "Invalid bit count, expected an unsigned integer without a suffix.".to_string(),
            ));
        }

        let bits = lit.base10_parse::<u32>().map_err(|_| {
            create_user_parsing_compiler_error(
                lit.span(),
                format!(
                    "Bit count '{lit}' is too big, the maximum amount of bits is '4,294,967,295.'"
                ),
            )
        })?;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(Some((bits, lit.span())))
    }
}
