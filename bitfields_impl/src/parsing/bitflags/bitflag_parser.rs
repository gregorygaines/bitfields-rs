use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, ItemEnum, Meta, Token, Variant};

use crate::parsing::bitflags::bitflag::{Bitflag, BitflagVariant};
use crate::parsing::bitflags::bitflag_attribute_parser::BitflagAttribute;
use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::const_expr::ConstExpr;
use crate::parsing::common::spanned_data_type::{DataType, IntegerType, SpannedDataTypeToken};
use crate::parsing::common::spanned_token::SpannedToken;
use crate::parsing::common::visibility::Visibility;

/// Parses an enum annotated with `#[bitflag(..)]` into a [`Bitflag`].
pub fn parse_bitflag_enum(args: TokenStream, input: TokenStream) -> syn::Result<Bitflag> {
    let enum_tokens = parse_enum_tokens(&input)?;
    let user_attributes_tokens =
        enum_tokens.attrs.iter().map(quote::ToTokens::into_token_stream).collect();
    let visibility = Visibility::new(&enum_tokens.vis);
    let bitflag_attribute = parse_bitflag_attribute(args)?;
    let repr_type = parse_repr_type(&enum_tokens.attrs)?;
    let has_integer_repr = repr_type.is_some();
    let spanned_data_type_token =
        resolve_bitflag_type(bitflag_attribute.spanned_data_type_token(), repr_type)?;
    let name_ident = enum_tokens.ident.clone();
    let variants = parse_variants(&enum_tokens.variants)?;

    check_base_bitflag_variants(&enum_tokens, &variants)?;

    Ok(Bitflag::new(
        user_attributes_tokens,
        spanned_data_type_token,
        has_integer_repr,
        visibility,
        name_ident,
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

/// Finds an integer representation while preserving other hints, such as
/// alignment.
fn parse_repr_type(attributes: &[Attribute]) -> syn::Result<Option<SpannedDataTypeToken>> {
    let mut repr_type = None;
    for attribute in attributes.iter().filter(|attribute| attribute.path().is_ident("repr")) {
        let hints = attribute.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for hint in hints {
            let Meta::Path(path) = hint else { continue };
            let Some(ident) = path.get_ident() else { continue };
            let Ok(DataType::Integer(integer_type)) = DataType::new(&ident.to_string()) else {
                continue;
            };
            if integer_type == IntegerType::Bool {
                continue;
            }
            if !matches!(
                integer_type,
                IntegerType::U8
                    | IntegerType::U16
                    | IntegerType::U32
                    | IntegerType::U64
                    | IntegerType::U128
            ) {
                return Err(create_user_parsing_compiler_error(
                    ident.span(),
                    format!(
                        "Bitflag representation '{ident}' is unsupported; use u8, u16, u32, u64, \
                         or u128."
                    ),
                ));
            }
            if repr_type.is_some() {
                return Err(create_user_parsing_compiler_error(
                    ident.span(),
                    "Multiple integer representations are not supported for a bitflag.",
                ));
            }
            repr_type = Some(
                SpannedDataTypeToken::new(&syn::parse_quote!(#path))
                    .expect("Supported integer representation"),
            );
        }
    }
    Ok(repr_type)
}

/// Resolves the backing type and checks that explicit and inferred types agree.
fn resolve_bitflag_type(
    explicit_type: Option<SpannedDataTypeToken>,
    repr_type: Option<SpannedDataTypeToken>,
) -> syn::Result<SpannedDataTypeToken> {
    match (explicit_type, repr_type) {
        (Some(explicit), Some(repr)) if explicit.data_type() != repr.data_type() => {
            Err(create_user_parsing_compiler_error(
                repr.span(),
                format!("Bitflag type '{explicit}' does not match #[repr({repr})]."),
            ))
        },
        (Some(data_type), _) | (None, Some(data_type)) => Ok(data_type),
        (None, None) => Err(create_user_parsing_compiler_error(
            Span::call_site(),
            "The bitflag attribute requires an unsigned integer type as its first argument or in \
             #[repr(...)].",
        )),
    }
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

    let name_ident = variant.ident.clone();

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
                    "Bitflag variant '{}' must have an explicit value (e.g. `{} = 0b0001`).",
                    name_ident, name_ident
                ),
            ));
        },
    };

    Ok(BitflagVariant::new(user_attributes_tokens, name_ident, value, base, default))
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
