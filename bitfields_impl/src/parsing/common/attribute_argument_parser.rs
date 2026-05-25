use std::collections::HashSet;

use getset::CloneGetters;
use quote::quote;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{Ident, Token};

use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::spanned_token::SpannedToken;

/// Represents a key-value attribute argument.
#[derive(Debug, CloneGetters)]
pub struct AttributeArgument {
    #[getset(get_clone = "pub")]
    key: SpannedToken,

    #[getset(get_clone = "pub")]
    value: SpannedToken,
}

impl AttributeArgument {
    pub const fn new(key: SpannedToken, value: SpannedToken) -> Self {
        Self {
            key,
            value,
        }
    }
}

/// Parses arguments from an attribute.
pub fn parse_attribute_arguments(
    input: ParseStream,
    valid_keys: HashSet<String>,
    internal_keys: HashSet<String>,
) -> syn::Result<Vec<AttributeArgument>> {
    let mut arguments = Vec::new();

    // Consume the comma separating the type from the key=value arguments
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    }

    while !input.is_empty() {
        // Consume a leading comma separator (between arguments)
        if !arguments.is_empty() {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }

        // Parse key
        let key: Ident = match input.parse::<Ident>() {
            Ok(ident) => ident,
            Err(err) => {
                return Err(create_user_parsing_compiler_error(
                    err.span(),
                    "Invalid argument, expected an identifier",
                ));
            },
        };

        // Validate key
        if !valid_keys.contains(&key.to_string()) && !internal_keys.contains(&key.to_string()) {
            let keys = get_keys_list(valid_keys);
            return Err(create_user_parsing_compiler_error(
                key.span(),
                format!("Unknown argument '{key}'. Valid arguments are: {keys}."),
            ));
        }

        // Parse `=`
        let Ok(eq_token) = input.parse::<Token![=]>() else {
            return Err(create_user_parsing_compiler_error(
                key.span(),
                "Expected '=' after argument",
            ));
        };

        if input.is_empty() {
            return Err(create_user_parsing_compiler_error(
                eq_token.span(),
                "Expected value after '='",
            ));
        }

        // Parse value — accept an ident/path (e.g. `lsb` or `CustomType::A`) or
        // a literal (e.g. `true`, `42`)
        let expr: syn::Expr = match input.parse() {
            Ok(expr) => expr,
            Err(_) => {
                return Err(create_user_parsing_compiler_error(
                    eq_token.span(),
                    "Expected argument value",
                ));
            },
        };

        let value_span = expr.span();
        let value = quote!(#expr).to_string();

        arguments.push(AttributeArgument::new(
            SpannedToken::new(key.to_string(), key.span()),
            SpannedToken::new(value, value_span),
        ));
    }

    Ok(arguments)
}

/// Return a sorted, human-readable list of keys for error messages.
fn get_keys_list(keys: HashSet<String>) -> String {
    let mut keys = keys.iter().map(|k| format!("'{k}'")).collect::<Vec<_>>();
    keys.sort();
    keys.join(", ")
}

/// Parses a boolean attribute argument.
pub fn parse_boolean_attribute_argument(argument: AttributeArgument) -> syn::Result<bool> {
    argument.value().token().parse::<bool>().map_err(|_| {
        create_user_parsing_compiler_error(
            argument.value().span(),
            format!(
                "Invalid value for boolean argument '{}'. Valid values are 'true' or 'false'",
                argument.key().token(),
            ),
        )
    })
}
