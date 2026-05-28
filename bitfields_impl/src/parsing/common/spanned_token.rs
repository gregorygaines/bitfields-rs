use getset::{CloneGetters, Getters};
use proc_macro2::Span;

/// Represents a token and its span for error reporting.
#[derive(Clone, Debug, Getters, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct SpannedToken {
    token: String,
    span: Span,
}

impl SpannedToken {
    /// Creates a new [`SpannedToken`] instance.
    pub const fn new(token: String, span: Span) -> Self {
        Self {
            token,
            span,
        }
    }
}
