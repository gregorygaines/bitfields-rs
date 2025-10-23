use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generation::common::generate_setting_fields_from_bits_tokens;
use crate::parsing::bitfield_attribute::{BitfieldAttribute, Endian};
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the `From` trait implementation.
pub(crate) fn generate_from_bitfield_type_for_bitfield_implementation_tokens(
    bitfield_struct_name: Ident,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    bitfield_type: &syn::Type,
) -> TokenStream {
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_type,
        fields,
        Some(quote! { Self }),
        false,
        !ignored_fields.is_empty(),
        true, // From trait should set read-only fields (like from_bits)
    );

    let ignored_fields_defaults = ignored_fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;
        quote! {
            #field_name: <#field_ty>::default(),
        }
    });

    let initialize_struct_tokens = if !ignored_fields.is_empty() {
        quote! {
            Self {
                val: 0,
                #( #ignored_fields_defaults )*
            }
        }
    } else {
        quote! {
            Self(0)
        }
    };

    quote! {
        impl From<#bitfield_type> for #bitfield_struct_name {
            fn from(bits: #bitfield_type) -> Self {
                let mut this = #initialize_struct_tokens;
                #setting_fields_from_bits_tokens
                this
            }
        }
    }
}

pub(crate) fn generate_from_bitfield_for_bitfield_type_implementation_tokens(
    bitfield_struct_name: Ident,
    bitfield_attribute: &BitfieldAttribute,
    ignored_fields_struct: bool,
) -> TokenStream {
    let bitfield_type = &bitfield_attribute.ty;

    let from_bitfield_impl = match bitfield_attribute.into_endian {
        Endian::Big => {
            if ignored_fields_struct {
                quote! {
                    b.val
                }
            } else {
                quote! {
                    b.0
                }
            }
        }
        Endian::Little => {
            if ignored_fields_struct {
                quote! {
                    b.val.swap_bytes()
                }
            } else {
                quote! {
                    b.0.swap_bytes()
                }
            }
        }
    };

    quote! {
        impl From<#bitfield_struct_name> for #bitfield_type {
            fn from(b: #bitfield_struct_name) -> #bitfield_type {
                #from_bitfield_impl
            }
        }
    }
}
