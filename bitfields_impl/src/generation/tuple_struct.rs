use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Visibility;

use crate::generation::common::get_bitfield_type_tokens;
use crate::parsing::bitfield_attribute::BitfieldAttribute;
use crate::parsing::bitfield_field::BitfieldField;

const BITFIELD_DOCUMENTATION: &str = "Represents a bitfield.";

/// Generates the tokens for the bitfield tuple struct.
pub(crate) fn generate_tuple_struct_tokens(
    name: &Ident,
    vis: &Visibility,
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let documentation = get_struct_documentation();
    let bitfield_type_tokens = get_bitfield_type_tokens(bitfield_attribute);
    quote! {
        #[doc = #documentation]
        #vis struct #name (#bitfield_type_tokens);
    }
}

/// Generates the tokens for the bitfield tuple struct with fields.
pub(crate) fn generate_struct_with_fields_tokens(
    name: &Ident,
    vis: &Visibility,
    ignored_fields: &[BitfieldField],
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let field_tokens = ignored_fields.iter().map(generate_struct_with_fields_tokens_helper);

    let documentation = get_struct_documentation();
    let bitfield_type_tokens = get_bitfield_type_tokens(bitfield_attribute);
    quote! {
        #[doc = #documentation]
        #vis struct #name {
            val: #bitfield_type_tokens,

            #( #field_tokens )*
        }
    }
}

fn generate_struct_with_fields_tokens_helper(field: &BitfieldField) -> TokenStream {
    let field_name = &field.name;
    let field_ty = &field.ty;
    let field_vis = field.vis.as_ref();
    let field_vis = field_vis.as_ref().map(|v| quote!(#v)).unwrap_or_default();
    quote! {
        #field_vis #field_name: #field_ty,
    }
}

fn get_struct_documentation() -> &'static str {
    BITFIELD_DOCUMENTATION
}
