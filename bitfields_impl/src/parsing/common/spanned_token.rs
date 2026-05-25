use getset::{CloneGetters, Getters};
use proc_macro2::{Span, TokenStream};
use quote::quote;

use crate::parsing::common::to_tokens::ToTokens;

/// Represents a token and its span for error reporting.
#[derive(Clone, Debug, Getters, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct SpannedToken {
    token: String,
    span: Span,
}

impl SpannedToken {
    /// Creates a new [`SpannedToken`] instance.
    pub const fn new(token: String, span: Span) -> Self {
        Self {
            token,
            span,
        }
    }
}

impl ToTokens for SpannedToken {
    fn to_tokens(&self) -> TokenStream {
        let ty = syn::parse_str::<syn::Type>(&self.token).expect("Expected type");
        quote! { #ty }
    }
}
