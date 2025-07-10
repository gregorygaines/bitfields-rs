use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::{Type, Visibility};

use crate::generation::common::{does_field_have_getter, does_field_have_setter};
use crate::parsing::bitfield_attribute::{BitOrder, BitfieldAttribute};
use crate::parsing::bitfield_field::{BitfieldField, FieldAccess, FieldType};
use crate::parsing::types::IntegerType::Bool;
use crate::parsing::types::{get_bits_from_type, get_integer_type_from_type};

/// Error message for when the value is too big to fit within the field bits.
const BITS_TOO_BIG_ERROR_MESSAGE: &str = "Value is too big to fit within the field bits.";

/// Generates the field constants for the bitfield.
pub(crate) fn generate_field_constants_tokens(
    vis: Visibility,
    fields: &[BitfieldField],
) -> TokenStream {
    fields
        .iter()
        .filter(|field| !field.padding)
        .filter(|field| does_field_have_getter(field) || does_field_have_setter(field))
        .map(|field| {
            let field_bits = field.bits as u32;
            let field_offset = field.offset as u32;
            let field_name_upper = field.name.to_string().to_ascii_uppercase();
            let field_bits_const_ident = format_ident!("{}_BITS", field_name_upper);
            let field_offset_const_ident = format_ident!("{}_OFFSET", field_name_upper);

            let bits_documentation =
                format!("The number of bits `{}` occupies in the bitfield.", field.name);
            let offset_documentation = format!("The bitfield start bit of `{}`.", field.name);
            quote! {
                #[doc = #bits_documentation]
                #vis const #field_bits_const_ident: u32 = #field_bits;
                #[doc = #offset_documentation]
                #vis const #field_offset_const_ident: u32 = #field_offset;
            }
        })
        .collect()
}

/// Generates the field getters for the bitfield.
pub(crate) fn generate_field_getters_functions_tokens(
    default_vis: Visibility,
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
    ignored_fields_struct: bool,
    generate_neg_getter: bool,
) -> syn::Result<TokenStream> {
    let tokens = fields.iter().filter(|field| !field.padding).filter(|field| does_field_have_getter(field)).map(|field| {
        let bitfield_type = &bitfield_attribute.ty;
        let field_name = field.name.clone().to_string();
        let field_name_uppercase = field.name.clone().to_string().to_ascii_uppercase();
        let field_bits = field.bits;
        let field_type = field.ty.clone();

        let field_name_ident = format_ident!("{}", field_name);
        let neg_field_name_ident = format_ident!("neg_{}", field_name);
        let field_bits_const_ident = format_ident!("{}_BITS", field_name_uppercase);
        let field_offset_const_ident = format_ident!("{}_OFFSET", field_name_uppercase);
        let vis = match field.vis {
            Some(_) => {
                field.vis.clone().unwrap()
            }
            None => default_vis.clone(),
        };

        let struct_val_ident = if ignored_fields_struct {
            quote! { self.val }
        } else {
            quote! { self.0 }
        };

        let field_offset = field.offset;
        let field_bits_end = field.offset + field.bits - 1;

        let common_field_getter_documentation = if field.bits == 1 {
            format!("Returns bit `{field_offset}`.")
        } else if bitfield_attribute.bit_order == BitOrder::Msb {
            format!("Returns bits `{field_bits_end}..={field_offset}`.")
        } else {
            format!("Returns bits `{field_offset}..={field_bits_end}`.")
        };
        let common_neg_field_getter_documentation = if field.bits == 1 {
            format!("Returns bit `{field_offset}`, inverted.")
        } else if bitfield_attribute.bit_order == BitOrder::Msb {
            format!("Returns bits `{field_bits_end}..={field_offset}`, inverted.")
        } else {
            format!("Returns bits `{field_offset}..={field_bits_end}`, inverted.")
        };
        if field.field_type == FieldType::CustomFieldType {
            let neg_getter = if generate_neg_getter {
                quote! {
                    #[doc = #common_neg_field_getter_documentation]
                    #vis const fn #neg_field_name_ident(&self) -> #field_type {
                        let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - Self::#field_bits_const_ident);
                        let this = (!(#struct_val_ident >> Self::#field_offset_const_ident) & mask);
                        #field_type::from_bits(this as _)
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                #[doc = #common_field_getter_documentation]
                   #vis const fn #field_name_ident(&self) -> #field_type {
                    let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - Self::#field_bits_const_ident);
                    let this = ((#struct_val_ident >> Self::#field_offset_const_ident) & mask);
                    #field_type::from_bits(this as _)
                }

                #neg_getter
            }
        } else {
            let field_type_bits = get_bits_from_type(&field_type).unwrap();

            if get_integer_type_from_type(&field_type) == Bool {
                let bool_field_getter_documentation = format!("Returns bit `{}`.", field_offset);
                let neg_bool_field_getter_documentation = format!("Returns bit `{}`, inverted.", field_offset);

                let neg_getter = if generate_neg_getter {
                    quote! {
                        #[doc = #neg_bool_field_getter_documentation]
                        #vis const fn #neg_field_name_ident(&self) -> #field_type {
                            let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - Self::#field_bits_const_ident);
                            let this = (!(#struct_val_ident >> Self::#field_offset_const_ident) & mask);
                            this != 0
                        }
                    }
                } else {
                    quote! {}
                };

                return quote! {
                    #[doc = #bool_field_getter_documentation]
                    #vis const fn #field_name_ident(&self) -> #field_type {
                        let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - Self::#field_bits_const_ident);
                        let this = ((#struct_val_ident >> Self::#field_offset_const_ident) & mask);
                        this != 0
                    }

                    #neg_getter
                }
            }

            let sign_extend_tokens = (!field.unsigned).then(|| {
                quote! {
                    let shift = #field_type_bits - #field_bits;
                    let this = ((this as #field_type) << shift) >> shift;
                }
            });

            let field_getter_documentation = if field.unsigned || field.bits == 1 {
                common_field_getter_documentation
            } else {
                format!("Returns sign-extended bits `{field_offset}..={field_bits_end}` from the sign-bit `{field_offset}`.")
            };

            let neg_field_getter_documentation = if field.unsigned || field.bits == 1 {
                common_neg_field_getter_documentation
            } else {
                format!("Returns sign-extended bits `{field_offset}..={field_bits_end}` from the sign-bit `{field_offset}`, inverted.")
            };

            let neg_getter = if generate_neg_getter {
                quote! {
                    #[doc = #neg_field_getter_documentation]
                    #vis const fn #neg_field_name_ident(&self) -> #field_type {
                        let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - Self::#field_bits_const_ident);
                        let this = (!(#struct_val_ident >> Self::#field_offset_const_ident) & mask) as #field_type;
                        #sign_extend_tokens
                        this
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                #[doc = #field_getter_documentation]
                #vis const fn #field_name_ident(&self) -> #field_type {
                    let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - Self::#field_bits_const_ident);
                    let this = ((#struct_val_ident >> Self::#field_offset_const_ident) & mask) as #field_type;
                    #sign_extend_tokens
                    this
                }

                #neg_getter
            }
        }
    }).collect();

    Ok(tokens)
}

/// Generates the field setters for the bitfield.
pub(crate) fn generate_field_setters_functions_tokens(
    default_vis: Visibility,
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
    ignored_fields_struct: bool,
) -> TokenStream {
    fields.iter().filter(|field| !field.padding).filter(|field| does_field_have_setter(field)).map(|field| {
        let field_name = field.name.clone().to_string();
        let field_type = field.ty.clone();
        let bitfield_type = &bitfield_attribute.ty;

        let field_offset = field.offset;
        let field_bits_end = field.offset + field.bits - 1;

        let field_offset_setter_ident = format_ident!("set_{}", field_name);
        let checked_field_offset_setter_ident = format_ident!("checked_set_{}", field_name);
        let vis = match field.vis {
            Some(_) => {
                field.vis.clone().unwrap()
            }
            None => default_vis.clone(),
        };

        let setter_impl_tokens = generate_setter_impl_tokens(bitfield_type, field.clone(), None, quote! { bits }, false, ignored_fields_struct, None);
        let setter_with_size_check_impl_tokens = generate_setter_impl_tokens(bitfield_type, field.clone(), None, quote! { bits }, true, ignored_fields_struct, None);

        let setter_documentation =  if field.bits == 1 {
            format!(r"Sets bit `{field_offset}`.")
        } else if bitfield_attribute.bit_order == BitOrder::Msb {
            format!(r"Sets bits `{field_bits_end}..={field_offset}`.")
        } else {
            format!(r"Sets bits `{field_offset}..={field_bits_end}`.")
        };
        let checked_setter_documentation = if field.bits == 1 {
            format!(r"{setter_documentation} Returns an error if the value is too big to fit within the field bit.")
        } else {
            format!(r"{setter_documentation} Returns an error if the value is too big to fit within the field bits.")
        };
        quote! {
            #[doc = #setter_documentation]
            #vis const fn #field_offset_setter_ident(&mut self, bits: #field_type) {
                let this = self;
                #setter_impl_tokens
            }

            #[doc = #checked_setter_documentation]
            #vis const fn #checked_field_offset_setter_ident(&mut self, bits: #field_type) -> Result<(), &'static str> {
                let this = self;
                #setter_with_size_check_impl_tokens
            }
        }
    }).collect()
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
        (get_integer_type_from_type(&field_type) != Bool).then(|| {
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
                Ok(())
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
            #struct_val_ident = (#struct_val_ident & !(mask << #field_offset)) | (((#value_ident as #bitfield_type & mask) << #field_offset) as #bitfield_type);
            Ok(())
        }
    } else {
        quote! {
            let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - #field_bits_ident);
            #struct_val_ident = (#struct_val_ident & !(mask << #field_offset)) | (((#value_ident as #bitfield_type & mask) << #field_offset) as #bitfield_type);
        }
    }
}
