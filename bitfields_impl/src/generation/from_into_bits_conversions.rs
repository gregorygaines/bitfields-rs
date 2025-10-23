use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

use crate::generation::common::generate_setting_fields_from_bits_tokens;
use crate::parsing::bitfield_attribute::{BitfieldAttribute, Endian};
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the `from_bits` function for the bitfield.
pub(crate) fn generate_from_bits_function_tokens(
    vis: Visibility,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    bitfield_type: &syn::Type,
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_type,
        fields,
        Some(quote! { Self }),
        false,
        !ignored_fields.is_empty(),
        true, // from_bits should set read-only fields
    );

    let swap_bits_endian_tokens = (bitfield_attribute.from_endian == Endian::Little).then(|| {
        quote! {
            let bits = bits.swap_bytes();
        }
    });

    let ignored_fields_defaults = ignored_fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;
        quote! {
            #field_name: <#field_ty>::default(),
        }
    });

    let initialize_struct_tokens = if !ignored_fields.is_empty() {
        quote! {
            Self {
                val: 0,
                #( #ignored_fields_defaults )*
            }
        }
    } else {
        quote! {
            Self(0)
        }
    };

    let const_ident_tokens = ignored_fields.is_empty().then(|| quote! { const });

    quote! {
        #[doc = "Creates a new bitfield instance from the given bits."]
        #vis #const_ident_tokens fn from_bits(bits: #bitfield_type) -> Self {
            #swap_bits_endian_tokens
            let mut this = #initialize_struct_tokens;
            #setting_fields_from_bits_tokens
            this
        }
    }
}

/// Generates the `from_bits_with_defaults` function for the bitfield.
pub(crate) fn generate_from_bits_with_defaults_function_tokens(
    vis: Visibility,
    fields: &[BitfieldField],
    bitfield_type: &syn::Type,
    bitfield_attribute: &BitfieldAttribute,
    ignored_fields_struct: bool,
) -> TokenStream {
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_type,
        fields,
        Some(quote! { Self }),
        true,
        ignored_fields_struct,
        true, // from_bits_with_defaults should set read-only fields
    );

    let swap_bits_endian_tokens = (bitfield_attribute.from_endian == Endian::Little).then(|| {
        quote! {
            let bits = bits.swap_bytes();
        }
    });

    let const_ident_tokens = (!ignored_fields_struct).then(|| quote! { const });

    quote! {
        #[doc = "Creates a new bitfield instance from the given bits while respecting field default values."]
        #vis #const_ident_tokens fn from_bits_with_defaults(bits: #bitfield_type) -> Self {
            #swap_bits_endian_tokens
            let mut this = Self::from_bits(bits);
            #setting_fields_from_bits_tokens
            this
        }
    }
}

pub(crate) fn generate_into_bits_function_tokens(
    vis: Visibility,
    bitfield_attribute: &BitfieldAttribute,
    ignored_fields_struct: bool,
) -> TokenStream {
    let struct_val_ident = if ignored_fields_struct {
        quote! { self.val }
    } else {
        quote! { self.0 }
    };

    let into_bits_impl = match bitfield_attribute.into_endian {
        Endian::Big => {
            quote! {
                 #struct_val_ident
            }
        }
        Endian::Little => {
            quote! {
                 #struct_val_ident.swap_bytes()
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
