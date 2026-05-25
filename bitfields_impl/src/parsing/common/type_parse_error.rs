use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum TypeParsingError {
    #[error("a non type was passed where a type was expected: {0}")]
    NonType(String),

    #[error("unexpected end of input while parsing type")]
    UnexpectedEndOfInput,

    #[error("isize/usize types are not supported, we cannot guarantee this size at runtime")]
    SizeTypeNotSupported,

    #[error("Floats are not supported in the bitfield")]
    UnexpectedFloat,

    #[error("Array types must be an `u8` integer")]
    NonIntegerArrayType,

    #[error("Array length must greater than 0")]
    ZeroArrayLength,

    #[error("unexpected parsing error: {0}")]
    Unexpected(String),
}
