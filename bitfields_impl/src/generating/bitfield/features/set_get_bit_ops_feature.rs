use proc_macro2::TokenStream;
use quote::quote;

use crate::generating::bitfield::feature::{Feature, FeaturePosition, is_bit_ops_feature_enabled};
use crate::generating::bitfield::features::common::generator_helper::{
    ProtectionType, generate_protected_bits_mask_tokens, get_function_modifier_tokens,
};
use crate::parsing::bitfields::bitfield::Bitfield;
use crate::parsing::common::to_tokens::ToTokens;

/// Generates set/get bit operations for bitfield.
pub struct SetGetBitOpsFeature;

impl Feature for SetGetBitOpsFeature {
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream {
        Self::generate_set_get_bit_ops_feature_tokens(bitfield)
    }

    fn enabled(&self, bitfield: &Bitfield) -> bool {
        is_bit_ops_feature_enabled(
            bitfield,
            bitfield.arguments().generate_set_get_bit_ops(),
            bitfield.arguments().user_set_set_get_bit_ops(),
        )
    }

    fn feature_position(&self) -> FeaturePosition {
        FeaturePosition::Inside
    }

    fn order_priority(&self) -> u32 {
        6
    }
}

#[derive(Copy, Clone)]
enum GuardReturnType {
    ReturnFalse,
    NoOp,
    ReturnZero,
    OffsetOutOfRangeError,
    LengthOutOfRangeError,
    AttemptedToWriteNonWritableBitsError,
    AttemptedToReadNonReadableBitsError,
}

impl SetGetBitOpsFeature {
    fn generate_set_get_bit_ops_feature_tokens(bitfield: &Bitfield) -> TokenStream {
        let set_bit_ops_tokens = Self::generate_set_bit_ops_tokens(bitfield);
        let get_bit_ops_tokens = Self::generate_get_bit_ops_tokens(bitfield);

        quote! {
            #set_bit_ops_tokens
            #get_bit_ops_tokens
        }
    }

    fn generate_set_bit_ops_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bitfield_internal_value_ident_tokens =
            bitfield.bitfield_internal_value_ident_tokens(/* builder_caller= */ false);
        let check_offset_bitfield_range_tokens =
            Self::generate_check_offset_bitfield_range_tokens(bitfield, GuardReturnType::NoOp);
        let check_offset_and_length_bitfield_range_tokens =
            Self::generate_check_offset_and_length_bitfield_range_tokens(
                bitfield,
                GuardReturnType::NoOp,
            );
        let check_offset_bitfield_range_error_tokens =
            Self::generate_check_offset_bitfield_range_tokens(
                bitfield,
                GuardReturnType::OffsetOutOfRangeError,
            );
        let check_offset_and_length_bitfield_range_error_tokens =
            Self::generate_check_offset_and_length_bitfield_range_tokens(
                bitfield,
                GuardReturnType::LengthOutOfRangeError,
            );
        let field_bit_write_guard_no_op = Self::generate_field_bits_access_guard_tokens(
            bitfield,
            GuardReturnType::NoOp,
            /* read_access= */ false,
        );
        let field_bit_write_guard_error = Self::generate_field_bits_access_guard_tokens(
            bitfield,
            GuardReturnType::AttemptedToWriteNonWritableBitsError,
            /* read_access= */ false,
        );
        let protected_mask =
            generate_protected_bits_mask_tokens(bitfield, ProtectionType::ReadOnly);

        if bitfield.is_integer_backed() {
            quote! {
                #[doc = "Sets a bit in the bitfield."]
                #visibility_tokens #function_modifier_tokens fn set_bit(&mut self, offset: u32, bit: bool) {
                    let this = self;

                    #check_offset_bitfield_range_tokens
                    #field_bit_write_guard_no_op

                    if bit {
                        #bitfield_internal_value_ident_tokens = (#bitfield_internal_value_ident_tokens & !(1 << offset)) | (1 << offset);
                    } else {
                        #bitfield_internal_value_ident_tokens &= !(1 << offset);
                    }
                }

                #[doc = "Sets a bit in the bitfield. Returns an error if the \
                         offset is outside the bitfield range."]
                #visibility_tokens #function_modifier_tokens fn checked_set_bit(&mut self, offset: u32, bit: bool) -> ::core::result::Result<(), &'static str> {
                    let this = self;

                    #check_offset_bitfield_range_error_tokens
                    #field_bit_write_guard_error

                    if bit {
                        #bitfield_internal_value_ident_tokens = (#bitfield_internal_value_ident_tokens & !(1 << offset)) | (1 << offset);
                    } else {
                        #bitfield_internal_value_ident_tokens &= !(1 << offset);
                    }

                    Ok(())
                }

                #[doc = "Sets bits in the bitfield starting from the offset \
                         to the length with the provided value."]
                #visibility_tokens #function_modifier_tokens fn set_bits_range(&mut self, offset: u32, length: u32, value: #bitfield_data_type_tokens) {
                    let this = self;
                    #check_offset_bitfield_range_tokens
                    #check_offset_and_length_bitfield_range_tokens
                    let range_mask = (#bitfield_data_type_tokens::MAX >> (#bitfield_data_type_tokens::BITS - length)) << offset;
                    #protected_mask
                    let writable_mask = range_mask & !protected_mask;
                    #bitfield_internal_value_ident_tokens = (#bitfield_internal_value_ident_tokens & !writable_mask) | ((value << offset) & writable_mask);
                }

                #[doc = "Sets bits in the bitfield starting from the offset \
                         to the length with the provided value. Returns an \
                         error if the offset and length is outside the \
                         bitfield range, if the value is too large for the \
                         length, or if any bit in the range is non-writable."]
                #visibility_tokens #function_modifier_tokens fn checked_set_bits_range(&mut self, offset: u32, length: u32, value: #bitfield_data_type_tokens) -> ::core::result::Result<(), &'static str> {
                    let this = self;
                    #check_offset_bitfield_range_error_tokens
                    #check_offset_and_length_bitfield_range_error_tokens
                    let mask = #bitfield_data_type_tokens::MAX >> (#bitfield_data_type_tokens::BITS - length);
                    #protected_mask
                    let range_mask = mask << offset;
                    if range_mask & protected_mask != 0 {
                        return Err("Attempted to write to non-writable bit(s).");
                    }
                    #bitfield_internal_value_ident_tokens = (#bitfield_internal_value_ident_tokens & !(mask << offset)) | ((value & mask) << offset);
                    Ok(())
                }
            }
        } else {
            quote! {
                #[doc = "Sets a bit in the bitfield."]
                #visibility_tokens #function_modifier_tokens fn set_bit(&mut self, offset: u32, bit: bool) {
                    let this = self;

                    #check_offset_bitfield_range_tokens
                    #field_bit_write_guard_no_op

                    let byte_idx = offset / 8;
                    let bit_in_byte = offset % 8;
                    if bit {
                        #bitfield_internal_value_ident_tokens[byte_idx as usize] |= 1 << bit_in_byte;
                    } else {
                        #bitfield_internal_value_ident_tokens[byte_idx as usize] &= !(1 << bit_in_byte);
                    }
                }

                #[doc = "Sets a bit in the bitfield. Returns an error if the \
                         offset is outside the bitfield range."]
                #visibility_tokens #function_modifier_tokens fn checked_set_bit(&mut self, offset: u32, bit: bool) -> ::core::result::Result<(), &'static str> {
                    let this = self;

                    #check_offset_bitfield_range_error_tokens
                    #field_bit_write_guard_error

                    let byte_idx = offset / 8;
                    let bit_in_byte = offset % 8;
                    if bit {
                        #bitfield_internal_value_ident_tokens[byte_idx as usize] |= 1 << bit_in_byte;
                    } else {
                        #bitfield_internal_value_ident_tokens[byte_idx as usize] &= !(1 << bit_in_byte);
                    }

                    Ok(())
                }

                #[doc = "Sets bytes in the bitfield starting from the offset \
                         to the length with the provided value."]
                #visibility_tokens #function_modifier_tokens fn set_bytes_range(&mut self, offset: u32, length: u32, value: #bitfield_data_type_tokens) {
                    let this = self;
                    #check_offset_bitfield_range_tokens
                    #check_offset_and_length_bitfield_range_tokens
                    #protected_mask
                    let mut i = 0u32;
                    while i < length {
                        let bit_val = ((value[(i / 8) as usize] >> (i % 8)) & 1) != 0;
                        let dst_bit = offset + i;
                        let dst_byte_idx = (dst_bit / 8) as usize;
                        let dst_bit_in_byte = dst_bit % 8;
                        let is_protected = (protected_mask[dst_byte_idx] >> dst_bit_in_byte) & 1 != 0;
                        if !is_protected {
                            if bit_val {
                                #bitfield_internal_value_ident_tokens[dst_byte_idx] |= 1 << dst_bit_in_byte;
                            } else {
                                #bitfield_internal_value_ident_tokens[dst_byte_idx] &= !(1 << dst_bit_in_byte);
                            }
                        }
                        i += 1;
                    }
                }

                #[doc = "Sets bytes in the bitfield starting from the offset \
                         to the length with the provided value. Returns an \
                         error if the offset and length is outside the \
                         bitfield range, if the value is too large for the \
                         length, or if any bit in the range is non-writable."]
                #visibility_tokens #function_modifier_tokens fn checked_set_bytes_range(&mut self, offset: u32, length: u32, value: #bitfield_data_type_tokens) -> ::core::result::Result<(), &'static str> {
                    let this = self;
                    #check_offset_bitfield_range_error_tokens
                    #check_offset_and_length_bitfield_range_error_tokens
                    #protected_mask
                    let mut i = 0u32;
                    while i < length {
                        let dst_bit = offset + i;
                        let dst_byte_idx = (dst_bit / 8) as usize;
                        let dst_bit_in_byte = dst_bit % 8;
                        let is_protected = (protected_mask[dst_byte_idx] >> dst_bit_in_byte) & 1 != 0;
                        if is_protected {
                            return Err("Attempted to write to non-writable bit(s).");
                        }
                        i += 1;
                    }
                    let mut i = 0u32;
                    while i < length {
                        let bit_val = ((value[(i / 8) as usize] >> (i % 8)) & 1) != 0;
                        let dst_bit = offset + i;
                        let dst_byte_idx = (dst_bit / 8) as usize;
                        let dst_bit_in_byte = dst_bit % 8;
                        if bit_val {
                            #bitfield_internal_value_ident_tokens[dst_byte_idx] |= 1 << dst_bit_in_byte;
                        } else {
                            #bitfield_internal_value_ident_tokens[dst_byte_idx] &= !(1 << dst_bit_in_byte);
                        }
                        i += 1;
                    }
                    Ok(())
                }
            }
        }
    }

    fn generate_get_bit_ops_tokens(bitfield: &Bitfield) -> TokenStream {
        let visibility_tokens = bitfield.visibility().to_tokens();
        let function_modifier_tokens = get_function_modifier_tokens(bitfield);
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bitfield_internal_value_ident_tokens =
            bitfield.bitfield_internal_value_ident_tokens(/* builder_caller= */ false);
        let check_offset_bitfield_range_tokens = Self::generate_check_offset_bitfield_range_tokens(
            bitfield,
            GuardReturnType::ReturnFalse,
        );
        let check_offset_bitfield_range_error_tokens =
            Self::generate_check_offset_bitfield_range_tokens(
                bitfield,
                GuardReturnType::OffsetOutOfRangeError,
            );
        let check_offset_and_length_bitfield_range_error_tokens =
            Self::generate_check_offset_and_length_bitfield_range_tokens(
                bitfield,
                GuardReturnType::LengthOutOfRangeError,
            );
        let check_offset_bitfield_range_return_zero_tokens =
            Self::generate_check_offset_bitfield_range_tokens(
                bitfield,
                GuardReturnType::ReturnZero,
            );
        let check_offset_and_length_bitfield_range_return_zero_tokens =
            Self::generate_check_offset_and_length_bitfield_range_tokens(
                bitfield,
                GuardReturnType::ReturnZero,
            );

        let field_bit_write_guard_return_false = Self::generate_field_bits_access_guard_tokens(
            bitfield,
            GuardReturnType::ReturnFalse,
            /* read_access= */ true,
        );
        let field_bit_read_guard_error = Self::generate_field_bits_access_guard_tokens(
            bitfield,
            GuardReturnType::AttemptedToReadNonReadableBitsError,
            /* read_access= */ true,
        );
        let protected_mask =
            generate_protected_bits_mask_tokens(bitfield, ProtectionType::WriteOnly);
        let array_len = bitfield.spanned_data_type_token().array_length().unwrap_or_default();

        if bitfield.is_integer_backed() {
            quote! {
                #[doc = "Returns a bit from the bitfield."]
                #visibility_tokens #function_modifier_tokens fn get_bit(&self, offset: u32) -> bool {
                    let this = self;
                    #check_offset_bitfield_range_tokens
                    #field_bit_write_guard_return_false
                    (#bitfield_internal_value_ident_tokens >> offset) & 1 != 0
                }

                #[doc = "Returns a bit from the bitfield. Returns an error if \
                         the offset is outside the bitfield range."]
                #visibility_tokens #function_modifier_tokens fn checked_get_bit(&self, offset: u32) -> ::core::result::Result<bool, &'static str> {
                    let this = self;
                    #check_offset_bitfield_range_error_tokens
                    #field_bit_read_guard_error
                    Ok((#bitfield_internal_value_ident_tokens >> offset) & 1 != 0)
                }

                #[doc = "Returns bits from the bitfield."]
                #visibility_tokens #function_modifier_tokens fn get_bits_range(&self, offset: u32, length: u32) -> #bitfield_data_type_tokens {
                    let this = self;
                    #check_offset_bitfield_range_return_zero_tokens
                    #check_offset_and_length_bitfield_range_return_zero_tokens
                    let mask = #bitfield_data_type_tokens::MAX >> (#bitfield_data_type_tokens::BITS - length);
                    #protected_mask
                    let shifted_protected_mask = (protected_mask >> offset) & mask;
                    let readable_mask = mask & !shifted_protected_mask;
                    ((#bitfield_internal_value_ident_tokens >> offset) & mask) & readable_mask
                }

                #[doc = "Returns bits from the bitfield starting from the \
                         offset to the length. Returns an error if the offset \
                         and length is outside the bitfield range or if any \
                         bit in the range is non-readable."]
                #visibility_tokens #function_modifier_tokens fn checked_get_bits_range(&self, offset: u32, length: u32) -> ::core::result::Result<#bitfield_data_type_tokens, &'static str> {
                    let this = self;
                    #check_offset_bitfield_range_error_tokens
                    #check_offset_and_length_bitfield_range_error_tokens
                    let mask = #bitfield_data_type_tokens::MAX >> (#bitfield_data_type_tokens::BITS - length);
                    #protected_mask
                    let range_mask = mask << offset;
                    if range_mask & protected_mask != 0 {
                        return Err("Attempted to read from non-readable bit(s).");
                    }
                    Ok((#bitfield_internal_value_ident_tokens >> offset) & mask)
                }
            }
        } else {
            quote! {
                #[doc = "Returns a bit from the bitfield."]
                #visibility_tokens #function_modifier_tokens fn get_bit(&self, offset: u32) -> bool {
                    let this = self;
                    #check_offset_bitfield_range_tokens
                    #field_bit_write_guard_return_false
                    let byte_idx = offset / 8;
                    let bit_in_byte = offset % 8;
                    (#bitfield_internal_value_ident_tokens[byte_idx as usize] >> bit_in_byte) & 1 != 0
                }

                #[doc = "Returns a bit from the bitfield. Returns an error if \
                         the offset is outside the bitfield range."]
                #visibility_tokens #function_modifier_tokens fn checked_get_bit(&self, offset: u32) -> ::core::result::Result<bool, &'static str> {
                    let this = self;
                    #check_offset_bitfield_range_error_tokens
                    #field_bit_read_guard_error
                    let byte_idx = offset / 8;
                    let bit_in_byte = offset % 8;
                    Ok((#bitfield_internal_value_ident_tokens[byte_idx as usize] >> bit_in_byte) & 1 != 0)
                }

                #[doc = "Returns bytes from the bitfield."]
                #visibility_tokens #function_modifier_tokens fn get_bytes_range(&self, offset: u32, length: u32) -> #bitfield_data_type_tokens {
                    let this = self;
                    #check_offset_bitfield_range_return_zero_tokens
                    #check_offset_and_length_bitfield_range_return_zero_tokens
                    #protected_mask
                    let mut res: #bitfield_data_type_tokens = [0; #array_len];
                    let mut i = 0u32;
                    while i < length {
                        let src_bit = offset + i;
                        let src_byte = (src_bit / 8) as usize;
                        let src_bit_in_byte = src_bit % 8;
                        let is_protected = (protected_mask[src_byte] >> src_bit_in_byte) & 1 != 0;
                        if !is_protected {
                            let bit_val = (#bitfield_internal_value_ident_tokens[src_byte] >> src_bit_in_byte) & 1;
                            let dst_byte = (i / 8) as usize;
                            let dst_bit_in_byte = i % 8;
                            res[dst_byte] |= bit_val << dst_bit_in_byte;
                        }
                        i += 1;
                    }
                    res
                }

                #[doc = "Returns bytes from the bitfield starting from the \
                         offset to the length. Returns an error if the offset \
                         and length is outside the bitfield range or if any \
                         bit in the range is non-readable."]
                #visibility_tokens #function_modifier_tokens fn checked_get_bytes_range(&self, offset: u32, length: u32) -> ::core::result::Result<#bitfield_data_type_tokens, &'static str> {
                    let this = self;
                    #check_offset_bitfield_range_error_tokens
                    #check_offset_and_length_bitfield_range_error_tokens
                    #protected_mask
                    let mut i = 0u32;
                    while i < length {
                        let src_bit = offset + i;
                        let src_byte = (src_bit / 8) as usize;
                        let src_bit_in_byte = src_bit % 8;
                        let is_protected = (protected_mask[src_byte] >> src_bit_in_byte) & 1 != 0;
                        if is_protected {
                            return Err("Attempted to read from non-readable bit(s).");
                        }
                        i += 1;
                    }
                    let mut res: #bitfield_data_type_tokens = [0; #array_len];
                    let mut i = 0u32;
                    while i < length {
                        let src_bit = offset + i;
                        let src_byte = (src_bit / 8) as usize;
                        let src_bit_in_byte = src_bit % 8;
                        let bit_val = (#bitfield_internal_value_ident_tokens[src_byte] >> src_bit_in_byte) & 1;
                        let dst_byte = (i / 8) as usize;
                        let dst_bit_in_byte = i % 8;
                        res[dst_byte] |= bit_val << dst_bit_in_byte;
                        i += 1;
                    }
                    Ok(res)
                }
            }
        }
    }

    fn generate_check_offset_bitfield_range_tokens(
        bitfield: &Bitfield,
        guard_return_type: GuardReturnType,
    ) -> TokenStream {
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bitfield_data_type_bits_tokens = if bitfield.is_integer_backed() {
            quote! {
                #bitfield_data_type_tokens::BITS
            }
        } else {
            let total_bits =
                bitfield.spanned_data_type_token().array_length().unwrap_or_default() as u32 * 8;
            quote! { #total_bits }
        };
        let guard_return_tokens = Self::get_guard_return_tokens(bitfield, guard_return_type);

        quote! {
            if offset >= #bitfield_data_type_bits_tokens {
                #guard_return_tokens
            }
        }
    }

    fn generate_check_offset_and_length_bitfield_range_tokens(
        bitfield: &Bitfield,
        guard_return_type: GuardReturnType,
    ) -> TokenStream {
        let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
        let bitfield_data_type_bits_tokens = if bitfield.is_integer_backed() {
            quote! {
                #bitfield_data_type_tokens::BITS
            }
        } else {
            let total_bits =
                bitfield.spanned_data_type_token().array_length().unwrap_or_default() as u32 * 8;
            quote! { #total_bits }
        };
        let guard_return_tokens = Self::get_guard_return_tokens(bitfield, guard_return_type);

        quote! {
            if offset + length > #bitfield_data_type_bits_tokens {
                #guard_return_tokens
            }
        }
    }

    fn generate_field_bits_access_guard_tokens(
        bitfield: &Bitfield,
        guard_return_type: GuardReturnType,
        read_access: bool,
    ) -> TokenStream {
        bitfield
            .fields()
            .iter()
            .filter(
                |field| if read_access { !field.has_read_access() } else { !field.has_setter() },
            )
            .map(|field| {
                let field_offset = field.offset();
                let field_end_bits = field_offset + field.bits();
                let guard_return_tokens =
                    Self::get_guard_return_tokens(bitfield, guard_return_type);

                if bitfield.has_ignored_fields() {
                    quote! {
                        if (#field_offset..#field_end_bits).contains(&offset) {
                            #guard_return_tokens
                        }
                    }
                } else {
                    quote! {
                        if offset >= #field_offset && offset < #field_end_bits {
                            #guard_return_tokens
                        }
                    }
                }
            })
            .collect()
    }

    fn get_guard_return_tokens(
        bitfield: &Bitfield,
        guard_return_type: GuardReturnType,
    ) -> TokenStream {
        match guard_return_type {
            GuardReturnType::ReturnFalse => {
                quote! {
                    return false;
                }
            },
            GuardReturnType::NoOp => {
                quote! {
                    return;
                }
            },
            GuardReturnType::ReturnZero => {
                if bitfield.is_integer_backed() {
                    quote! {
                        return 0;
                    }
                } else {
                    let array_len =
                        bitfield.spanned_data_type_token().array_length().unwrap_or_default();
                    quote! {
                        return [0; #array_len];
                    }
                }
            },
            GuardReturnType::OffsetOutOfRangeError => {
                quote! {
                    return Err("The offset is outside the bitfield range.");
                }
            },
            GuardReturnType::LengthOutOfRangeError => {
                quote! {
                    return Err("The length is outside the bitfield range.");
                }
            },
            GuardReturnType::AttemptedToWriteNonWritableBitsError => {
                quote! {
                    return Err("Attempted to write to non-writable bit(s).");
                }
            },
            GuardReturnType::AttemptedToReadNonReadableBitsError => {
                quote! {
                    return Err("Attempted to read from non-readable bit(s).");
                }
            },
        }
    }
}
