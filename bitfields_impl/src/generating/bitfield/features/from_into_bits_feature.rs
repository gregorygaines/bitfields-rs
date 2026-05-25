use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition};
use crate::generating::bitfield::features::common::generator_helper::{
    ProtectionType, generate_backing_data_param_ident,
    generate_bitfield_struct_initialization_tokens,
    generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens,
    generate_setting_fields_to_default_value_tokens_list, get_function_modifier_tokens,
};
use crate::generating::common::endian_conversion::generate_bits_variable_endian_conversion_tokens;
use crate::parsing::bitfields::bitfield::Bitfield;
use crate::parsing::common::conversion_endian::ConversionEndian;
use crate::parsing::common::to_tokens::ToTokens;

/// Generates from/into bits/bytes functions.
pub struct FromIntoBitsFeature;

impl Feature for FromIntoBitsFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_from_into_bits_feature_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        bitfield.arguments().generate_from_into_bits()
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Inside
    }

    fn order_priority(&self) -> u32 {
        2
    }
}

impl FromIntoBitsFeature {
    fn generate_from_into_bits_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        if bitfield.is_integer_backed() {
            Self::generate_integer_backed_from_into_bits_feature_tokens(bitfield)
        } else {
            Self::generate_array_backed_from_into_bits_feature_tokens(bitfield)
        }
    }
}

impl FromIntoBitsFeature {
    fn generate_integer_backed_from_into_bits_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let from_bits_tokens = Self::generate_from_bits_tokens(bitfield);
        let into_bits_tokens = Self::generate_integer_backed_into_bits_tokens(bitfield);

        quote! {
            #from_bits_tokens
            #into_bits_tokens
        }
    }

    fn generate_from_bits_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let bitfield_struct_initialization_tokens = generate_bitfield_struct_initialization_tokens(
            bitfield, /* builder_caller= */ false,
        );
        let extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens =
            generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens(
                bitfield,
                ProtectionType::None,
            );
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let setting_fields_to_default_value_tokens_list =
            generate_setting_fields_to_default_value_tokens_list(bitfield);
        let bits_variable_endian_conversion_tokens =
            generate_bits_variable_endian_conversion_tokens(bitfield.arguments().from_endian());
        let source_param = generate_backing_data_param_ident(bitfield);

        quote! {
            #[doc = "Creates a new bitfield instance from the given bits."]
            #visibility_tokens #function_modifier_tokens fn from_bits(#source_param: #bitfield_data_type_tokens) -> Self {
                let bits = #source_param;
                #bits_variable_endian_conversion_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                this
            }

            #[doc = "Creates a new bitfield instance from the given bits while \
                     respecting defaults."]
            #visibility_tokens #function_modifier_tokens fn from_bits_with_defaults(#source_param: #bitfield_data_type_tokens) -> Self {
                let bits = #source_param;
                #bits_variable_endian_conversion_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }

            #[doc = "Creates a new bitfield instance from the given little-endian bits."]
            #visibility_tokens #function_modifier_tokens fn from_le_bits(#source_param: #bitfield_data_type_tokens) -> Self {
                let bits = #source_param.swap_bytes();
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                this
            }

            #[doc = "Creates a new bitfield instance from the given little-endian \
                     bits while respecting defaults."]
            #visibility_tokens #function_modifier_tokens fn from_le_bits_with_defaults(#source_param: #bitfield_data_type_tokens) -> Self {
                let bits = #source_param.swap_bytes();
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }

            #[doc = "Creates a new bitfield instance from the given big-endian bits."]
            #visibility_tokens #function_modifier_tokens fn from_be_bits(#source_param: #bitfield_data_type_tokens) -> Self {
                let bits = #source_param;
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                this
            }

            #[doc = "Creates a new bitfield instance from the given big-endian bits \
                     while respecting defaults."]
            #visibility_tokens #function_modifier_tokens fn from_be_bits_with_defaults(#source_param: #bitfield_data_type_tokens) -> Self {
                let bits = #source_param;
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }
        }
    }

    fn generate_integer_backed_into_bits_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bits_variable_endian_conversion_tokens =
            generate_bits_variable_endian_conversion_tokens(bitfield.arguments().into_endian());
        let bitfield_internal_value_ident_tokens =
            bitfield.bitfield_internal_value_ident_tokens(/* builder_caller= */ false);
        let into_bits_little_endian_conversion_tokens =
            generate_bits_variable_endian_conversion_tokens(ConversionEndian::Little);
        let into_bits_big_endian_conversion_tokens =
            generate_bits_variable_endian_conversion_tokens(ConversionEndian::Big);

        quote! {
            #[doc = "Returns the bits of the bitfield."]
            #visibility_tokens #function_modifier_tokens fn into_bits(self) -> #bitfield_data_type_tokens {
                let this = self;
                let bits = #bitfield_internal_value_ident_tokens;
                #bits_variable_endian_conversion_tokens
                bits
            }

            #[doc = "Returns the bits of the bitfield in little-endian order."]
            #visibility_tokens #function_modifier_tokens fn into_le_bits(self) -> #bitfield_data_type_tokens {
                let this = self;
                let bits = #bitfield_internal_value_ident_tokens;
                #into_bits_little_endian_conversion_tokens
                bits
            }

            #[doc = "Returns the bits of the bitfield in big-endian order."]
            #visibility_tokens #function_modifier_tokens fn into_be_bits(self) -> #bitfield_data_type_tokens {
                let this = self;
                let bits = #bitfield_internal_value_ident_tokens;
                #into_bits_big_endian_conversion_tokens
                bits
            }
        }
    }
}

impl FromIntoBitsFeature {
    fn generate_array_backed_from_into_bits_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let from_bits_tokens = Self::generate_array_backed_from_bits_tokens(bitfield);
        let from_slice_tokens = Self::generate_array_backed_from_slice_tokens(bitfield);
        let into_bits_tokens = Self::generate_array_backed_into_bits_tokens(bitfield);
        let into_slice_tokens = Self::generate_array_backed_into_slice_tokens(bitfield);

        quote! {
            #from_bits_tokens
            #from_slice_tokens
            #into_bits_tokens
            #into_slice_tokens
        }
    }

    fn generate_array_backed_from_bits_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let bitfield_struct_initialization_tokens = generate_bitfield_struct_initialization_tokens(
            bitfield, /* builder_caller= */ false,
        );
        let extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens =
            generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens(
                bitfield,
                ProtectionType::None,
            );
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let setting_fields_to_default_value_tokens_list =
            generate_setting_fields_to_default_value_tokens_list(bitfield);
        let source_param = generate_backing_data_param_ident(bitfield);
        let array_len = bitfield
            .spanned_data_type_token()
            .array_length()
            .expect("array-backed bitfield must have a known length");
        let half_len = array_len / 2;
        let last_idx = array_len - 1;

        let from_bits_convert_tokens = |endian| match endian {
            ConversionEndian::Little => quote! {
                let bits = #source_param;
            },
            ConversionEndian::Big => quote! {
                let mut bits = #source_param;
                let mut i = 0;
                while i < #half_len {
                    let j = #last_idx - i;
                    bits[i] ^= bits[j];
                    bits[j] ^= bits[i];
                    bits[i] ^= bits[j];
                    i += 1;
                }
            },
        };

        let default_endian_from_bits = from_bits_convert_tokens(bitfield.arguments().from_endian());
        let le_endian_from_bits = from_bits_convert_tokens(ConversionEndian::Little);
        let be_endian_from_bits = from_bits_convert_tokens(ConversionEndian::Big);

        quote! {
            #[doc = "Creates a new bitfield instance from the given bytes."]
            #visibility_tokens #function_modifier_tokens fn from_bytes(#source_param: #bitfield_data_type_tokens) -> Self {
                #default_endian_from_bits
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                this
            }

            #[doc = "Creates a new bitfield instance from the given bytes while \
                     respecting defaults."]
            #visibility_tokens #function_modifier_tokens fn from_bytes_with_defaults(#source_param: #bitfield_data_type_tokens) -> Self {
                #default_endian_from_bits
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }

            #[doc = "Creates a new bitfield instance from the given little-endian bytes."]
            #visibility_tokens #function_modifier_tokens fn from_le_bytes(#source_param: #bitfield_data_type_tokens) -> Self {
                #le_endian_from_bits
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                this
            }

            #[doc = "Creates a new bitfield instance from the given little-endian \
                     bytes while respecting defaults."]
            #visibility_tokens #function_modifier_tokens fn from_le_bytes_with_defaults(#source_param: #bitfield_data_type_tokens) -> Self {
                #le_endian_from_bits
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }

            #[doc = "Creates a new bitfield instance from the given big-endian bytes."]
            #visibility_tokens #function_modifier_tokens fn from_be_bytes(#source_param: #bitfield_data_type_tokens) -> Self {
                #be_endian_from_bits
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                this
            }

            #[doc = "Creates a new bitfield instance from the given big-endian bytes \
                     while respecting defaults."]
            #visibility_tokens #function_modifier_tokens fn from_be_bytes_with_defaults(#source_param: #bitfield_data_type_tokens) -> Self {
                #be_endian_from_bits
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }
        }
    }

    /// Generates `from_slice` and `checked_from_slice` variants (default, le,
    /// be endian).
    fn generate_array_backed_from_slice_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bitfield_struct_initialization_tokens = generate_bitfield_struct_initialization_tokens(
            bitfield, /* builder_caller= */ false,
        );
        let extract_all_field_bits_tokens =
            generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens(
                bitfield,
                ProtectionType::None,
            );
        let setting_fields_to_default_value_tokens_list =
            generate_setting_fields_to_default_value_tokens_list(bitfield);
        let array_len = bitfield
            .spanned_data_type_token()
            .array_length()
            .expect("array-backed bitfield must have a known length");
        let half_len = array_len / 2;
        let last_idx = array_len - 1;

        let copy_from_slice_tokens = quote! {
            let mut bits: #bitfield_data_type_tokens = [0; #array_len];
            let copy_len =
                if slice.len() < #array_len { slice.len() } else { #array_len };
            let mut i = 0usize;
            while i < copy_len {
                bits[i] = slice[i];
                i += 1;
            }
        };

        let copy_from_slice_exact_tokens = quote! {
            let mut bits: #bitfield_data_type_tokens = [0; #array_len];
            let mut i = 0usize;
            while i < #array_len {
                bits[i] = slice[i];
                i += 1;
            }
        };

        let size_check_tokens = quote! {
            if slice.len() < #array_len {
                return Err("Slice is too small to fill the bitfield.");
            }
        };

        let slice_endian_conversion_tokens = |endian: ConversionEndian| -> TokenStream {
            match endian {
                ConversionEndian::Little => quote! {},
                ConversionEndian::Big => quote! {
                    let mut i = 0usize;
                    while i < #half_len {
                        let j = #last_idx - i;
                        bits[i] ^= bits[j];
                        bits[j] ^= bits[i];
                        bits[i] ^= bits[j];
                        i += 1;
                    }
                },
            }
        };

        let default_endian_tokens =
            slice_endian_conversion_tokens(bitfield.arguments().from_endian());
        let le_endian_tokens = slice_endian_conversion_tokens(ConversionEndian::Little);
        let be_endian_tokens = slice_endian_conversion_tokens(ConversionEndian::Big);

        quote! {
            #[doc = "Creates a new bitfield instance from the given byte slice. \
                     If the slice is shorter than the bitfield, the remaining bytes \
                     are treated as zero."]
            #visibility_tokens #function_modifier_tokens fn from_slice(slice: &[u8]) -> Self {
                #copy_from_slice_tokens
                #default_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                this
            }

            #[doc = "Creates a new bitfield instance from the given byte slice while \
                     respecting defaults. If the slice is shorter than the bitfield, \
                     the remaining bytes are treated as zero."]
            #visibility_tokens #function_modifier_tokens fn from_slice_with_defaults(slice: &[u8]) -> Self {
                #copy_from_slice_tokens
                #default_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }

            #[doc = "Creates a new bitfield instance from the given byte slice. \
                     Returns an error if the slice is too small to fill the bitfield."]
            #visibility_tokens #function_modifier_tokens fn checked_from_slice(slice: &[u8]) -> ::core::result::Result<Self, &'static str> {
                #size_check_tokens
                #copy_from_slice_exact_tokens
                #default_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                Ok(this)
            }

            #[doc = "Creates a new bitfield instance from the given byte slice while \
                     respecting defaults. Returns an error if the slice is too small \
                     to fill the bitfield."]
            #visibility_tokens #function_modifier_tokens fn checked_from_slice_with_defaults(slice: &[u8]) -> ::core::result::Result<Self, &'static str> {
                #size_check_tokens
                #copy_from_slice_exact_tokens
                #default_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                Ok(this)
            }

            #[doc = "Creates a new bitfield instance from the given little-endian byte \
                     slice. If the slice is shorter than the bitfield, the remaining \
                     bytes are treated as zero."]
            #visibility_tokens #function_modifier_tokens fn from_le_slice(slice: &[u8]) -> Self {
                #copy_from_slice_tokens
                #le_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                this
            }

            #[doc = "Creates a new bitfield instance from the given little-endian byte \
                     slice while respecting defaults. If the slice is shorter than the \
                     bitfield, the remaining bytes are treated as zero."]
            #visibility_tokens #function_modifier_tokens fn from_le_slice_with_defaults(slice: &[u8]) -> Self {
                #copy_from_slice_tokens
                #le_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }

            #[doc = "Creates a new bitfield instance from the given little-endian byte \
                     slice. Returns an error if the slice is too small to fill the \
                     bitfield."]
            #visibility_tokens #function_modifier_tokens fn checked_from_le_slice(slice: &[u8]) -> ::core::result::Result<Self, &'static str> {
                #size_check_tokens
                #copy_from_slice_exact_tokens
                #le_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                Ok(this)
            }

            #[doc = "Creates a new bitfield instance from the given little-endian byte \
                     slice while respecting defaults. Returns an error if the slice is \
                     too small to fill the bitfield."]
            #visibility_tokens #function_modifier_tokens fn checked_from_le_slice_with_defaults(slice: &[u8]) -> ::core::result::Result<Self, &'static str> {
                #size_check_tokens
                #copy_from_slice_exact_tokens
                #le_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                Ok(this)
            }

            #[doc = "Creates a new bitfield instance from the given big-endian byte \
                     slice. If the slice is shorter than the bitfield, the remaining \
                     bytes are treated as zero."]
            #visibility_tokens #function_modifier_tokens fn from_be_slice(slice: &[u8]) -> Self {
                #copy_from_slice_tokens
                #be_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                this
            }

            #[doc = "Creates a new bitfield instance from the given big-endian byte \
                     slice while respecting defaults. If the slice is shorter than the \
                     bitfield, the remaining bytes are treated as zero."]
            #visibility_tokens #function_modifier_tokens fn from_be_slice_with_defaults(slice: &[u8]) -> Self {
                #copy_from_slice_tokens
                #be_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                this
            }

            #[doc = "Creates a new bitfield instance from the given big-endian byte \
                     slice. Returns an error if the slice is too small to fill the \
                     bitfield."]
            #visibility_tokens #function_modifier_tokens fn checked_from_be_slice(slice: &[u8]) -> ::core::result::Result<Self, &'static str> {
                #size_check_tokens
                #copy_from_slice_exact_tokens
                #be_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                Ok(this)
            }

            #[doc = "Creates a new bitfield instance from the given big-endian byte \
                     slice while respecting defaults. Returns an error if the slice is \
                     too small to fill the bitfield."]
            #visibility_tokens #function_modifier_tokens fn checked_from_be_slice_with_defaults(slice: &[u8]) -> ::core::result::Result<Self, &'static str> {
                #size_check_tokens
                #copy_from_slice_exact_tokens
                #be_endian_tokens
                let mut this = #bitfield_struct_initialization_tokens;
                #extract_all_field_bits_tokens
                #( #setting_fields_to_default_value_tokens_list )*
                Ok(this)
            }
        }
    }

    fn generate_array_backed_into_bits_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bitfield_internal_value_ident_tokens =
            bitfield.bitfield_internal_value_ident_tokens(/* builder_caller= */ false);
        let array_len = bitfield
            .spanned_data_type_token()
            .array_length()
            .expect("array-backed bitfield must have a known length");
        let half_len = array_len / 2;
        let last_idx = array_len - 1;

        let is_heap = bitfield.arguments().array_heap();

        let into_bits_convert_tokens = |endian| match endian {
            ConversionEndian::Little => {
                if is_heap {
                    quote! {
                        *#bitfield_internal_value_ident_tokens
                    }
                } else {
                    quote! {
                        #bitfield_internal_value_ident_tokens
                    }
                }
            },
            ConversionEndian::Big => {
                if is_heap {
                    quote! {
                        let mut bits = *#bitfield_internal_value_ident_tokens;
                        let mut i = 0;
                        while i < #half_len {
                            let j = #last_idx - i;
                            bits[i] ^= bits[j];
                            bits[j] ^= bits[i];
                            bits[i] ^= bits[j];
                            i += 1;
                        }
                        bits
                    }
                } else {
                    quote! {
                        let mut bits = #bitfield_internal_value_ident_tokens;
                        let mut i = 0;
                        while i < #half_len {
                            let j = #last_idx - i;
                            bits[i] ^= bits[j];
                            bits[j] ^= bits[i];
                            bits[i] ^= bits[j];
                            i += 1;
                        }
                        bits
                    }
                }
            },
        };

        let default_endian_tokens = into_bits_convert_tokens(bitfield.arguments().into_endian());
        let le_endian_tokens = into_bits_convert_tokens(ConversionEndian::Little);
        let be_endian_tokens = into_bits_convert_tokens(ConversionEndian::Big);

        quote! {
            #[doc = "Returns the bytes of the bitfield."]
            #visibility_tokens #function_modifier_tokens fn into_bytes(&self) -> #bitfield_data_type_tokens {
                let this = self;
                #default_endian_tokens
            }

            #[doc = "Returns the bytes of the bitfield in little-endian order."]
            #visibility_tokens #function_modifier_tokens fn into_le_bytes(&self) -> #bitfield_data_type_tokens {
                let this = self;
                #le_endian_tokens
            }

            #[doc = "Returns the bytes of the bitfield in big-endian order."]
            #visibility_tokens #function_modifier_tokens fn into_be_bytes(&self) -> #bitfield_data_type_tokens {
                let this = self;
                #be_endian_tokens
            }
        }
    }

    /// Generates `into_slice` and `checked_into_slice` variants (default, le,
    /// be endian).
    fn generate_array_backed_into_slice_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let bitfield_internal_value_ident_tokens =
            bitfield.bitfield_internal_value_ident_tokens(/* builder_caller= */ false);
        let array_len = bitfield
            .spanned_data_type_token()
            .array_length()
            .expect("array-backed bitfield must have a known length");
        let half_len = array_len / 2;
        let last_idx = array_len - 1;

        let bits_access_tokens = if bitfield.arguments().array_heap() {
            quote! { *#bitfield_internal_value_ident_tokens }
        } else {
            quote! { #bitfield_internal_value_ident_tokens }
        };

        let get_bits_var = |endian: ConversionEndian| -> TokenStream {
            match endian {
                ConversionEndian::Little => quote! {
                    let bits = #bits_access_tokens;
                },
                ConversionEndian::Big => quote! {
                    let mut bits = #bits_access_tokens;
                    let mut i = 0usize;
                    while i < #half_len {
                        let j = #last_idx - i;
                        bits[i] ^= bits[j];
                        bits[j] ^= bits[i];
                        bits[i] ^= bits[j];
                        i += 1;
                    }
                },
            }
        };

        let default_get_bits = get_bits_var(bitfield.arguments().into_endian());
        let le_get_bits = get_bits_var(ConversionEndian::Little);
        let be_get_bits = get_bits_var(ConversionEndian::Big);

        let copy_to_slice_tokens = quote! {
            let copy_len = if slice.len() < #array_len { slice.len() } else { #array_len };
            let mut i = 0usize;
            while i < copy_len {
                slice[i] = bits[i];
                i += 1;
            }
        };

        let copy_exact_to_slice_tokens = quote! {
            let mut i = 0usize;
            while i < #array_len {
                slice[i] = bits[i];
                i += 1;
            }
        };

        let size_check_tokens = quote! {
            if slice.len() < #array_len {
                return Err("Slice is too small to hold the bitfield.");
            }
        };

        quote! {

            #[doc = "Writes the bitfield bytes into the provided slice. \
                     If the slice is shorter than the bitfield only the bytes \
                     that fit are written."]
            #visibility_tokens #function_modifier_tokens fn into_slice(&self, slice: &mut [u8]) {
                let this = self;
                #default_get_bits
                #copy_to_slice_tokens
            }

            #[doc = "Writes the bitfield bytes into the provided slice. \
                     Returns an error if the slice is too small to hold the \
                     entire bitfield."]
            #visibility_tokens #function_modifier_tokens fn checked_into_slice(&self, slice: &mut [u8]) -> ::core::result::Result<(), &'static str> {
                let this = self;
                #size_check_tokens
                #default_get_bits
                #copy_exact_to_slice_tokens
                Ok(())
            }

            #[doc = "Writes the bitfield bytes in little-endian order into the \
                     provided slice. If the slice is shorter than the bitfield \
                     only the bytes that fit are written."]
            #visibility_tokens #function_modifier_tokens fn into_le_slice(&self, slice: &mut [u8]) {
                let this = self;
                #le_get_bits
                #copy_to_slice_tokens
            }

            #[doc = "Writes the bitfield bytes in little-endian order into the \
                     provided slice. Returns an error if the slice is too small \
                     to hold the entire bitfield."]
            #visibility_tokens #function_modifier_tokens fn checked_into_le_slice(&self, slice: &mut [u8]) -> ::core::result::Result<(), &'static str> {
                let this = self;
                #size_check_tokens
                #le_get_bits
                #copy_exact_to_slice_tokens
                Ok(())
            }

            #[doc = "Writes the bitfield bytes in big-endian order into the \
                     provided slice. If the slice is shorter than the bitfield \
                     only the bytes that fit are written."]
            #visibility_tokens #function_modifier_tokens fn into_be_slice(&self, slice: &mut [u8]) {
                let this = self;
                #be_get_bits
                #copy_to_slice_tokens
            }

            #[doc = "Writes the bitfield bytes in big-endian order into the \
                     provided slice. Returns an error if the slice is too small \
                     to hold the entire bitfield."]
            #visibility_tokens #function_modifier_tokens fn checked_into_be_slice(&self, slice: &mut [u8]) -> ::core::result::Result<(), &'static str> {
                let this = self;
                #size_check_tokens
                #be_get_bits
                #copy_exact_to_slice_tokens
                Ok(())
            }
        }
    }
}
