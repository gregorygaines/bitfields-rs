use proc_macro2::Span;

use crate::create_syn_error;
use crate::generation::common::PANIC_ERROR_MESSAGE;
use crate::parsing::types::IntegerType::{
    Bool, I8, I16, I32, I64, I128, Isize, U8, U16, U32, U64, U128, UnknownInteger, Usize,
};

#[derive(PartialEq, Debug)]
pub(crate) enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Bool,
    UnknownInteger,
}

const UNSIGNED_INTEGER_TYPES: [IntegerType; 6] = [U8, U16, U32, U64, U128, Usize];
const SIGNED_INTEGER_TYPES: [IntegerType; 6] = [I8, I16, I32, I64, I128, Isize];
const SUPPORTED_BITFIELD_TYPES: [IntegerType; 5] = [U8, U16, U32, U64, U128];
const SIZE_TYPES: [IntegerType; 2] = [Usize, Isize];

/// Returns the integer type from the type.
pub(crate) fn get_integer_type_from_type(ty: &syn::Type) -> IntegerType {
    match &*get_type_ident(ty).unwrap() {
        "u8" => U8,
        "u16" => U16,
        "u32" => U32,
        "u64" => U64,
        "u128" => U128,
        "usize" => Usize,
        "i8" => I8,
        "i16" => I16,
        "i32" => I32,
        "i64" => I64,
        "i128" => I128,
        "isize" => Isize,
        "bool" => Bool,
        _ => UnknownInteger,
    }
}

/// Returns the integer suffix from the integer type.
pub(crate) fn get_signed_integer_suffix_from_integer_type(
    integer_type: IntegerType,
) -> syn::Result<String> {
    match integer_type {
        I8 => Ok("i8".to_string()),
        I16 => Ok("i16".to_string()),
        I32 => Ok("i32".to_string()),
        I64 => Ok("i64".to_string()),
        I128 => Ok("i128".to_string()),
        Isize => Ok("isize".to_string()),
        // unreachable
        _ => Err(create_syn_error(Span::call_site(), PANIC_ERROR_MESSAGE))?,
    }
}

/// Returns if the type is a supported bitfield type.
pub(crate) fn is_supported_bitfield_type(ty: &syn::Type) -> bool {
    SUPPORTED_BITFIELD_TYPES.contains(&get_integer_type_from_type(ty))
}

/// Returns if the type is an unsigned integer.
pub(crate) fn is_unsigned_integer_type(ty: &syn::Type) -> bool {
    UNSIGNED_INTEGER_TYPES.contains(&get_integer_type_from_type(ty)) || is_bool_type(ty)
}

/// Returns if the type is an unsigned integer.
pub(crate) fn is_signed_integer_type(ty: &syn::Type) -> bool {
    SIGNED_INTEGER_TYPES.contains(&get_integer_type_from_type(ty))
}

/// Returns if the type is an unsigned integer.
pub(crate) fn is_bool_type(ty: &syn::Type) -> bool {
    get_integer_type_from_type(ty) == Bool
}

/// Returns the number of bits of the integer type.
pub(crate) fn get_bits_from_type(ty: &syn::Type) -> syn::Result<u32> {
    let type_bits = match get_type_ident(ty).unwrap().as_str() {
        "bool" => 1,
        "u8" | "i8" => 8,
        "u16" | "i16" => 16,
        "u32" | "i32" => 32,
        "u64" | "i64" => 64,
        "u128" | "i128" => 128,
        // unreachable
        _ => return Err(create_syn_error(Span::call_site(), PANIC_ERROR_MESSAGE)),
    };

    Ok(type_bits)
}

/// Returns if the type is a size type.
pub(crate) fn is_size_type(ty: &syn::Type) -> bool {
    SIZE_TYPES.contains(&get_integer_type_from_type(ty))
}

/// Returns if the type is a custom field type.
pub(crate) fn is_custom_field_type(ty: &syn::Type) -> bool {
    !is_unsigned_integer_type(ty) && !is_signed_integer_type(ty) && !is_bool_type(ty)
}

/// Returns the identifier of the type.
pub(crate) fn get_type_ident(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(ty) = ty {
        if let Some(segment) = ty.path.segments.first() {
            return Some(segment.ident.to_string());
        }
    }

    // unreachable
    panic!("{:?}", create_syn_error(Span::call_site(), PANIC_ERROR_MESSAGE))
}
