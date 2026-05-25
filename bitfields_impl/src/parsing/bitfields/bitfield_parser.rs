use std::cmp::Ordering;
use std::collections::HashSet;

use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Fields, ItemStruct, Meta};

use crate::parsing::bitfields::bitfield::{Bitfield, Field};
use crate::parsing::bitfields::bitfield_attribute::bitfield_arguments::BitOrder;
use crate::parsing::bitfields::bitfield_attribute::bitfield_attribute_parser::BitfieldAttribute;
use crate::parsing::bitfields::bits_attribute::bits_arguments::FieldAccess;
use crate::parsing::bitfields::bits_attribute::bits_attribute_parser::BitsAttribute;
use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::const_expr::ConstExpr;
use crate::parsing::common::spanned_data_type::{DataType, SpannedDataTypeToken};
use crate::parsing::common::type_parse_error::TypeParsingError;
use crate::parsing::common::visibility::Visibility;

/// Parses a bitfield struct annotated with `#[bitfield(..)]`.
pub fn parse_bitfield_struct(args: TokenStream, input: TokenStream) -> syn::Result<Bitfield> {
    let struct_tokens = parse_struct_tokens(&input)?;
    let user_attributes_tokens =
        struct_tokens.attrs.iter().map(quote::ToTokens::into_token_stream).collect();
    let visibility = Visibility::new(&struct_tokens.vis);
    let bitfield_attribute = parse_bitfield_attribute(args)?;
    let parsed_fields = parse_fields(&bitfield_attribute, &visibility, &struct_tokens)?;
    let name = struct_tokens.ident.to_string();

    check_fields_fit_in_bitfield_type(&bitfield_attribute, &parsed_fields.non_ignored)?;

    Ok(Bitfield::new(
        user_attributes_tokens,
        visibility,
        name,
        bitfield_attribute.spanned_data_type_token(),
        parsed_fields.non_ignored,
        parsed_fields.ignored,
        bitfield_attribute.arguments(),
    ))
}

fn parse_struct_tokens(input: &TokenStream) -> syn::Result<ItemStruct> {
    syn::parse2::<ItemStruct>(input.clone())
}

fn parse_bitfield_attribute(args: TokenStream) -> syn::Result<BitfieldAttribute> {
    syn::parse2(args)
}

/// Represents parsed fields.
struct ParsedFields {
    non_ignored: Vec<Field>,
    ignored: Vec<Field>,
}

fn parse_fields(
    bitfield_attribute: &BitfieldAttribute,
    bitfield_visibility: &Visibility,
    struct_tokens: &ItemStruct,
) -> syn::Result<ParsedFields> {
    let Fields::Named(field_tokens) = &struct_tokens.fields else {
        return Err(create_user_parsing_compiler_error(
            struct_tokens.span(),
            "Non-named fields are not supported.",
        ));
    };

    let mut non_ignored_parsed_fields: Vec<Field> = Vec::new();
    let mut ignored_fields: Vec<Field> = Vec::new();
    let mut seen_field_names: HashSet<String> = HashSet::new();

    for field in &field_tokens.named {
        let field_name = field.ident.as_ref().expect("Expected field to have a name").to_string();
        if seen_field_names.contains(&field_name) {
            return Err(create_user_parsing_compiler_error(
                field.span(),
                format!(
                    "Duplicate field name '{}' found, all fields must have unique names.",
                    field.ident.as_ref().expect("Expected field to have a name")
                ),
            ));
        }

        let parsed_field = parse_field_helper(
            bitfield_attribute,
            bitfield_visibility,
            field,
            &non_ignored_parsed_fields,
        )?;

        if !parsed_field.reserved() {
            seen_field_names.insert(field_name);
        }
        if parsed_field.ignored() {
            ignored_fields.push(parsed_field);
        } else {
            non_ignored_parsed_fields.push(parsed_field);
        }
    }

    Ok(ParsedFields {
        non_ignored: non_ignored_parsed_fields,
        ignored: ignored_fields,
    })
}

fn parse_field_helper(
    bitfield_attribute: &BitfieldAttribute,
    bitfield_visibility: &Visibility,
    field_tokens: &syn::Field,
    prev_fields: &[Field],
) -> syn::Result<Field> {
    let bits_attribute = get_bits_attribute(field_tokens)?;
    if is_ignored_field(bits_attribute.as_ref()) {
        return Ok(parse_ignored_field(field_tokens));
    }

    let visibility = get_field_visibility(bitfield_visibility, field_tokens);
    let reserved = is_reserved_field(field_tokens);
    let spanned_data_type_token = get_field_data_type_spanned_token(field_tokens)?;
    let bits = get_field_bits(bits_attribute.as_ref(), &spanned_data_type_token)?;

    check_bits(bits_attribute.as_ref(), bits)?;

    if matches!(spanned_data_type_token.data_type(), DataType::Integer(..)) {
        check_default_value_fit_in_field(bits_attribute.as_ref(), bits, &spanned_data_type_token)?;
        check_field_data_type_can_hold_bits(
            bits_attribute.as_ref(),
            bits,
            &spanned_data_type_token,
        )?;
    }

    let offset = calculate_field_offset(bitfield_attribute, field_tokens, bits, prev_fields)?;
    let access = get_field_access(bits_attribute.as_ref(), reserved)?;
    let name = field_tokens.ident.as_ref().expect("Expected field identifier").to_string();
    let arguments = bits_attribute.map(|attr| attr.arguments());
    Ok(Field::new(
        visibility,
        name,
        spanned_data_type_token,
        bits,
        offset,
        reserved,
        access,
        arguments,
        /* ignored= */ false,
    ))
}

fn parse_ignored_field(field_tokens: &syn::Field) -> Field {
    let spanned_data_type_token = SpannedDataTypeToken::new(&field_tokens.ty)
        .expect("Expected field type kind for ignored field");
    Field::new(
        Visibility::new(&field_tokens.vis),
        field_tokens.ident.as_ref().expect("Expected field identifier").to_string(),
        spanned_data_type_token,
        0,
        0,
        false,
        FieldAccess::NoAccess,
        /* arguments= */ None,
        /* ignored= */ true,
    )
}

/// Returns the bits attribute if the field is attributed.
fn get_bits_attribute(field_tokens: &syn::Field) -> syn::Result<Option<BitsAttribute>> {
    let bits_attribute = field_tokens
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("bits") && attr.style == syn::AttrStyle::Outer);

    let Some(attr) = bits_attribute else { return Ok(None) };

    let Meta::List(bits_attribute_tokens) = &attr.meta else {
        return Err(create_user_parsing_compiler_error(
            field_tokens.span(),
            "The '#[bits]' attribute must be a list.",
        ));
    };

    Ok(Some(syn::parse2::<BitsAttribute>(bits_attribute_tokens.tokens.clone())?))
}

/// Returns true when the parsed `#[bits]` attribute marks the field as ignored.
fn is_ignored_field(bits_attribute: Option<&BitsAttribute>) -> bool {
    bits_attribute.is_some_and(|attr| attr.arguments().ignored())
}

fn is_reserved_field(field_tokens: &syn::Field) -> bool {
    field_tokens.ident.as_ref().is_some_and(|ident| ident.to_string().starts_with('_'))
}

fn get_field_data_type_spanned_token(
    field_tokens: &syn::Field,
) -> syn::Result<SpannedDataTypeToken> {
    let spanned_data_type_token = match SpannedDataTypeToken::new(&field_tokens.ty) {
        Ok(tk) => tk,
        Err(err) => {
            return match err {
                TypeParsingError::SizeTypeNotSupported => Err(create_user_parsing_compiler_error(
                    field_tokens.ty.span(),
                    "The `isize` and `usize` types are not supported, we cannot guarantee their \
                     size at runtime. Switch to a sized integer type like `u8`, `i16`, etc.,"
                        .to_string(),
                )),
                TypeParsingError::UnexpectedFloat => Err(create_user_parsing_compiler_error(
                    field_tokens.ty.span(),
                    "Floats are not supported as a field type.".to_string(),
                )),
                TypeParsingError::ZeroArrayLength => Err(create_user_parsing_compiler_error(
                    field_tokens.ty.span(),
                    "Array fields must have a length greater than 0.".to_string(),
                )),
                TypeParsingError::NonIntegerArrayType => Err(create_user_parsing_compiler_error(
                    field_tokens.ty.span(),
                    "Array fields can only have `u8` as their element type.".to_string(),
                )),
                _ => Err(create_user_parsing_compiler_error(
                    field_tokens.ty.span(),
                    "A field can only have `#[bits]` attributed custom or integers type."
                        .to_string(),
                )),
            };
        },
    };

    Ok(spanned_data_type_token)
}

/// Returns the field bits, inferring from the data type if not explicitly set.
fn get_field_bits(
    bits_attribute: Option<&BitsAttribute>,
    spanned_data_type_token: &SpannedDataTypeToken,
) -> syn::Result<u32> {
    if let Some(bits_attr) = bits_attribute {
        if let Some(bits) = bits_attr.bits() {
            return Ok(bits);
        }
    }

    if matches!(spanned_data_type_token.data_type(), DataType::Custom) {
        return Err(create_user_parsing_compiler_error(
            spanned_data_type_token.span(),
            "Custom and nested field types require a defined bit size, otherwise we can't \
             determine the size of the field."
                .to_string(),
        ));
    }

    // Get field bits from data type if bits attribute doesn't provide any.
    Ok(spanned_data_type_token.data_type().bit_size())
}

/// Validates the bits argument of a field.
fn check_bits(bits_attribute: Option<&BitsAttribute>, bits: u32) -> syn::Result<()> {
    // The user passed 0 for bits
    if bits == 0 {
        return Err(create_user_parsing_compiler_error(
            bits_attribute
                .expect("Expected bits attribute for bit checking")
                .span()
                .expect("Expected span for bit checking"),
            "The field bits must be greater than 0.".to_string(),
        ));
    }

    Ok(())
}

/// Validate that a provided default value (if any) fits within the field's
/// bit width and type constraints. Returns an error when the default is
/// incompatible.
fn check_default_value_fit_in_field(
    bits_attribute: Option<&BitsAttribute>,
    bits: u32,
    spanned_data_type_token: &SpannedDataTypeToken,
) -> syn::Result<()> {
    let Some(bits_attr) = bits_attribute else {
        return Ok(());
    };
    let Some(default_value_expr) = bits_attr.arguments().default_value_expr() else {
        return Ok(());
    };

    // Check if the user is trying to use a const variable or const
    // function or something as a default, leave this to the compiler
    // there's nothing we can do.
    if !matches!(default_value_expr, ConstExpr::Literal { .. }) {
        return Ok(());
    }

    let ConstExpr::Literal {
        number,
        negative_sign,
        spanned_token,
        ..
    } = &default_value_expr
    else {
        unreachable!("Expected a const expr literal");
    };

    if spanned_data_type_token.data_type().unsigned() && *negative_sign {
        return Err(create_user_parsing_compiler_error(
            spanned_token.span(),
            "Unsigned fields cannot have negative default values.".to_string(),
        ));
    }

    // If checked_shl returns None, bits == 128 and no field type can exceed the
    // bit range.
    let (negative_bits_min_value, positive_bits_max_value) = min_max_for_bits(bits);

    if *negative_sign {
        let negative_default_value = (*number as i128)
            .checked_neg()
            .ok_or_else(|| unreachable!("The compiler won't allow a number less than -i128 exist"))
            .expect("Expected a negative default value for negative default value checking");

        if negative_default_value < negative_bits_min_value {
            return Err(create_user_parsing_compiler_error(
                default_value_expr.span(),
                format!(
                    "The negative default value '-{number}' is below the minimum value for the \
                     specified '{bits} bits ({negative_bits_min_value})'.",
                ),
            ));
        }
    } else if *number > positive_bits_max_value {
        return Err(create_user_parsing_compiler_error(
            default_value_expr.span(),
            format!(
                "The default value '{number}' exceeds the maximum value for the specified '{bits} \
                 bits ({positive_bits_max_value})'.",
            ),
        ));
    }

    Ok(())
}

/// Checks if the field can contain the defined bits.
fn check_field_data_type_can_hold_bits(
    bits_attribute: Option<&BitsAttribute>,
    bits: u32,
    spanned_data_type_token: &SpannedDataTypeToken,
) -> syn::Result<()> {
    let Some(bits_attr) = bits_attribute else {
        return Ok(());
    };

    let field_max_bits = spanned_data_type_token.data_type().bit_size();
    let spanned_data_type_tokens = spanned_data_type_token.get_data_type_tokens();
    if bits > field_max_bits {
        return Err(create_user_parsing_compiler_error(
            bits_attr.span().expect("Expected bits attribute for bits checking"),
            format!(
                "The field type '{spanned_data_type_tokens}' is too small to hold the specified \
                 '{bits} bits'."
            ),
        ));
    }

    Ok(())
}

/// Compute the minimum and maximum representable signed/unsigned values for
/// a given number of bits.
const fn min_max_for_bits(bits: u32) -> (i128, u128) {
    let max: u128 = if bits == 128 { u128::MAX } else { (1u128 << bits) - 1 };
    let min: i128 = if bits == 128 { i128::MIN } else { -(1i128 << (bits - 1)) };
    (min, max)
}

fn calculate_field_offset(
    bitfield_attribute: &BitfieldAttribute,
    field_tokens: &syn::Field,
    bits: u32,
    prev_fields: &[Field],
) -> syn::Result<u32> {
    let offset = prev_fields.iter().map(Field::bits).sum::<u32>();

    match bitfield_attribute.arguments().order() {
        BitOrder::Lsb => Ok(offset),
        BitOrder::Msb => {
            let bitfield_bit_size =
                bitfield_attribute.spanned_data_type_token().data_type().bit_size();
            // We calculate offset starting from the left. There's a chance that
            // the total bits of all fields is greater than the number of bits
            // of the bitfield type. We will catch it later so
            // we can ignore for now.
            let total_bits = offset + bits;
            if total_bits <= bitfield_bit_size {
                Ok(bitfield_bit_size - bits - offset)
            } else {
                // We've overflown the bitfield type.
                Err(create_user_parsing_compiler_error(
                    field_tokens.span(),
                    format!(
                        "The total bits of the fields ({total_bits} bits) exceeds the bit size of \
                         the bitfield type ({bitfield_bit_size} bits)."
                    ),
                ))
            }
        },
    }
}

/// Determine the effective access level for a field based on its
/// `#[bits]` arguments and whether it is reserved.
fn get_field_access(
    bits_attribute: Option<&BitsAttribute>,
    reserved: bool,
) -> syn::Result<FieldAccess> {
    if bits_attribute.is_none() && reserved {
        return Ok(FieldAccess::NoAccess);
    }

    if let Some(bits_attribute) = bits_attribute {
        if bits_attribute.arguments().user_set_access() && reserved {
            return Err(create_user_parsing_compiler_error(
                bits_attribute.arguments().access_span().expect("Expected span for access"),
                "Reserved fields cannot have a defined access.".to_string(),
            ));
        }

        if bits_attribute.arguments().default_value_expr().is_some() && reserved {
            return Ok(FieldAccess::ReadOnly);
        }
    }

    if bits_attribute.is_none() {
        return Ok(FieldAccess::ReadWrite);
    }

    Ok(bits_attribute
        .expect("Expected bits attribute for getting field access")
        .arguments()
        .access())
}

/// Get the visibility that should be applied to a field: if the field is
/// private, inherit the bitfield's visibility; otherwise use the field's.
fn get_field_visibility(bitfield_visibility: &Visibility, field_tokens: &syn::Field) -> Visibility {
    let visibility = Visibility::new(&field_tokens.vis);

    if visibility == Visibility::Private {
        return bitfield_visibility.clone();
    }

    visibility
}

/// Ensure the total bits occupied by fields exactly match the bitfield type
/// size. Returns an error if too many or few bits are used.
fn check_fields_fit_in_bitfield_type(
    bitfield_attribute: &BitfieldAttribute,
    fields: &[Field],
) -> syn::Result<()> {
    let total_field_bits = fields.iter().map(Field::bits).sum::<u32>();
    let bitfield_bit_size = bitfield_attribute.spanned_data_type_token().data_type().bit_size();

    match total_field_bits.cmp(&bitfield_bit_size) {
        Ordering::Greater => Err(create_user_parsing_compiler_error(
            bitfield_attribute.spanned_data_type_token().span(),
            format!(
                "The total number of bits of the fields '{} bits' is greater than the number of \
                 bits of the bitfield '{} ({} bits)'.",
                total_field_bits,
                bitfield_attribute.spanned_data_type_token(),
                bitfield_bit_size
            ),
        )),
        Ordering::Less => {
            let remaining_bits = bitfield_bit_size - total_field_bits;

            Err(create_user_parsing_compiler_error(
                bitfield_attribute.spanned_data_type_token().span(),
                format!(
                    "The total number of bits of the fields '{} bits' is less than the number of \
                     bits of the bitfield '{} ({} bits)', you can add a reserved field (prefixed \
                     with '_') to fill the remaining '{} bits'.",
                    total_field_bits,
                    bitfield_attribute.spanned_data_type_token(),
                    bitfield_bit_size,
                    remaining_bits,
                ),
            ))
        },
        Ordering::Equal => {
            // The total number of bits of all fields is equal to the number of
            // bits, we're good.
            Ok(())
        },
    }
}
