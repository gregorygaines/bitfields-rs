use std::str::FromStr;

use proc_macro2::TokenTree;
use syn::Token;
use syn::parse::{Parse, ParseStream};

use crate::create_syn_error;
use crate::parsing::types::{get_bits_from_type, is_supported_bitfield_type};

/// Represents the `#[bitfield]` attribute.
#[derive(Clone)]
pub(crate) struct BitfieldAttribute {
    /// The integer type of the bitfield.
    pub(crate) ty: syn::Type,
    /// The number of bits of the bitfield.
    pub(crate) bits: u32,
    /// The order of the bits in the bitfield.
    pub(crate) bit_order: BitOrder,
    /// The endianness of the `from_bits` input.
    pub(crate) from_endian: Endian,
    /// The endianness of the `into_bits` output.
    pub(crate) into_endian: Endian,
    /// Whether to generate the `new` or `new_without_defaults` function.
    pub(crate) generate_new_func: bool,
    /// Whether to generate the `into_bits` function.
    pub(crate) generate_into_bits_func: bool,
    /// Whether to generate the `from_bits` function.
    pub(crate) generate_from_bits_func: bool,
    /// Whether to generate the from trait functions.
    pub(crate) generate_from_trait_funcs: bool,
    /// Whether to generate the `default` implementation.
    pub(crate) generate_default_impl: bool,
    /// Whether to generate the builder.
    pub(crate) generate_builder: bool,
    /// Whether to generate the `debug_impl` function.
    pub(crate) generate_debug_impl: bool,
    /// Whether to generate the bit operations.
    pub(crate) generate_bit_ops: bool,
    /// Whether to generate the set bits implementation.
    pub(crate) generate_set_bits_impl: bool,
    /// Whether to generate the clear bits implementation.
    pub(crate) generate_clear_bits_impl: bool,
    /// Whether to generate the `to_builder` function.
    pub(crate) generate_to_builder: bool,
    /// Whether to generate the `neg` function.
    pub(crate) generate_neg_func: bool,
}

/// The order of the bits in the bitfield.
///
/// If the order is `Lsb`, the top most field will be the least significant bit
/// and the bottom most field is the most significant bit. If the order is
/// `Msb`, the top most field will be the most significant bit and the bottom
/// most field is the least significant bit.
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum BitOrder {
    Lsb,
    Msb,
}

impl FromStr for BitOrder {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "lsb" => Ok(BitOrder::Lsb),
            "msb" => Ok(BitOrder::Msb),
            // unreachable
            _ => Err(()),
        }
    }
}

/// The endianness of the bytes.
#[derive(Clone, PartialEq)]
pub(crate) enum Endian {
    Little,
    Big,
}

impl FromStr for Endian {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            // unreachable
            _ => Err(()),
        }
    }
}

/// Represents a parsed bitfield type.
struct ParsedBitfieldType {
    /// The parsed type of the bitfield.
    ty: syn::Type,
}

impl Parse for BitfieldAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed_bitfield_type = Self::parse_bitfield_type(input)?;

        Self::check_supported_bitfield_type(input, &parsed_bitfield_type.ty)?;
        let bits = get_bits_from_type(&parsed_bitfield_type.ty)?;

        // Move to the next argument.
        if !input.is_empty() {
            <Token![,]>::parse(input)?;
        }

        let mut bit_order = BitOrder::Lsb;
        let mut from_endian = Endian::Big;
        let mut into_endian = Endian::Big;
        let mut generate_new_func = true;
        let mut generate_into_bits_func = true;
        let mut generate_from_bits_func = true;
        let mut generate_from_trait_funcs = true;
        let mut generate_default_impl = true;
        let mut generate_builder = true;
        let mut generate_debug_impl = true;
        let mut generate_bit_ops = false;
        let mut generate_set_bits_impl = true;
        let mut generate_clear_bits_impl = true;
        let mut generate_to_builder = false;
        let mut generate_neg_func = false;

        // Parse the remaining arguments which is in the form of `[key] = [val]`.
        while !input.is_empty() {
            // Parse the argument key.
            let arg_ident = syn::Ident::parse(input)?;

            // Move to the key part of the argument.
            <Token![=]>::parse(input)?;

            match arg_ident.to_string().as_str() {
                "order" => {
                    let order_str = match input.parse::<syn::Ident>() {
                        Ok(order) => order.to_string(),
                        Err(_) => {
                            let current_token = Self::parse_current_stream_token_ident(input)?;
                            return Err(create_syn_error(
                                input.span(),
                                format!("Failed to parse the 'order' arg '{}'.", current_token),
                            ));
                        }
                    };
                    bit_order = match BitOrder::from_str(&order_str) {
                        Ok(order) => order,
                        Err(_) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Invalid order: '{}', must be either 'lsb' or 'msb'.",
                                    order_str
                                ),
                            ));
                        }
                    };
                }
                // Doesn't apply to user passed functions.
                "from_endian" => {
                    let from_endian_str = match input.parse::<syn::Ident>() {
                        Ok(from_endian) => from_endian.to_string(),
                        Err(_) => {
                            let current_token = Self::parse_current_stream_token_ident(input)?;
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'from_endian' arg '{}', must be either 'little' or 'big'.",
                                    current_token
                                ),
                            ));
                        }
                    };
                    from_endian = Self::parse_endian_attribute(input, &from_endian_str)?
                }
                // Doesn't apply to user passed functions.
                "into_endian" => {
                    let into_endian_str = match input.parse::<syn::Ident>() {
                        Ok(into_endian) => into_endian.to_string(),
                        Err(_) => {
                            let current_token = Self::parse_current_stream_token_ident(input)?;
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'into_endian' arg '{}', must be either 'little' or 'big'.",
                                    current_token
                                ),
                            ));
                        }
                    };
                    into_endian = Self::parse_endian_attribute(input, &into_endian_str)?
                }
                "new" => generate_new_func = Self::parse_boolean_literal(input, "new")?,
                "into_bits" => {
                    generate_into_bits_func = Self::parse_boolean_literal(input, "into_bits")?
                }
                "from_bits" => {
                    generate_from_bits_func = Self::parse_boolean_literal(input, "from_bits")?
                }
                "from" => {
                    generate_from_trait_funcs = Self::parse_boolean_literal(input, "from_type")?
                }
                "default" => generate_default_impl = Self::parse_boolean_literal(input, "default")?,
                "builder" => generate_builder = Self::parse_boolean_literal(input, "builder")?,
                "to_builder" => {
                    generate_to_builder = Self::parse_boolean_literal(input, "builder")?
                }
                "debug" => generate_debug_impl = Self::parse_boolean_literal(input, "debug")?,
                "bit_ops" => generate_bit_ops = Self::parse_boolean_literal(input, "bit_ops")?,
                "set_bits" => {
                    generate_set_bits_impl = Self::parse_boolean_literal(input, "set_bits")?
                }
                "clear_bits" => {
                    generate_clear_bits_impl = Self::parse_boolean_literal(input, "clear_bits")?
                }
                "neg" => generate_neg_func = Self::parse_boolean_literal(input, "neg")?,
                _ => {
                    return Err(create_syn_error(
                        arg_ident.span(),
                        format!("Unknown bitfield argument: {}", arg_ident),
                    ));
                }
            }

            // Move to the next argument.
            if !input.is_empty() {
                <Token![,]>::parse(input)?;
            }
        }

        Ok(Self {
            ty: parsed_bitfield_type.ty,
            bits,
            bit_order,
            from_endian,
            into_endian,
            generate_new_func,
            generate_into_bits_func,
            generate_from_bits_func,
            generate_default_impl,
            generate_debug_impl,
            generate_builder,
            generate_from_trait_funcs,
            generate_bit_ops,
            generate_set_bits_impl,
            generate_clear_bits_impl,
            generate_to_builder,
            generate_neg_func,
        })
    }
}

impl BitfieldAttribute {
    fn parse_bitfield_type(input: ParseStream) -> syn::Result<ParsedBitfieldType> {
        Self::parse_integer_backed_bitfield_type(input)
    }

    fn parse_integer_backed_bitfield_type(input: ParseStream) -> syn::Result<ParsedBitfieldType> {
        match Self::parse_type(input) {
            Ok(parsed_type) => Ok(ParsedBitfieldType { ty: parsed_type }),
            Err(_) => Err(create_syn_error(
                input.span(),
                "Failed to parse the bitfield attribute type. The bitfield attribute must have an unsigned integer as its first argument.",
            )),
        }
    }

    fn parse_type(input: ParseStream) -> syn::Result<syn::Type> {
        match input.parse() {
            Ok(ty) => Ok(ty),
            Err(err) => Err(err),
        }
    }

    fn check_supported_bitfield_type(input: ParseStream, ty: &syn::Type) -> syn::Result<()> {
        let ident = match ty {
            syn::Type::Path(type_path) => {
                type_path.path.segments.last().map(|s| s.ident.to_string()).unwrap()
            }
            _ => {
                return Err(create_syn_error(
                    input.span(),
                    "The bitfield attribute must have an unsigned integer as its first argument, non-path types are unsupported.",
                ));
            }
        };

        if !is_supported_bitfield_type(ty) {
            return Err(create_syn_error(
                input.span(),
                format!(
                    "The bitfield attribute must have an unsigned integer as its first argument, '{}' is unsupported.",
                    ident
                ),
            ));
        }
        Ok(())
    }

    fn parse_boolean_literal(input: ParseStream, arg_name: &str) -> syn::Result<bool> {
        match input.parse::<syn::LitBool>() {
            Ok(val) => Ok(val.value),
            Err(_) => {
                let current_token = Self::parse_current_stream_token_ident(input)?;
                Err(create_syn_error(
                    input.span(),
                    format!(
                        "Failed to parse the '{}' arg '{}', must be either 'true' or 'false'.",
                        arg_name, current_token
                    ),
                ))
            }
        }
    }

    fn parse_current_stream_token_ident(input: ParseStream) -> syn::Result<String> {
        let token: TokenTree = input.parse()?;
        Ok(match token {
            TokenTree::Ident(ident) => ident.to_string(),
            _ => token.to_string(),
        })
    }

    fn parse_endian_attribute(input: ParseStream, endian_string: &str) -> syn::Result<Endian> {
        match Endian::from_str(endian_string) {
            Ok(endian) => Ok(endian),
            Err(_) => Err(create_syn_error(
                input.span(),
                format!("Invalid endian '{}', must be either 'little' or 'big'.", endian_string),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn test_parse_bitfield_no_args_returns_error() {
        let args = quote!();
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_err());
        assert_eq!(
            args.err().unwrap().to_string(),
            "Failed to parse the bitfield attribute type. The bitfield attribute must have an unsigned integer as its first argument."
        );
    }

    #[test]
    fn test_parse_bitfield_invalid_type_arg_returns_error() {
        let args = quote!(eff);
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_err());
        assert_eq!(
            args.err().unwrap().to_string(),
            "The bitfield attribute must have an unsigned integer as its first argument, 'eff' is unsupported."
        );

        let args = quote!(i32);
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_err());
        assert_eq!(
            args.err().unwrap().to_string(),
            "The bitfield attribute must have an unsigned integer as its first argument, 'i32' is unsupported."
        );
    }

    #[test]
    fn test_parse_bitfield_unsigned_integer_types_returns_args() {
        let tokens = quote!(u8);
        let args = syn::parse2::<BitfieldAttribute>(tokens);
        assert!(args.is_ok());
        assert_eq!(args.clone().unwrap().bits, 8);

        let tokens = quote!(u16);
        let args = syn::parse2::<BitfieldAttribute>(tokens);
        assert!(args.is_ok());
        assert_eq!(args.clone().unwrap().bits, 16);

        let tokens = quote!(u32);
        let args = syn::parse2::<BitfieldAttribute>(tokens);
        assert!(args.is_ok());
        assert_eq!(args.clone().unwrap().bits, 32);

        let tokens = quote!(u64);
        let args = syn::parse2::<BitfieldAttribute>(tokens);
        assert!(args.is_ok());
        assert_eq!(args.clone().unwrap().bits, 64);

        let tokens = quote!(u128);
        let args = syn::parse2::<BitfieldAttribute>(tokens);
        assert!(args.is_ok());
        assert_eq!(args.clone().unwrap().bits, 128);
    }
}
