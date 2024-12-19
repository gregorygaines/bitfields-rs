use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

use crate::generation::common::{
    generate_setting_fields_default_values_tokens, generate_setting_fields_to_zero_tokens,
};
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the `new` function for the bitfield.
///
/// Output:
///
/// ```ignore
/// fn new() -> Self { ... }
/// ```
pub(crate) fn generate_new_function_tokens(
    vis: Visibility,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    bitfield_type: &syn::Type,
) -> TokenStream {
    let setting_fields_default_values_tokens = generate_setting_fields_default_values_tokens(
        bitfield_type,
        fields,
        None,
        !ignored_fields.is_empty(),
    );
    let documentation = "Creates a new bitfield instance.";

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

    let const_ident_tokens = ignored_fields.is_empty().then(|| quote! { const });

    quote! {
        #[doc = #documentation]
        #vis #const_ident_tokens fn new() -> Self {
            let mut this = #initialize_struct_tokens;
            #setting_fields_default_values_tokens
            this
        }
    }
}

/// Generates the `new` function for the bitfield without setting default
/// values.
///
/// Output:
///
/// ```ignore
/// fn new_without_defaults() -> Self { ... }
/// ```
pub(crate) fn generate_new_without_defaults_function_tokens(
    vis: Visibility,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    bitfield_type: &syn::Type,
) -> TokenStream {
    let setting_fields_to_zero_tokens = generate_setting_fields_to_zero_tokens(
        bitfield_type,
        fields,
        None,
        !ignored_fields.is_empty(),
    );
    let documentation = "Creates a new bitfield instance without setting any default values.";

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

    let const_ident_tokens = ignored_fields.is_empty().then(|| quote! { const });

    quote! {
        #[doc = #documentation]
        #vis #const_ident_tokens fn new_without_defaults() -> Self {
            let mut this = #initialize_struct_tokens;
            #setting_fields_to_zero_tokens
            this
        }
    }
}
