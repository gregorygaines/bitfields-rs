use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

use crate::generation::common::{
    BitfieldStructReferenceIdent, does_field_have_getter, does_field_have_setter,
    get_bitfield_struct_internal_value_identifier_tokens,
};
use crate::parsing::bitfield_field::BitfieldField;
use crate::parsing::types::get_bits_from_type;

#[derive(Copy, Clone)]
enum ReturnType {
    False,
    NoOp,
    Error,
}

pub(crate) fn generate_get_bit_tokens(
    vis: &Visibility,
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
    has_ignored_fields: bool,
) -> TokenStream {
    let (false_return_for_non_readable_fields, error_return_for_write_only_fields): (
        Vec<TokenStream>,
        Vec<TokenStream>,
    ) = fields
        .iter()
        .filter(|field| !does_field_have_getter(field) && !field.padding)
        .map(|field| {
            generate_field_guard_tokens(
                field,
                /* no_op_return_false= */ true,
                "Can't read from a write-only field.",
            )
        })
        .unzip();

    let bitfield_struct_internal_value_ident = get_bitfield_struct_internal_value_identifier_tokens(
        &BitfieldStructReferenceIdent::SelfVariable,
        has_ignored_fields,
    );

    let guard_clause_tokens = generate_guard_clause_tokens(bitfield_type, ReturnType::False);
    let checked_guard_clause_tokens =
        generate_guard_clause_tokens(bitfield_type, ReturnType::Error);

    quote! {
        #[doc = "Returns a bit from the given index. Returns false for out-of-bounds and fields without read access."]
        #vis const fn get_bit(&self, index: usize) -> bool {
            #guard_clause_tokens

            #( #false_return_for_non_readable_fields )*

            (#bitfield_struct_internal_value_ident >> index) & 1 != 0
        }

        #[doc = "Returns a bit from the given index. Returns an error for out-of-bounds and fields without read access."]
        #vis const fn checked_get_bit(&self, index: usize) -> ::core::result::Result<bool, &'static str> {
            #checked_guard_clause_tokens

            #( #error_return_for_write_only_fields )*

            Ok((#bitfield_struct_internal_value_ident >> index) & 1 != 0)
        }
    }
}

pub(crate) fn generate_set_bit_tokens(
    vis: &Visibility,
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
    has_ignored_fields: bool,
) -> TokenStream {
    let (no_op_for_non_writable_fields, error_return_for_non_writable_fields): (
        Vec<TokenStream>,
        Vec<TokenStream>,
    ) = fields
        .iter()
        .filter(|field| !does_field_have_setter(field))
        .map(|field| {
            generate_field_guard_tokens(
                field,
                /* no_op_return_false= */ false,
                "Can't write to a non-writable or padding field.",
            )
        })
        .unzip();

    let bitfield_struct_internal_value_ident = get_bitfield_struct_internal_value_identifier_tokens(
        &BitfieldStructReferenceIdent::SelfVariable,
        has_ignored_fields,
    );

    let guard_clause_tokens = generate_guard_clause_tokens(bitfield_type, ReturnType::NoOp);
    let checked_guard_clause_tokens =
        generate_guard_clause_tokens(bitfield_type, ReturnType::Error);

    quote! {
        #[doc = "Sets a bit at given index with the given value. Is no-op for out-of-bounds and fields without write access."]
        #vis const fn set_bit(&mut self, index: usize, bit: bool) {
            #guard_clause_tokens

            #( #no_op_for_non_writable_fields )*

            if bit {
                #bitfield_struct_internal_value_ident |= 1 << index;
            } else {
                #bitfield_struct_internal_value_ident &= !(1 << index);
            }
        }

        #[doc = "Sets a bit at given index with the given value. Returns an error for out-of-bounds and fields without write access."]
        #vis const fn checked_set_bit(&mut self, index: usize, bit: bool) -> ::core::result::Result<(), &'static str> {
            #checked_guard_clause_tokens

            #( #error_return_for_non_writable_fields )*

            if bit {
                #bitfield_struct_internal_value_ident |= 1 << index;
            } else {
                #bitfield_struct_internal_value_ident &= !(1 << index);
            }

            Ok(())
        }
    }
}

fn generate_guard_clause_tokens(bitfield_type: &syn::Type, return_type: ReturnType) -> TokenStream {
    let bitfield_type_bits = get_bits_from_type(bitfield_type).unwrap() as usize;

    match return_type {
        ReturnType::False => {
            quote! {
                if index > #bitfield_type_bits {
                    return false;
                }
            }
        }
        ReturnType::NoOp => {
            quote! {
                if index > #bitfield_type_bits {
                    return;
                }
            }
        }
        ReturnType::Error => {
            quote! {
                if index > #bitfield_type_bits {
                    return Err("Index out of bounds.");
                }
            }
        }
    }
}

/// Generates guard tokens for fields that should be restricted (no-op/false
/// return + error return).
fn generate_field_guard_tokens(
    field: &BitfieldField,
    no_op_return_false: bool,
    error_message: &str,
) -> (TokenStream, TokenStream) {
    let field_bits = field.bits as usize;
    let field_offset = field.offset as usize;
    let field_end_bits = field_offset + field_bits;

    (generate_no_op_return_tokens(field_offset, field_end_bits, no_op_return_false), {
        quote! {
            if index >= #field_offset && index < #field_end_bits {
                return Err(#error_message);
            }
        }
    })
}

fn generate_no_op_return_tokens(
    field_offset: usize,
    field_end_bits: usize,
    return_false: bool,
) -> TokenStream {
    let return_tokens = if return_false {
        quote! {
            return false;
        }
    } else {
        quote! {
            return;
        }
    };

    quote! {
        if index >= #field_offset && index < #field_end_bits {
            #return_tokens
        }
    }
}
