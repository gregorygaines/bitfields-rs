use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::parsing::common::to_tokens::ToTokens;

/// Represents the visibility of an item.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Visibility {
    /// A public visibility level.
    Public,

    /// A restricted visibility level.
    Restricted(String),

    /// A private visibility level.
    Private,
}

impl Visibility {
    /// Convert a new `[Visibility]` from `[syn::Visibility]` representation.
    pub fn new(vis: &syn::Visibility) -> Self {
        match vis {
            syn::Visibility::Public(_) => Self::Public,
            syn::Visibility::Restricted(r) => {
                let path = r
                    .path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<String>();
                Self::Restricted(path)
            },
            syn::Visibility::Inherited => Self::Private,
        }
    }
}

impl ToTokens for Visibility {
    fn to_tokens(&self) -> TokenStream {
        match self {
            Self::Public => {
                quote! {
                    pub
                }
            },
            Self::Restricted(path) => {
                let path_ident = format_ident!("{}", path);
                quote! {
                    pub(#path_ident)
                }
            },
            Self::Private => {
                quote! {}
            },
        }
    }
}
