use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generation::common::generate_setting_fields_default_values_tokens;
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the default implementation.
pub(crate) fn generate_default_implementation_tokens(
    bitfield_struct_name: Ident,
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
) -> TokenStream {
    let setting_fields_default_values_tokens =
        generate_setting_fields_default_values_tokens(bitfield_type, fields, None);

    quote! {
        impl core::default::Default for #bitfield_struct_name {
            fn default() -> Self {
                let mut this = #bitfield_struct_name(0);
                #setting_fields_default_values_tokens
                this
            }
        }
    }
}
