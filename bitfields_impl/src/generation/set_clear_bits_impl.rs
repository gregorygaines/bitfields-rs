use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

use crate::generation::common::{
    BitfieldStructReferenceIdent, generate_setting_fields_from_bits_tokens,
    generate_setting_fields_to_zero_tokens,
};
use crate::parsing::bitfield_attribute::BitfieldAttribute;
use crate::parsing::bitfield_field::BitfieldField;

/// Generates `set_bits` and `set_bits_with_defaults` functions.
pub(crate) fn generate_set_bits_functions_tokens(
    vis: &Visibility,
    fields: &[BitfieldField],
    bitfield_attribute: &BitfieldAttribute,
    has_ignored_fields: bool,
) -> TokenStream {
    let setting_fields_from_bits_without_respecting_defaults_tokens =
        generate_setting_fields_from_bits_tokens(
            bitfield_attribute,
            fields,
            &BitfieldStructReferenceIdent::SelfReference,
            /* respect_defaults= */ false,
            has_ignored_fields,
            /* include_read_only_fields= */ false,
        );
    let setting_fields_from_bits_respecting_defaults_tokens =
        generate_setting_fields_from_bits_tokens(
            bitfield_attribute,
            fields,
            &BitfieldStructReferenceIdent::SelfReference,
            /* respect_defaults= */ true,
            has_ignored_fields,
            /* include_read_only_fields= */ false,
        );

    let bitfield_type = &bitfield_attribute.ty;
    quote! {
        #[doc = "Sets the writable bits of the bitfield."]
        #vis fn set_bits(&mut self, bits: #bitfield_type) {
            let mut this = self;
            #setting_fields_from_bits_without_respecting_defaults_tokens
        }

        #[doc = "Sets the writable bits of the bitfield while respecting defaults."]
        #vis fn set_bits_with_defaults(&mut self, bits: #bitfield_type) {
            let mut this = self;
            #setting_fields_from_bits_respecting_defaults_tokens
        }
    }
}

pub(crate) fn generate_clear_bits_functions_tokens(
    vis: &Visibility,
    fields: &[BitfieldField],
    bitfield_attribute: &BitfieldAttribute,
    has_ignored_fields: bool,
) -> TokenStream {
    let setting_fields_to_zero_tokens = generate_setting_fields_to_zero_tokens(
        &bitfield_attribute.ty,
        fields,
        &BitfieldStructReferenceIdent::SelfReference,
        has_ignored_fields,
    );
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_attribute,
        fields,
        &BitfieldStructReferenceIdent::SelfReference,
        /* respect_defaults= */ true,
        has_ignored_fields,
        /* include_read_only_fields= */ false,
    );

    quote! {
        #[doc = "Clears the writable bits of the bitfield."]
        #vis fn clear_bits(&mut self) {
            let this = self;
            #setting_fields_to_zero_tokens
        }

        #[doc = "Clears the writable bits of the bitfield."]
        #vis fn clear_bits_with_defaults(&mut self) {
            let this = self;
            let bits = 0;
            #setting_fields_from_bits_tokens
        }
    }
}
