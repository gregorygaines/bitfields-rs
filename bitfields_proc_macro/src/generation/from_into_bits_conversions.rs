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
    bitfield_type: &syn::Type,
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_type,
        fields,
        Some(quote! { Self }),
        false,
    );

    let swap_bits_endian_tokens = (bitfield_attribute.from_endian == Endian::Little).then(|| {
        quote! {
            let bits = bits.swap_bytes();
        }
    });

    quote! {
        #[doc = "Creates a new bitfield instance from the given bits."]
        #vis const fn from_bits(bits: #bitfield_type) -> Self {
            #swap_bits_endian_tokens
            let mut this = Self(0);
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
) -> TokenStream {
    let setting_fields_from_bits_tokens = generate_setting_fields_from_bits_tokens(
        bitfield_type,
        fields,
        Some(quote! { Self }),
        true,
    );

    let swap_bits_endian_tokens = (bitfield_attribute.from_endian == Endian::Little).then(|| {
        quote! {
            let bits = bits.swap_bytes();
        }
    });

    quote! {
        #[doc = "Creates a new bitfield instance from the given bits while respecting field default values."]
        #vis const fn from_bits_with_defaults(bits: #bitfield_type) -> Self {
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
) -> TokenStream {
    let into_bits_impl = match bitfield_attribute.into_endian {
        Endian::Big => {
            quote! {
                 self.0
            }
        }
        Endian::Little => {
            quote! {
                 self.0.swap_bytes()
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
