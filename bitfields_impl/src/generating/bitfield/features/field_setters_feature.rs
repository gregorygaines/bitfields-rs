use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::common::generator_helper::{
    generate_setting_field_from_variable_tokens, get_function_modifier_tokens,
    get_setter_documentation,
};
use crate::parsing::bitfields::bitfield::{Bitfield, Field};
use crate::parsing::common::to_tokens::ToTokens;

/// Generates setters for fields.
///
/// # Example
///
/// let bitfield = ...
/// `bitfield.set_a(99)`;
#[derive(Default)]
pub struct FieldSettersFeature;

impl Feature for FieldSettersFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_field_setters_feature_tokens(bitfield)
    }

    fn enabled(&self, _: &Bitfield) -> bool {
        true
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Inside
    }

    fn order_priority(&self) -> u32 {
        4
    }
}

impl FieldSettersFeature {
    /// Generate the field setters functions.
    fn generate_field_setters_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        bitfield
            .fields()
            .iter()
            .filter(|field: &&Field| field.has_setter())
            .map(|field| Self::generate_field_setters_functions(bitfield, field))
            .collect()
    }

    /// Generates the setter and checked setter functions.
    fn generate_field_setters_functions(bitfield: &Bitfield, field: &Field) -> TokenStream {
        let visibility_tokens = field.visibility().to_tokens();
        let setter_documentation = get_setter_documentation(
            bitfield, field, /* checked_setter= */ false, /* builder_caller= */ false,
        );
        let checked_setter_documentation = get_setter_documentation(
            bitfield, field, /* checked_setter= */ true, /* builder_caller= */ false,
        );
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let field_data_type_tokens = field.spanned_data_type_token().to_tokens();
        let field_setter_ident_tokens = field.setter_ident_tokens();
        let checked_field_setter_ident_tokens = field.checked_setter_ident_tokens();
        let set_bits_logic_tokens = generate_setting_field_from_variable_tokens(
            bitfield, field, /* use_setter= */ false, /* cast_bits= */ true,
            /* check_bit_size= */ false, /* builder_caller= */ false,
        );
        let checked_set_bits_logic_tokens = generate_setting_field_from_variable_tokens(
            bitfield, field, /* use_setter= */ false, /* cast_bits= */ true,
            /* check_bit_size= */ true, /* builder_caller= */ false,
        );

        quote! {
            #[doc = #setter_documentation]
            #visibility_tokens #function_modifier_tokens fn #field_setter_ident_tokens(&mut self, bits: #field_data_type_tokens) {
                let this = self;
                #set_bits_logic_tokens
            }

            #[doc = #checked_setter_documentation]
            #visibility_tokens #function_modifier_tokens fn #checked_field_setter_ident_tokens(&mut self, bits: #field_data_type_tokens) -> ::core::result::Result<(), &'static str> {
                let this = self;
                #checked_set_bits_logic_tokens
                Ok(())
            }
        }
    }
}
