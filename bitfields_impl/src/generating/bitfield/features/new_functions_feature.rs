use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::common::generator_helper::{
    generate_new_function_implementation_tokens, get_function_modifier_tokens,
};
use crate::parsing::bitfields::bitfield::Bitfield;
use crate::parsing::common::to_tokens::ToTokens;

/// Generator for new initialization functions.
///
/// Examples:
///
/// ```rust,ignore
/// pub const fn new() { ... }
/// pub const fn new_without_defaults() { ... }
/// ```
pub struct NewFunctionsFeature;

impl Feature for NewFunctionsFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_new_functions_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        bitfield.arguments().generate_new()
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Inside
    }

    fn order_priority(&self) -> u32 {
        1
    }
}

impl NewFunctionsFeature {
    /// Generates new functions tokens.
    fn generate_new_functions_tokens(bitfield: &Bitfield) -> TokenStream {
        let new_function_feature_tokens = Self::generate_new_function_feature_tokens(bitfield);
        let new_without_defaults_function_feature_tokens =
            Self::generate_new_without_defaults_function_feature_tokens(bitfield);

        quote! {
            #new_function_feature_tokens
            #new_without_defaults_function_feature_tokens
        }
    }

    /// Generate `new` function tokens.
    fn generate_new_function_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let function_implementation = generate_new_function_implementation_tokens(
            bitfield, /* generate_setting_defaults= */ true, /* builder_caller= */ false,
            /* existing_bitfield= */ false,
        );

        quote! {
            #[doc = "Creates a new bitfield instance."]
            #visibility_tokens #function_modifier_tokens fn new() -> Self {
                #function_implementation
                this
            }
        }
    }

    /// Generates `new_without_defaults` function tokens.
    fn generate_new_without_defaults_function_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let function_implementation = generate_new_function_implementation_tokens(
            bitfield, /* generate_setting_defaults= */ false,
            /* builder_caller= */ false, /* existing_bitfield= */ false,
        );

        quote! {
            #[doc = "Creates a new bitfield instance without respecting \
                     default values."]
            #visibility_tokens #function_modifier_tokens fn new_without_defaults() -> Self {
                #function_implementation
                this
            }
        }
    }
}
