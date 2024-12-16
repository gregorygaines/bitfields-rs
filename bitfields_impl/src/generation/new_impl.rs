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
    bitfield_type: &syn::Type,
) -> TokenStream {
    let setting_fields_default_values_tokens =
        generate_setting_fields_default_values_tokens(bitfield_type, fields, None);
    let documentation = "Creates a new bitfield instance.";

    quote! {
        #[doc = #documentation]
        #vis const fn new() -> Self {
            let mut this = Self(0);
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
    bitfield_type: &syn::Type,
) -> TokenStream {
    let setting_fields_to_zero_tokens =
        generate_setting_fields_to_zero_tokens(bitfield_type, fields, None);
    let documentation = "Creates a new bitfield instance without setting any default values.";

    quote! {
        #[doc = #documentation]
        #vis const fn new_without_defaults() -> Self {
            let mut this = Self(0);
            #setting_fields_to_zero_tokens
            this
        }
    }
}
