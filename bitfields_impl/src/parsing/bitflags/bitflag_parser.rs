use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{ItemEnum, Token, Variant};

use crate::parsing::bitflags::bitflag::{Bitflag, BitflagVariant};
use crate::parsing::bitflags::bitflag_attribute_parser::BitflagAttribute;
use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::const_expr::ConstExpr;
use crate::parsing::common::spanned_token::SpannedToken;
use crate::parsing::common::visibility::Visibility;

/// Parses an enum annotated with `#[bitflag(..)]` into a [`Bitflag`].
pub fn parse_bitflag_enum(args: TokenStream, input: TokenStream) -> syn::Result<Bitflag> {
    let enum_tokens = parse_enum_tokens(&input)?;
    let user_attributes_tokens =
        enum_tokens.attrs.iter().map(quote::ToTokens::into_token_stream).collect();
    let visibility = Visibility::new(&enum_tokens.vis);
    let bitflag_attribute = parse_bitflag_attribute(args)?;
    let name = enum_tokens.ident.to_string();
    let variants = parse_variants(&enum_tokens.variants)?;

    check_base_bitflag_variants(&enum_tokens, &variants)?;

    Ok(Bitflag::new(
        user_attributes_tokens,
        bitflag_attribute.spanned_data_type_token(),
        visibility,
        name,
        variants,
        bitflag_attribute.arguments(),
    ))
}

fn parse_enum_tokens(input: &TokenStream) -> syn::Result<ItemEnum> {
    syn::parse2::<ItemEnum>(input.clone())
}

fn parse_bitflag_attribute(args: TokenStream) -> syn::Result<BitflagAttribute> {
    syn::parse2(args)
}

/// Parses each enum variant into a [`BitflagVariant`].
fn parse_variants(variants: &Punctuated<Variant, Token![,]>) -> syn::Result<Vec<BitflagVariant>> {
    variants.into_iter().map(parse_variant_helper).collect()
}

const BASE_BITFLAG_ENTRY_MACRO_NAME: &str = "base";
const DEFAULT_MACRO_NAME: &str = "default";

/// Converts a single [`Variant`] into a [`BitflagVariant`].
fn parse_variant_helper(variant: &Variant) -> syn::Result<BitflagVariant> {
    let base = variant.attrs.iter().any(|attr| attr.path().is_ident(BASE_BITFLAG_ENTRY_MACRO_NAME));
    let default = variant.attrs.iter().any(|attr| attr.path().is_ident(DEFAULT_MACRO_NAME));

    let user_attributes_tokens = variant
        .attrs
        .iter()
        .filter(|attr| !attr.path().is_ident(BASE_BITFLAG_ENTRY_MACRO_NAME))
        .map(quote::ToTokens::into_token_stream)
        .collect();

    let name = variant.ident.to_string();

    let value = match variant.discriminant.clone() {
        Some((_, expr)) => {
            let token = quote!(#expr).to_string();
            let span = expr.span();
            let spanned_token = SpannedToken::new(token, span);
            ConstExpr::new(&spanned_token)?
        },
        None => {
            return Err(create_user_parsing_compiler_error(
                variant.ident.span(),
                format!(
                    "Bitflag variant '{name}' must have an explicit value (e.g. `{name} = \
                     0b0001`)."
                ),
            ));
        },
    };

    Ok(BitflagVariant::new(user_attributes_tokens, name, value, base, default))
}

fn check_base_bitflag_variants(
    item_enum: &ItemEnum,
    variants: &[BitflagVariant],
) -> syn::Result<()> {
    let base_count = variants.iter().filter(|variant| variant.base()).count();
    if base_count > 1 {
        return Err(create_user_parsing_compiler_error(
            item_enum.ident.span(),
            "There can only be one bitflag variant marked as `#[base]`.",
        ));
    }

    let has_base_or_default = variants.iter().any(|variant| variant.base() || variant.default());
    if !has_base_or_default {
        return Err(create_user_parsing_compiler_error(
            item_enum.ident.span(),
            "The bitflag must have at least one variant marked with `#[base]` or `#[default]`. If \
             a variant is marked with `#[base]`, it will take precedence over `#[default]`.",
        ));
    }

    Ok(())
}
