use proc_macro2::{Literal, TokenStream};
use quote::quote;

use crate::parsing::bitfields::bitfield::{Bitfield, Field};
use crate::parsing::bitfields::bitfield_attribute::bitfield_arguments::BitOrder;
use crate::parsing::common::spanned_data_type::{DataType, IntegerType};
use crate::parsing::common::to_tokens::ToTokens;

/// Represents the source of the bits being set on a field
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BitsSource {
    /// The bits are coming from the bitfield internal value.
    Bitfield,
    /// The bits are coming from a variable.
    IntegerVariable,
}

/// Returns the function modifiers for generated functions (e.g., `const`).
pub fn get_function_modifier_tokens(bitfield: &Bitfield) -> Option<TokenStream> {
    let is_heap_array = bitfield.arguments().array_heap() && !bitfield.is_integer_backed();
    (!bitfield.has_ignored_fields() && !is_heap_array && supports_const_mut_refs())
        .then(|| quote::quote! { const })
}

/// Returns if the rust version supports const mut references.
const fn supports_const_mut_refs() -> bool {
    rustversion::cfg!(any(all(stable, since(1.83)), all(nightly, since(1.41))))
}

/// Generates bitfield struct initialization tokens.
///
/// # Examples:
///
/// ```rust,ignore
/// Self(0)
/// ```
///
/// ```rust,ignore
/// Self {
///   val: 0,
///   ignored: Ignored::Default(),
/// }
/// ```
pub fn generate_bitfield_struct_initialization_tokens(
    bitfield: &Bitfield,
    builder_caller: bool,
) -> TokenStream {
    let bitfield_value_default = match bitfield.spanned_data_type_token().data_type() {
        DataType::Integer(_) => {
            quote! {
                0
            }
        },
        DataType::Array {
            length,
        } => {
            let length = length as usize;
            if bitfield.arguments().array_heap() {
                quote! {
                    ::std::boxed::Box::new([0u8; #length])
                }
            } else {
                quote! {
                    [0u8; #length]
                }
            }
        },
        DataType::Custom => {
            unreachable!("Expected integer or array data type for bitfield backing type")
        },
    };
    let bitfield_reference = get_bitfield_reference_tokens(bitfield, builder_caller);

    if bitfield.has_ignored_fields() {
        let ignored_fields_default_calls_token_list =
            generate_ignored_fields_default_calls_token_list(bitfield);
        quote! {
            #bitfield_reference {
                val: #bitfield_value_default,
                #( #ignored_fields_default_calls_token_list, )*
            }
        }
    } else {
        quote! {
            #bitfield_reference(#bitfield_value_default)
        }
    }
}

/// Generates tokens for calling default for each ignored field.
///
/// # Example:
///
/// - `player: Player:default()`
fn generate_ignored_fields_default_calls_token_list(bitfield: &Bitfield) -> Vec<TokenStream> {
    bitfield
        .ignored_fields()
        .iter()
        .map(|field| {
            let field_name_tokens = field.name_tokens();
            let field_data_type_tokens = field.spanned_data_type_token().to_tokens();

            if matches!(field.spanned_data_type_token().data_type(), DataType::Integer(..)) {
                return quote! {
                    #field_name_tokens: <#field_data_type_tokens>::default()
                };
            }

            quote! {
                #field_name_tokens: #field_data_type_tokens::default()
            }
        })
        .collect()
}

/// Generates a tokens setting all fields to their default value list.
///
/// # Example
///
/// ```rust,ignore
/// set_a(100);
/// set_custom(Custom::A);
/// let mask = ...;
/// this.val = mask << ...;
/// ```
pub fn generate_setting_fields_to_default_value_tokens_list(
    bitfield: &Bitfield,
) -> Vec<TokenStream> {
    bitfield
        .fields()
        .iter()
        .filter(|field| field.has_default_value())
        .map(|field| generate_setting_field_to_default_tokens(bitfield, field))
        .collect()
}

/// Generates tokens for setting a field to default.
pub fn generate_setting_field_to_default_tokens(bitfield: &Bitfield, field: &Field) -> TokenStream {
    let field_default_value_tokens = field
        .arguments()
        .expect("Expected field arguments when setting default value")
        .default_value_expr()
        .expect("Expected default value when setting field defaults")
        .to_tokens();

    if field.has_setter() {
        return generate_field_setter_call_tokens(
            bitfield,
            field,
            field_default_value_tokens,
            /* builder_caller= */ false,
        );
    }

    generate_setting_field_without_setter_tokens(
        bitfield,
        field,
        field_default_value_tokens,
        /* builder_caller= */ false,
    )
}

/// Generates tokens for setting fields to zero list.
///
/// # Example
///
/// - `this.set_a(0)`
/// - `self.this.set_b(0)`
fn generate_setting_fields_to_zero_tokens_list(
    bitfield: &Bitfield,
    skip_fields_with_defaults: bool,
) -> Vec<TokenStream> {
    bitfield
        .fields()
        .iter()
        .filter(|field| !(skip_fields_with_defaults && field.has_default_value()))
        .map(|field| generate_setting_field_to_zero_tokens(bitfield, field))
        .collect()
}

/// Generates setting a field to zero.
pub fn generate_setting_field_to_zero_tokens(bitfield: &Bitfield, field: &Field) -> TokenStream {
    let value_tokens = match field.spanned_data_type_token().data_type() {
        DataType::Integer(integer_type) => {
            if integer_type == IntegerType::Bool {
                quote! { false }
            } else {
                quote! { 0 }
            }
        },
        DataType::Custom => {
            let custom_field_data_type_tokens = field.spanned_data_type_token().to_tokens();
            quote! { #custom_field_data_type_tokens::from_bits(0) }
        },
        DataType::Array {
            length,
        } => {
            if bitfield.is_integer_backed() {
                quote! { 0 }
            } else {
                let len = length as usize;
                quote! { [0u8; #len] }
            }
        },
    };

    if field.has_setter() {
        generate_field_setter_call_tokens(
            bitfield,
            field,
            value_tokens,
            /* builder_caller= */ false,
        )
    } else {
        generate_setting_field_without_setter_tokens(
            bitfield,
            field,
            value_tokens,
            /* builder_caller= */ false,
        )
    }
}

/// Generates tokens for setting a field from a variable.
pub fn generate_setting_field_from_variable_tokens(
    bitfield: &Bitfield,
    field: &Field,
    use_setter: bool,
    cast_bits: bool,
    check_bit_size: bool,
    builder_caller: bool,
) -> TokenStream {
    let is_signed_integer = !field.spanned_data_type_token().data_type().unsigned();

    let is_bool_field =
        matches!(field.spanned_data_type_token().data_type(), DataType::Integer(IntegerType::Bool));

    let is_array_field =
        matches!(field.spanned_data_type_token().data_type(), DataType::Array { .. });

    let using_setter = field.has_setter() && use_setter;

    let pre_extract_check_tokens = (!using_setter && check_bit_size && is_signed_integer)
        .then(|| generate_signed_bit_size_check_tokens(field));

    let post_extract_check_tokens = (!using_setter
        && check_bit_size
        && !is_signed_integer
        && !is_bool_field
        && !is_array_field)
        .then(generate_bit_size_check_tokens);

    let extract_field_bits_into_variable_tokens =
        generate_extract_field_bits_from_source_into_variable_tokens(
            bitfield,
            field,
            BitsSource::IntegerVariable,
            cast_bits,
            /* invert_bits= */ false,
            builder_caller,
        );

    let value_variable_tokens = get_value_variable_tokens(field);
    let set_field_to_extracted_bits_from_variable_tokens = if using_setter {
        generate_field_setter_call_tokens(bitfield, field, value_variable_tokens, builder_caller)
    } else {
        generate_setting_field_without_setter_tokens(
            bitfield,
            field,
            value_variable_tokens,
            builder_caller,
        )
    };

    quote! {
        #pre_extract_check_tokens
        #extract_field_bits_into_variable_tokens
        #post_extract_check_tokens
        #set_field_to_extracted_bits_from_variable_tokens
    }
}

fn get_value_variable_tokens(field: &Field) -> TokenStream {
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
        _ => {
            quote! { value }
        },
    }
}

/// Generates an overflow check for unsigned/custom fields.
fn generate_bit_size_check_tokens() -> TokenStream {
    quote! {
        if bits > mask {
            return Err("Value is too big to fit within the field bits.");
        }
    }
}

/// Generates an overflow check for signed integer fields.
/// Must run before `bits` is reinterpreted as unsigned.
/// Skipped when the field's bit-width equals the full type width.
fn generate_signed_bit_size_check_tokens(field: &Field) -> TokenStream {
    let field_bits = field.bits();
    let type_bits = field.spanned_data_type_token().data_type().bit_size();

    if field_bits >= type_bits {
        return quote! {};
    }

    let max_u: u128 = (1u128 << (field_bits - 1)) - 1;
    let min_abs_u: u128 = max_u + 1;

    let max_lit = make_signed_literal(field, max_u as i128);
    let min_abs_lit = make_signed_literal(field, min_abs_u as i128);

    quote! {
        if bits > #max_lit || bits < -(#min_abs_lit) {
            return Err("Value is too big to fit within the field bits.");
        }
    }
}

/// Creates a typed suffixed integer literal matching the signed integer type of
/// `field`.
fn make_signed_literal(field: &Field, value: i128) -> Literal {
    match field.spanned_data_type_token().data_type() {
        DataType::Integer(IntegerType::I8) => Literal::i8_suffixed(value as i8),
        DataType::Integer(IntegerType::I16) => Literal::i16_suffixed(value as i16),
        DataType::Integer(IntegerType::I32) => Literal::i32_suffixed(value as i32),
        DataType::Integer(IntegerType::I64) => Literal::i64_suffixed(value as i64),
        DataType::Integer(IntegerType::I128) => Literal::i128_suffixed(value),
        _ => unreachable!(
            "Expected signed integer type for generating signed bit size check literal"
        ),
    }
}

/// Generates extracting a field's bits from a source into a variable tokens.
pub fn generate_extract_field_bits_from_source_into_variable_tokens(
    bitfield: &Bitfield,
    field: &Field,
    bits_source: BitsSource,
    cast_bits: bool,
    invert_bits: bool,
    builder_caller: bool,
) -> TokenStream {
    match bits_source {
        BitsSource::Bitfield | BitsSource::IntegerVariable => {
            let bits_source_tokens = match bits_source {
                BitsSource::Bitfield => {
                    bitfield.bitfield_internal_value_ident_tokens(builder_caller)
                },
                BitsSource::IntegerVariable => quote! {
                    bits
                },
            };

            let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();

            let field_bits_tokens = get_field_bits_tokens(bitfield, field, builder_caller);
            let field_offset_tokens = get_field_offset_tokens(bitfield, field, builder_caller);

            let invert_value_tokens = invert_bits.then(|| {
                quote! {
                   !
                }
            });

            if bitfield.is_integer_backed() {
                let casting_bits_to_bitfield_data_type_tokens =
                    (bits_source == BitsSource::IntegerVariable && cast_bits).then(|| {
                        if matches!(field.spanned_data_type_token().data_type(), DataType::Custom) {
                            quote! {
                                #[allow(clippy::unnecessary_cast)]
                                let bits = bits.into_bits() as #bitfield_data_type_tokens;
                            }
                        } else if let DataType::Array {
                            length,
                        } = field.spanned_data_type_token().data_type()
                        {
                            let len = length as usize;
                            let pack_len = (field.bits() as usize / 8).min(len);
                            quote! {
                                let bits = {
                                    let __arr = bits;
                                    let mut __v: #bitfield_data_type_tokens = 0;
                                    let mut __i: usize = 0;
                                    while __i < #pack_len {
                                        __v |= (__arr[__i] as #bitfield_data_type_tokens)
                                            << (__i * 8);
                                        __i += 1;
                                    }
                                    __v
                                };
                            }
                        } else {
                            quote! {
                                let bits = bits as #bitfield_data_type_tokens;
                            }
                        }
                    });

                let value_extraction_tokens = if bits_source == BitsSource::IntegerVariable
                    && cast_bits
                {
                    quote! {
                        let value = #invert_value_tokens #bits_source_tokens & mask;
                    }
                } else {
                    quote! {
                        let value = #invert_value_tokens(#bits_source_tokens >> #field_offset_tokens) & mask;
                    }
                };

                quote! {
                    let mask = #bitfield_data_type_tokens::MAX >> (#bitfield_data_type_tokens::BITS - #field_bits_tokens);
                    #casting_bits_to_bitfield_data_type_tokens
                    #value_extraction_tokens
                }
            } else if bits_source == BitsSource::IntegerVariable && cast_bits {
                if matches!(
                    field.spanned_data_type_token().data_type(),
                    DataType::Integer(IntegerType::Bool)
                ) {
                    return quote! {
                        let value = bits as u128;
                    };
                }
                if matches!(field.spanned_data_type_token().data_type(), DataType::Array { .. }) {
                    return quote! {
                        let value = bits;
                    };
                }
                if matches!(field.spanned_data_type_token().data_type(), DataType::Custom) {
                    return quote! {
                        let bits = bits.into_bits() as u128;
                        let mask = if #field_bits_tokens == 128 { u128::MAX } else { (1u128 << #field_bits_tokens) - 1 };
                        let value = {
                            let val = bits & mask;
                            #invert_value_tokens val
                        };
                    };
                }
                let field_data_type_tokens = field.spanned_data_type_token().to_tokens();
                quote! {
                    let mask = (if #field_bits_tokens == 128 { u128::MAX } else { (1u128 << #field_bits_tokens) - 1 }) as #field_data_type_tokens;
                    let value = {
                        let val = bits & mask;
                        #invert_value_tokens val
                    };
                }
            } else {
                if let DataType::Array {
                    length,
                } = field.spanned_data_type_token().data_type()
                {
                    let len = length as usize;
                    return if invert_bits {
                        quote! {
                            let value = {
                                let mut __arr = [0u8; #len];
                                let mut __i: u32 = 0;
                                while __i < #field_bits_tokens {
                                    let bit_index = #field_offset_tokens + __i;
                                    let byte_idx = (bit_index / 8) as usize;
                                    let bit_in_byte = bit_index % 8;
                                    let bit_val = ((#bits_source_tokens[byte_idx] >> bit_in_byte) & 1) ^ 1;
                                    __arr[(__i / 8) as usize] |= bit_val << (__i % 8);
                                    __i += 1;
                                }
                                __arr
                            };
                        }
                    } else {
                        quote! {
                            let value = {
                                let mut __arr = [0u8; #len];
                                let mut __i: u32 = 0;
                                while __i < #field_bits_tokens {
                                    let bit_index = #field_offset_tokens + __i;
                                    let byte_idx = (bit_index / 8) as usize;
                                    let bit_in_byte = bit_index % 8;
                                    let bit_val = (#bits_source_tokens[byte_idx] >> bit_in_byte) & 1;
                                    __arr[(__i / 8) as usize] |= bit_val << (__i % 8);
                                    __i += 1;
                                }
                                __arr
                            };
                        }
                    };
                }

                let final_val_tokens = if invert_bits {
                    quote! {
                        (!val) & (if #field_bits_tokens == 128 { u128::MAX } else { (1u128 << #field_bits_tokens) - 1 })
                    }
                } else {
                    quote! { val }
                };
                quote! {
                    let value = {
                        let mut val = 0u128;
                        let mut i: u32 = 0;
                        while i < #field_bits_tokens {
                            let bit_index = #field_offset_tokens + i;
                            let byte_idx = bit_index / 8;
                            let bit_in_byte = bit_index % 8;
                            let bit_val = ((#bits_source_tokens[byte_idx as usize] >> bit_in_byte) & 1) as u128;
                            val |= bit_val << i;
                            i += 1;
                        }
                        #final_val_tokens
                    };
                }
            }
        },
    }
}

/// Generates tokens for calling the field setter with the provided value.
///
/// # Example
///
/// - `this.set_a(99)`
/// - `self.this.set_b(88)`
fn generate_field_setter_call_tokens(
    bitfield: &Bitfield,
    field: &Field,
    value_tokens: TokenStream,
    builder_caller: bool,
) -> TokenStream {
    let field_setter_ident_tokens = field.setter_ident_tokens();

    let bitfield_variable_reference = if builder_caller {
        quote! { self.this }
    } else {
        quote! { this }
    };

    match field.spanned_data_type_token().data_type() {
        DataType::Integer(integer_type) => {
            if integer_type == IntegerType::Bool {
                quote! {
                    #bitfield_variable_reference.#field_setter_ident_tokens(#value_tokens);
                }
            } else {
                quote! {
                    #bitfield_variable_reference.#field_setter_ident_tokens(#value_tokens as _);
                }
            }
        },
        DataType::Custom => {
            quote! {
                #bitfield_variable_reference.#field_setter_ident_tokens(#value_tokens);
            }
        },
        DataType::Array {
            length,
        } => {
            let len = length as usize;
            if bitfield.is_integer_backed() {
                quote! {
                    #bitfield_variable_reference.#field_setter_ident_tokens({
                        let __int_val = (#value_tokens) as u128;
                        let mut __arr = [0u8; #len];
                        let mut __i: usize = 0;
                        while __i < #len {
                            __arr[__i] = ((__int_val >> (__i * 8)) & 0xFF) as u8;
                            __i += 1;
                        }
                        __arr
                    });
                }
            } else {
                quote! {
                    #bitfield_variable_reference.#field_setter_ident_tokens(#value_tokens);
                }
            }
        },
    }
}

/// Generates tokens to bitwise set a field to a value directly.
fn generate_setting_field_without_setter_tokens(
    bitfield: &Bitfield,
    field: &Field,
    value_tokens: TokenStream,
    builder_caller: bool,
) -> TokenStream {
    let bitfield_internal_value_ident_tokens =
        bitfield.bitfield_internal_value_ident_tokens(builder_caller);
    let bitfield_data_type_tokens = bitfield.spanned_data_type_token().to_tokens();
    let field_bits_tokens = get_field_bits_tokens(bitfield, field, builder_caller);
    let field_offset_tokens = get_field_offset_tokens(bitfield, field, builder_caller);

    if !bitfield.is_integer_backed() {
        if let DataType::Array {
            ..
        } = field.spanned_data_type_token().data_type()
        {
            return quote! {
                {
                    let __src = #value_tokens;
                    let mut __i: u32 = 0;
                    while __i < #field_bits_tokens {
                        let bit_index = #field_offset_tokens + __i;
                        let byte_idx = (bit_index / 8) as usize;
                        let bit_in_byte = bit_index % 8;
                        let src_bit = (__src[(__i / 8) as usize] >> (__i % 8)) & 1;
                        if src_bit == 1 {
                            #bitfield_internal_value_ident_tokens[byte_idx] |= 1 << bit_in_byte;
                        } else {
                            #bitfield_internal_value_ident_tokens[byte_idx] &= !(1 << bit_in_byte);
                        }
                        __i += 1;
                    }
                }
            };
        }

        let val_to_set_tokens = match field.spanned_data_type_token().data_type() {
            DataType::Integer(IntegerType::Bool) => quote! {
                if #value_tokens { 1u128 } else { 0u128 }
            },
            DataType::Custom => quote! {
                #value_tokens.into_bits() as u128
            },
            _ => quote! {
                #value_tokens as u128
            },
        };

        return quote! {
            {
                let val_to_set = #val_to_set_tokens;
                let mut i: u32 = 0;
                while i < #field_bits_tokens {
                    let bit_index = #field_offset_tokens + i;
                    let byte_idx = bit_index / 8;
                    let bit_in_byte = bit_index % 8;
                    let bit_val = (val_to_set >> i) & 1;
                    if bit_val == 1 {
                        #bitfield_internal_value_ident_tokens[byte_idx as usize] |= 1 << bit_in_byte;
                    } else {
                        #bitfield_internal_value_ident_tokens[byte_idx as usize] &= !(1 << bit_in_byte);
                    }
                    i += 1;
                }
            }
        };
    }

    if matches!(field.spanned_data_type_token().data_type(), DataType::Integer(IntegerType::Bool)) {
        return quote! {
            if #value_tokens {
                #bitfield_internal_value_ident_tokens = (#bitfield_internal_value_ident_tokens & !(1 << #field_offset_tokens)) | (1 << #field_offset_tokens);
            } else {
                #bitfield_internal_value_ident_tokens &= !(1 << #field_offset_tokens);
            }
        };
    }

    if matches!(field.spanned_data_type_token().data_type(), DataType::Custom) {
        return quote! {
            let mask = #bitfield_data_type_tokens::MAX >> (#bitfield_data_type_tokens::BITS - #field_bits_tokens);
            #[allow(clippy::unnecessary_cast)]
            let field_bits = #value_tokens.into_bits() as #bitfield_data_type_tokens;
            #bitfield_internal_value_ident_tokens = (#bitfield_internal_value_ident_tokens & !(mask << #field_offset_tokens)) | ((field_bits & mask) << #field_offset_tokens);
        };
    }

    quote! {
        let mask = #bitfield_data_type_tokens::MAX >> (#bitfield_data_type_tokens::BITS - #field_bits_tokens);
        let field_bits = #value_tokens;
        #bitfield_internal_value_ident_tokens = (#bitfield_internal_value_ident_tokens & !(mask << #field_offset_tokens)) | ((field_bits & mask) << #field_offset_tokens);
    }
}

/// Specifies which fields to protect during an operation.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ProtectionType {
    /// Protect fields that have no setter (read-only).
    ReadOnly,
    /// Protect fields that have no read access (write-only).
    WriteOnly,
    None,
}

pub fn generate_extract_all_field_bits_from_variable_into_variable_tokens_and_set_fields_tokens(
    bitfield: &Bitfield,
    protection_type: ProtectionType,
) -> TokenStream {
    bitfield
        .fields()
        .iter()
        .filter(|field| match protection_type {
            ProtectionType::ReadOnly => field.has_setter(),
            ProtectionType::WriteOnly => field.has_read_access(),
            ProtectionType::None => true,
        })
        .map(|field: &Field| {
            generate_setting_field_from_variable_tokens(
                bitfield, field, /* use_setter= */ true, /* cast_bits= */ false,
                /* check_bit_size= */ false, /* builder_caller= */ false,
            )
        })
        .collect()
}

/// Returns the field bits tokens, using the bits constant if available.
pub fn get_field_bits_tokens(
    bitfield: &Bitfield,
    field: &Field,
    builder_caller: bool,
) -> TokenStream {
    let bitfield_reference = get_bitfield_reference_tokens(bitfield, builder_caller);

    if field.has_constants() {
        let field_bits_constant_ident_tokens = field.bits_constant_ident_tokens();
        quote! {
            #bitfield_reference::#field_bits_constant_ident_tokens
        }
    } else {
        let field_bits = field.bits();

        quote! {
            #field_bits
        }
    }
}

/// Returns the field offset tokens, using the offset constant if available.
pub fn get_field_offset_tokens(
    bitfield: &Bitfield,
    field: &Field,
    builder_caller: bool,
) -> TokenStream {
    let bitfield_reference = get_bitfield_reference_tokens(bitfield, builder_caller);

    if field.has_constants() {
        let field_offset_constant_ident_tokens = field.offset_constant_ident_tokens();
        quote! {
            #bitfield_reference::#field_offset_constant_ident_tokens
        }
    } else {
        let field_offset = field.offset();

        quote! {
            #field_offset
        }
    }
}

/// Generates bitfield reference tokens.
fn get_bitfield_reference_tokens(bitfield: &Bitfield, builder_caller: bool) -> TokenStream {
    if builder_caller {
        let bitfield_name_tokens = bitfield.name_tokens();
        quote! {
            #bitfield_name_tokens
        }
    } else {
        quote! {
            Self
        }
    }
}

/// Returns `("bit", "bits")` for integer/custom fields and `("byte", "bytes")`
/// for array fields.
pub fn get_field_unit_terms(field: &Field) -> (&'static str, &'static str) {
    match field.spanned_data_type_token().data_type() {
        DataType::Array {
            ..
        } => ("byte", "bytes"),
        _ => ("bit", "bits"),
    }
}

/// Returns the setter documentation for a field.
pub fn get_setter_documentation(
    bitfield: &Bitfield,
    field: &Field,
    checked_setter: bool,
    builder_caller: bool,
) -> String {
    let offset = field.offset();
    let builder_caller_prefix = if builder_caller { "builder" } else { "" };
    let (unit, units) = get_field_unit_terms(field);

    if field.bits() == 1 {
        let suffix = if checked_setter
            && !matches!(
                field.spanned_data_type_token().data_type(),
                DataType::Integer(IntegerType::Bool)
            ) {
            format!(". Returns an error if the value is too big to fit within the field {unit}")
        } else {
            String::new()
        };
        return format!("Sets {builder_caller_prefix} {unit} `{offset}`{suffix}.");
    }

    let bits_end = offset + field.bits() - 1;

    let (documentation_bits_start, documentation_bits_end) =
        if bitfield.arguments().order() == BitOrder::Msb {
            (bits_end, offset)
        } else {
            (offset, bits_end)
        };

    let suffix = if checked_setter {
        format!(". Returns an error if the value is too big to fit within the field {units}")
    } else {
        String::new()
    };
    format!(
        "Sets {builder_caller_prefix} {units} \
         `{documentation_bits_start}..={documentation_bits_end}`{suffix}."
    )
}

/// Generates the internal implementation of a new function.
pub fn generate_new_function_implementation_tokens(
    bitfield: &Bitfield,
    generate_setting_defaults: bool,
    builder_caller: bool,
    existing_bitfield: bool,
) -> TokenStream {
    let bitfield_struct_initialization_tokens =
        generate_bitfield_struct_initialization_tokens(bitfield, builder_caller);
    let setting_fields_to_zero_tokens_list =
        generate_setting_fields_to_zero_tokens_list(bitfield, generate_setting_defaults);
    let setting_fields_to_default_value_tokens_list = if generate_setting_defaults {
        generate_setting_fields_to_default_value_tokens_list(bitfield)
    } else {
        Vec::default()
    };
    let setting_reserved_fields_to_default_value_token_list = if !generate_setting_defaults {
        bitfield
            .fields()
            .iter()
            .filter(|field: &&Field| field.is_reserved() && field.has_default_value())
            .map(|field| generate_setting_field_to_default_tokens(bitfield, field))
            .collect()
    } else {
        Vec::default()
    };

    let this_mutable_modifier_tokens = generate_this_mutable_modifier_tokens(
        !setting_fields_to_zero_tokens_list.is_empty(),
        !setting_fields_to_default_value_tokens_list.is_empty(),
    );

    let bitfield_reference = (!existing_bitfield).then(|| {
        quote! {
            let #this_mutable_modifier_tokens this = #bitfield_struct_initialization_tokens;
        }
    });

    quote! {
        #bitfield_reference
        #( #setting_fields_to_zero_tokens_list )*
        #( #setting_fields_to_default_value_tokens_list )*
        #( #setting_reserved_fields_to_default_value_token_list )*
    }
}

/// Generates the mutable modifier if the variable will mutated.
fn generate_this_mutable_modifier_tokens(
    setting_fields_to_zero: bool,
    setting_fields_default_value: bool,
) -> Option<TokenStream> {
    (setting_fields_to_zero || setting_fields_default_value).then(|| {
        quote! {
            mut
        }
    })
}

/// Generates sign-extension tokens for signed fields.
pub fn generate_sign_extend_bit_operation_tokens(field: &Field) -> TokenStream {
    if matches!(field.spanned_data_type_token().data_type(), DataType::Custom)
        || field.spanned_data_type_token().data_type().unsigned()
    {
        return quote! {};
    }

    let field_data_type_tokens = field.spanned_data_type_token().to_tokens();
    let field_data_type_bit_size = field.spanned_data_type_token().data_type().bit_size();
    let field_bits = field.bits();

    quote! {
        let shift = #field_data_type_bit_size - #field_bits;
        let value = value as #field_data_type_tokens;
        let value = (value << shift) >> shift;
    }
}

pub fn generate_protected_bits_mask_tokens(
    bitfield: &Bitfield,
    protection_type: ProtectionType,
) -> TokenStream {
    let bitfield_type = bitfield.spanned_data_type_token().to_tokens();

    let bitfield_fields = bitfield.fields();
    let protected_fields: Vec<&Field> = bitfield_fields
        .iter()
        .filter(|field| match protection_type {
            ProtectionType::ReadOnly => !field.has_setter(),
            ProtectionType::WriteOnly => !field.has_read_access(),
            ProtectionType::None => false,
        })
        .collect();

    if bitfield.is_integer_backed() {
        let mut protected_mask: u128 = 0;
        for field in &protected_fields {
            let field_offset = field.offset();
            let field_end_bits = field_offset + field.bits();
            for bit in field_offset..field_end_bits {
                protected_mask |= 1u128 << bit;
            }
        }
        quote! {
            let protected_mask = #protected_mask as #bitfield_type;
        }
    } else {
        let num_bytes = (bitfield.spanned_data_type_token().data_type().bit_size() / 8) as usize;
        let mut bytes_list = vec![0u8; num_bytes];
        for field in &protected_fields {
            let field_offset = field.offset();
            let field_end_bits = field_offset + field.bits();
            for bit in field_offset..field_end_bits {
                let byte_idx = (bit / 8) as usize;
                let bit_in_byte = (bit % 8) as u8;
                if byte_idx < num_bytes {
                    bytes_list[byte_idx] |= 1u8 << bit_in_byte;
                }
            }
        }
        quote! {
            let protected_mask: #bitfield_type = [ #(#bytes_list),* ];
        }
    }
}

/// Returns `bits` for integer-backed bitfields and `bytes` for array-backed
/// bitfields.
pub fn generate_backing_data_param_ident(bitfield: &Bitfield) -> TokenStream {
    if bitfield.is_integer_backed() {
        quote! { bits }
    } else {
        quote! { bytes }
    }
}

/// Returns the term used for the backing data of a bitfield.
pub const fn get_bits_or_bytes_term(bitfield: &Bitfield) -> &'static str {
    if bitfield.is_integer_backed() { "bits" } else { "bytes" }
}
