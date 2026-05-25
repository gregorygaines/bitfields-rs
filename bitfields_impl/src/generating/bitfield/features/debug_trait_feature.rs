use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::common::generator_helper::{
    BitsSource, generate_extract_field_bits_from_source_into_variable_tokens,
};
use crate::parsing::bitfields::bitfield::Bitfield;

/// Generates `Debug` trait implementation.
pub struct DebugTraitFeature;

impl Feature for DebugTraitFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_default_trait_feature_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        bitfield.arguments().generate_debug()
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Below
    }

    fn order_priority(&self) -> u32 {
        2
    }
}

impl DebugTraitFeature {
    /// Generates debug trait feature tokens.
    fn generate_default_trait_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let bitfield_name_tokens = bitfield.name_tokens();
        let bitfield_name = bitfield.name();
        let set_debug_fields = Self::generate_debug_set_field_tokens(bitfield);

        quote! {
            impl core::fmt::Debug for #bitfield_name_tokens {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    let this = self;
                    let mut debug = f.debug_struct(#bitfield_name);
                    #set_debug_fields
                    debug.finish()
                }
            }
        }
    }

    fn generate_debug_set_field_tokens(bitfield: &Bitfield) -> TokenStream {
        bitfield
            .fields()
            .iter()
            .map(|field| {
                let field_name = field.name();
                let extract_field_bits_from_bitfield_into_variable =
                    generate_extract_field_bits_from_source_into_variable_tokens(
                        bitfield,
                        field,
                        BitsSource::Bitfield,
                        /* cast_bits= */ false,
                        /* invert_bits= */ false,
                        /* builder_caller= */ false,
                    );

                quote! {
                    #extract_field_bits_from_bitfield_into_variable
                    debug.field(#field_name, &value);
                }
            })
            .collect()
    }
}
