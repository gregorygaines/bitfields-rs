use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generation::common::generate_setting_fields_from_bits_tokens;
use crate::parsing::bitfield_attribute::{BitfieldAttribute, Endian};
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the `From` trait implementation.
pub(crate) fn generate_from_bitfield_type_for_bitfield_implementation_tokens(
    bitfield_struct_name: Ident,
    fields: &[BitfieldField],
    bitfield_type: &syn::Type,
) -> TokenStream {
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_type,
        fields,
        Some(quote! { Self }),
        false,
    );

    quote! {
        impl From<#bitfield_type> for #bitfield_struct_name {
            fn from(bits: #bitfield_type) -> Self {
                let mut this = Self(0);
                #setting_fields_from_bits_tokens
                this
            }
        }
    }
}

pub(crate) fn generate_from_bitfield_for_bitfield_type_implementation_tokens(
    bitfield_struct_name: Ident,
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let bitfield_type = &bitfield_attribute.ty;

    let from_bitfield_impl = match bitfield_attribute.into_endian {
        Endian::Big => {
            quote! {
                 val.0
            }
        }
        Endian::Little => {
            quote! {
                 val.0.swap_bytes()
            }
        }
    };

    quote! {
        impl From<#bitfield_struct_name> for #bitfield_type {
            fn from(val: #bitfield_struct_name) -> #bitfield_type {
                #from_bitfield_impl
            }
        }
    }
}
