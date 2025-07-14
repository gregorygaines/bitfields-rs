use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

use crate::generation::common::{
    does_field_have_getter, does_field_have_setter, supports_const_mut_refs,
};
use crate::parsing::bitfield_field::BitfieldField;
use crate::parsing::types::get_bits_from_type;

pub(crate) fn generate_get_bit_tokens(
    vis: Visibility,
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
    ignored_fields_struct: bool,
) -> TokenStream {
    let bitfield_type_bits = get_bits_from_type(bitfield_type).unwrap() as usize;
    let get_bit_documentation = "Returns a bit from the given index. Returns false for out-of-bounds and fields without read access.".to_string();
    let checked_get_bit_documentation = "Returns a bit from the given index. Returns an error for out-of-bounds and fields without read access.".to_string();

    let false_return_for_non_readable_fields = fields
        .iter()
        .filter(|field| !does_field_have_getter(field) && !field.padding)
        .map(|field| {
            let field_bits = field.bits as usize;
            let field_offset = field.offset as usize;
            let field_end_bits = field_offset + field_bits;
            quote! {
                if index >= #field_offset && index < #field_end_bits {
                    return false;
                }
            }
        });

    let error_return_for_write_only_fields = fields
        .iter()
        .filter(|field| !does_field_have_getter(field) && !field.padding)
        .map(|field| {
            let field_bits = field.bits as usize;
            let field_offset = field.offset as usize;
            let field_end_bits = field_offset + field_bits;
            quote! {
                if index >= #field_offset && index < #field_end_bits {
                    return Err("Can't read from a write-only field.");
                }
            }
        });

    let struct_val_ident = if ignored_fields_struct {
        quote! { self.val }
    } else {
        quote! { self.0 }
    };

    quote! {
        #[doc = #get_bit_documentation]
        #vis const fn get_bit(&self, index: usize) -> bool {
            if index > #bitfield_type_bits {
                return false;
            }

            #( #false_return_for_non_readable_fields )*

            (#struct_val_ident >> index) & 1 != 0
        }

        #[doc = #checked_get_bit_documentation]
        #vis const fn checked_get_bit(&self, index: usize) -> Result<bool, &'static str> {
            if index > #bitfield_type_bits {
                return Err("Index out of bounds.");
            }

            #( #error_return_for_write_only_fields )*

            Ok((#struct_val_ident >> index) & 1 != 0)
        }
    }
}

pub(crate) fn generate_set_bit_tokens(
    vis: Visibility,
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
    ignored_fields_struct: bool,
) -> TokenStream {
    let bitfield_type_bits = get_bits_from_type(bitfield_type).unwrap() as usize;
    let set_bit_documentation = "Sets a bit at given index with the given value. Is no-op for out-of-bounds and fields without write access.".to_string();
    let checked_set_bit_documentation = "Sets a bit at given index with the given value. Returns an error for out-of-bounds and fields without write access.".to_string();

    let no_op_for_non_writable_fields =
        fields.iter().filter(|field| !does_field_have_setter(field)).map(|field| {
            let field_bits = field.bits as usize;
            let field_offset = field.offset as usize;
            let field_end_bits = field_offset + field_bits;
            quote! {
                if index >= #field_offset && index < #field_end_bits {
                    return;
                }
            }
        });

    let error_return_for_non_writable_fields =
        fields.iter().filter(|field| !does_field_have_setter(field)).map(|field| {
            let field_bits = field.bits as usize;
            let field_offset = field.offset as usize;
            let field_end_bits = field_offset + field_bits;
            quote! {
                if index >= #field_offset && index < #field_end_bits {
                    return Err("Can't write to a non-writable or padding field.");
                }
            }
        });

    let struct_val_ident = if ignored_fields_struct {
        quote! { self.val }
    } else {
        quote! { self.0 }
    };

    let constness = supports_const_mut_refs().then(|| quote! { const });

    quote! {
        #[doc = #set_bit_documentation]
        #vis #constness fn set_bit(&mut self, index: usize, bit: bool) {
            if index > #bitfield_type_bits {
                return;
            }

            #( #no_op_for_non_writable_fields )*

            if bit {
                #struct_val_ident |= 1 << index;
            } else {
                #struct_val_ident &= !(1 << index);
            }
        }

        #[doc = #checked_set_bit_documentation]
        #vis #constness fn checked_set_bit(&mut self, index: usize, bit: bool) -> Result<(), &'static str> {
            if index > #bitfield_type_bits {
                return Err("Index out of bounds.");
            }

            #( #error_return_for_non_writable_fields )*

            if bit {
                #struct_val_ident |= 1 << index;
            } else {
                #struct_val_ident &= !(1 << index);
            }

            Ok(())
        }
    }
}
