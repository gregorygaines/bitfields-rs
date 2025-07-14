use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::Token;
use syn::parse::{Parse, ParseStream};

use crate::create_syn_error;

/// Represents a field in a bitfield struct.
#[derive(Clone)]
pub(crate) struct BitfieldField {
    /// The name of the field.
    pub(crate) name: syn::Ident,
    /// The type of the field.
    pub(crate) ty: syn::Type,
    /// The visibility of the field.
    pub(crate) vis: Option<syn::Visibility>,
    /// The number of bits the field occupies.
    pub(crate) bits: u8,
    /// The offset of the field.
    pub(crate) offset: u8,
    /// The default value of the field as a token stream. This allows us to
    /// insert the default value in any token stream without complicated
    /// conversions or parsing.
    pub(crate) default_value_tokens: Option<TokenStream>,
    /// Whether the field is unsigned.
    pub(crate) unsigned: bool,
    /// Whether the field is padding.
    pub(crate) padding: bool,
    /// The access of the field.
    pub(crate) access: FieldAccess,
    /// The type of the field.
    pub(crate) field_type: FieldType,
    /// Whether the field is ignored.
    pub(crate) ignore: bool,
}

/// Represents the type of field.
#[derive(Clone, PartialEq)]
pub(crate) enum FieldType {
    IntegerFieldType,
    CustomFieldType,
}

/// Represents the `#[bits]` attribute.
#[derive(Clone)]
pub(crate) struct BitsAttribute {
    pub(crate) bits: Option<u8>,
    pub(crate) default_value_expr: Option<syn::Expr>,
    pub(crate) access: Option<FieldAccess>,
    pub(crate) ignore: bool,
}

/// Represents the access of a field.
#[derive(Clone, PartialEq)]
pub(crate) enum FieldAccess {
    /// The field is read-only.
    ReadOnly,
    /// The field is write-only.
    WriteOnly,
    /// The field is read-write.
    ReadWrite,
    /// The field is inaccessible.
    None,
}

impl FromStr for FieldAccess {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "ro" => Ok(FieldAccess::ReadOnly),
            "wo" => Ok(FieldAccess::WriteOnly),
            "rw" => Ok(FieldAccess::ReadWrite),
            "none" => Ok(FieldAccess::None),
            _ => Err(()),
        }
    }
}

impl Parse for BitsAttribute {
    /// Parser a field with the `#[bits]` attribute.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut bitfield_attribute_args =
            BitsAttribute { bits: None, default_value_expr: None, access: None, ignore: false };

        // First check for the number of bits.
        if input.peek(syn::LitInt) {
            let num_bits = match input.parse::<syn::LitInt>() {
                Ok(lit_int) => lit_int.base10_parse::<u8>()?,
                Err(_) => {
                    return Err(syn::Error::new(input.span(), "Unable to parse field bits"));
                }
            };

            // Move to the next argument.
            if !input.is_empty() {
                <Token![,]>::parse(input)?;
            }

            bitfield_attribute_args.bits = Some(num_bits)
        };

        // Parse the remaining arguments which is in the form of `[key] = [val]`.
        while !input.is_empty() {
            // Parse the argument key.
            let arg_ident = syn::Ident::parse(input)?;

            // Move to the key part of the argument.
            <Token![=]>::parse(input)?;

            match arg_ident.to_string().as_str() {
                "default" => {
                    let default_value_tokens = match input.parse::<syn::Expr>() {
                        Ok(expr) => expr,
                        Err(_) => {
                            return Err(syn::Error::new(
                                input.span(),
                                "Unable to parse default value",
                            ));
                        }
                    };
                    bitfield_attribute_args.default_value_expr = Some(default_value_tokens);
                }
                "access" => {
                    let access_str = match input.parse::<syn::Ident>() {
                        Ok(ident) => ident.to_string(),
                        Err(_) => {
                            return Err(syn::Error::new(
                                input.span(),
                                "Unable to parse access value, expected 'ro', 'wo', 'rw', or 'none'",
                            ));
                        }
                    };
                    let access = match FieldAccess::from_str(&access_str) {
                        Ok(access) => access,
                        Err(_) => {
                            return Err(syn::Error::new(
                                input.span(),
                                "Unable to parse access value, expected 'ro', 'wo', 'rw', or 'none'",
                            ));
                        }
                    };
                    bitfield_attribute_args.access = Some(access);
                }
                "ignore" => {
                    bitfield_attribute_args.ignore = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'ignore' arg '{}', must be either 'true' or 'false'.",
                                    err
                                ),
                            ));
                        }
                    };
                }
                _ => {}
            }

            // Move to the next argument.
            if !input.is_empty() {
                <Token![,]>::parse(input)?;
            }
        }

        Ok(bitfield_attribute_args)
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn parse_bits_attributes() {
        let args = quote!(8);
        let args = syn::parse2::<BitsAttribute>(args).unwrap();

        assert_eq!(args.bits, Some(8));
    }

    #[test]
    fn parse_bits_attributes_no_bits() {
        let args = quote!();
        let args = syn::parse2::<BitsAttribute>(args).unwrap();

        assert_eq!(args.bits, None);
    }
}
