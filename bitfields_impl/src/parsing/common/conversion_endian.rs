use std::str::FromStr;

/// The conversion endianness of data coming to or from the bitfield.
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum ConversionEndian {
    /// Little endian conversion.
    Little,

    /// Big endian conversion.
    Big,
}

impl FromStr for ConversionEndian {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "little" => Ok(Self::Little),
            "big" => Ok(Self::Big),
            _ => Err(format!("Invalid endian argument '{s}'. Valid values are 'little' or 'big'.")),
        }
    }
}

/// Returns `Little` if `little` is set, `Big` if `big` is set, or `default`
/// otherwise.
pub const fn resolve_endian_feature(
    little: bool,
    big: bool,
    default: ConversionEndian,
) -> ConversionEndian {
    if little && big {
        default
    } else if little {
        ConversionEndian::Little
    } else if big {
        ConversionEndian::Big
    } else {
        default
    }
}
