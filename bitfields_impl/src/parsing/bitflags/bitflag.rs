use getset::{CloneGetters, Getters};
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;

use crate::parsing::bitflags::bitflag_arguments::BitflagArguments;
use crate::parsing::common::const_expr::ConstExpr;
use crate::parsing::common::spanned_data_type::SpannedDataTypeToken;
use crate::parsing::common::visibility::Visibility;

/// Represents an annotated enum that represents a bitflag.
#[derive(Getters, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct Bitflag {
    /// The user defined attributes of the bitflag.
    user_attributes_tokens: Vec<TokenStream>,

    /// The type of the bitfield.
    spanned_data_type_token: SpannedDataTypeToken,

    /// The visibility of the bitflag.
    visibility: Visibility,

    /// The bitflag ident.
    #[getset(skip)]
    name_ident: Ident,

    /// The variants of the bitflag.
    variants: Vec<BitflagVariant>,

    /// The arguments of the bitflag.
    arguments: BitflagArguments,
}

impl Bitflag {
    /// Creates a new [`Bitflag`] instance.
    pub fn new(
        user_attributes_tokens: Vec<TokenStream>,
        spanned_data_type_token: SpannedDataTypeToken,
        visibility: Visibility,
        name_ident: Ident,
        variants: Vec<BitflagVariant>,
        arguments: BitflagArguments,
    ) -> Self {
        let _name = name_ident.to_string();
        Self {
            user_attributes_tokens,
            spanned_data_type_token,
            visibility,
            name_ident,
            variants,
            arguments,
        }
    }

    /// Returns the name as tokens.
    pub fn name_tokens(&self) -> TokenStream {
        self.name_ident.to_token_stream()
    }
}

/// Represents a bitflag variant.
#[derive(Getters, CloneGetters, Clone)]
#[getset(get_clone = "pub")]
pub struct BitflagVariant {
    /// The user defined attributes of the bitflag variant.
    user_attributes_tokens: Vec<TokenStream>,

    /// The ident of the bitflag variant.
    #[getset(skip)]
    name_ident: Ident,

    /// The value of the bitflag variant.
    value: ConstExpr,

    /// Whether the bitflag variant is the annotated with the base attribute.
    base: bool,

    /// Whether the bitflag variant is annotated with the default attribute.
    default: bool,
}

impl BitflagVariant {
    /// Creates a new [`BitflagVariant`] instance.
    pub fn new(
        user_attributes_tokens: Vec<TokenStream>,
        name_ident: Ident,
        value: ConstExpr,
        base: bool,
        default: bool,
    ) -> Self {
        Self {
            user_attributes_tokens,
            name_ident,
            value,
            base,
            default,
        }
    }

    /// Returns the name as tokens.
    pub fn name_tokens(&self) -> TokenStream {
        self.name_ident.to_token_stream()
    }
}
