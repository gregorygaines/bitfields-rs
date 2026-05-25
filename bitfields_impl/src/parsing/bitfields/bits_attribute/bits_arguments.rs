use std::collections::HashSet;
use std::str::FromStr;

use getset::{CloneGetters, CopyGetters, Getters};
use proc_macro2::Span;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use syn::parse::{Parse, ParseStream};

use crate::parsing::common::attribute_argument_parser::{
    parse_attribute_arguments, parse_boolean_attribute_argument,
};
use crate::parsing::common::compiler_error::create_user_parsing_compiler_error;
use crate::parsing::common::const_expr::ConstExpr;

/// Represents the access of a field.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldAccess {
    /// The field is read-only.
    ReadOnly,

    /// The field is write-only.
    WriteOnly,

    /// The field is read-write.
    ReadWrite,

    /// The field is inaccessible.
    NoAccess,
}

impl FromStr for FieldAccess {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "ro" => Ok(Self::ReadOnly),
            "wo" => Ok(Self::WriteOnly),
            "rw" => Ok(Self::ReadWrite),
            "na" => Ok(Self::NoAccess),
            _ => Err(format!(
                "Invalid field access argument '{s}'. Valid values are 'ro', 'wo', 'rw', or 'na'."
            )),
        }
    }
}

/// Represents the arguments of the `#[bits]` attribute.
#[derive(Clone, Debug, Getters, CopyGetters, CloneGetters)]
pub struct BitsArguments {
    /// The access of the field.
    #[getset(get_copy = "pub")]
    access: FieldAccess,

    #[getset(get_copy = "pub")]
    access_span: Option<Span>,

    #[getset(get_copy = "pub")]
    user_set_access: bool,

    /// Whether the field is ignored.
    #[getset(get_copy = "pub")]
    ignored: bool,

    /// The field default value expression.
    #[getset(get_clone = "pub")]
    default_value_expr: Option<ConstExpr>,
}

impl Default for BitsArguments {
    fn default() -> Self {
        Self {
            access: FieldAccess::ReadWrite,
            access_span: None,
            user_set_access: false,
            ignored: false,
            default_value_expr: None,
        }
    }
}

#[derive(Display, EnumString, EnumIter)]
enum BitsArgumentKey {
    #[strum(serialize = "access")]
    Access,

    #[strum(serialize = "ignore")]
    Ignore,

    #[strum(serialize = "default")]
    Default,
}

impl Parse for BitsArguments {
    /// Parses bits attribute arguments from the given input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let valid_keys = BitsArgumentKey::iter().map(|k| k.to_string()).collect();
        let attribute_arguments = parse_attribute_arguments(
            input,
            valid_keys,
            /* internal_keys= */ HashSet::default(),
        )?;
        let mut bits_arguments = Self::default();

        for argument in attribute_arguments {
            match BitsArgumentKey::from_str(argument.key().token().as_str())
                .expect("This should be caught by the known keys check")
            {
                BitsArgumentKey::Access => {
                    bits_arguments.access =
                        FieldAccess::from_str(argument.value().token().as_str()).map_err(
                            |err| create_user_parsing_compiler_error(argument.value().span(), err),
                        )?;
                    bits_arguments.user_set_access = true;
                    bits_arguments.access_span = Some(argument.value().span());
                },
                BitsArgumentKey::Ignore => {
                    bits_arguments.ignored = parse_boolean_attribute_argument(argument)?;
                },
                BitsArgumentKey::Default => {
                    bits_arguments.default_value_expr = Some(ConstExpr::new(&argument.value())?);
                },
            }
        }

        Ok(bits_arguments)
    }
}
