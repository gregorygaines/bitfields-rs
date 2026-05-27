use proc_macro2::TokenStream;
use quote::{ToTokens as QuoteToTokens, format_ident, quote};

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::common::generator_helper::{
    generate_new_function_implementation_tokens, generate_setting_field_from_variable_tokens,
    get_function_modifier_tokens, get_setter_documentation,
};
use crate::parsing::bitfields::bitfield::{Bitfield, Field};
use crate::parsing::common::to_tokens::ToTokens;

/// Generates the builder feature bitfields.
///
/// # Example:
///
/// ```rust,ignore
/// let builder = BitfieldBuilder::new();
/// builder.set_a(99);
/// let bitfield = bitfield.build();
/// ```
pub struct BuilderFeature;

impl Feature for BuilderFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_builder_feature_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        bitfield.arguments().generate_builder()
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Below
    }

    fn order_priority(&self) -> u32 {
        100
    }
}

impl BuilderFeature {
    fn generate_builder_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let builder_ident_tokens =
            format_ident!("{}Builder", bitfield.name(), span = bitfield.name_ident().span())
                .to_token_stream();
        let visibility_tokens = bitfield.visibility().to_tokens();
        let bitfield_name_tokens = bitfield.name_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let builder_setters = Self::generate_builder_setters(bitfield);
        let new_implementation_tokens = Self::generate_builder_new_implementation_tokens(
            bitfield, /* generate_setting_defaults= */ true,
        );
        let new_without_defaults_implementation_tokens =
            Self::generate_builder_new_implementation_tokens(
                bitfield, /* generate_setting_defaults= */ false,
            );

        quote! {
            #[doc = "A builder for the bitfield."]
            #visibility_tokens struct #builder_ident_tokens {
                this: #bitfield_name_tokens,
            }

            impl Default for #builder_ident_tokens {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl #builder_ident_tokens {
                #[doc = "Creates a new bitfield builder instance."]
                #visibility_tokens #function_modifier_tokens fn new() -> Self {
                    #new_implementation_tokens
                }

                #[doc = "Creates a new bitfield builder instance without \
                         respecting defaults."]
                #visibility_tokens #function_modifier_tokens fn new_without_defaults() -> Self {
                    #new_without_defaults_implementation_tokens
                }

                #builder_setters

                #[doc = "Builds a bitfield instance from the builder."]
                #visibility_tokens #function_modifier_tokens fn build(self) -> #bitfield_name_tokens {
                    self.this
                }
            }
        }
    }

    fn generate_builder_new_implementation_tokens(
        bitfield: &Bitfield,
        generate_setting_defaults: bool,
    ) -> TokenStream {
        if bitfield.arguments().generate_new() {
            let bitfield_name_tokens = bitfield.name_tokens();

            if generate_setting_defaults {
                quote! {
                Self {
                    this: #bitfield_name_tokens::new(),
                }
                    }
            } else {
                quote! {
                    Self {
                        this: #bitfield_name_tokens::new_without_defaults(),
                    }
                }
            }
        } else {
            let new_implementation_tokens = generate_new_function_implementation_tokens(
                bitfield,
                generate_setting_defaults,
                /* builder_caller= */ true,
                /* existing_bitfield= */ false,
            );
            quote! {
                #new_implementation_tokens
                Self {
                    this,
                }
            }
        }
    }

    fn generate_builder_setters(bitfield: &Bitfield) -> TokenStream {
        bitfield
            .fields()
            .iter()
            .filter(|field: &&Field| !field.is_reserved())
            .map(|field| Self::generate_generate_builder_setters_setter_helper(bitfield, field))
            .collect()
    }

    fn generate_generate_builder_setters_setter_helper(
        bitfield: &Bitfield,
        field: &Field,
    ) -> TokenStream {
        let visibility_tokens = field.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let data_type_tokens = field.spanned_data_type_token().to_tokens();
        let builder_setter_name_token_stream =
            format_ident!("with_{}", field.name(), span = field.name_ident().span())
                .to_token_stream();
        let builder_checked_setter_name_token_stream =
            format_ident!("checked_with_{}", field.name(), span = field.name_ident().span())
                .to_token_stream();
        let set_bits_logic_tokens = generate_setting_field_from_variable_tokens(
            bitfield, field, /* use_setter= */ true, /* cast_bits= */ true,
            /* check_bit_size= */ false, /* builder_caller= */ true,
        );
        let checked_set_bits_logic_tokens = generate_setting_field_from_variable_tokens(
            bitfield, field, /* use_setter= */ true, /* cast_bits= */ true,
            /* check_bit_size= */ true, /* builder_caller= */ true,
        );

        let setter_documentation = get_setter_documentation(
            bitfield, field, /* checked_setter= */ false, /* builder_caller= */ true,
        );
        let checked_setter_documentation = get_setter_documentation(
            bitfield, field, /* checked_setter= */ true, /* builder_caller= */ true,
        );

        quote! {
            #[doc = #setter_documentation]
            #visibility_tokens #function_modifier_tokens fn #builder_setter_name_token_stream(mut self, bits: #data_type_tokens) -> Self {
                #set_bits_logic_tokens
                self
            }

            #[doc = #checked_setter_documentation]
            #visibility_tokens #function_modifier_tokens fn #builder_checked_setter_name_token_stream(mut self, bits: #data_type_tokens) -> ::core::result::Result<Self, &'static str> {
                #checked_set_bits_logic_tokens
                Ok(self)
            }
        }
    }
}
