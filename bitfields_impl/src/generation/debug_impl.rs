use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generation::bit_manipulation_common::generate_extract_value_from_value_tokens;
use crate::generation::common::{
    BitfieldStructReferenceIdent, get_bitfield_struct_internal_value_identifier_tokens,
};
use crate::parsing::bitfield_attribute::{BitOrder, BitfieldAttribute};
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the debug implementation.
pub(crate) fn generate_debug_implementation(
    bitfield_struct_name: &Ident,
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
    has_ignored_fields: bool,
) -> TokenStream {
    let mut fields_msb_to_lsb = fields.to_owned();
    if bitfield_attribute.bit_order == BitOrder::Lsb {
        fields_msb_to_lsb.reverse()
    }
    let bitfield_struct_name_str = bitfield_struct_name.to_string();
    let mut debug_impl = Vec::new();
    debug_impl.push(quote! {
       let mut debug = f.debug_struct(#bitfield_struct_name_str);
    });

    let bitfield_struct_internal_value_ident = get_bitfield_struct_internal_value_identifier_tokens(
        &BitfieldStructReferenceIdent::SelfVariable,
        has_ignored_fields,
    );

    fields_msb_to_lsb.iter().for_each(|field| {
        let field_name = &field.name;
        let field_bits = field.bits;
        let field_offset = field.offset;
        let bitfield_type = &bitfield_attribute.ty;

        let value_extract = generate_extract_value_from_value_tokens(
            bitfield_type,
            quote! { #bitfield_struct_internal_value_ident },
            quote! { #field_bits },
            quote! { #field_offset },
            quote! { this },
            /* type_to_cast_output_value_ident= */ None,
            /* negate_source_value= */ false,
        );

        debug_impl.push(quote! {
            #value_extract
            debug.field(stringify!(#field_name), &this);
        });
    });

    debug_impl.push(quote! {
        debug.finish()
    });

    quote! {
        impl core::fmt::Debug for #bitfield_struct_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                #( #debug_impl )*
            }
        }
    }
}
