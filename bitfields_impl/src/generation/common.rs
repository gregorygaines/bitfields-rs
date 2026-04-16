use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Type;

use crate::parsing::bitfield_field::{BitfieldField, FieldAccess, FieldType};
use crate::parsing::types::{IntegerType, get_integer_type_from_type};

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
                        /* struct_value_ident= */ None,
                    )
                }
                None => {
                    if field.field_type == FieldType::CustomFieldType {
                        return generate_setter_impl_tokens(
                            bitfield_type,
                            field.clone(),
                            const_reference_tokens.clone(),
                            quote! { #field_type_ident::from_bits(0) },
                            /* check_value_bit_size= */ false,
                            ignored_fields_struct,
                            /* struct_value_ident= */ None,
                        );
                    }
                    if field_integer_type == IntegerType::Bool {
                        return generate_setter_impl_tokens(
                            bitfield_type,
                            field.clone(),
                            const_reference_tokens.clone(),
                            quote! { false },
                            /* check_value_bit_size= */ false,
                            ignored_fields_struct,
                            /* struct_value_ident= */ None,
                        );
                    }
                    generate_setter_impl_tokens(
                        bitfield_type,
                        field.clone(),
                        const_reference_tokens.clone(),
                        quote! { 0 },
                        /* check_value_bit_size= */ false,
                        ignored_fields_struct,
                        /* struct_value_ident= */ None,
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
                    /* const_reference_tokens= */ None,
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

            if field.field_type == FieldType::CustomFieldType {
                return generate_setter_impl_tokens(
                    bitfield_type,
                    field.clone(),
                    const_reference_tokens.clone(),
                    quote! { #field_type_ident::from_bits(0) },
                    /* check_value_bit_size= */ false,
                    ignored_fields_struct,
                    /* struct_value_ident= */ None,
                );
            }

            if field_integer_type == IntegerType::Bool {
                return generate_setter_impl_tokens(
                    bitfield_type,
                    field.clone(),
                    const_reference_tokens.clone(),
                    quote! { false },
                    /* check_value_bit_size= */ false,
                    ignored_fields_struct,
                    /* struct_value_ident= */ None,
                );
            }

            generate_setter_impl_tokens(
                bitfield_type,
                field.clone(),
                const_reference_tokens.clone(),
                quote! { 0 },
                /* check_value_bit_size= */ false,
                ignored_fields_struct,
                /* struct_value_ident= */ None,
            )
        })
        .collect()
}

/// Generates tokens to set the fields from a `bits` variable.
pub(crate) fn generate_setting_fields_from_bits_tokens(
    bitfield_type: &syn::Type,
    fields: &[BitfieldField],
    const_reference_tokens: Option<TokenStream>,
    respect_defaults: bool,
    ignored_fields_struct: bool,
    include_read_only_fields: bool,
) -> TokenStream {
    fields
        .iter()
        .filter(|field| {
            if field.access == FieldAccess::ReadOnly {
                include_read_only_fields
            } else {
                true
            }
        })
        .map(|field| {
            // Padding fields default values are respected.
            if field.padding {
                return generate_setting_fields_default_values_tokens(
                    bitfield_type,
                    [field.clone()].as_ref(),
                    /* const_reference_tokens= */ None,
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

            if field.field_type == FieldType::CustomFieldType {
                let setter_impl_tokens = generate_setter_impl_tokens(
                    bitfield_type,
                    field.clone(),
                    const_reference_tokens.clone(),
                    quote! { #field_type_ident::from_bits(value as _) },
                    /* check_value_bit_size= */ false,
                    ignored_fields_struct,
                    /* struct_value_ident= */ None,
                );
                return quote! {
                    #extract_value_bits
                    #setter_impl_tokens
                }
            }

            if field_integer_type == IntegerType::Bool {
                let setter_impl_tokens = generate_setter_impl_tokens(
                    bitfield_type,
                    field.clone(),
                    const_reference_tokens.clone(),
                    quote! { (value != 0) },
                    /* check_value_bit_size= */ false,
                    ignored_fields_struct,
                    /* struct_value_ident= */ None,
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
                /* struct_value_ident= */ None,
            );

            quote! {
                #extract_value_bits
                #setter_impl_tokens
            }
        })
        .collect()
}

/// Helper function to generate the setter implementation tokens.
pub(crate) fn generate_setter_impl_tokens(
    bitfield_type: &Type,
    field: BitfieldField,
    bitfield_struct_name: Option<TokenStream>,
    value_ident: TokenStream,
    check_value_bit_size: bool,
    ignored_fields_struct: bool,
    struct_val_ident: Option<TokenStream>,
) -> TokenStream {
    let field_type = field.ty.clone();

    let bits_bigger_than_mask_check =
        (get_integer_type_from_type(&field_type) != IntegerType::Bool).then(|| {
            quote! {
                if #value_ident > mask as #field_type {
                    return Err(#BITS_TOO_BIG_ERROR_MESSAGE);
                }
            }
        });

    let field_name_uppercase = field.name.clone().to_string().to_ascii_uppercase();
    let field_bits_const_ident = format_ident!("{}_BITS", field_name_uppercase);
    let field_offset_const_ident = format_ident!("{}_OFFSET", field_name_uppercase);
    let bitfield_struct_name_ident = bitfield_struct_name.unwrap_or_else(|| quote! { Self });

    let field_bits_ident = match field.access {
        FieldAccess::ReadOnly => {
            Some(quote! { #bitfield_struct_name_ident::#field_bits_const_ident })
        }
        FieldAccess::WriteOnly | FieldAccess::ReadWrite | FieldAccess::None => {
            let field_bits = field.bits as u32;
            Some(quote! { #field_bits })
        }
    };

    let field_offset = match field.access {
        FieldAccess::ReadOnly => {
            Some(quote! { #bitfield_struct_name_ident::#field_offset_const_ident })
        }
        FieldAccess::WriteOnly | FieldAccess::ReadWrite | FieldAccess::None => {
            let field_offset = field.offset as u32;
            Some(quote! { #field_offset })
        }
    };

    let struct_val_ident = if ignored_fields_struct {
        if struct_val_ident.is_some() {
            quote! { #struct_val_ident.val }
        } else {
            quote! { this.val }
        }
    } else if struct_val_ident.is_some() {
        quote! { #struct_val_ident.0 }
    } else {
        quote! { this.0 }
    };

    if field.field_type == FieldType::CustomFieldType {
        if check_value_bit_size {
            quote! {
                let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - #field_bits_ident);
                let bits = #value_ident.into_bits();
                if bits as #bitfield_type > mask {
                    return Err(#BITS_TOO_BIG_ERROR_MESSAGE);
                }
                #struct_val_ident = (#struct_val_ident & !(mask << #field_offset)) | ((((bits as #bitfield_type) & mask) << #field_offset) as #bitfield_type);
            }
        } else {
            quote! {
                let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - #field_bits_ident);
                #struct_val_ident = (#struct_val_ident & !(mask << #field_offset)) | ((((#value_ident.into_bits() as #bitfield_type) & mask) << #field_offset) as #bitfield_type);
            }
        }
    } else if check_value_bit_size {
        quote! {
            let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - #field_bits_ident);
            #bits_bigger_than_mask_check
            #[allow(clippy::unnecessary_cast)]
            let value = #value_ident as #bitfield_type;
            #struct_val_ident = (#struct_val_ident & !(mask << #field_offset)) | (((value & mask) << #field_offset) as #bitfield_type);
        }
    } else {
        quote! {
            let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - #field_bits_ident);
            #[allow(clippy::unnecessary_cast)]
            let value = #value_ident as #bitfield_type;
            #struct_val_ident = (#struct_val_ident & !(mask << #field_offset)) | (((value & mask) << #field_offset) as #bitfield_type);
        }
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
