mod generation;
mod parsing;

use std::cmp::Ordering;
use std::fmt;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Expr, ExprLit, ExprUnary, Fields, Lit, LitInt, Meta, Type, Visibility};

use crate::generation::bit_operations::{generate_get_bit_tokens, generate_set_bit_tokens};
use crate::generation::builder_struct::{generate_builder_tokens, generate_to_builder_tokens};
use crate::generation::common::PANIC_ERROR_MESSAGE;
use crate::generation::debug_impl::generate_debug_implementation;
use crate::generation::default_impl::generate_default_implementation_tokens;
use crate::generation::field_const_getter_setter::{
    generate_field_constants_tokens, generate_field_getters_functions_tokens,
    generate_field_setters_functions_tokens,
};
use crate::generation::from_into_bits_conversions::{
    generate_from_bits_functions_tokens, generate_into_bits_function_tokens,
};
use crate::generation::from_types_impl::{
    generate_from_bitfield_for_bitfield_type_implementation_tokens,
    generate_from_bitfield_type_for_bitfield_implementation_tokens,
};
use crate::generation::new_impl::generate_new_function_tokens;
use crate::generation::set_clear_bits_impl::{
    generate_clear_bits_functions_tokens, generate_set_bits_functions_tokens,
};
use crate::generation::tuple_struct::{
    generate_struct_with_fields_tokens, generate_tuple_struct_tokens,
};
use crate::parsing::bitfield_attribute::{BitOrder, BitfieldAttribute};
use crate::parsing::bitfield_field::{BitfieldField, BitsAttribute, FieldAccess, FieldType};
use crate::parsing::number_parser::{NumberParseError, ParsedNumber, parse_number_string};
use crate::parsing::types::{
    IntegerType, get_bits_from_type, get_integer_suffix_from_integer_type,
    get_integer_type_from_type, get_type_ident, is_custom_field_type, is_size_type,
    is_supported_field_type, is_unsigned_integer_type,
};

/// The `#[bit]` attribute name.
pub(crate) const BIT_ATTRIBUTE_NAME: &str = "bits";

/// The ident prefix for padding fields.
pub(crate) const PADDING_FIELD_NAME_PREFIX: &str = "_";

#[proc_macro_attribute]
pub fn bitfield(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match parse_bitfield(args.into(), input.into()) {
        Ok(res) => res.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

/// Parses the bitfield attribute, struct, and fields.
fn parse_bitfield(args: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    // Parse the struct tokens
    let struct_tokens = syn::parse2::<syn::ItemStruct>(input.clone())?;

    // Parse the arguments of the '#[bitfield(arg, arg)]' attribute
    let bitfield_attribute: BitfieldAttribute = match syn::parse2(args) {
        Ok(bitfield_attribute) => bitfield_attribute,
        Err(err) => {
            return Err(create_syn_error(input.span(), err.to_string()));
        }
    };

    // Check if the bitfield type can contain the fields.
    let all_fields = parse_fields(&bitfield_attribute, &struct_tokens)?;
    let fields = all_fields.0;
    let ignored_fields = all_fields.1;
    check_bitfield_type_contain_field_bits(&bitfield_attribute, &fields)?;
    check_bitfield_names_unique(&fields)?;

    // Generate the bitfield functions.
    generate_functions(&bitfield_attribute, &fields, &ignored_fields, &struct_tokens)
}

/// Check if the bitfield type can contain the field bits.
fn check_bitfield_type_contain_field_bits(
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
) -> syn::Result<()> {
    let total_field_bits = fields.iter().map(|field| field.bits).sum::<u32>();

    match total_field_bits.cmp(&bitfield_attribute.bits) {
        Ordering::Greater => Err(create_syn_error(
            bitfield_attribute.ty.span(),
            format!(
                "The total number of bits of the fields ({} bits) is greater than the number of bits of the bitfield type '{}' ({} bits).",
                total_field_bits,
                get_type_ident(&bitfield_attribute.ty).unwrap(),
                bitfield_attribute.bits
            ),
        )),
        Ordering::Less => {
            let remaining_bits = bitfield_attribute.bits - total_field_bits;
            Err(create_syn_error(
                bitfield_attribute.ty.span(),
                format!(
                    "The total number of bits of the fields ({} bits) is less than the number of bits of the bitfield type '{}' ({} bits), you can add a padding field (prefixed with '_') to fill the remaining '{} bits'.",
                    total_field_bits,
                    get_type_ident(&bitfield_attribute.ty).unwrap(),
                    bitfield_attribute.bits,
                    remaining_bits,
                ),
            ))
        }
        Ordering::Equal => {
            // The total number of bits of all fields is equal to the number of bits, we're
            // good.
            Ok(())
        }
    }
}

fn check_bitfield_names_unique(fields: &[BitfieldField]) -> syn::Result<()> {
    let mut field_names = Vec::new();
    for field in fields {
        if field_names.contains(&field.name) {
            return Err(create_syn_error(
                field.name.span(),
                format!(
                    "The field name '{}' is duplicated, each field must have a unique name.",
                    field.name
                ),
            ));
        }
        if !field.padding {
            field_names.push(field.name.clone());
        }
    }

    Ok(())
}

/// Parses all the fields into a list of [`BitfieldField`]s.
fn parse_fields(
    bitfield_attribute: &BitfieldAttribute,
    struct_tokens: &syn::ItemStruct,
) -> syn::Result<(Vec<BitfieldField>, Vec<BitfieldField>)> {
    let fields_tokens = match &struct_tokens.fields {
        Fields::Named(named_files) => named_files,
        _ => {
            return Err(create_syn_error(
                struct_tokens.span(),
                "Non-named fields are not supported.",
            ));
        }
    };

    let mut fields = Vec::new();
    let mut ignored_fields = Vec::new();
    for field_token in &fields_tokens.named {
        let field = do_parse_field(bitfield_attribute, field_token, &fields)?;
        if field.ignore {
            ignored_fields.push(field);
        } else {
            fields.push(field);
        }
    }

    Ok((fields, ignored_fields))
}

/// Internal implementation of [`parse_fields`] to parse a single field.
fn do_parse_field(
    bitfield_attribute: &BitfieldAttribute,
    field_tokens: &syn::Field,
    prev_fields: &[BitfieldField],
) -> syn::Result<BitfieldField> {
    // Parse field attribute, a field could have multiple attributes, but we only
    // care about our 'bits' attribute.
    let field_bit_attribute = field_tokens.attrs.iter().find(|attr| {
        attr.path().is_ident(BIT_ATTRIBUTE_NAME) && attr.style == syn::AttrStyle::Outer
    });

    let visibility = match field_tokens.vis {
        // Pass the visibility to the field.
        Visibility::Public(_) | Visibility::Restricted(_) => Some(field_tokens.vis.clone()),
        // Use the visibility of the struct
        Visibility::Inherited => None,
    };

    let field_type = if is_custom_field_type(&field_tokens.ty) {
        FieldType::CustomFieldType
    } else {
        FieldType::IntegerFieldType
    };

    let padding =
        field_tokens.ident.as_ref().unwrap().to_string().starts_with(PADDING_FIELD_NAME_PREFIX);

    let bitfield = if field_bit_attribute.is_none() {
        if !is_supported_field_type(&field_tokens.ty) {
            return Err(create_syn_error(
                field_tokens.span(),
                format!(
                    "The field type {:?} is not supported.",
                    get_type_ident(&field_tokens.ty).unwrap()
                ),
            ));
        }

        // We have to determine the number of bits from the field type since there's no
        // '#[bits]' attribute.
        if is_size_type(&field_tokens.ty) {
            return Err(create_syn_error(
                field_tokens.span(),
                "The types isize and usize require a bit size, otherwise we can't determine the size of the field.",
            ));
        }

        if field_type != FieldType::IntegerFieldType {
            return Err(create_syn_error(
                field_tokens.span(),
                "Custom and nested field types require a defined bit size, otherwise we can't determine the size of the field.",
            ));
        }

        let bits = get_bits_from_type(&field_tokens.ty)?;
        let offset = calculate_field_offset(bits, bitfield_attribute, prev_fields)?;
        let access = if padding { FieldAccess::None } else { FieldAccess::ReadWrite };

        // Create a bitfield field with default values since we don't have one to
        // parse.
        BitfieldField {
            name: field_tokens.ident.clone().unwrap(),
            ty: field_tokens.ty.clone(),
            vis: visibility,
            bits,
            offset,
            default_value_tokens: None,
            unsigned: true,
            padding,
            access,
            field_type: FieldType::IntegerFieldType,
            ignore: false,
        }
    } else {
        let bit_attribute_tokens = match &field_bit_attribute.unwrap().meta {
            Meta::List(list) => list,
            _ => {
                return Err(create_syn_error(
                    field_tokens.span(),
                    "The '#[bits]' attribute must be a list.",
                ));
            }
        };

        let bits_attribute: BitsAttribute = syn::parse2(bit_attribute_tokens.tokens.clone())?;

        if bits_attribute.ignore {
            return Ok(BitfieldField {
                ty: field_tokens.ty.clone(),
                vis: Some(field_tokens.clone().vis),
                bits: 0,
                offset: 0,
                default_value_tokens: None,
                unsigned: false,
                padding,
                access: FieldAccess::ReadOnly,
                name: field_tokens.ident.clone().unwrap(),
                ignore: true,
                field_type,
            });
        }

        if !is_supported_field_type(&field_tokens.ty) {
            return Err(create_syn_error(
                field_tokens.span(),
                format!(
                    "The field type {:?} is not supported.",
                    get_type_ident(&field_tokens.ty).unwrap()
                ),
            ));
        }

        let bits = match bits_attribute.bits {
            Some(bits) => {
                // Make sure the type of the field can contain the specified number of bits if
                // not a custom type.
                if field_type == FieldType::IntegerFieldType
                    && bits > get_bits_from_type(&field_tokens.ty)?
                {
                    return Err(create_syn_error(
                        bit_attribute_tokens.span(),
                        format!(
                            "The field type {:?} ({} bits) is too small to hold the specified '{} bits'.",
                            get_type_ident(&field_tokens.ty).unwrap(),
                            get_bits_from_type(&field_tokens.ty,)?,
                            bits
                        ),
                    ));
                }

                bits
            }
            None => {
                if field_type != FieldType::IntegerFieldType {
                    return Err(create_syn_error(
                        field_tokens.span(),
                        "Custom and nested field types require a defined bit size, otherwise we can't determine the size of the field.",
                    ));
                }

                get_bits_from_type(&field_tokens.ty)?
            }
        };

        // Make sure the field bits are greater than 0.
        if bits == 0 {
            return Err(create_syn_error(
                bit_attribute_tokens.span(),
                "The field bits must be greater than 0.",
            ));
        }

        // Make sure the default value is within the field bits. If a number was unable
        // to be parsed, let's take a chance and see if the user is trying to
        // use a const variable or a const function.
        let parsed_number = if field_type == FieldType::IntegerFieldType
            && bits_attribute.default_value_expr.is_some()
        {
            check_default_value_fit_in_field(
                &bits_attribute.default_value_expr.clone().unwrap(),
                bits,
                &field_tokens.ty,
            )?
        } else {
            None
        };

        let unsigned =
            field_type != FieldType::IntegerFieldType || is_unsigned_integer_type(&field_tokens.ty);
        let access = if padding {
            if bits_attribute.access.is_some() {
                return Err(create_syn_error(
                    bit_attribute_tokens.span(),
                    "Padding fields can't have a specified access.",
                ));
            }

            FieldAccess::None
        } else {
            bits_attribute.access.unwrap_or(FieldAccess::ReadWrite)
        };
        let offset = calculate_field_offset(bits, bitfield_attribute, prev_fields)?;

        let default_value_tokens = match bits_attribute.default_value_expr {
            None => None,
            Some(ref expr) => {
                // We want to add integer literals to default values expressions if the
                // expression is a negative number without a suffix. We do alot of casting
                // so what happens is, if there is the default value expr `-125`, when we
                // try to cast later like `-125 as u8`, Rust will complain that the number
                // is too large for the type. Adding the integer suffix will fix this since
                // Rust will know the type of the number and will cast it.
                if unsigned
                    || field_type != FieldType::IntegerFieldType
                    || parsed_number.is_none()
                    || parsed_number.unwrap().has_integer_suffix
                {
                    Some(quote! {
                        #expr
                    })
                } else {
                    let tokens = add_integer_literals_to_expr(expr, &field_tokens.ty)?;

                    Some(quote! {
                        #tokens
                    })
                }
            }
        };

        BitfieldField {
            name: field_tokens.ident.clone().unwrap(),
            ty: field_tokens.ty.clone(),
            vis: visibility,
            bits,
            offset,
            default_value_tokens,
            unsigned,
            padding,
            access,
            field_type,
            ignore: false,
        }
    };

    Ok(bitfield)
}

/// Checks if the default value can fit in the field bits.
fn check_default_value_fit_in_field(
    default_value_expr: &Expr,
    bits: u32,
    field_type: &Type,
) -> syn::Result<Option<ParsedNumber>> {
    let default_value_str = &quote!(#default_value_expr).to_string();

    let parsed_number = match parse_number_string(default_value_str) {
        Ok(number) => number,
        Err(err) => {
            return match err {
                NumberParseError::FloatNotSupported => Err(create_syn_error(
                    default_value_expr.span(),
                    "Floats are not supported as default values.".to_string(),
                )),
                // Maybe the user is trying to use a const variable or a const
                // function call as a default.
                NumberParseError::InvalidNumberString => Ok(None),
            };
        }
    };

    let bits_max_value = 1 << bits as u128;
    if parsed_number.number >= bits_max_value {
        if parsed_number.negative {
            return Err(create_syn_error(
                default_value_expr.span(),
                format!(
                    "The default value -'{}' is too large to fit into the specified '{} bits'.",
                    parsed_number.number, bits,
                ),
            ));
        }
        return Err(create_syn_error(
            default_value_expr.span(),
            format!(
                "The default value '{}' is too large to fit into the specified '{} bits'.",
                parsed_number.number, bits,
            ),
        ));
    }

    let default_value_too_big_for_type = match get_integer_type_from_type(field_type) {
        IntegerType::Bool => parsed_number.number > 1,
        IntegerType::U8 => parsed_number.number > u8::MAX as u128,
        IntegerType::U16 => parsed_number.number > u16::MAX as u128,
        IntegerType::U32 => parsed_number.number > u32::MAX as u128,
        IntegerType::U64 => parsed_number.number > u64::MAX as u128,
        IntegerType::U128 => {
            // Unable to happen, this is Rust's max unsigned type value.
            false
        }
        IntegerType::Usize => parsed_number.number > usize::MAX as u128,
        IntegerType::Isize => {
            if parsed_number.negative {
                parsed_number.number > isize::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > isize::MAX as u128
            }
        }
        IntegerType::I8 => {
            if parsed_number.negative {
                parsed_number.number > i8::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > i8::MAX as u128
            }
        }
        IntegerType::I16 => {
            if parsed_number.negative {
                parsed_number.number > i16::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > i16::MAX as u128
            }
        }
        IntegerType::I32 => {
            if parsed_number.negative {
                parsed_number.number > i32::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > i32::MAX as u128
            }
        }
        IntegerType::I64 => {
            if parsed_number.negative {
                parsed_number.number > i64::MIN.unsigned_abs() as u128
            } else {
                parsed_number.number > i64::MAX as u128
            }
        }
        IntegerType::I128 => {
            if parsed_number.negative {
                parsed_number.number > i128::MIN.unsigned_abs()
            } else {
                parsed_number.number > i128::MAX as u128
            }
        }
        _ => Err(create_syn_error(default_value_expr.span(), PANIC_ERROR_MESSAGE))?,
    };

    if default_value_too_big_for_type {
        let negative_str = if parsed_number.negative { "-" } else { "" };
        return Err(create_syn_error(
            default_value_expr.span(),
            format!(
                "The default value '{}{}' is too large to fit into the field type '{}'.",
                negative_str,
                parsed_number.number,
                get_type_ident(field_type).unwrap()
            ),
        ));
    }

    Ok(Some(parsed_number))
}

/// Calculate the offset of a field based on previous fields.
fn calculate_field_offset(
    bits: u32,
    bitfield_attribute: &BitfieldAttribute,
    prev_fields: &[BitfieldField],
) -> syn::Result<u32> {
    let offset = prev_fields.iter().map(|field| field.bits).sum::<u32>();

    match bitfield_attribute.bit_order {
        BitOrder::Lsb => Ok(offset),
        BitOrder::Msb => {
            let bitfield_type_bits = get_bits_from_type(&bitfield_attribute.ty)?;
            // We calculate offset starting from the left. There's a chance that
            // the total bits of all fields is greater than the number of bits
            // of the bitfield type. We will catch it later so
            // we can ignore for now.
            if offset + bits < bitfield_type_bits {
                Ok(bitfield_type_bits - bits - offset)
            } else {
                // We've underflow the bitfield type, this will be caught later.
                Ok(0)
            }
        }
    }
}

/// Adds the field type integer literal suffix to the expression.
///
/// For example, if the expression is '-1' and the field type is 'i8', the
/// expression will be updated to '1i8'.
fn add_integer_literals_to_expr(expr: &Expr, field_type: &Type) -> syn::Result<TokenStream> {
    let updated_expr = if let Expr::Unary(unary) = expr {
        let attrs = unary.attrs.clone();
        let op = unary.op;

        let updated_expr = if let Expr::Lit(expr_lit) = *unary.expr.clone() {
            let new_lit = create_expr_lit_with_integer_suffix(&expr_lit, field_type)?;

            Expr::Lit(ExprLit { attrs: expr_lit.attrs, lit: new_lit.lit })
        } else {
            Err(create_syn_error(expr.span(), PANIC_ERROR_MESSAGE))?
        };

        Expr::Unary(ExprUnary { attrs, op, expr: Box::new(updated_expr) })
    } else if let Expr::Lit(expr_lit) = expr {
        let new_lit = create_expr_lit_with_integer_suffix(expr_lit, field_type)?;

        Expr::Lit(ExprLit { attrs: expr_lit.clone().attrs, lit: new_lit.lit })
    } else {
        Err(create_syn_error(expr.span(), PANIC_ERROR_MESSAGE))?
    };

    Ok(quote! {
        #updated_expr
    })
}

/// Helper for creating an integer literal with the integer suffix.
fn create_expr_lit_with_integer_suffix(lit: &ExprLit, field_type: &Type) -> syn::Result<ExprLit> {
    let integer_type = get_integer_type_from_type(field_type);
    let integer_suffix = get_integer_suffix_from_integer_type(integer_type)?;

    let new_lit = match &lit.lit {
        Lit::Int(lit_int) => {
            let new_lit_int =
                LitInt::new(&format!("{}{}", lit_int.token(), integer_suffix), lit_int.span());
            ExprLit { attrs: lit.attrs.clone(), lit: Lit::Int(new_lit_int) }
        }
        _ => Err(create_syn_error(lit.span(), PANIC_ERROR_MESSAGE))?,
    };

    Ok(new_lit)
}

/// Generate the bitfield functions.
fn generate_functions(
    bitfield_attribute: &BitfieldAttribute,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    struct_tokens: &syn::ItemStruct,
) -> syn::Result<TokenStream> {
    let struct_attributes: TokenStream =
        struct_tokens.attrs.iter().map(ToTokens::to_token_stream).collect();
    let struct_name = &struct_tokens.ident;

    let bitfield_struct = if !ignored_fields.is_empty() {
        generate_struct_with_fields_tokens(
            struct_name,
            &struct_tokens.vis,
            ignored_fields,
            bitfield_attribute,
        )
    } else {
        generate_tuple_struct_tokens(struct_name, &struct_tokens.vis, bitfield_attribute)
    };
    let new_functions = bitfield_attribute.generate_new_func.then(|| {
        generate_new_function_tokens(&struct_tokens.vis, fields, ignored_fields, bitfield_attribute)
    });
    let from_bits_functions = bitfield_attribute.generate_from_bits_func.then(|| {
        generate_from_bits_functions_tokens(
            &struct_tokens.vis,
            fields,
            ignored_fields,
            bitfield_attribute,
        )
    });
    let generate_into_bits_function = bitfield_attribute.generate_into_bits_func.then(|| {
        generate_into_bits_function_tokens(
            &struct_tokens.vis,
            bitfield_attribute,
            !ignored_fields.is_empty(),
        )
    });
    let field_consts_tokens = generate_field_constants_tokens(&struct_tokens.vis, fields);
    let field_getters_tokens = generate_field_getters_functions_tokens(
        &struct_tokens.vis,
        bitfield_attribute,
        fields,
        !ignored_fields.is_empty(),
    )?;
    let field_setters_tokens = generate_field_setters_functions_tokens(
        &struct_tokens.vis,
        bitfield_attribute,
        fields,
        !ignored_fields.is_empty(),
    );
    let default_function = bitfield_attribute.generate_default_impl.then(|| {
        generate_default_implementation_tokens(
            struct_name,
            fields,
            ignored_fields,
            bitfield_attribute,
        )
    });
    let builder_tokens = bitfield_attribute.generate_builder.then(|| {
        generate_builder_tokens(
            &struct_tokens.vis,
            struct_name,
            fields,
            ignored_fields,
            bitfield_attribute,
        )
    });

    let from_bitfield_type_for_bitfield_function_tokens =
        bitfield_attribute.generate_from_trait_funcs.then(|| {
            generate_from_bitfield_type_for_bitfield_implementation_tokens(
                struct_name,
                fields,
                ignored_fields,
                bitfield_attribute,
            )
        });
    let from_bitfield_for_bitfield_type_function_tokens =
        bitfield_attribute.generate_from_trait_funcs.then(|| {
            generate_from_bitfield_for_bitfield_type_implementation_tokens(
                struct_name,
                bitfield_attribute,
                !ignored_fields.is_empty(),
            )
        });
    let debug_impl = bitfield_attribute.generate_debug_impl.then(|| {
        generate_debug_implementation(
            struct_name,
            bitfield_attribute,
            fields,
            !ignored_fields.is_empty(),
        )
    });
    let get_bit_operations = bitfield_attribute.generate_bit_ops.then(|| {
        generate_get_bit_tokens(
            &struct_tokens.vis,
            &bitfield_attribute.ty,
            fields,
            !ignored_fields.is_empty(),
        )
    });
    let set_bit_operations = bitfield_attribute.generate_bit_ops.then(|| {
        generate_set_bit_tokens(
            &struct_tokens.vis,
            &bitfield_attribute.ty,
            fields,
            !ignored_fields.is_empty(),
        )
    });
    let to_builder_tokens = (bitfield_attribute.generate_builder
        && bitfield_attribute.generate_to_builder)
        .then(|| generate_to_builder_tokens(&struct_tokens.vis, struct_name));
    let set_bits_operations = bitfield_attribute.generate_set_bits_impl.then(|| {
        generate_set_bits_functions_tokens(
            &struct_tokens.vis,
            fields,
            bitfield_attribute,
            !ignored_fields.is_empty(),
        )
    });
    let clear_bits_operations = bitfield_attribute.generate_clear_bits_impl.then(|| {
        generate_clear_bits_functions_tokens(
            &struct_tokens.vis,
            fields,
            bitfield_attribute,
            !ignored_fields.is_empty(),
        )
    });
    let default_attrs = if ignored_fields.is_empty() {
        quote! {
            #[repr(transparent)]
        }
    } else {
        quote! {
            #[repr(C)]
        }
    };

    Ok(quote! {
        #struct_attributes
        #default_attrs
        #bitfield_struct

        impl #struct_name {
            #new_functions

            #from_bits_functions

            #generate_into_bits_function

            #field_consts_tokens
            #field_getters_tokens
            #field_setters_tokens

            #set_bits_operations
            #clear_bits_operations

            #get_bit_operations
            #set_bit_operations

            #to_builder_tokens
        }

        #default_function

        #builder_tokens

        #from_bitfield_type_for_bitfield_function_tokens
        #from_bitfield_for_bitfield_type_function_tokens

        #debug_impl
    })
}

/// Creates a syn error with the specified message that occurred at the
/// specified span.
pub(crate) fn create_syn_error(span: proc_macro2::Span, msg: impl fmt::Display) -> syn::Error {
    syn::Error::new(span, msg)
}
