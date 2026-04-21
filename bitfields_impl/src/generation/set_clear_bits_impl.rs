use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

use crate::generation::common::{
    BitfieldStructReferenceIdent, generate_setting_fields_from_bits_tokens,
    generate_setting_fields_to_zero_tokens,
};
use crate::parsing::bitfield_attribute::BitfieldAttribute;
use crate::parsing::bitfield_field::{BitfieldField, FieldAccess};

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

    // If no fields have setter, than bits won't be used???
    let set_bit_bits_parameter_ident = if has_any_writable_non_padding_field(fields) {
        quote! { bits }
    } else {
        quote! { _ }
    };

    // If all fields have default values, then the bits parameter won't be used.
    let set_bits_with_defaults_bits_parameter_ident =
        if has_any_writable_non_padding_field_without_default(fields) {
            quote! { bits }
        } else {
            quote! { _ }
        };

    quote! {
        #[doc = "Sets the writable bits of the bitfield."]
        #vis fn set_bits(&mut self, #set_bit_bits_parameter_ident: #bitfield_type) {
            let this = self;
            #setting_fields_from_bits_without_respecting_defaults_tokens
        }

        #[doc = "Sets the writable bits of the bitfield while respecting defaults."]
        #vis fn set_bits_with_defaults(&mut self, #set_bits_with_defaults_bits_parameter_ident: #bitfield_type) {
            let this = self;
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

    // If all fields have default values, then the bits variable won't be used.
    let bits_variable_declaration_optional =
        if has_any_writable_non_padding_field_without_default(fields) {
            quote! { let bits = 0; }
        } else {
            quote! {}
        };
    quote! {
        #[doc = "Clears the writable bits of the bitfield."]
        #vis fn clear_bits(&mut self) {
            let this = self;
            #setting_fields_to_zero_tokens
        }

        #[doc = "Clears the writable bits of the bitfield."]
        #vis fn clear_bits_with_defaults(&mut self) {
            let this = self;
            #bits_variable_declaration_optional
            #setting_fields_from_bits_tokens
        }
    }
}

/// Returns if any field that is not padding and is writable (i.e. not
/// read-only).
fn has_any_writable_non_padding_field(fields: &[BitfieldField]) -> bool {
    fields.iter().any(|field| !field.padding && field.access != FieldAccess::ReadOnly)
}

/// Returns true if any writable non-padding field has no default value,
/// meaning the `bits` parameter/variable is needed in default-respecting
/// functions.
fn has_any_writable_non_padding_field_without_default(fields: &[BitfieldField]) -> bool {
    fields.iter().any(|field| {
        !field.padding
            && field.access != FieldAccess::ReadOnly
            && field.default_value_tokens.is_none()
    })
}
