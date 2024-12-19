use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Type, Visibility};

use crate::parsing::bitfield_field::BitfieldField;

/// Generates the tokens for the bitfield tuple struct.
pub(crate) fn generate_tuple_struct_tokens(name: Ident, vis: Visibility, ty: Type) -> TokenStream {
    let documentation = "Represents a bitfield.";
    quote! {
        #[doc = #documentation]
        #vis struct #name (#ty);
    }
}

/// Generates the tokens for the bitfield tuple struct with fields.
pub(crate) fn generate_struct_with_fields_tokens(
    name: Ident,
    vis: Visibility,
    ty: Type,
    ignored_fields: &[BitfieldField],
) -> TokenStream {
    let field_tokens = ignored_fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;
        let field_vis = field.vis.as_ref();
        let field_vis = field_vis.as_ref().map(|v| quote!(#v)).unwrap_or_default();
        quote! {
            #field_vis #field_name: #field_ty,
        }
    });

    let documentation = "Represents a bitfield.";
    quote! {
        #[doc = #documentation]
        #vis struct #name {
            val: #ty,

            #( #field_tokens )*
        }
    }
}
