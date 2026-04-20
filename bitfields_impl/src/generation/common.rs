use std::iter::Map;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Type;

use crate::generation::bit_manipulation_common::{
    generate_extract_value_from_value_tokens, generate_mask_implementation_tokens,
};
use crate::parsing::bitfield_attribute::{BitOrder, BitfieldAttribute};
use crate::parsing::bitfield_field::{BitfieldField, FieldAccess, FieldType};
use crate::parsing::types::{IntegerType, get_integer_type_from_type};

/// Represent ways a struct can be referenced.
#[derive(Clone)]
pub(crate) enum BitfieldStructReferenceIdent {
    SelfReference,
    SelfVariable,
    ThisVariable,
    NameReference(String),
}

impl BitfieldStructReferenceIdent {
    /// Returns token stream of reference.
    pub(crate) fn to_token_stream(&self) -> TokenStream {
        match self {
            BitfieldStructReferenceIdent::SelfReference => quote! { Self },
            BitfieldStructReferenceIdent::SelfVariable => quote! { self },
            BitfieldStructReferenceIdent::ThisVariable => quote! { this },
            BitfieldStructReferenceIdent::NameReference(name) => {
                let name = format_ident!("{}", name);
                quote! { #name }
            }
        }
    }
}

/// An error message to display when a panic occurs, which should never happen.
pub(crate) const PANIC_ERROR_MESSAGE: &str = "A major unexpected error has occurred. If possible, please file an issue with the code that caused this error at https://github.com/gregorygaines/bitfields-rs/issues.";

/// Error message for when the value is too big to fit within the field bits.
const BITS_TOO_BIG_ERROR_MESSAGE: &str = "Value is too big to fit within the field bits.";

/// Generates tokens to set the default values for non-padding fields or zero if
/// no default value is provided.
///
/// By default, it uses the field's setter method. If the field is read-only,
/// then it inlines bitwise operations to set the field.
pub(crate) fn generate_setting_fields_default_values_tokens(
    bitfield_type: &Type,
    fields: &[BitfieldField],
    bitfield_struct_reference_ident: &BitfieldStructReferenceIdent,
    has_ignored_fields: bool,
) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            generate_setting_fields_default_values_tokens_internal_helper(
                bitfield_type,
                field,
                bitfield_struct_reference_ident,
                has_ignored_fields,
            )
        })
        .collect()
}

fn generate_setting_fields_default_values_tokens_internal_helper(
    bitfield_type: &Type,
    field: &BitfieldField,
    bitfield_struct_reference_ident: &BitfieldStructReferenceIdent,
    has_ignored_fields: bool,
) -> TokenStream {
    let default_value = &field.default_value_tokens;
    let field_type_ident = &field.ty;
    let field_integer_type = get_integer_type_from_type(&field.ty);

    let field_has_setter = does_field_have_setter(field);
    if field_has_setter {
        let field_offset_setter_ident = get_field_setter_method_identifier(&field.name.to_string());

        return match default_value {
            Some(default_value) => {
                quote! {
                    this.#field_offset_setter_ident(#default_value);
                }
            }
            None => {
                if field.field_type == FieldType::CustomFieldType {
                    return quote! {
                        this.#field_offset_setter_ident(#field_type_ident::from_bits(0));
                    };
                }

                if field_integer_type == IntegerType::Bool {
                    return quote! {
                        this.#field_offset_setter_ident(false);
                    };
                }

                quote! {
                    this.#field_offset_setter_ident(0);
                }
            }
        };
    }

    match default_value {
        Some(default_value) => {
            generate_setter_impl_tokens(
                bitfield_type,
                field,
                bitfield_struct_reference_ident,
                quote! { #default_value },
                /* check_value_bit_size= */ false,
                get_bitfield_struct_internal_value_identifier_tokens(
                    &BitfieldStructReferenceIdent::ThisVariable,
                    has_ignored_fields,
                ),
            )
        }
        None => {
            if field.field_type == FieldType::CustomFieldType {
                return generate_setter_impl_tokens(
                    bitfield_type,
                    field,
                    bitfield_struct_reference_ident,
                    quote! { #field_type_ident::from_bits(0) },
                    /* check_value_bit_size= */ false,
                    get_bitfield_struct_internal_value_identifier_tokens(
                        &BitfieldStructReferenceIdent::ThisVariable,
                        has_ignored_fields,
                    ),
                );
            }
            if field_integer_type == IntegerType::Bool {
                return generate_setter_impl_tokens(
                    bitfield_type,
                    field,
                    bitfield_struct_reference_ident,
                    quote! { false },
                    /* check_value_bit_size= */ false,
                    get_bitfield_struct_internal_value_identifier_tokens(
                        &BitfieldStructReferenceIdent::ThisVariable,
                        has_ignored_fields,
                    ),
                );
            }
            generate_setter_impl_tokens(
                bitfield_type,
                field,
                bitfield_struct_reference_ident,
                quote! { 0 },
                /* check_value_bit_size= */ false,
                get_bitfield_struct_internal_value_identifier_tokens(
                    &BitfieldStructReferenceIdent::ThisVariable,
                    has_ignored_fields,
                ),
            )
        }
    }
}

/// Generates tokens to set the fields to zero.
pub(crate) fn generate_setting_fields_to_zero_tokens(
    bitfield_type: &Type,
    fields: &[BitfieldField],
    bitfield_struct_reference_ident: &BitfieldStructReferenceIdent,
    has_ignored_fields: bool,
) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            generate_setting_fields_to_zero_tokens_internal_helper(
                bitfield_type,
                field,
                bitfield_struct_reference_ident,
                has_ignored_fields,
            )
        })
        .collect()
}

fn generate_setting_fields_to_zero_tokens_internal_helper(
    bitfield_type: &Type,
    field: &BitfieldField,
    bitfield_struct_reference_ident: &BitfieldStructReferenceIdent,
    has_ignored_fields: bool,
) -> TokenStream {
    // Padding fields default values are respected.
    if field.padding {
        return generate_setting_fields_default_values_tokens(
            bitfield_type,
            [field.clone()].as_ref(),
            bitfield_struct_reference_ident,
            has_ignored_fields,
        );
    }

    let field_type_ident = &field.ty;

    let field_integer_type = get_integer_type_from_type(&field.ty);

    let field_has_setter = does_field_have_setter(field);
    if field_has_setter {
        let field_offset_setter_ident = get_field_setter_method_identifier(&field.name.to_string());

        if field.field_type == FieldType::CustomFieldType {
            return quote! {
                this.#field_offset_setter_ident(#field_type_ident::from_bits(0));
            };
        }

        if field_integer_type == IntegerType::Bool {
            return quote! {
                this.#field_offset_setter_ident(false);
            };
        }

        return quote! {
            this.#field_offset_setter_ident(0);
        };
    }

    if field.field_type == FieldType::CustomFieldType {
        return generate_setter_impl_tokens(
            bitfield_type,
            field,
            bitfield_struct_reference_ident,
            quote! { #field_type_ident::from_bits(0) },
            /* check_value_bit_size= */ false,
            get_bitfield_struct_internal_value_identifier_tokens(
                &BitfieldStructReferenceIdent::ThisVariable,
                has_ignored_fields,
            ),
        );
    }

    if field_integer_type == IntegerType::Bool {
        return generate_setter_impl_tokens(
            bitfield_type,
            field,
            bitfield_struct_reference_ident,
            quote! { false },
            /* check_value_bit_size= */ false,
            get_bitfield_struct_internal_value_identifier_tokens(
                &BitfieldStructReferenceIdent::ThisVariable,
                has_ignored_fields,
            ),
        );
    }

    generate_setter_impl_tokens(
        bitfield_type,
        field,
        bitfield_struct_reference_ident,
        quote! { 0 },
        /* check_value_bit_size= */ false,
        get_bitfield_struct_internal_value_identifier_tokens(
            &BitfieldStructReferenceIdent::ThisVariable,
            has_ignored_fields,
        ),
    )
}

/// Generates tokens to set the fields from a `bits` variable.
pub(crate) fn generate_setting_fields_from_bits_tokens(
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
    bitfield_struct_reference_ident: &BitfieldStructReferenceIdent,
    respect_defaults: bool,
    has_ignored_fields: bool,
    include_read_only_fields: bool,
) -> TokenStream {
    let bitfield_type = &bitfield_attribute.ty;
    fields
        .iter()
        .filter(
            |field| {
                if field.access == FieldAccess::ReadOnly { include_read_only_fields } else { true }
            },
        )
        .map(|field| {
            generate_setting_fields_from_bits_tokens_internal_helper(
                bitfield_type,
                field,
                bitfield_struct_reference_ident,
                respect_defaults,
                has_ignored_fields,
            )
        })
        .collect()
}

fn generate_setting_fields_from_bits_tokens_internal_helper(
    bitfield_type: &Type,
    field: &BitfieldField,
    bitfield_struct_reference_ident: &BitfieldStructReferenceIdent,
    respect_defaults: bool,
    has_ignored_fields: bool,
) -> TokenStream {
    // Padding fields default values are respected.
    if field.padding {
        return generate_setting_fields_default_values_tokens(
            bitfield_type,
            [field.clone()].as_ref(),
            &BitfieldStructReferenceIdent::SelfReference,
            has_ignored_fields,
        );
    }

    let field_type_ident = &field.ty;
    let field_integer_type = get_integer_type_from_type(&field.ty);

    let field_has_setter = does_field_have_setter(field);
    if field_has_setter {
        let field_bits_const_ident = get_field_bits_constant_identifier(&field.name.to_string());
        let field_offset_const_ident =
            get_field_offset_constant_identifier(&field.name.to_string());
        let default_value = &field.default_value_tokens;
        let field_offset_setter_ident = get_field_setter_method_identifier(&field.name.to_string());
        let bitfield_struct_reference_tokens = bitfield_struct_reference_ident.to_token_stream();

        if default_value.is_some() && respect_defaults {
            return generate_setting_fields_default_values_tokens(
                bitfield_type,
                [field.clone()].as_ref(),
                bitfield_struct_reference_ident,
                has_ignored_fields,
            );
        }

        let extract_value_bits = generate_extract_value_from_value_tokens(
            bitfield_type,
            quote! { bits },
            quote! { #bitfield_struct_reference_tokens::#field_bits_const_ident },
            quote! { #bitfield_struct_reference_tokens::#field_offset_const_ident },
            quote! { value },
            /* type_to_cast_output_value_ident= */ None,
            /* negate_source_value= */ false,
        );

        if field.field_type == FieldType::CustomFieldType {
            return quote! {
                #extract_value_bits
                this.#field_offset_setter_ident(#field_type_ident::from_bits(value as _));
            };
        }

        if field_integer_type == IntegerType::Bool {
            return quote! {
                #extract_value_bits
                this.#field_offset_setter_ident(value != 0);
            };
        }

        return quote! {
            #extract_value_bits
            this.#field_offset_setter_ident(value as _);
        };
    }

    let default_value = &field.default_value_tokens;
    if default_value.is_some() && respect_defaults {
        return generate_setting_fields_default_values_tokens(
            bitfield_type,
            [field.clone()].as_ref(),
            bitfield_struct_reference_ident,
            has_ignored_fields,
        );
    }

    let field_bits = field.bits;
    let field_offset = field.offset;
    let extract_value_bits = generate_extract_value_from_value_tokens(
        bitfield_type,
        quote! { bits },
        quote! { #field_bits },
        quote! { #field_offset },
        quote! { value },
        /* type_to_cast_output_value_ident= */ None,
        /* negate_source_value= */ false,
    );

    if field.field_type == FieldType::CustomFieldType {
        let setter_impl_tokens = generate_setter_impl_tokens(
            bitfield_type,
            field,
            bitfield_struct_reference_ident,
            quote! { #field_type_ident::from_bits(value as _) },
            /* check_value_bit_size= */ false,
            get_bitfield_struct_internal_value_identifier_tokens(
                &BitfieldStructReferenceIdent::ThisVariable,
                has_ignored_fields,
            ),
        );
        return quote! {
            #extract_value_bits
            #setter_impl_tokens
        };
    }

    if field_integer_type == IntegerType::Bool {
        let setter_impl_tokens = generate_setter_impl_tokens(
            bitfield_type,
            field,
            bitfield_struct_reference_ident,
            quote! { (value != 0) },
            /* check_value_bit_size= */ false,
            get_bitfield_struct_internal_value_identifier_tokens(
                &BitfieldStructReferenceIdent::ThisVariable,
                has_ignored_fields,
            ),
        );
        return quote! {
            #extract_value_bits
            #setter_impl_tokens
        };
    }

    let setter_impl_tokens = generate_setter_impl_tokens(
        bitfield_type,
        field,
        bitfield_struct_reference_ident,
        quote! { value },
        /* check_value_bit_size= */ false,
        get_bitfield_struct_internal_value_identifier_tokens(
            &BitfieldStructReferenceIdent::ThisVariable,
            has_ignored_fields,
        ),
    );

    quote! {
        #extract_value_bits
        #setter_impl_tokens
    }
}

/// Helper function to generate the setter implementation tokens.
pub(crate) fn generate_setter_impl_tokens(
    bitfield_type: &Type,
    field: &BitfieldField,
    bitfield_struct_reference_ident: &BitfieldStructReferenceIdent,
    value_ident: TokenStream,
    check_value_bit_size: bool,
    bitfield_struct_internal_value_identifier_tokens: TokenStream,
) -> TokenStream {
    let field_type = &field.ty;

    let bits_bigger_than_mask_check = (get_integer_type_from_type(field_type) != IntegerType::Bool)
        .then(|| {
            quote! {
                if #value_ident > mask as #field_type {
                    return Err(#BITS_TOO_BIG_ERROR_MESSAGE);
                }
            }
        });

    let field_bits_const_ident = get_field_bits_constant_identifier(&field.name.to_string());
    let field_offset_const_ident = get_field_offset_constant_identifier(&field.name.to_string());
    let bitfield_struct_name_ident = bitfield_struct_reference_ident.to_token_stream();

    let field_bits_ident = match field.access {
        FieldAccess::ReadOnly => {
            Some(quote! { #bitfield_struct_name_ident::#field_bits_const_ident })
        }
        FieldAccess::WriteOnly | FieldAccess::ReadWrite | FieldAccess::None => {
            let field_bits = field.bits;
            Some(quote! { #field_bits })
        }
    };

    let field_offset = match field.access {
        FieldAccess::ReadOnly => {
            Some(quote! { #bitfield_struct_name_ident::#field_offset_const_ident })
        }
        FieldAccess::WriteOnly | FieldAccess::ReadWrite | FieldAccess::None => {
            let field_offset = field.offset;
            Some(quote! { #field_offset })
        }
    };

    let mask_tokens =
        generate_mask_implementation_tokens(bitfield_type, quote! { #field_bits_ident });
    if field.field_type == FieldType::CustomFieldType {
        if check_value_bit_size {
            let set_bits_impl = generate_set_bits_implementation_tokens(
                quote! { #bitfield_struct_internal_value_identifier_tokens },
                bitfield_type,
                quote! { bits as #bitfield_type },
                quote! { #field_offset },
            );
            quote! {
                #mask_tokens
                let bits = #value_ident.into_bits();
                if bits as #bitfield_type > mask {
                    return Err(#BITS_TOO_BIG_ERROR_MESSAGE);
                }
                #set_bits_impl
            }
        } else {
            let set_bits_impl = generate_set_bits_implementation_tokens(
                quote! { #bitfield_struct_internal_value_identifier_tokens },
                bitfield_type,
                quote! { #value_ident.into_bits() as #bitfield_type },
                quote! { #field_offset },
            );
            quote! {
                #mask_tokens
                #set_bits_impl
            }
        }
    } else {
        let set_bits_impl = generate_set_bits_implementation_tokens(
            quote! { #bitfield_struct_internal_value_identifier_tokens },
            bitfield_type,
            quote! { value },
            quote! { #field_offset },
        );
        if check_value_bit_size {
            quote! {
                #mask_tokens
                #bits_bigger_than_mask_check
                #[allow(clippy::unnecessary_cast)]
                let value = #value_ident as #bitfield_type;
                #set_bits_impl
            }
        } else {
            quote! {
                #mask_tokens
                #[allow(clippy::unnecessary_cast)]
                let value = #value_ident as #bitfield_type;
                #set_bits_impl
            }
        }
    }
}

/// Returns implementation tokens to set bits in a field using a mask and
/// offset.
fn generate_set_bits_implementation_tokens(
    target_value_ident: TokenStream,
    bitfield_type: &Type,
    source_bits_expr: TokenStream,
    field_offset: TokenStream,
) -> TokenStream {
    quote! {
        #target_value_ident = (#target_value_ident & !(mask << #field_offset)) | (((#source_bits_expr & mask) << #field_offset) as #bitfield_type);
    }
}

/// Returns whether the field has a setter method.
pub(crate) fn does_field_have_setter(field: &BitfieldField) -> bool {
    (field.access == FieldAccess::ReadWrite || field.access == FieldAccess::WriteOnly)
        && !field.padding
}

/// Returns whether the field has a getter method.
pub(crate) fn does_field_have_getter(field: &BitfieldField) -> bool {
    (field.access == FieldAccess::ReadWrite || field.access == FieldAccess::ReadOnly)
        && !field.padding
}

/// Returns bitfield struct initialization tokens.
///
/// By default, it references the struct as `Self` unless a struct name is
/// passed.
///
/// # Example Output:
///
/// - With ignored fields struct: `BitfieldStruct { val: 0, ignored_field:
///   IgnoredFieldType::default() }`
/// - With struct name and ignored fields struct: `BitfieldStructName { val: 0,
///   ignored_field: IgnoredFieldType::default() }`
/// - With struct name and no ignored fields struct: `BitfieldStructName(0)`
/// - With no struct name and no ignored fields struct: `Self(0)`
pub(crate) fn generate_bitfield_struct_initialization_tokens(
    ignored_fields: &[BitfieldField],
    bitfield_struct_reference_ident: &BitfieldStructReferenceIdent,
) -> TokenStream {
    let ignored_fields_default_calls = generate_ignored_fields_default_call_tokens(ignored_fields);

    let val = quote! {
        0
    };

    let bitfield_ident = bitfield_struct_reference_ident.to_token_stream();
    if !ignored_fields.is_empty() {
        quote! {
             #bitfield_ident {
                val: #val,
                #( #ignored_fields_default_calls )*
            }
        }
    } else {
        quote! {
            #bitfield_ident(#val)
        }
    }
}

/// Generates tokens for calling default for each ignored field.
///
/// # Example Output:
///
/// `player: Player:default(),`
fn generate_ignored_fields_default_call_tokens(
    ignored_fields: &[BitfieldField],
) -> Map<std::slice::Iter<BitfieldField>, fn(&BitfieldField) -> TokenStream> {
    ignored_fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;
        quote! {
            #field_name: <#field_ty>::default(),
        }
    })
}

/// Returns the bitfield type tokens.
///
/// # Example Output:
///
/// - Integer backed: `u8`
pub(crate) fn get_bitfield_type_tokens(bitfield_attribute: &BitfieldAttribute) -> TokenStream {
    let bitfield_type = &bitfield_attribute.ty;

    quote! {
        #bitfield_type
    }
}

/// Returns the bitfield internal value identifier.
pub(crate) fn get_bitfield_struct_internal_value_identifier_tokens(
    bitfield_struct_reference_ident: &BitfieldStructReferenceIdent,
    has_ignored_fields: bool,
) -> TokenStream {
    let bitfield_struct_reference_ident_tokens = bitfield_struct_reference_ident.to_token_stream();
    if has_ignored_fields {
        quote! { #bitfield_struct_reference_ident_tokens.val }
    } else {
        quote! { #bitfield_struct_reference_ident_tokens.0 }
    }
}

/// Returns the const modifier tokens.
pub(crate) fn get_const_modifier_tokens() -> TokenStream {
    quote! { const }
}

/// Returns field bits constant identifier.
pub(crate) fn get_field_bits_constant_identifier(field_name: &str) -> TokenStream {
    let ident = format_ident!("{}_BITS", field_name.to_ascii_uppercase());
    quote! { #ident }
}

/// Returns the field offset identifier.
pub(crate) fn get_field_offset_constant_identifier(field_name: &str) -> TokenStream {
    let ident = format_ident!("{}_OFFSET", field_name.to_ascii_uppercase());
    quote! { #ident }
}

/// Returns the field setter identifier.
pub(crate) fn get_field_setter_method_identifier(field_name: &str) -> TokenStream {
    let ident = format_ident!("set_{}", field_name);
    quote! { #ident }
}

/// Returns the field checked setter identifier.
pub(crate) fn get_field_checked_setter_method_identifier(field_name: &str) -> TokenStream {
    let ident = format_ident!("checked_set_{}", field_name);
    quote! { #ident }
}

/// Returns the bits order for field documentation.
pub(crate) fn get_documentation_field_bits_order(
    field: &BitfieldField,
    bit_order: BitOrder,
) -> (u32, u32) {
    let field_offset = field.offset;
    let field_bits_end = field.offset + field.bits - 1;

    if bit_order == BitOrder::Msb {
        (field_bits_end, field_offset)
    } else {
        (field_offset, field_bits_end)
    }
}
