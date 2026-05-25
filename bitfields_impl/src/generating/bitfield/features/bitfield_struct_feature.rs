use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::parsing::bitfields::bitfield::Bitfield;
use crate::parsing::common::to_tokens::ToTokens;

/// Generator for bitfield struct that hold the backend data.
///
/// # Example
///
/// ```rust,ignore
/// <attributes>
/// struct Bitfield {
///  ...
/// }
/// ```
#[derive(Default)]
pub struct BitfieldStructFeatureGenerator;

impl Feature for BitfieldStructFeatureGenerator {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_bitfield_struct_feature_tokens(bitfield)
    }

    fn enabled(&self, _: &Bitfield) -> bool {
        true
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Above
    }

    fn order_priority(&self) -> u32 {
        0
    }
}

const STRUCT_DOCUMENTATION: &str = "Represents a bitfield.";

impl BitfieldStructFeatureGenerator {
    /// Generates a bitfield struct with either named fields or a tuple struct
    /// based on whether the bitfield has ignored fields.
    fn generate_bitfield_struct_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let attributes_token_stream = Self::generate_struct_attributes_tokens(bitfield);

        if Self::should_generate_name_field_struct_tokens(bitfield) {
            Self::generate_named_field_struct_tokens(&attributes_token_stream, bitfield)
        } else {
            Self::generate_tuple_struct_tokens(&attributes_token_stream, bitfield)
        }
    }

    /// Generates the struct attributes for the bitfield struct.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    ///   #[derive(Debug, Clone)]
    ///   #[repr(C)]
    /// ```
    fn generate_struct_attributes_tokens(bitfield: &Bitfield) -> Vec<TokenStream> {
        let mut all_attributes = Self::get_default_attributes(bitfield);
        all_attributes.extend(bitfield.user_attributes_tokens());
        all_attributes
    }

    /// Returns the default attributes of the bitfield.
    fn get_default_attributes(bitfield: &Bitfield) -> Vec<TokenStream> {
        let mut attributes_tokens = Vec::new();

        if Self::should_generate_name_field_struct_tokens(bitfield) {
            attributes_tokens.push(quote! {
                #[repr(C)]
            });
        } else {
            attributes_tokens.push(quote! {
                #[repr(transparent)]
            });
        }

        if bitfield.arguments().derive_copy() {
            let is_heap_array = bitfield.arguments().array_heap() && !bitfield.is_integer_backed();
            if is_heap_array {
                attributes_tokens.push(quote! {
                    #[derive(core::clone::Clone)]
                });
            } else {
                attributes_tokens.push(quote! {
                    #[derive(std::marker::Copy, core::clone::Clone)]
                });
            }
        }

        attributes_tokens
    }

    /// Returns if a named field struct should be generated for the bitfield.
    ///
    /// Required if we have named fields for ignored fields
    fn should_generate_name_field_struct_tokens(bitfield: &Bitfield) -> bool {
        bitfield.has_ignored_fields()
    }

    /// Returns the backing storage type tokens for the bitfield struct field.
    fn get_backing_field_type_tokens(bitfield: &Bitfield) -> TokenStream {
        let inner = bitfield.spanned_data_type_token().to_tokens();
        if bitfield.arguments().array_heap() && !bitfield.is_integer_backed() {
            quote! { ::std::boxed::Box<#inner> }
        } else {
            inner
        }
    }

    /// Generates a tuple struct for the bitfield when there are no ignored
    /// fields.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    ///   struct Bitfield(u8);
    /// ```
    fn generate_tuple_struct_tokens(
        attributes: &[TokenStream],
        bitfield: &Bitfield,
    ) -> TokenStream {
        let bitfield_name_tokens = bitfield.name_tokens();
        let backing_field_type_tokens = Self::get_backing_field_type_tokens(bitfield);
        let visibility_tokens = bitfield.visibility().to_tokens();

        quote! {
            #( #attributes )*
            #[doc = #STRUCT_DOCUMENTATION]
            #visibility_tokens struct #bitfield_name_tokens(#backing_field_type_tokens);
        }
    }

    /// Generates a named field struct to hold ignored fields.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    ///   struct Bitfield {
    ///     val: u8,
    ///     ignored_a: Custom,
    ///     ignored_b: u8,
    ///   }
    /// ```
    fn generate_named_field_struct_tokens(
        attributes: &[TokenStream],
        bitfield: &Bitfield,
    ) -> TokenStream {
        let bitfield_name_tokens = bitfield.name_tokens();
        let backing_field_type_tokens = Self::get_backing_field_type_tokens(bitfield);
        let visibility_tokens = bitfield.visibility().to_tokens();

        let ignored_fields_field_definitions_tokens =
            Self::generate_ignored_fields_struct_field_definition(bitfield);

        quote! {
            #( #attributes )*
            #[doc = #STRUCT_DOCUMENTATION]
            #visibility_tokens struct #bitfield_name_tokens {
                val: #backing_field_type_tokens,
                #( #ignored_fields_field_definitions_tokens, )*
             }
        }
    }

    /// Generates struct field definition for ignored fields.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    ///   pub ignored_field: u8
    /// ```
    fn generate_ignored_fields_struct_field_definition(bitfield: &Bitfield) -> Vec<TokenStream> {
        bitfield
            .ignored_fields()
            .iter()
            .map(|field| {
                let visibility_tokens = field.visibility().to_tokens();
                let field_name_tokens = field.name_tokens();
                let field_type_tokens = field.spanned_data_type_token().to_tokens();
                quote! {
                    #visibility_tokens #field_name_tokens: #field_type_tokens
                }
            })
            .collect()
    }
}
