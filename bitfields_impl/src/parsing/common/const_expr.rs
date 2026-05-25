use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Expr;

use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::spanned_token::SpannedToken;
use crate::parsing::common::to_tokens::ToTokens;

const FLOAT_IDENTIFIERS: [&str; 2] = ["f32", "f64"];
const FLOAT_DOT: &str = ".";

const INTEGER_IDENTIFIERS: [&str; 12] =
    ["u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize"];
const BOOLEAN_IDENTIFIERS: [&str; 2] = ["true", "false"];
const NEGATIVE_SIGN: &str = "-";

/// Represents a constant expression.
#[derive(Clone, Debug)]
pub enum ConstExpr {
    /// An integer or boolean literal with optional negation.
    Literal { number: u128, negative_sign: bool, spanned_token: SpannedToken },

    /// A constant path.
    Path { spanned_token: SpannedToken },
}

enum IntegerPrefix {
    Hex,
    Binary,
    Octal,
}

impl IntegerPrefix {
    const HEX_PREFIX: &str = "0x";
    const HEX_RADIX: u32 = 16;

    const BINARY_PREFIX: &str = "0b";
    const BINARY_RADIX: u32 = 2;

    const OCTAL_PREFIX: &str = "0o";
    const OCTAL_RADIX: u32 = 8;

    fn prefix(&self) -> String {
        match &self {
            Self::Hex => Self::HEX_PREFIX.to_string(),
            Self::Binary => Self::BINARY_PREFIX.to_string(),
            Self::Octal => Self::OCTAL_PREFIX.to_string(),
        }
    }

    const fn radix(&self) -> u32 {
        match self {
            Self::Hex => Self::HEX_RADIX,
            Self::Binary => Self::BINARY_RADIX,
            Self::Octal => Self::OCTAL_RADIX,
        }
    }
}

impl ConstExpr {
    pub fn new(spanned_token: &SpannedToken) -> syn::Result<Self> {
        let trimmed_number_str =
            spanned_token.token().trim().replace([' ', '_'], "").to_ascii_lowercase();

        Self::check_float(&trimmed_number_str, spanned_token.span())?;

        if Self::is_boolean(&trimmed_number_str) {
            return Ok(Self::parse_boolean(&trimmed_number_str, spanned_token));
        }

        let parsed_integer_result = Self::parse_integer(&trimmed_number_str, spanned_token);
        if let Ok(parsed_integer) = parsed_integer_result {
            return Ok(parsed_integer);
        }

        // Parsing as integer failed; assume it's a path and let the compiler handle it.
        Ok(Self::parse_path(spanned_token))
    }

    fn check_float(number_str: &str, span: Span) -> syn::Result<()> {
        if !number_str.starts_with(&IntegerPrefix::Hex.prefix()) {
            let contains_float_identifier =
                FLOAT_IDENTIFIERS.iter().any(|&identifier| number_str.ends_with(identifier));
            if number_str.contains(FLOAT_DOT) || contains_float_identifier {
                return Err(create_user_parsing_compiler_error(
                    span,
                    "Float are not supported.".to_string(),
                ));
            }
        }

        Ok(())
    }

    fn is_boolean(number_str: &str) -> bool {
        BOOLEAN_IDENTIFIERS.contains(&number_str)
    }

    fn parse_boolean(number_str: &str, spanned_token: &SpannedToken) -> Self {
        match number_str {
            "true" => Self::Literal {
                number: 1,
                negative_sign: false,
                spanned_token: spanned_token.clone(),
            },
            "false" => Self::Literal {
                number: 0,
                negative_sign: false,
                spanned_token: spanned_token.clone(),
            },
            _ => unreachable!("We've already made sure the type is a Boolean"),
        }
    }

    fn parse_integer(number_str: &str, spanned_token: &SpannedToken) -> syn::Result<Self> {
        let stripped_suffix_number_str = Self::strip_integer_suffix(number_str);
        let negative_number = Self::is_negative(&stripped_suffix_number_str);
        let stripped_negative_number_str = Self::strip_negative_sign(&stripped_suffix_number_str);
        let parsed_number =
            Self::parse_integer_helper(&stripped_negative_number_str, spanned_token)?;

        Ok(Self::Literal {
            number: parsed_number,
            negative_sign: negative_number,
            spanned_token: spanned_token.clone(),
        })
    }

    fn strip_integer_suffix(number_str: &str) -> String {
        number_str
            .strip_suffix(
                INTEGER_IDENTIFIERS
                    .iter()
                    .find(|&&identifier| number_str.ends_with(identifier))
                    .copied()
                    .unwrap_or(""),
            )
            .unwrap_or(number_str)
            .to_string()
    }

    fn is_negative(number_str: &str) -> bool {
        number_str.starts_with(NEGATIVE_SIGN)
    }

    fn strip_negative_sign(number_str: &str) -> String {
        number_str.trim_start_matches(NEGATIVE_SIGN).to_string()
    }

    fn parse_integer_helper(number_str: &str, spanned_token: &SpannedToken) -> syn::Result<u128> {
        if number_str.starts_with(&IntegerPrefix::Hex.prefix()) {
            return match u128::from_str_radix(
                number_str.trim_start_matches(&IntegerPrefix::Hex.prefix()),
                IntegerPrefix::Hex.radix(),
            ) {
                Ok(num) => Ok(num),
                Err(err) => unreachable!("Hex parsing should succeed: {:?}", err),
            };
        }

        if number_str.starts_with(&IntegerPrefix::Binary.prefix()) {
            return match u128::from_str_radix(
                number_str.trim_start_matches(&IntegerPrefix::Binary.prefix()),
                IntegerPrefix::Binary.radix(),
            ) {
                Ok(num) => Ok(num),
                Err(err) => unreachable!("Binary parsing should succeed: {:?}", err),
            };
        }

        if number_str.starts_with(&IntegerPrefix::Octal.prefix()) {
            return match u128::from_str_radix(
                number_str.trim_start_matches(&IntegerPrefix::Octal.prefix()),
                IntegerPrefix::Octal.radix(),
            ) {
                Ok(num) => Ok(num),
                Err(err) => unreachable!("Octal parsing should succeed: {:?}", err),
            };
        }

        // Regular number
        match number_str.parse::<u128>() {
            Ok(num) => Ok(num),
            Err(err) => Err(create_user_parsing_compiler_error(
                spanned_token.span(),
                format!("Failed to parse integer literal: {err}"),
            )),
        }
    }

    fn parse_path(spanned_token: &SpannedToken) -> Self {
        Self::Path {
            spanned_token: spanned_token.clone(),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Literal {
                spanned_token, ..
            }
            | Self::Path {
                spanned_token,
            } => spanned_token.span(),
        }
    }
}

impl ToTokens for ConstExpr {
    fn to_tokens(&self) -> TokenStream {
        match self {
            Self::Literal {
                spanned_token, ..
            } => {
                let expr: Expr = syn::parse_str(&spanned_token.token())
                    .expect("Failed to parse previously accepted literal expression");
                quote! { #expr }
            },
            Self::Path {
                spanned_token,
            } => {
                let expr: Expr = syn::parse_str(&spanned_token.token())
                    .expect("Failed to parse previously accepted path expression");
                quote! { #expr }
            },
        }
    }
}
