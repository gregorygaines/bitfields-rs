use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::common::generator_helper::{
    BitsSource, generate_extract_field_bits_from_source_into_variable_tokens,
    generate_sign_extend_bit_operation_tokens, get_field_unit_terms, get_function_modifier_tokens,
};
use crate::parsing::bitfields::bitfield::{Bitfield, Field};
use crate::parsing::bitfields::bitfield_attribute::bitfield_arguments::BitOrder;
use crate::parsing::common::spanned_data_type::{DataType, IntegerType};
use crate::parsing::common::to_tokens::ToTokens;

/// Generates field getters.
#[derive(Default)]
pub struct FieldGettersFeature;

impl Feature for FieldGettersFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_field_getters_feature_tokens(bitfield)
    }

    fn enabled(&self, _: &Bitfield) -> bool {
        true
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Inside
    }

    fn order_priority(&self) -> u32 {
        3
    }
}

impl FieldGettersFeature {
    fn generate_field_getters_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        bitfield
            .fields()
            .iter()
            .filter(|field: &&Field| field.has_getter())
            .map(|field| {
                let visibility_tokens = field.visibility().to_tokens();
                let documentation = Self::get_getter_documentation(bitfield, field);
                let function_modifier_tokens = get_function_modifier_tokens(bitfield);
                let name_tokens = field.name_tokens();
                let field_data_type_tokens = field.spanned_data_type_token().to_tokens();
                let extract_field_bits_from_source_into_variable_tokens =
                    generate_extract_field_bits_from_source_into_variable_tokens(
                        bitfield,
                        field,
                        BitsSource::Bitfield,
                        /* cast_bits= */ false,
                        /* invert_bits= */ false,
                        /* builder_caller= */ false,
                    );
                let sign_extend_tokens_stream = generate_sign_extend_bit_operation_tokens(field);
                let value_return_token_stream = Self::generate_value_return_tokens(bitfield, field);

                quote! {
                    #[doc = #documentation]
                    #visibility_tokens #function_modifier_tokens fn #name_tokens(&self) -> #field_data_type_tokens {
                        let this = self;
                        #extract_field_bits_from_source_into_variable_tokens
                        #sign_extend_tokens_stream
                        #value_return_token_stream
                    }
                }
            })
            .collect()
    }

    /// Generates tokens for `this` return value.
    fn generate_value_return_tokens(bitfield: &Bitfield, field: &Field) -> TokenStream {
        match field.spanned_data_type_token().data_type() {
            DataType::Integer(integer_type) => {
                if matches!(integer_type, IntegerType::Bool) {
                    return quote! {
                        value != 0
                    };
                }

                quote! {
                    value as _
                }
            },
            DataType::Custom => {
                let custom_field_data_type_tokens = field.spanned_data_type_token().to_tokens();
                quote! {
                    #custom_field_data_type_tokens::from_bits(value as _)
                }
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
        }
    }

    /// Returns field getter documentation.
    fn get_getter_documentation(bitfield: &Bitfield, field: &Field) -> String {
        let offset = field.offset();
        let (unit, units) = get_field_unit_terms(field);

        if field.bits() == 1 {
            return if field.spanned_data_type_token().data_type().unsigned() {
                format!("Returns {unit} `{offset}`.")
            } else {
                format!("Returns sign-extended {unit} `{offset}`.")
            };
        }

        let bits_end = offset + field.bits() - 1;

        let (documentation_bits_start, documentation_bits_end) =
            if bitfield.arguments().order() == BitOrder::Msb {
                (bits_end, offset)
            } else {
                (offset, bits_end)
            };

        if field.spanned_data_type_token().data_type().unsigned() {
            format!("Returns {units} `{documentation_bits_start}..={documentation_bits_end}`.")
        } else {
            format!(
                "Returns sign-extended {units} \
                 `{documentation_bits_start}..={documentation_bits_end}`, from the sign-{unit} \
                 `{offset}."
            )
        }
    }
}
