use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::parsing::bitfield_attribute::{BitOrder, BitfieldAttribute};
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the debug implementation.
pub(crate) fn generate_debug_implementation(
    bitfield_struct_name: Ident,
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
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

    fields_msb_to_lsb.iter().for_each(|field| {
        let field_name = &field.name;
        let field_bits = field.bits as usize;
        let field_offset = field.offset as usize;
        let bitfield_type = &bitfield_attribute.ty;

        debug_impl.push(quote! {
            let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - #field_bits as u32);
            let this = ((self.0 >> #field_offset) & mask) as #bitfield_type;
            debug.field(stringify!(#field_name), &((self.0 >> #field_offset) & mask));
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
