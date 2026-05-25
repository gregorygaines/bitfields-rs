use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::bitfield_struct_feature::BitfieldStructFeatureGenerator;
use crate::generating::bitfield::features::builder_feature::BuilderFeature;
use crate::generating::bitfield::features::clear_bit_ops_feature::ClearBitOpsFeature;
use crate::generating::bitfield::features::debug_trait_feature::DebugTraitFeature;
use crate::generating::bitfield::features::default_trait_feature::DefaultTraitFeature;
use crate::generating::bitfield::features::field_constants_feature::FieldConstantsFeature;
use crate::generating::bitfield::features::field_getters_feature::FieldGettersFeature;
use crate::generating::bitfield::features::field_setters_feature::FieldSettersFeature;
use crate::generating::bitfield::features::from_into_bits_feature::FromIntoBitsFeature;
use crate::generating::bitfield::features::from_traits_feature::FromTraitsFeature;
use crate::generating::bitfield::features::invert_bit_ops_feature::InvertBitOpsFeature;
use crate::generating::bitfield::features::new_functions_feature::NewFunctionsFeature;
use crate::generating::bitfield::features::set_get_bit_ops_feature::SetGetBitOpsFeature;
use crate::generating::bitfield::features::write_bit_ops_feature::WriteBitOpsFeature;
use crate::parsing::bitfields::bitfield::Bitfield;

/// Represents the generated features partitioned into their proper sections.
struct GeneratedFeatures {
    /// Generated features that are above the `impl` block.
    above_features: Vec<TokenStream>,

    /// Generated features that are inside the `impl` block.
    inside_features: Vec<TokenStream>,

    /// Generated features that are below the `impl` block.
    below_features: Vec<TokenStream>,
}

/// Generates a bitfield.
pub fn generate_bitfield(bitfield: &Bitfield) -> TokenStream {
    let enabled_features = get_enabled_features(bitfield);
    let generated_features = generate_features(&enabled_features, bitfield);
    do_generate_bitfield(bitfield, &generated_features)
}

/// Returns a list of enabled feature generators.
fn get_enabled_features(bitfield: &Bitfield) -> Vec<Box<dyn Feature>> {
    let mut features: Vec<Box<dyn Feature>> = vec![
        Box::new(BitfieldStructFeatureGenerator),
        Box::new(FieldConstantsFeature),
        Box::new(NewFunctionsFeature),
        Box::new(FieldSettersFeature),
        Box::new(FieldGettersFeature),
        Box::new(FromTraitsFeature),
        Box::new(DebugTraitFeature),
        Box::new(FromIntoBitsFeature),
        Box::new(BuilderFeature),
        Box::new(DefaultTraitFeature),
        Box::new(WriteBitOpsFeature),
        Box::new(SetGetBitOpsFeature),
        Box::new(ClearBitOpsFeature),
        Box::new(InvertBitOpsFeature),
    ];
    features.sort_by_key(|a| a.order_priority());
    features.into_iter().filter(|f| f.enabled(bitfield)).collect()
}

/// Returns the features partitioned into their proper sections.
fn generate_features(features: &[Box<dyn Feature>], bitfield: &Bitfield) -> GeneratedFeatures {
    GeneratedFeatures {
        above_features: generate_features_helper(features, bitfield, FeaturePosition::Above),
        inside_features: generate_features_helper(features, bitfield, FeaturePosition::Inside),
        below_features: generate_features_helper(features, bitfield, FeaturePosition::Below),
    }
}

/// Helper for generating features of a specific position.
fn generate_features_helper(
    features: &[Box<dyn Feature>],
    bitfield: &Bitfield,
    position: FeaturePosition,
) -> Vec<TokenStream> {
    features
        .iter()
        .filter(|f| f.feature_position() == position)
        .map(|f| f.generate_feature(bitfield))
        .collect()
}

/// Returns the final bitfield generation results.
fn do_generate_bitfield(
    bitfield: &Bitfield,
    generated_features: &GeneratedFeatures,
) -> TokenStream {
    let bitfield_name_tokens = bitfield.name_tokens();

    let features_above_impl_block = &generated_features.above_features;
    let features_inside_impl_block = &generated_features.inside_features;
    let features_below_impl_block = &generated_features.below_features;

    quote! {
         #( #features_above_impl_block )*
            #[allow(clippy::manual_swap)]
            impl #bitfield_name_tokens {
                #( #features_inside_impl_block )*
            }
            #( #features_below_impl_block )*
    }
}
