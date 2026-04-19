use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

use crate::generation::common::{
    BitfieldStructReferenceIdent, generate_bitfield_struct_initialization_tokens,
    generate_setting_fields_from_bits_tokens, get_bitfield_struct_internal_value_identifier_tokens,
    get_const_modifier_tokens,
};
use crate::parsing::bitfield_attribute::{BitfieldAttribute, Endian};
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the `from_bits` and `from_bits_with_defaults` functions for the
/// bitfield.
pub(crate) fn generate_from_bits_functions_tokens(
    vis: &Visibility,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let has_ignored_fields = !ignored_fields.is_empty();

    let setting_fields_from_bits_without_respecting_defaults_tokens =
        generate_setting_fields_from_bits_tokens(
            bitfield_attribute,
            fields,
            &BitfieldStructReferenceIdent::SelfReference,
            /* respect_defaults= */ false,
            has_ignored_fields,
            /* include_read_only_fields= */ true,
        );
    let setting_fields_from_bits_respecting_defaults_tokens =
        generate_setting_fields_from_bits_tokens(
            bitfield_attribute,
            fields,
            &BitfieldStructReferenceIdent::SelfReference,
            /* respect_defaults= */ true,
            has_ignored_fields,
            /* include_read_only_fields= */ true,
        );

    let swap_bits_endian_tokens = (bitfield_attribute.from_endian == Endian::Little).then(|| {
        quote! {
            let bits = bits.swap_bytes();
        }
    });

    let initialize_struct_tokens = generate_bitfield_struct_initialization_tokens(
        ignored_fields,
        &BitfieldStructReferenceIdent::SelfReference,
    );

    let const_modifier_tokens = (!has_ignored_fields).then(get_const_modifier_tokens);
    let bitfield_type = &bitfield_attribute.ty;
    quote! {
        #[doc = "Creates a new bitfield instance from the given bits."]
        #vis #const_modifier_tokens fn from_bits(bits: #bitfield_type) -> Self {
            #swap_bits_endian_tokens
            let mut this = #initialize_struct_tokens;
            #setting_fields_from_bits_without_respecting_defaults_tokens
            this
        }

        #[doc = "Creates a new bitfield instance from the given bits while respecting field default values."]
        #vis #const_modifier_tokens fn from_bits_with_defaults(bits: #bitfield_type) -> Self {
            #swap_bits_endian_tokens
            let mut this = Self::from_bits(bits);
            #setting_fields_from_bits_respecting_defaults_tokens
            this
        }
    }
}

pub(crate) fn generate_into_bits_function_tokens(
    vis: &Visibility,
    bitfield_attribute: &BitfieldAttribute,
    has_ignored_fields: bool,
) -> TokenStream {
    let bitfield_struct_internal_value_ident = get_bitfield_struct_internal_value_identifier_tokens(
        &BitfieldStructReferenceIdent::SelfVariable,
        has_ignored_fields,
    );

    let into_bits_impl = match bitfield_attribute.into_endian {
        Endian::Big => {
            quote! {
                 #bitfield_struct_internal_value_ident
            }
        }
        Endian::Little => {
            quote! {
                 #bitfield_struct_internal_value_ident.swap_bytes()
            }
        }
    };

    let bitfield_type = &bitfield_attribute.ty;
    quote! {
        #[doc = "Returns the bits of the bitfield."]
        #vis const fn into_bits(self) -> #bitfield_type {
            #into_bits_impl
        }
    }
}
