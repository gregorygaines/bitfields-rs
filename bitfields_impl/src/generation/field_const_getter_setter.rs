use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::generation::bit_manipulation_common::generate_extract_value_from_value_tokens;
use crate::generation::common::{
    BitfieldStructReferenceIdent, does_field_have_getter, does_field_have_setter,
    generate_setter_impl_tokens, get_bitfield_struct_internal_value_identifier_tokens,
    get_documentation_field_bits_order, get_field_bits_constant_identifier,
    get_field_checked_setter_method_identifier, get_field_offset_constant_identifier,
    get_field_setter_method_identifier,
};
use crate::parsing::bitfield_attribute::{BitOrder, BitfieldAttribute};
use crate::parsing::bitfield_field::{BitfieldField, FieldType};
use crate::parsing::types::{IntegerType, get_bits_from_type, get_integer_type_from_type};

/// Generates the field constants for the bitfield.
pub(crate) fn generate_field_constants_tokens(
    vis: &syn::Visibility,
    fields: &[BitfieldField],
) -> TokenStream {
    let field_constants_tokens = fields
        .iter()
        .filter(|field| !field.padding)
        .filter(|field| does_field_have_getter(field) || does_field_have_setter(field))
        .map(|field| generate_field_constants_tokens_helper(vis, field));

    quote! {
        #( #field_constants_tokens )*
    }
}

fn generate_field_constants_tokens_helper(
    vis: &syn::Visibility,
    field: &BitfieldField,
) -> TokenStream {
    let field_bits = field.bits;
    let field_offset = field.offset;
    let field_bits_const_ident = get_field_bits_constant_identifier(&field.name.to_string());
    let field_offset_const_ident = get_field_offset_constant_identifier(&field.name.to_string());

    let bits_documentation =
        format!("The number of bits `{}` occupies in the bitfield.", field.name);
    let offset_documentation = format!("The bitfield start bit of `{}`.", field.name);
    quote! {
        #[doc = #bits_documentation]
        #vis const #field_bits_const_ident: u32 = #field_bits;
        #[doc = #offset_documentation]
        #vis const #field_offset_const_ident: u32 = #field_offset;
    }
}

/// Generates the field getters for the bitfield.
pub(crate) fn generate_field_getters_functions_tokens(
    default_vis: &syn::Visibility,
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
    has_ignored_fields: bool,
) -> syn::Result<TokenStream> {
    let tokens = fields
        .iter()
        .filter(|field| !field.padding)
        .filter(|field| does_field_have_getter(field))
        .map(|field| {
            generate_field_getters_functions_tokens_helper(
                default_vis,
                bitfield_attribute,
                field,
                has_ignored_fields,
            )
        })
        .collect();

    Ok(tokens)
}

fn generate_field_getters_functions_tokens_helper(
    default_vis: &syn::Visibility,
    bitfield_attribute: &BitfieldAttribute,
    field: &BitfieldField,
    has_ignored_fields: bool,
) -> TokenStream {
    let bitfield_type = &bitfield_attribute.ty;
    let field_name = &field.name.to_string();
    let field_bits = field.bits;
    let field_type = &field.ty;

    let field_name_ident = format_ident!("{}", field_name);
    let neg_field_name_ident = format_ident!("neg_{}", field_name);
    let field_bits_const_ident = get_field_bits_constant_identifier(&field.name.to_string());
    let field_offset_const_ident = get_field_offset_constant_identifier(&field.name.to_string());
    let vis = get_field_visibility(field.vis.clone(), default_vis.clone());

    let bitfield_struct_internal_value_ident = get_bitfield_struct_internal_value_identifier_tokens(
        &BitfieldStructReferenceIdent::SelfVariable,
        has_ignored_fields,
    );

    let field_getter_documentation =
        get_field_getter_documentation(bitfield_attribute, field, /* neg_getter= */ false);
    let neg_field_getter_documentation =
        get_field_getter_documentation(bitfield_attribute, field, /* neg_getter= */ true);

    if field.field_type == FieldType::CustomFieldType {
        let custom_field_neg_getter = bitfield_attribute.generate_neg_func.then(|| {
            let neg_getter_extract_value_tokens = generate_extract_value_from_value_tokens(
                bitfield_type,
                quote! { #bitfield_struct_internal_value_ident },
                quote! { Self::#field_bits_const_ident },
                quote! { Self::#field_offset_const_ident },
                quote! { this },
                /* type_to_cast_output_value_ident= */ None,
                /* negate_source_value= */ true,
            );

            quote! {
                #[doc = #neg_field_getter_documentation]
                #vis const fn #neg_field_name_ident(&self) -> #field_type {
                    #neg_getter_extract_value_tokens
                    #field_type::from_bits(this as _)
                }
            }
        });

        let getter_extract_value = generate_extract_value_from_value_tokens(
            bitfield_type,
            quote! { #bitfield_struct_internal_value_ident },
            quote! { Self::#field_bits_const_ident },
            quote! { Self::#field_offset_const_ident },
            quote! { this },
            /* type_to_cast_output_value_ident= */ None,
            /* negate_source_value= */ false,
        );
        quote! {
            #[doc = #field_getter_documentation]
               #vis const fn #field_name_ident(&self) -> #field_type {
                #getter_extract_value
                #field_type::from_bits(this as _)
            }

            #custom_field_neg_getter
        }
    } else {
        let field_type_bits = get_bits_from_type(field_type).unwrap();

        if get_integer_type_from_type(field_type) == IntegerType::Bool {
            let neg_getter = bitfield_attribute.generate_neg_func.then(|| {
                let neg_getter_extract_value_tokens = generate_extract_value_from_value_tokens(
                    bitfield_type,
                    quote! { #bitfield_struct_internal_value_ident },
                    quote! { Self::#field_bits_const_ident },
                    quote! { Self::#field_offset_const_ident },
                    quote! { this },
                    /* type_to_cast_output_value_ident= */ None,
                    /* negate_source_value= */ true,
                );

                quote! {
                    #[doc = #neg_field_getter_documentation]
                    #vis const fn #neg_field_name_ident(&self) -> #field_type {
                        #neg_getter_extract_value_tokens
                        this != 0
                    }
                }
            });

            let getter_extract_value = generate_extract_value_from_value_tokens(
                bitfield_type,
                quote! { #bitfield_struct_internal_value_ident },
                quote! { Self::#field_bits_const_ident },
                quote! { Self::#field_offset_const_ident },
                quote! { this },
                /* type_to_cast_output_value_ident= */ None,
                /* negate_source_value= */ false,
            );
            return quote! {
                #[doc = #field_getter_documentation]
                #vis const fn #field_name_ident(&self) -> #field_type {
                    #getter_extract_value
                    this != 0
                }

                #neg_getter
            };
        }

        let sign_extend_tokens = (!field.unsigned).then(|| {
            quote! {
                let shift = #field_type_bits - #field_bits;
                let this = ((this as #field_type) << shift) >> shift;
            }
        });

        let neg_getter = bitfield_attribute.generate_neg_func.then(|| {
            let neg_field_getter_documentation = get_field_getter_documentation(
                bitfield_attribute,
                field,
                /* neg_getter= */ true,
            );
            let neg_getter_extract_value_tokens = generate_extract_value_from_value_tokens(
                bitfield_type,
                quote! { #bitfield_struct_internal_value_ident },
                quote! { Self::#field_bits_const_ident },
                quote! { Self::#field_offset_const_ident },
                quote! { this },
                Some(quote! { #field_type }),
                /* negate_source_value= */ true,
            );
            quote! {
                #[doc = #neg_field_getter_documentation]
                #vis const fn #neg_field_name_ident(&self) -> #field_type {
                    #neg_getter_extract_value_tokens
                    #sign_extend_tokens
                    this
                }
            }
        });

        let getter_extract_value_tokens = generate_extract_value_from_value_tokens(
            bitfield_type,
            quote! { #bitfield_struct_internal_value_ident },
            quote! { Self::#field_bits_const_ident },
            quote! { Self::#field_offset_const_ident },
            quote! { this },
            Some(quote! { #field_type }),
            /* negate_source_value= */ false,
        );
        quote! {
            #[doc = #field_getter_documentation]
            #vis const fn #field_name_ident(&self) -> #field_type {
                #getter_extract_value_tokens
                #sign_extend_tokens
                this
            }

            #neg_getter
        }
    }
}

const FIELD_DOCUMENTATION_INVERTED_SUFFIX: &str = ", inverted";

/// Returns field getter documentation.
fn get_field_getter_documentation(
    bitfield_attribute: &BitfieldAttribute,
    field: &BitfieldField,
    neg_getter: bool,
) -> String {
    let field_offset = field.offset;
    let field_bits_end = field.offset + field.bits - 1;

    let field_documentation_ending_suffix =
        neg_getter.then_some(FIELD_DOCUMENTATION_INVERTED_SUFFIX).unwrap_or_default();

    if field.bits == 1 {
        return if field.unsigned {
            format!("Returns bit `{field_offset}`{field_documentation_ending_suffix}.")
        } else {
            format!(
                "Returns sign-extended bit `{field_offset}`{field_documentation_ending_suffix}."
            )
        };
    }

    let (documentation_bits_start, documentation_bits_end) =
        if bitfield_attribute.bit_order == BitOrder::Msb {
            (field_bits_end, field_offset)
        } else {
            (field_offset, field_bits_end)
        };

    if field.unsigned {
        format!(
            "Returns bits `{documentation_bits_start}..={documentation_bits_end}`{field_documentation_ending_suffix}."
        )
    } else {
        format!(
            "Returns sign-extended bits `{documentation_bits_start}..={documentation_bits_end}` from the sign-bit `{field_offset}`{field_documentation_ending_suffix}."
        )
    }
}

/// Generates the field setters for the bitfield.
pub(crate) fn generate_field_setters_functions_tokens(
    default_vis: &syn::Visibility,
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
    has_ignored_fields: bool,
) -> TokenStream {
    fields
        .iter()
        .filter(|field| !field.padding)
        .filter(|field| does_field_have_setter(field))
        .map(|field| {
            generate_field_setters_functions_tokens_helper(
                default_vis,
                bitfield_attribute,
                field,
                has_ignored_fields,
            )
        })
        .collect()
}

fn generate_field_setters_functions_tokens_helper(
    default_vis: &syn::Visibility,
    bitfield_attribute: &BitfieldAttribute,
    field: &BitfieldField,
    has_ignored_fields: bool,
) -> TokenStream {
    let field_type = &field.ty;
    let bitfield_type = &bitfield_attribute.ty;

    let field_offset_setter_ident = get_field_setter_method_identifier(&field.name.to_string());
    let checked_field_offset_setter_ident =
        get_field_checked_setter_method_identifier(&field.name.to_string());
    let vis = get_field_visibility(field.vis.clone(), default_vis.clone());

    let setter_impl_tokens = generate_setter_impl_tokens(
        bitfield_type,
        field,
        &BitfieldStructReferenceIdent::SelfReference,
        quote! { bits },
        /* check_value_bit_size= */ false,
        get_bitfield_struct_internal_value_identifier_tokens(
            &BitfieldStructReferenceIdent::ThisVariable,
            has_ignored_fields,
        ),
    );

    let setter_with_size_check_impl_tokens = generate_setter_impl_tokens(
        bitfield_type,
        field,
        &BitfieldStructReferenceIdent::SelfReference,
        quote! { bits },
        /* check_value_bit_size= */ true,
        get_bitfield_struct_internal_value_identifier_tokens(
            &BitfieldStructReferenceIdent::ThisVariable,
            has_ignored_fields,
        ),
    );

    let setter_documentation =
        get_field_setter_documentation(bitfield_attribute, field, /* checked_setter= */ false);
    let checked_setter_documentation =
        get_field_setter_documentation(bitfield_attribute, field, /* checked_setter= */ true);
    quote! {
        #[doc = #setter_documentation]
        #vis const fn #field_offset_setter_ident(&mut self, bits: #field_type) {
            let this = self;
            #setter_impl_tokens
        }

        #[doc = #checked_setter_documentation]
        #vis const fn #checked_field_offset_setter_ident(&mut self, bits: #field_type) -> ::core::result::Result<(), &'static str> {
            let this = self;
            #setter_with_size_check_impl_tokens
            Ok(())
        }
    }
}

/// Returns field setter documentation.
fn get_field_setter_documentation(
    bitfield_attribute: &BitfieldAttribute,
    field: &BitfieldField,
    checked_setter: bool,
) -> String {
    let field_offset = field.offset;

    let setter_documentation = if field.bits == 1 {
        format!("Sets bit `{field_offset}`.")
    } else {
        let (documentation_bits_start, documentation_bits_end) =
            get_documentation_field_bits_order(field, bitfield_attribute.bit_order);
        format!("Sets bits `{documentation_bits_start}..={documentation_bits_end}`.")
    };

    if checked_setter {
        return if field.bits == 1 {
            format!(
                "{setter_documentation} Returns an error if the value is too big to fit within the field bit."
            )
        } else {
            format!(
                "{setter_documentation} Returns an error if the value is too big to fit within the field bits."
            )
        };
    }

    setter_documentation
}

/// Returns field visibility.
fn get_field_visibility(
    field_vis: Option<syn::Visibility>,
    struct_vis: syn::Visibility,
) -> syn::Visibility {
    field_vis.unwrap_or(struct_vis)
}
