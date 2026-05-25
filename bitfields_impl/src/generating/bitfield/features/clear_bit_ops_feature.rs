use proc_macro2::TokenStream;
use quote::{ToTokens as QuoteTokens, format_ident, quote};

use crate::generating::bitfield::feature::{Feature, FeaturePosition, is_bit_ops_feature_enabled};
use crate::generating::bitfield::features::common::generator_helper::{
    generate_new_function_implementation_tokens, generate_setting_field_to_default_tokens,
    generate_setting_field_to_zero_tokens, get_bits_or_bytes_term, get_field_unit_terms,
    get_function_modifier_tokens,
};
use crate::parsing::bitfields::bitfield::{Bitfield, Field};
use crate::parsing::bitfields::bitfield_attribute::bitfield_arguments::BitOrder;
use crate::parsing::common::to_tokens::ToTokens;

/// Generates clear bit ops feature for the bitfield.
pub struct ClearBitOpsFeature;

impl Feature for ClearBitOpsFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_clear_bit_ops_feature_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        is_bit_ops_feature_enabled(
            bitfield,
            bitfield.arguments().generate_clear_bit_ops(),
            bitfield.arguments().user_set_clear_bit_ops(),
        )
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Inside
    }

    fn order_priority(&self) -> u32 {
        7
    }
}

impl ClearBitOpsFeature {
    fn generate_clear_bit_ops_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let clear_bitfield_bit_ops_tokens = Self::generate_clear_bitfield_bit_ops_tokens(bitfield);
        let clear_fields_bit_ops_tokens = Self::generate_clear_fields_bit_ops_tokens(bitfield);
        let clear_fields_to_defaults_bit_ops_tokens =
            Self::generate_clear_fields_to_defaults_bit_ops_tokens(bitfield);

        quote! {
            #clear_bitfield_bit_ops_tokens
            #clear_fields_bit_ops_tokens
            #clear_fields_to_defaults_bit_ops_tokens
        }
    }

    fn generate_clear_bitfield_bit_ops_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let clear_bitfield_implementation_tokens = generate_new_function_implementation_tokens(
            bitfield, /* generate_setting_defaults= */ false,
            /* builder_caller= */ false, /* existing_bitfield= */ true,
        );
        let clear_bitfield_and_set_defaults_implementation_tokens =
            generate_new_function_implementation_tokens(
                bitfield, /* generate_setting_defaults= */ true,
                /* builder_caller= */ false, /* existing_bitfield= */ true,
            );
        let bob = get_bits_or_bytes_term(bitfield);
        let clear_fn = format_ident!("clear_{}", bob);
        let clear_with_defaults_fn = format_ident!("clear_{}_with_defaults", bob);
        let clear_doc = format!("Clears all {bob} in the bitfield.");
        let clear_with_defaults_doc =
            format!("Clears all {bob} in the bitfield and set field defaults.");

        quote! {
            #[doc = #clear_doc]
            #visibility_tokens #function_modifier_tokens fn #clear_fn(&mut self) {
                let this = self;
                #clear_bitfield_implementation_tokens
            }

            #[doc = #clear_with_defaults_doc]
            #visibility_tokens #function_modifier_tokens fn #clear_with_defaults_fn(&mut self) {
                let this = self;
                #clear_bitfield_and_set_defaults_implementation_tokens
            }
        }
    }

    fn generate_clear_fields_bit_ops_tokens(bitfield: &Bitfield) -> TokenStream {
        bitfield
            .fields()
            .iter()
            .filter(|field| field.has_setter())
            .map(|field| {
                let visibility_tokens = bitfield.visibility().to_tokens();
                let function_modifier_tokens = get_function_modifier_tokens(bitfield);
                let documentation = Self::get_clear_field_documentation(bitfield, field);
                let clear_field_ident_tokens = format_ident!("clear_{}", field.name()).to_token_stream();
                let clear_field_implementation_tokens =
                    generate_setting_field_to_zero_tokens(bitfield, field);
                quote! {
                    #[doc = #documentation]
                    #visibility_tokens #function_modifier_tokens fn #clear_field_ident_tokens(&mut self) {
                        let this = self;
                        #clear_field_implementation_tokens
                    }
                }
            })
            .collect()
    }

    fn generate_clear_fields_to_defaults_bit_ops_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);

        bitfield.fields().iter().filter(|field| field.has_setter()).filter(|field: &&Field| field.has_default_value()).map(|field| {
            let documentation = Self::get_clear_field_documentation(bitfield, field);
            let clear_field_to_default_ident_tokens =  format_ident!("clear_{}_to_default", field.name()).to_token_stream();
            let clear_field_to_default_implementation_tokens = generate_setting_field_to_default_tokens(bitfield, field);
            quote! {
                #[doc = #documentation]
                #visibility_tokens #function_modifier_tokens fn #clear_field_to_default_ident_tokens(&mut self) {
                    let this = self;
                    #clear_field_to_default_implementation_tokens
                }
            }
        }).collect()
    }

    /// Returns clear field documentation.
    fn get_clear_field_documentation(bitfield: &Bitfield, field: &Field) -> String {
        let offset = field.offset();
        let (unit, units) = get_field_unit_terms(field);

        if field.bits() == 1 {
            return format!("Clears {unit} `{offset}`.");
        }

        let bits_end = offset + field.bits() - 1;

        let (documentation_bits_start, documentation_bits_end) =
            if bitfield.arguments().order() == BitOrder::Msb {
                (bits_end, offset)
            } else {
                (offset, bits_end)
            };

        format!("Clears {units} `{documentation_bits_start}..={documentation_bits_end}`.")
    }
}
