use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

use crate::generation::common::{
    generate_setting_fields_from_bits_tokens, generate_setting_fields_to_zero_tokens,
};
use crate::parsing::bitfield_field::BitfieldField;

pub(crate) fn generate_set_bits_function_tokens(
    vis: Visibility,
    fields: &[BitfieldField],
    bitfield_type: &syn::Type,
    ignored_fields_struct: bool,
) -> TokenStream {
    let documentation = "Sets the writable bits of the bitfield.";
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_type,
        fields,
        Some(quote! { Self }),
        false,
        ignored_fields_struct,
        false, // set_bits should NOT set read-only fields
    );

    quote! {
        #[doc = #documentation]
        #vis fn set_bits(&mut self, bits: #bitfield_type) {
            let mut this = self;
            #setting_fields_from_bits_tokens
        }
    }
}

pub(crate) fn generate_set_bits_with_defaults_function_tokens(
    vis: Visibility,
    fields: &[BitfieldField],
    bitfield_type: &syn::Type,
    ignored_fields_struct: bool,
) -> TokenStream {
    let documentation = "Sets the writable bits of the bitfield while respecting defaults.";
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_type,
        fields,
        Some(quote! { Self }),
        true,
        ignored_fields_struct,
        false, // set_bits_with_defaults should NOT set read-only fields
    );

    quote! {
        #[doc = #documentation]
        #vis fn set_bits_with_defaults(&mut self, bits: #bitfield_type) {
            let mut this = self;
            #setting_fields_from_bits_tokens
        }
    }
}

pub(crate) fn generate_clear_bits_function_tokens(
    vis: Visibility,
    fields: &[BitfieldField],
    bitfield_type: &syn::Type,
    ignored_fields_struct: bool,
) -> TokenStream {
    let documentation = "Clears the writable bits of the bitfield.";
    let setting_fields_to_zero_tokens =
        generate_setting_fields_to_zero_tokens(bitfield_type, fields, None, ignored_fields_struct);

    quote! {
        #[doc = #documentation]
        #vis fn clear_bits(&mut self) {
            let this = self;
            #setting_fields_to_zero_tokens
        }
    }
}

pub(crate) fn generate_clear_bits_preserve_defaults_function_tokens(
    vis: Visibility,
    fields: &[BitfieldField],
    bitfield_type: &syn::Type,
    ignored_fields_struct: bool,
) -> TokenStream {
    let documentation = "Clears the writable bits of the bitfield.";
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_type,
        fields,
        Some(quote! { Self }),
        true,
        ignored_fields_struct,
        false, // clear_bits_with_defaults should NOT set read-only fields
    );

    quote! {
        #[doc = #documentation]
        #vis fn clear_bits_with_defaults(&mut self) {
            let this = self;
            let bits = 0;
            #setting_fields_from_bits_tokens
        }
    }
}
