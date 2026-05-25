use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::common::endian_conversion::generate_bits_variable_endian_conversion_tokens;
use crate::parsing::bitflags::bitflag::{Bitflag, BitflagVariant};
use crate::parsing::common::to_tokens::ToTokens;

pub fn generate_bitflag(bitflag: &Bitflag) -> TokenStream {
    let user_attributes_tokens = bitflag.user_attributes_tokens();
    let variants_tokens_list = generate_variants_tokens_list(bitflag);
    let visibility = bitflag.visibility().to_tokens();
    let name_tokens = bitflag.name_tokens();
    let from_bits_tokens = generate_from_bits_tokens(bitflag);
    let into_bits_tokens = generate_into_bits_tokens(bitflag);
    let repr_tokens = generate_repr_tokens(bitflag);
    let copy_derive_tokens = generate_copy_derive_tokens(bitflag);

    quote! {
        #repr_tokens
        #copy_derive_tokens
        #( #user_attributes_tokens )*
        #visibility enum #name_tokens {
            #( #variants_tokens_list, )*
        }

        impl #name_tokens {
            #from_bits_tokens
            #into_bits_tokens
        }
    }
}

fn generate_repr_tokens(bitflag: &Bitflag) -> TokenStream {
    let data_type_tokens = bitflag.spanned_data_type_token().to_tokens();
    quote! {
        #[repr(#data_type_tokens)]
    }
}

fn generate_copy_derive_tokens(bitflag: &Bitflag) -> TokenStream {
    if bitflag.arguments().derive_copy() {
        quote! {
            #[derive(std::marker::Copy, core::clone::Clone)]
        }
    } else {
        quote! {}
    }
}

fn generate_variants_tokens_list(bitflag: &Bitflag) -> Vec<TokenStream> {
    bitflag.variants().iter().map(generate_variants_tokens_helper).collect()
}

fn generate_variants_tokens_helper(bitflag_variant: &BitflagVariant) -> TokenStream {
    let name_tokens = bitflag_variant.name_tokens();
    let value_expr_tokens = bitflag_variant.value().to_tokens();
    let user_attributes_tokens = bitflag_variant.user_attributes_tokens();

    quote! {
        #( #user_attributes_tokens )*
        #name_tokens = #value_expr_tokens
    }
}

fn generate_from_bits_tokens(bitflag: &Bitflag) -> TokenStream {
    let visibility_tokens = bitflag.visibility().to_tokens();
    let bitflag_data_type_tokens = bitflag.spanned_data_type_token().to_tokens();
    let bits_variable_endian_conversion_tokens =
        generate_bits_variable_endian_conversion_tokens(bitflag.arguments().from_endian());
    let cases_tokens_list = generate_match_cases_tokens_list(bitflag);

    quote! {
        #[doc = "Creates a new bitflag instance from the given bits."]
        #visibility_tokens const fn from_bits(bits: #bitflag_data_type_tokens) -> Self {
            #bits_variable_endian_conversion_tokens
            match bits {
                #( #cases_tokens_list, )*
            }
        }
    }
}

fn generate_match_cases_tokens_list(bitflag: &Bitflag) -> Vec<TokenStream> {
    let mut match_cases: Vec<TokenStream> = bitflag
        .variants()
        .iter()
        .filter(|variant| !variant.base())
        .map(|variant| {
            let const_expr = variant.value().to_tokens();
            let name_tokens = variant.name_tokens();
            quote! {
                #const_expr => Self::#name_tokens
            }
        })
        .collect();

    let base_or_default_case = bitflag
        .variants()
        .into_iter()
        .find(BitflagVariant::base)
        .or_else(|| bitflag.variants().into_iter().find(BitflagVariant::default));
    if let Some(wildcard_case) = base_or_default_case {
        let name_tokens = wildcard_case.name_tokens();
        match_cases.push(quote! {
            _ => Self::#name_tokens
        });
    } else {
        panic!("No base or default bitflag variant")
    }

    match_cases
}

fn generate_into_bits_tokens(bitflag: &Bitflag) -> TokenStream {
    let visibility_tokens = bitflag.visibility().to_tokens();
    let bitflag_data_type_tokens = bitflag.spanned_data_type_token().to_tokens();
    let bits_variable_endian_conversion_tokens =
        generate_bits_variable_endian_conversion_tokens(bitflag.arguments().into_endian());

    quote! {
        #[doc = "Returns the bits of the bitflag."]
        #visibility_tokens const fn into_bits(self) -> #bitflag_data_type_tokens {
            let bits = self as #bitflag_data_type_tokens;
            #bits_variable_endian_conversion_tokens
            bits
        }
    }
}
