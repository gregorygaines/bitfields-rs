use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::common::generator_helper::get_field_unit_terms;
use crate::parsing::bitfields::bitfield::Bitfield;
use crate::parsing::common::to_tokens::ToTokens;

/// Generates field constants.
///
/// # Example
///
/// ```rust,ignore
/// const FIELD_BITS = 3;
/// const FIELD_OFFSET = 4;
/// ```
#[derive(Default)]
pub struct FieldConstantsFeature;

impl Feature for FieldConstantsFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_field_constants_feature_tokens(bitfield)
    }

    fn enabled(&self, _: &Bitfield) -> bool {
        true
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Inside
    }

    fn order_priority(&self) -> u32 {
        0
    }
}

impl FieldConstantsFeature {
    /// Generate constant declarations for each field (bits and offset).
    ///
    /// Example:
    /// - const `A_BITS`: u32 = 8u32;
    /// - const `A_OFFSET`: u32 = 0u32;
    fn generate_field_constants_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        bitfield
            .fields()
            .iter()
            .filter(|field| field.has_constants())
            .map(|field| {
                let visibility_tokens = field.visibility().to_tokens();
                let field_bits = field.bits();
                let field_offset = field.offset();
                let field_bits_constant_ident_tokens = field.bits_constant_ident_tokens();
                let field_offset_constant_ident_tokens = field.offset_constant_ident_tokens();

                let (unit, units) = get_field_unit_terms(field);
                let bits_documentation =
                    format!("The number of {units} `{}` occupies in the bitfield.", field.name());
                let offset_documentation =
                    format!("The {unit} offset of `{}` in the bitfield.", field.name());
                quote! {
                    #[doc = #bits_documentation]
                    #visibility_tokens const #field_bits_constant_ident_tokens: u32 = #field_bits;
                    #[doc = #offset_documentation]
                    #visibility_tokens const #field_offset_constant_ident_tokens: u32 = #field_offset;
        }
            })
            .collect()
    }
}
