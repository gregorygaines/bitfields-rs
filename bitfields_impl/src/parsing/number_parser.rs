use thiserror::Error;

const FLOAT_IDENTIFIERS: [&str; 2] = ["f32", "f64"];
const INTEGER_IDENTIFIERS: [&str; 10] =
    ["u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128"];
const BOOLEAN_IDENTIFIERS: [&str; 2] = ["true", "false"];
const NEGATIVE_SIGN: &str = "-";

const HEX_PREFIX: &str = "0x";
const HEX_RADIX: u32 = 16;

const BINARY_PREFIX: &str = "0b";
const BINARY_RADIX: u32 = 2;

const OCTAL_PREFIX: &str = "0o";
const OCTAL_RADIX: u32 = 8;

#[derive(Error, Debug)]
pub(crate) enum NumberParseError {
    #[error("floats are not supported")]
    FloatNotSupported,
    #[error("invalid integer or boolean")]
    InvalidNumberString,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct ParsedNumber {
    pub(crate) number: u128,
    pub(crate) negative: bool,
    pub(crate) has_integer_suffix: bool,
}

/// Parses a number string.
pub(crate) fn parse_number_string(s: &str) -> Result<ParsedNumber, NumberParseError> {
    let trimmed_str = s.trim().replace(" ", "").replace("_", "").to_ascii_lowercase();

    // Check if the string is a float.
    if !trimmed_str.starts_with(HEX_PREFIX) {
        let contains_float_identifier =
            FLOAT_IDENTIFIERS.iter().any(|&identifier| trimmed_str.ends_with(identifier));
        if trimmed_str.contains(".") || contains_float_identifier {
            return Err(NumberParseError::FloatNotSupported);
        }
    }

    // Check if the string is a boolean.
    if BOOLEAN_IDENTIFIERS.contains(&trimmed_str.as_str()) {
        return match trimmed_str.as_str() {
            "true" => Ok(ParsedNumber { number: 1, negative: false, has_integer_suffix: false }),
            "false" => Ok(ParsedNumber { number: 0, negative: false, has_integer_suffix: false }),
            _ => Err(NumberParseError::InvalidNumberString),
        };
    }

    let has_integer_suffix =
        INTEGER_IDENTIFIERS.iter().any(|&identifier| trimmed_str.ends_with(identifier));

    // Remove integer identifiers.
    let trimmed_str = INTEGER_IDENTIFIERS
        .iter()
        .fold(trimmed_str, |acc, &identifier| acc.replace(identifier, ""));

    let negative_number = trimmed_str.starts_with(NEGATIVE_SIGN);
    let trimmed_str = trimmed_str.trim_start_matches(NEGATIVE_SIGN);

    // Check if the string is a hexadecimal number.
    if trimmed_str.starts_with(HEX_PREFIX) {
        Ok(ParsedNumber {
            number: u128::from_str_radix(trimmed_str.trim_start_matches(HEX_PREFIX), HEX_RADIX)
                .unwrap(),
            negative: negative_number,
            has_integer_suffix,
        })
    } else if trimmed_str.starts_with(BINARY_PREFIX) {
        Ok(ParsedNumber {
            number: u128::from_str_radix(
                trimmed_str.trim_start_matches(BINARY_PREFIX),
                BINARY_RADIX,
            )
            .unwrap(),
            negative: negative_number,
            has_integer_suffix,
        })
    } else if trimmed_str.starts_with(OCTAL_PREFIX) {
        Ok(ParsedNumber {
            number: u128::from_str_radix(trimmed_str.trim_start_matches(OCTAL_PREFIX), OCTAL_RADIX)
                .unwrap(),
            negative: negative_number,
            has_integer_suffix,
        })
    } else {
        // Regular number
        match trimmed_str.parse::<u128>() {
            Ok(number) => {
                Ok(ParsedNumber { number, negative: negative_number, has_integer_suffix })
            }
            Err(_) => Err(NumberParseError::InvalidNumberString),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal_string() {
        let integer_type = parse_number_string("12").unwrap();

        assert_eq!(
            integer_type,
            ParsedNumber { number: 12, negative: false, has_integer_suffix: false }
        );
    }

    #[test]
    fn test_parse_negative_decimal_string() {
        let integer_type = parse_number_string("-12").unwrap();

        assert_eq!(
            integer_type,
            ParsedNumber { number: 12, negative: true, has_integer_suffix: false }
        );
    }

    #[test]
    fn test_parse_decimal_integer_suffix_string() {
        let integer_type = parse_number_string("-12i8").unwrap();

        assert_eq!(
            integer_type,
            ParsedNumber { number: 12, negative: true, has_integer_suffix: true }
        );
    }

    #[test]
    fn test_parse_hex_string() {
        let integer_type = parse_number_string("0x1234").unwrap();

        assert_eq!(
            integer_type,
            ParsedNumber { number: 0x1234, negative: false, has_integer_suffix: false }
        );
    }

    #[test]
    fn test_parse_octal_string() {
        let integer_type = parse_number_string("0o12").unwrap();

        assert_eq!(
            integer_type,
            ParsedNumber { number: 10, negative: false, has_integer_suffix: false }
        );
    }
}
