use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::common::generator_helper::{
    generate_bitfield_struct_initialization_tokens,
    generate_setting_fields_to_default_value_tokens_list,
};
use crate::parsing::bitfields::bitfield::Bitfield;

/// Generates `Default` trait implementation.
pub struct DefaultTraitFeature;

impl Feature for DefaultTraitFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_default_trait_feature_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        bitfield.arguments().generate_default()
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Below
    }

    fn order_priority(&self) -> u32 {
        1
    }
}

impl DefaultTraitFeature {
    fn generate_default_trait_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let bitfield_name_tokens = bitfield.name_tokens();
        let default_trait_feature_implementation_tokens =
            Self::generate_default_trait_feature_implementation_tokens(bitfield);

        quote! {
            impl core::default::Default for #bitfield_name_tokens {
                fn default() -> #bitfield_name_tokens {
                    #default_trait_feature_implementation_tokens
                }
            }
        }
    }

    fn generate_default_trait_feature_implementation_tokens(bitfield: &Bitfield) -> TokenStream {
        if bitfield.arguments().generate_new() {
            quote! {
                Self::new()
            }
        } else {
            let bitfield_struct_initialization_tokens =
                generate_bitfield_struct_initialization_tokens(
                    bitfield, /* builder_caller= */ false,
                );
            let setting_fields_to_default_value_tokens_list =
                generate_setting_fields_to_default_value_tokens_list(bitfield);

            quote! {
                let mut this = #bitfield_struct_initialization_tokens;
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }
        }
    }
}
