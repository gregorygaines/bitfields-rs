use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::generation::field_const_getter_setter::generate_setter_impl_tokens;
use crate::parsing::bitfield_field::{BitfieldField, FieldAccess, FieldType};
use crate::parsing::types::{IntegerType, get_integer_type_from_type};

/// An error message to display when a panic occurs, which should never happen.
pub(crate) const PANIC_ERROR_MESSAGE: &str = "A major unexpected error has occurred. If possible, please file an issue with the code that caused this error at https://github.com/gregorygaines/bitfields-rs/issues.";

/// Generates tokens to set the default values for non-padding fields or zero if
/// no default value is provided.
///
/// By default, it uses the field's setter method. If the field is read-only,
/// then it inlines bitwise operations to set the field.
pub(crate) fn generate_setting_fields_default_values_tokens(
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
    const_reference_tokens: Option<TokenStream>,
    ignored_fields_struct: bool,
) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_name = field.name.clone();
            let default_value = field.default_value_tokens.clone();
            let field_type_ident = field.ty.clone();
            let field_integer_type = get_integer_type_from_type(&field.ty);

            let field_has_setter = does_field_have_setter(field);
            if field_has_setter {
                let field_offset_setter_ident = format_ident!("set_{}", field_name);

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
                        field.clone(),
                        const_reference_tokens.clone(),
                        quote! { #default_value },
                        /* check_value_bit_size= */ false,
                        ignored_fields_struct,
                        None,
                    )
                }
                None => {
                    if field_integer_type == IntegerType::Bool {
                        return generate_setter_impl_tokens(
                            bitfield_type,
                            field.clone(),
                            const_reference_tokens.clone(),
                            quote! { false },
                            /* check_value_bit_size= */ false,
                            ignored_fields_struct,
                            None,
                        );
                    }
                    generate_setter_impl_tokens(
                        bitfield_type,
                        field.clone(),
                        const_reference_tokens.clone(),
                        quote! { 0 },
                        /* check_value_bit_size= */ false,
                        ignored_fields_struct,
                        None,
                    )
                }
            }
        })
        .collect()
}

/// Generates tokens to set the fields to zero.
pub(crate) fn generate_setting_fields_to_zero_tokens(
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
    const_reference_tokens: Option<TokenStream>,
    ignored_fields_struct: bool,
) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            // Padding fields default values are respected.
            if field.padding {
                return generate_setting_fields_default_values_tokens(
                    bitfield_type,
                    [field.clone()].as_ref(),
                    None,
                    ignored_fields_struct,
                );
            }

            let field_name = field.name.clone();
            let field_type_ident = field.ty.clone();

            let field_integer_type = get_integer_type_from_type(&field.ty);

            let field_has_setter = does_field_have_setter(field);
            if field_has_setter {
                let field_offset_setter_ident = format_ident!("set_{}", field_name);

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

            if field_integer_type == IntegerType::Bool {
                return generate_setter_impl_tokens(
                    bitfield_type,
                    field.clone(),
                    const_reference_tokens.clone(),
                    quote! { false },
                    /* check_value_bit_size= */ false,
                    ignored_fields_struct,
                    None,
                );
            }

            generate_setter_impl_tokens(
                bitfield_type,
                field.clone(),
                const_reference_tokens.clone(),
                quote! { 0 },
                /* check_value_bit_size= */ false,
                ignored_fields_struct,
                None,
            )
        })
        .collect()
}

/// Generates tokens to set the fields from a `bits` variable.
/// 
/// When `include_read_only` is true, read-only fields will be set from bits.
/// When `include_read_only` is false, read-only fields will be skipped.
pub(crate) fn generate_setting_fields_from_bits_tokens(
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
    const_reference_tokens: Option<TokenStream>,
    respect_defaults: bool,
    ignored_fields_struct: bool,
    include_read_only: bool,
) -> TokenStream {
    fields
        .iter()
        .filter(|field| {
            // Include all fields except read-only fields when include_read_only is false
            include_read_only || field.access != FieldAccess::ReadOnly
        })
        .map(|field| {
            // Padding fields default values are respected.
            if field.padding {
                return generate_setting_fields_default_values_tokens(
                    bitfield_type,
                    [field.clone()].as_ref(),
                    None,
                    ignored_fields_struct,
                );
            }

            let field_name = field.name.clone();
            let field_type_ident = field.ty.clone();

            let field_integer_type = get_integer_type_from_type(&field.ty);

            let field_has_setter = does_field_have_setter(field);
            if field_has_setter {
                let field_name_uppercase = field.name.clone().to_string().to_ascii_uppercase();
                let field_bits_const_ident = format_ident!("{}_BITS", field_name_uppercase);
                let field_offset_const_ident = format_ident!("{}_OFFSET", field_name_uppercase);
                let default_value = field.default_value_tokens.clone();
                let field_offset_setter_ident = format_ident!("set_{}", field_name);

                if default_value.is_some() && respect_defaults {
                    return generate_setting_fields_default_values_tokens(
                        bitfield_type,
                        [field.clone()].as_ref(),
                        const_reference_tokens.clone(),
                        ignored_fields_struct,
                    );
                }

                let extract_value_bits = quote! {
                    let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - #const_reference_tokens::#field_bits_const_ident);
                    let value = (bits >> #const_reference_tokens::#field_offset_const_ident) & mask;
                };
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

            let default_value = field.default_value_tokens.clone();
            if default_value.is_some() && respect_defaults {
                return generate_setting_fields_default_values_tokens(
                    bitfield_type,
                    [field.clone()].as_ref(),
                    const_reference_tokens.clone(),
                    ignored_fields_struct,
                );
            }

            let field_bits = field.bits as u32;
            let field_offset = field.offset as u32;
            let extract_value_bits = quote! {
                let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - #field_bits);
                let value = (bits >> #field_offset) & mask;
            };

            if field_integer_type == IntegerType::Bool {
                let setter_impl_tokens = generate_setter_impl_tokens(
                    bitfield_type,
                    field.clone(),
                    const_reference_tokens.clone(),
                    quote! { (value != 0) },
                    /* check_value_bit_size= */ false,
                    ignored_fields_struct,
                    None,
                );
                return quote! {
                    #extract_value_bits
                    #setter_impl_tokens
                }
            }

            let setter_impl_tokens = generate_setter_impl_tokens(
                bitfield_type,
                field.clone(),
                const_reference_tokens.clone(),
                quote! { value },
                /* check_value_bit_size= */ false,
                ignored_fields_struct,
                None,
            );

            quote! {
                #extract_value_bits
                #setter_impl_tokens
            }
        })
        .collect()
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
