use std::str::FromStr;

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
    pub(crate) bits: u8,
    /// The order of the bits in the bitfield.
    pub(crate) bit_order: BitOrder,
    /// The endianness of the [`from_bits`] input.
    pub(crate) from_endian: Endian,
    /// The endianness of the [`into_bits`] output.
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
            _ => Err(()),
        }
    }
}

impl Parse for BitfieldAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Get the type of the bitfield.
        let ty = match input.parse() {
            Ok(ty) => ty,
            Err(err) => {
                return Err(create_syn_error(
                    input.span(),
                    format!(
                        "Failed to parse the 'bitfield' attribute field type: {}. The 'bitfield' attribute must have an unsigned integer type as its first argument.",
                        err
                    ),
                ));
            }
        };

        if !is_supported_bitfield_type(&ty) {
            return Err(create_syn_error(
                input.span(),
                "The 'bitfield' attribute must have an unsigned integer type as its first argument.",
            ));
        }

        let bits = get_bits_from_type(&ty)?;

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
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!("Failed to parse the 'order' arg '{}'.", err),
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
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'from_endian' arg '{}', must be either 'little' or 'big'.",
                                    err
                                ),
                            ));
                        }
                    };
                    from_endian = match Endian::from_str(&from_endian_str) {
                        Ok(endian) => endian,
                        Err(_) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Invalid endian: '{}', must be either 'little' or 'big'.",
                                    from_endian_str
                                ),
                            ));
                        }
                    };
                }
                // Doesn't apply to user passed functions.
                "into_endian" => {
                    let into_endian_str = match input.parse::<syn::Ident>() {
                        Ok(into_endian) => into_endian.to_string(),
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'into_endian' arg '{}', must be either 'little' or 'big'.",
                                    err
                                ),
                            ));
                        }
                    };
                    into_endian = match Endian::from_str(&into_endian_str) {
                        Ok(endian) => endian,
                        Err(_) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Invalid endian: '{}', must be either 'little' or 'big'.",
                                    into_endian_str
                                ),
                            ));
                        }
                    };
                }
                "new" => {
                    generate_new_func = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'new' arg '{}', must be either 'true' or 'false'.",
                                    err
                                ),
                            ));
                        }
                    };
                }
                "into_bits" => {
                    generate_into_bits_func = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'into_bits' arg '{}', must be either 'true' or 'false'.",
                                    err
                                ),
                            ));
                        }
                    };
                }
                "from_bits" => {
                    generate_from_bits_func = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'from_bits' arg '{}', must be either 'true' or 'false'.",
                                    err
                                ),
                            ));
                        }
                    };
                }
                "from" => {
                    generate_from_trait_funcs = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'from_type' arg '{}', must be either 'true' or 'false'.",
                                    err
                                ),
                            ));
                        }
                    }
                }
                "default" => {
                    generate_default_impl = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'default' arg '{}', must be either 'true' or 'false'.",
                                    err
                                ),
                            ));
                        }
                    };
                }
                "builder" => {
                    generate_builder = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'builder' arg '{}', must be either 'true' or 'false'.",
                                    err
                                ),
                            ));
                        }
                    }
                }
                "debug" => {
                    generate_debug_impl = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'debug' arg '{}', must be either 'true', or 'false'.",
                                    err
                                ),
                            ));
                        }
                    };
                }
                "bit_ops" => {
                    generate_bit_ops = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'big_ops' arg '{}', must be either 'true', or 'false'.",
                                    err
                                ),
                            ));
                        }
                    };
                }
                "set_bits" => {
                    generate_set_bits_impl = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'set_bits' arg '{}', must be either 'true', or 'false'.",
                                    err
                                ),
                            ));
                        }
                    };
                }
                "clear_bits" => {
                    generate_clear_bits_impl = match input.parse::<syn::LitBool>() {
                        Ok(val) => val.value,
                        Err(err) => {
                            return Err(create_syn_error(
                                input.span(),
                                format!(
                                    "Failed to parse the 'set_bits' arg '{}', must be either 'true', or 'false'.",
                                    err
                                ),
                            ));
                        }
                    };
                }
                _ => {
                    return Err(create_syn_error(
                        arg_ident.span(),
                        format!("Unknown 'bitfield' argument: {}", arg_ident),
                    ));
                }
            }

            // Move to the next argument.
            if !input.is_empty() {
                <Token![,]>::parse(input)?;
            }
        }

        Ok(Self {
            ty,
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
        })
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
        assert!(args.err().unwrap().to_string().contains(
            "The 'bitfield' attribute must have an unsigned integer type as its first argument"
        ));
    }

    #[test]
    fn test_parse_bitfield_invalid_type_arg_returns_error() {
        let args = quote!(eff);
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_err());
        assert!(args.err().unwrap().to_string().contains(
            "The 'bitfield' attribute must have an unsigned integer type as its first argument."
        ));

        let args = quote!(i32);
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_err());
        assert!(args.err().unwrap().to_string().contains(
            "The 'bitfield' attribute must have an unsigned integer type as its first argument."
        ));
    }

    #[test]
    fn test_parse_bitfield_unsigned_integer_types() {
        let args = quote!(u8);
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().bits, 8);

        let args = quote!(u16);
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().bits, 16);

        let args = quote!(u32);
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().bits, 32);

        let args = quote!(u64);
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().bits, 64);

        let args = quote!(u128);
        let args = syn::parse2::<BitfieldAttribute>(args);
        assert!(args.is_ok());
        assert_eq!(args.unwrap().bits, 128);
    }
}
