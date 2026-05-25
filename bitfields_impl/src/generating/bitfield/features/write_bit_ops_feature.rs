use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::generating::bitfield::feature::{Feature, FeaturePosition, is_bit_ops_feature_enabled};
use crate::generating::bitfield::features::common::generator_helper::{
    ProtectionType, generate_backing_data_param_ident,
    generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens,
    generate_setting_fields_to_default_value_tokens_list, get_bits_or_bytes_term,
    get_function_modifier_tokens,
};
use crate::parsing::bitfields::bitfield::Bitfield;
use crate::parsing::common::conversion_endian::ConversionEndian;
use crate::parsing::common::to_tokens::ToTokens;

/// Generates write bit operations for bitfield.
pub struct WriteBitOpsFeature;

impl Feature for WriteBitOpsFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_write_bit_ops_feature_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        is_bit_ops_feature_enabled(
            bitfield,
            bitfield.arguments().generate_bit_ops(),
            bitfield.arguments().generate_set_get_bit_ops(),
        )
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Inside
    }

    fn order_priority(&self) -> u32 {
        5
    }
}

impl WriteBitOpsFeature {
    fn generate_write_bit_ops_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens =
            generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens(
                bitfield,
                ProtectionType::ReadOnly,
            );
        let setting_fields_to_default_value_tokens_list =
            generate_setting_fields_to_default_value_tokens_list(bitfield);
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bits_variable_endian_conversion_tokens =
            Self::generate_bits_variable_endian_conversion_tokens(
                bitfield,
                bitfield.arguments().write_endian(),
            );
        let le_endian_conversion_tokens = Self::generate_bits_variable_endian_conversion_tokens(
            bitfield,
            ConversionEndian::Little,
        );
        let be_endian_conversion_tokens =
            Self::generate_bits_variable_endian_conversion_tokens(bitfield, ConversionEndian::Big);
        let source_param = generate_backing_data_param_ident(bitfield);
        let bob = get_bits_or_bytes_term(bitfield);
        let write_fn = format_ident!("write_{}", bob);
        let write_with_defaults_fn = format_ident!("write_{}_with_defaults", bob);
        let write_le_fn = format_ident!("write_le_{}", bob);
        let write_le_with_defaults_fn = format_ident!("write_le_{}_with_defaults", bob);
        let write_be_fn = format_ident!("write_be_{}", bob);
        let write_be_with_defaults_fn = format_ident!("write_be_{}_with_defaults", bob);
        let write_doc = format!("Writes {bob} to the bitfield.");
        let write_with_defaults_doc =
            format!("Writes {bob} to the bitfield while setting defaults.");
        let write_le_doc = format!("Writes little-endian {bob} to the bitfield.");
        let write_le_with_defaults_doc =
            format!("Writes little-endian {bob} to the bitfield while setting defaults.");
        let write_be_doc = format!("Writes big-endian {bob} to the bitfield.");
        let write_be_with_defaults_doc =
            format!("Writes big-endian {bob} to the bitfield while setting defaults.");

        quote! {
            #[doc = #write_doc]
            #visibility_tokens #function_modifier_tokens fn #write_fn(&mut self, #source_param: #bitfield_data_type_tokens) {
                let this = self;
                let bits = #source_param;
                #bits_variable_endian_conversion_tokens
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
            }

            #[doc = #write_with_defaults_doc]
            #visibility_tokens #function_modifier_tokens fn #write_with_defaults_fn(&mut self, #source_param: #bitfield_data_type_tokens) {
                let this = self;
                let bits = #source_param;
                #bits_variable_endian_conversion_tokens
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                #( #setting_fields_to_default_value_tokens_list )*
            }

            #[doc = #write_le_doc]
            #visibility_tokens #function_modifier_tokens fn #write_le_fn(&mut self, #source_param: #bitfield_data_type_tokens) {
                let this = self;
                let bits = #source_param;
                #le_endian_conversion_tokens
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
            }

            #[doc = #write_le_with_defaults_doc]
            #visibility_tokens #function_modifier_tokens fn #write_le_with_defaults_fn(&mut self, #source_param: #bitfield_data_type_tokens) {
                let this = self;
                let bits = #source_param;
                #le_endian_conversion_tokens
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                #( #setting_fields_to_default_value_tokens_list )*
            }

            #[doc = #write_be_doc]
            #visibility_tokens #function_modifier_tokens fn #write_be_fn(&mut self, #source_param: #bitfield_data_type_tokens) {
                let this = self;
                let bits = #source_param;
                #be_endian_conversion_tokens
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
            }

            #[doc = #write_be_with_defaults_doc]
            #visibility_tokens #function_modifier_tokens fn #write_be_with_defaults_fn(&mut self, #source_param: #bitfield_data_type_tokens) {
                let this = self;
                let bits = #source_param;
                #be_endian_conversion_tokens
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                #( #setting_fields_to_default_value_tokens_list )*
            }

            #[doc = "Writes the bitfield defaults."]
            #visibility_tokens #function_modifier_tokens fn write_defaults(&mut self) {
                let this = self;
                #( #setting_fields_to_default_value_tokens_list )*
            }
        }
    }

    /// Generates bits variable endian conversion tokens depending on the
    /// configured endian.
    fn generate_bits_variable_endian_conversion_tokens(
        bitfield: &Bitfield,
        endian: ConversionEndian,
    ) -> TokenStream {
        match endian {
            ConversionEndian::Little => {
                if bitfield.is_integer_backed() {
                    quote! {
                        let bits = bits.swap_bytes();
                    }
                } else {
                    let array_len = bitfield
                        .spanned_data_type_token()
                        .array_length()
                        .expect("array-backed bitfield must have a known length");
                    let half_len = array_len / 2;
                    let last_idx = array_len - 1;
                    quote! {
                        let mut bits = bits;
                        let mut i = 0;
                        while i < #half_len {
                            let j = #last_idx - i;
                            bits[i] ^= bits[j];
                            bits[j] ^= bits[i];
                            bits[i] ^= bits[j];
                            i += 1;
                        }
                    }
                }
            },
            ConversionEndian::Big => {
                quote! {}
            },
        }
    }
}
