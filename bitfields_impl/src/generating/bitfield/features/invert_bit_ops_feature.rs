use proc_macro2::TokenStream;
use quote::{ToTokens as QuoteToTokens, format_ident, quote};

use crate::generating::bitfield::feature::{Feature, FeaturePosition, is_bit_ops_feature_enabled};
use crate::generating::bitfield::features::common::generator_helper::{
    BitsSource, ProtectionType,
    generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens,
    generate_extract_field_bits_from_source_into_variable_tokens,
    generate_protected_bits_mask_tokens, get_bits_or_bytes_term, get_field_unit_terms,
    get_function_modifier_tokens,
};
use crate::parsing::bitfields::bitfield::{Bitfield, Field};
use crate::parsing::bitfields::bitfield_attribute::bitfield_arguments::BitOrder;
use crate::parsing::common::spanned_data_type::{DataType, IntegerType};
use crate::parsing::common::to_tokens::ToTokens;

pub struct InvertBitOpsFeature;

impl Feature for InvertBitOpsFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_invert_bit_ops_feature_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        is_bit_ops_feature_enabled(
            bitfield,
            bitfield.arguments().generate_invert_bit_ops(),
            bitfield.arguments().user_set_invert_bit_ops(),
        )
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Inside
    }

    fn order_priority(&self) -> u32 {
        8
    }
}

impl InvertBitOpsFeature {
    fn generate_invert_bit_ops_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let invert_bitfield_tokens = Self::generate_invert_bitfield_tokens(bitfield);
        let invert_fields_tokens = Self::generate_invert_fields_tokens(bitfield);
        let inverted_fields_getter_tokens = Self::generate_inverted_fields_getter_tokens(bitfield);

        quote! {
            #invert_bitfield_tokens
            #invert_fields_tokens
            #inverted_fields_getter_tokens
        }
    }

    fn generate_invert_bitfield_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let bitfield_internal_value_ident_tokens =
            bitfield.bitfield_internal_value_ident_tokens(/* builder_caller= */ false);
        let protected_mask =
            generate_protected_bits_mask_tokens(bitfield, ProtectionType::ReadOnly);
        let extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens =
            generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens(
                bitfield,
                ProtectionType::None,
            );
        let bob = get_bits_or_bytes_term(bitfield);
        let invert_fn = format_ident!("invert_{}", bob);
        let invert_doc = format!("Inverts all writable {bob} in the bitfield.");

        if bitfield.is_integer_backed() {
            quote! {
                #[doc = #invert_doc]
                #visibility_tokens #function_modifier_tokens fn #invert_fn(&mut self) {
                    let this = self;
                    let bits = #bitfield_internal_value_ident_tokens;
                    #protected_mask
                    let bits = (!bits & !protected_mask) | (bits & protected_mask);
                    #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                }
            }
        } else {
            let array_len = bitfield
                .spanned_data_type_token()
                .array_length()
                .expect("array-backed bitfield must have a known length");

            let bits_init_tokens = if bitfield.arguments().array_heap() {
                quote! { let mut bits = *#bitfield_internal_value_ident_tokens; }
            } else {
                quote! { let mut bits = #bitfield_internal_value_ident_tokens; }
            };
            quote! {
                #[doc = #invert_doc]
                #visibility_tokens #function_modifier_tokens fn #invert_fn(&mut self) {
                    let this = self;
                    #bits_init_tokens
                    #protected_mask
                    let mut i = 0usize;
                    while i < #array_len {
                        bits[i] = (!bits[i] & !protected_mask[i]) | (bits[i] & protected_mask[i]);
                        i += 1;
                    }
                    #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                }
            }
        }
    }

    /// Returns tokens to invert a field in the bitfield.
    fn generate_invert_fields_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);

        bitfield
            .fields()
            .iter()
            .filter(|field| field.has_setter())
            .map(|field| {
                let field_setter_ident_tokens = field.setter_ident_tokens();
                let field_invert_ident_tokens =
                    format_ident!("invert_{}", field.name(), span = field.name_ident().span()).to_token_stream();
                let documentation = Self::get_invert_field_documentation(bitfield, field);
                let extract_field_bits_from_source_into_variable_tokens =
                    generate_extract_field_bits_from_source_into_variable_tokens(
                        bitfield,
                        field,
                        BitsSource::Bitfield,
                        /* cast_bits= */ false,
                        /* invert_bits= */ true,
                        /* builder_caller= */ false,
                    );

                let value_to_field_tokens = Self::generate_value_to_field_tokens(bitfield, field);
                quote! {
                    #[doc = #documentation]
                    #visibility_tokens #function_modifier_tokens fn #field_invert_ident_tokens(&mut self) {
                        let this = self;
                        #extract_field_bits_from_source_into_variable_tokens
                        this.#field_setter_ident_tokens(#value_to_field_tokens);
                    }
                }
            })
            .collect()
    }

    /// Returns tokens to return a field inverted.
    fn generate_inverted_fields_getter_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);

        bitfield
            .fields()
            .iter()
            .filter(|field| field.has_getter())
            .map(|field| {
                let field_inverted_ident_tokens =
                    format_ident!("{}_inverted", field.name(), span = field.name_ident().span()).to_token_stream();
                let documentation = Self::get_invert_field_documentation(bitfield, field);
                let value_to_field_tokens = Self::generate_value_to_field_tokens(bitfield, field);
                let field_data_type_tokens = field.spanned_data_type_token().to_tokens();
                let extract_field_bits_from_source_into_variable_tokens =
                    generate_extract_field_bits_from_source_into_variable_tokens(
                        bitfield,
                        field,
                        BitsSource::Bitfield,
                        /* cast_bits= */ false,
                        /* invert_bits= */ true,
                        /* builder_caller= */ false,
                    );
                quote! {
                    #[doc = #documentation]
                    #visibility_tokens #function_modifier_tokens fn #field_inverted_ident_tokens(&self) -> #field_data_type_tokens {
                        let this = self;
                        #extract_field_bits_from_source_into_variable_tokens
                        #value_to_field_tokens
                    }
                }
            })
            .collect()
    }

    fn generate_value_to_field_tokens(bitfield: &Bitfield, field: &Field) -> TokenStream {
        let custom_field_data_type_tokens = field.spanned_data_type_token().to_tokens();
        match field.spanned_data_type_token().data_type() {
            DataType::Custom => {
                quote! {
                    #custom_field_data_type_tokens::from_bits(value as _)
                }
            },
            DataType::Integer(IntegerType::Bool) => {
                quote! { value != 0 }
            },
            DataType::Array {
                length,
            } => {
                let len = length as usize;
                if bitfield.is_integer_backed() {
                    quote! {
                        {
                            let __val = value as u128;
                            let mut __arr = [0u8; #len];
                            let mut __i: usize = 0;
                            while __i < #len {
                                __arr[__i] = ((__val >> (__i * 8)) & 0xFF) as u8;
                                __i += 1;
                            }
                            __arr
                        }
                    }
                } else {
                    quote! { value }
                }
            },
            _ => {
                quote! { value as _ }
            },
        }
    }

    /// Returns invert field documentation.
    fn get_invert_field_documentation(bitfield: &Bitfield, field: &Field) -> String {
        let offset = field.offset();
        let (unit, units) = get_field_unit_terms(field);

        if field.bits() == 1 {
            return format!("Inverts {unit} `{offset}`.");
        }

        let bits_end = offset + field.bits() - 1;

        let (documentation_bits_start, documentation_bits_end) =
            if bitfield.arguments().order() == BitOrder::Msb {
                (bits_end, offset)
            } else {
                (offset, bits_end)
            };

        format!("Inverts {units} `{documentation_bits_start}..={documentation_bits_end}`.")
    }
}
