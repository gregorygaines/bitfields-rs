use std::cmp::PartialEq;

use derive_more::Display;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Type;
use syn::spanned::Spanned;

use crate::parsing::common::spanned_token::SpannedToken;
use crate::parsing::common::to_tokens::ToTokens;
use crate::parsing::common::type_parse_error::TypeParsingError;

/// Represents a kind of type that a bitfield or field can represent.
#[derive(Clone, Debug, Display)]
#[display("{}", spanned_token.token())]
pub struct SpannedDataTypeToken {
    data_type: DataType,
    spanned_token: SpannedToken,
}

impl SpannedDataTypeToken {
    /// Creates a new type kind from a syn type.
    pub fn new(syn_type: &Type) -> Result<Self, TypeParsingError> {
        let str_repr = Self::get_syn_type_string(syn_type);
        let data_type = Self::get_data_type(syn_type)?;

        Ok(Self {
            data_type,
            spanned_token: SpannedToken::new(str_repr, syn_type.span()),
        })
    }

    /// Returns a compact string representation of a `syn::Type` (e.g.
    /// `[u8;32]`).
    fn get_syn_type_string(syn_type: &Type) -> String {
        Self::remove_whitespace(&quote! { #syn_type }.to_string())
    }

    /// Removes all whitespace from a string.
    fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    /// Get the type category of the type.
    fn get_data_type(syn_type: &Type) -> Result<DataType, TypeParsingError> {
        match syn_type {
            Type::Path(type_path) => {
                let type_name =
                    type_path.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();

                if matches!(type_name.as_str(), "f32" | "f64") {
                    return Err(TypeParsingError::UnexpectedFloat);
                }

                let type_category = DataType::new(&type_name).unwrap_or(DataType::Custom);

                match type_category {
                    DataType::Integer(IntegerType::Usize | IntegerType::Isize) => {
                        Err(TypeParsingError::SizeTypeNotSupported)
                    },
                    _ => Ok(type_category),
                }
            },
            Type::Array(type_array) => {
                let element_type = Self::get_data_type(&type_array.elem)?;
                let DataType::Integer(integer_type) = element_type else {
                    return Err(TypeParsingError::NonIntegerArrayType);
                };

                if !matches!(integer_type, IntegerType::U8) {
                    return Err(TypeParsingError::NonIntegerArrayType);
                }

                let length = match &type_array.len {
                    syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                        syn::Lit::Int(lit_int) => lit_int.base10_parse::<u32>().map_err(|_| {
                            TypeParsingError::Unexpected("Invalid array length".to_string())
                        })?,
                        _ => {
                            return Err(TypeParsingError::Unexpected(
                                "Array length must be an integer literal".to_string(),
                            ));
                        },
                    },
                    _ => {
                        return Err(TypeParsingError::Unexpected(
                            "Array length must be an integer literal".to_string(),
                        ));
                    },
                };

                if length == 0 {
                    return Err(TypeParsingError::ZeroArrayLength);
                }

                Ok(DataType::Array {
                    length,
                })
            },
            _ => Err(TypeParsingError::Unexpected(format!("Unsupported type: {syn_type:?}"))),
        }
    }

    pub fn span(&self) -> Span {
        self.spanned_token.span()
    }

    /// Returns the category of the type kind.
    pub const fn data_type(&self) -> DataType {
        self.data_type
    }

    /// Returns the byte length of an array-backed type, or `None` for
    /// non-array types.
    ///
    /// For `[u8; N]` this returns `Some(N)`, which is the same value that
    /// `core::mem::size_of::<[u8; N]>()` produces at runtime.  Callers in
    /// code-generation paths use this to emit a literal constant instead of a
    /// `size_of` call.
    pub const fn array_length(&self) -> Option<usize> {
        match self.data_type {
            DataType::Array {
                length,
            } => Some(length as usize),
            _ => None,
        }
    }

    /// Returns the token representation of the data type.
    /// For array-backed types, returns `u128` as the internal backing type.
    pub fn get_data_type_tokens(&self) -> TokenStream {
        if matches!(self.data_type, DataType::Array { .. }) {
            return quote! {
                u128
            };
        }

        self.spanned_token.to_tokens()
    }
}

impl ToTokens for SpannedDataTypeToken {
    fn to_tokens(&self) -> TokenStream {
        self.spanned_token.to_tokens()
    }
}

/// Classifies a parsed type as a built-in integer, array, or custom type.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DataType {
    Integer(IntegerType),
    Array {
        length: u32,
    },
    /// A user-defined type.
    Custom,
}

impl DataType {
    pub fn new(data_type_str: &str) -> Result<Self, String> {
        match data_type_str.trim() {
            "u8" => Ok(Self::Integer(IntegerType::U8)),
            "u16" => Ok(Self::Integer(IntegerType::U16)),
            "u32" => Ok(Self::Integer(IntegerType::U32)),
            "u64" => Ok(Self::Integer(IntegerType::U64)),
            "u128" => Ok(Self::Integer(IntegerType::U128)),
            "usize" => Ok(Self::Integer(IntegerType::Usize)),
            "isize" => Ok(Self::Integer(IntegerType::Isize)),
            "i8" => Ok(Self::Integer(IntegerType::I8)),
            "i16" => Ok(Self::Integer(IntegerType::I16)),
            "i32" => Ok(Self::Integer(IntegerType::I32)),
            "i64" => Ok(Self::Integer(IntegerType::I64)),
            "i128" => Ok(Self::Integer(IntegerType::I128)),
            "bool" => Ok(Self::Integer(IntegerType::Bool)),
            _ => Err(format!("Unsupported type category '{data_type_str}'.")),
        }
    }

    /// Returns the bit size of the type, if any.
    pub fn bit_size(self) -> u32 {
        match self {
            Self::Integer(integer_type) => integer_type.bit_size(),
            Self::Array {
                length,
            } => length * IntegerType::U8.bit_size(),
            Self::Custom => unreachable!(
                "Custom types should not have a bit size and all calls should be guarded."
            ),
        }
    }

    pub const fn unsigned(self) -> bool {
        match self {
            Self::Integer(integer_type) => integer_type.is_unsigned(),
            Self::Custom
            | Self::Array {
                ..
            } => true, // Custom and array types are assumed to be unsigned
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum IntegerType {
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
}

impl IntegerType {
    fn bit_size(self) -> u32 {
        match self {
            Self::U8 | Self::I8 => 8,
            Self::U16 | Self::I16 => 16,
            Self::U32 | Self::I32 => 32,
            Self::U64 | Self::I64 => 64,
            Self::U128 | Self::I128 => 128,
            Self::Bool => 1,
            Self::Usize | Self::Isize => unreachable!(
                "Only usize and isize should not have a bit size, but they are filtered out \
                 earlier."
            ),
        }
    }

    const fn is_unsigned(self) -> bool {
        matches!(self, Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::U128 | Self::Bool)
    }
}

impl ToTokens for IntegerType {
    fn to_tokens(&self) -> TokenStream {
        match self {
            Self::U8 => quote! { u8 },
            Self::U16 => quote! { u16 },
            Self::U32 => quote! { u32 },
            Self::U64 => quote! { u64 },
            Self::U128 => quote! { u128 },
            Self::Usize => quote! { usize },
            Self::Isize => quote! { isize },
            Self::I8 => quote! { i8 },
            Self::I16 => quote! { i16 },
            Self::I32 => quote! { i32 },
            Self::I64 => quote! { i64 },
            Self::I128 => quote! { i128 },
            Self::Bool => quote! { bool },
        }
    }
}
