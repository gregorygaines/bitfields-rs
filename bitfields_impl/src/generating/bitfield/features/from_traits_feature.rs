use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::common::generator_helper::{
    ProtectionType, generate_backing_data_param_ident,
    generate_bitfield_struct_initialization_tokens,
    generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens,
};
use crate::parsing::bitfields::bitfield::Bitfield;
use crate::parsing::common::conversion_endian::ConversionEndian;
use crate::parsing::common::to_tokens::ToTokens;

/// Generates from and into traits.
///
/// Example:
///
/// - `impl From<u32> for Bitfield { ... }`
/// - `impl From<Bitfield> for u32 { ... }`
pub struct FromTraitsFeature;

impl Feature for FromTraitsFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_from_traits_feature_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        bitfield.arguments().generate_from_traits()
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Below
    }

    fn order_priority(&self) -> u32 {
        0
    }
}

impl FromTraitsFeature {
    /// Generate from traits for the bitfield.
    fn generate_from_traits_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let from_data_type_trait_tokens = Self::generate_from_data_type_trait_tokens(bitfield);
        let from_bitfield_trait_tokens = Self::generate_from_bitfield_trait_tokens(bitfield);

        quote! {
            #from_data_type_trait_tokens
            #from_bitfield_trait_tokens
        }
    }

    fn generate_from_data_type_trait_tokens(bitfield: &Bitfield) -> TokenStream {
        let initialize_struct_initialization_tokens =
            generate_bitfield_struct_initialization_tokens(
                bitfield, /* builder_caller= */ false,
            );
        let generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens =
            generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens(
                bitfield,
                ProtectionType::None,
            );

        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bitfield_name_tokens = bitfield.name_tokens();
        let bits_return_endian_conversion_tokens =
            Self::generate_bits_return_endian_conversion_tokens(
                bitfield, /* into_bits= */ false,
            );
        let source_param = generate_backing_data_param_ident(bitfield);

        quote! {
            impl core::convert::From<#bitfield_data_type_tokens> for #bitfield_name_tokens {
                fn from(#source_param: #bitfield_data_type_tokens) -> Self {
                    let mut this = #initialize_struct_initialization_tokens;
                    let bits = #bits_return_endian_conversion_tokens;
                    #generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                    this
                }
            }
        }
    }

    fn generate_from_bitfield_trait_tokens(bitfield: &Bitfield) -> TokenStream {
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bitfield_name_tokens = bitfield.name_tokens();
        let bits_return_endian_conversion_tokens =
            Self::generate_bits_return_endian_conversion_tokens(
                bitfield, /* into_bits= */ true,
            );

        quote! {
            impl core::convert::From<#bitfield_name_tokens> for #bitfield_data_type_tokens {
                fn from(val: #bitfield_name_tokens) -> Self {
                    let mut this = val;
                    #bits_return_endian_conversion_tokens
                }
            }
        }
    }

    /// Generates bits result conversion tokens depending on the configured
    /// endian.
    fn generate_bits_return_endian_conversion_tokens(
        bitfield: &Bitfield,
        into_bits: bool,
    ) -> TokenStream {
        let source_value_ident_tokens = if into_bits {
            bitfield.bitfield_internal_value_ident_tokens(/* builder_caller= */ false)
        } else {
            generate_backing_data_param_ident(bitfield)
        };

        let conversion_endian = if into_bits {
            bitfield.arguments().into_endian()
        } else {
            bitfield.arguments().from_endian()
        };

        let is_heap_array =
            into_bits && bitfield.arguments().array_heap() && !bitfield.is_integer_backed();

        match conversion_endian {
            ConversionEndian::Little => {
                if bitfield.is_integer_backed() {
                    quote! { #source_value_ident_tokens.swap_bytes() }
                } else if is_heap_array {
                    quote! { *#source_value_ident_tokens }
                } else {
                    quote! { #source_value_ident_tokens }
                }
            },
            ConversionEndian::Big => {
                if bitfield.is_integer_backed() {
                    quote! { #source_value_ident_tokens }
                } else if is_heap_array {
                    quote! {
                        {
                            let mut temp_bits = *#source_value_ident_tokens;
                            let mut i = 0;
                            while i < temp_bits.len() / 2 {
                                let temp = temp_bits[i];
                                temp_bits[i] = temp_bits[temp_bits.len() - 1 - i];
                                temp_bits[temp_bits.len() - 1 - i] = temp;
                                i += 1;
                            }
                            temp_bits
                        }
                    }
                } else {
                    quote! {
                        {
                            let mut temp_bits = #source_value_ident_tokens;
                            let mut i = 0;
                            while i < temp_bits.len() / 2 {
                                let temp = temp_bits[i];
                                temp_bits[i] = temp_bits[temp_bits.len() - 1 - i];
                                temp_bits[temp_bits.len() - 1 - i] = temp;
                                i += 1;
                            }
                            temp_bits
                        }
                    }
                }
            },
        }
    }
}
