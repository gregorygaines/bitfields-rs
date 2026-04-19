use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generation::common::{
    BitfieldStructReferenceIdent, generate_bitfield_struct_initialization_tokens,
    generate_setting_fields_from_bits_tokens, get_bitfield_struct_internal_value_identifier_tokens,
};
use crate::parsing::bitfield_attribute::{BitfieldAttribute, Endian};
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the `From` trait implementation.
pub(crate) fn generate_from_bitfield_type_for_bitfield_implementation_tokens(
    bitfield_struct_name: &Ident,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_attribute,
        fields,
        &BitfieldStructReferenceIdent::SelfReference,
        /* respect_defaults= */ false,
        !ignored_fields.is_empty(),
        /* include_read_only_fields= */ true,
    );

    let initialize_struct_tokens = generate_bitfield_struct_initialization_tokens(
        ignored_fields,
        &BitfieldStructReferenceIdent::SelfReference,
    );
    let bitfield_type = &bitfield_attribute.ty;
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
    bitfield_struct_name: &Ident,
    bitfield_attribute: &BitfieldAttribute,
    has_ignored_fields: bool,
) -> TokenStream {
    let bitfield_type = &bitfield_attribute.ty;
    let internal_value_identifier = get_bitfield_struct_internal_value_identifier_tokens(
        &BitfieldStructReferenceIdent::NameReference("b".to_string()),
        has_ignored_fields,
    );

    let from_bitfield_impl = match bitfield_attribute.into_endian {
        Endian::Big => internal_value_identifier,
        Endian::Little => {
            quote! {
                #internal_value_identifier.swap_bytes()
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
