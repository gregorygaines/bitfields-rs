use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generation::common::generate_setting_fields_default_values_tokens;
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the default implementation.
pub(crate) fn generate_default_implementation_tokens(
    bitfield_struct_name: Ident,
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
) -> TokenStream {
    let setting_fields_default_values_tokens = generate_setting_fields_default_values_tokens(
        bitfield_type,
        fields,
        None,
        !ignored_fields.is_empty(),
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
            #bitfield_struct_name {
                val: 0,
                #( #ignored_fields_defaults )*
            }
        }
    } else {
        quote! {
            #bitfield_struct_name(0)
        }
    };

    quote! {
        impl core::default::Default for #bitfield_struct_name {
            fn default() -> Self {
                let mut this = #initialize_struct_tokens;
                #setting_fields_default_values_tokens
                this
            }
        }
    }
}
