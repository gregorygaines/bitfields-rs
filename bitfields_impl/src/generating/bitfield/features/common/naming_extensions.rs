use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::parsing::bitfields::bitfield::{Bitfield, Field};

impl Bitfield {
    /// Generates the identifier of the bitfield internal value.
    pub fn bitfield_internal_value_ident_tokens(&self, builder_caller: bool) -> TokenStream {
        let self_prefix = builder_caller.then(|| quote! { self. });
        if self.has_ignored_fields() {
            quote! {
                #self_prefix this.val
            }
        } else {
            quote! {
                #self_prefix this.0
            }
        }
    }
}

impl Field {
    /// Generates the setter identifier token stream for the field.
    pub fn setter_ident_tokens(&self) -> TokenStream {
        format_ident!("set_{}", self.name()).to_token_stream()
    }

    /// Generates the checked setter identifier token stream for the field.
    pub fn checked_setter_ident_tokens(&self) -> TokenStream {
        format_ident!("checked_set_{}", self.name()).to_token_stream()
    }

    /// Generate the bits constant identifier tokens.
    pub fn bits_constant_ident_tokens(&self) -> TokenStream {
        format_ident!("{}_BITS", self.name().to_uppercase()).to_token_stream()
    }

    /// Generates the offset constant identifier tokens.
    pub fn offset_constant_ident_tokens(&self) -> TokenStream {
        format_ident!("{}_OFFSET", self.name().to_uppercase()).to_token_stream()
    }
}
