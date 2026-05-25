use std::fmt::Display;

/// Creates a user parsing error compiling pointing to the invalid input.
pub fn create_user_parsing_compiler_error(
    invalid_span: proc_macro2::Span,
    msg: impl Display,
) -> syn::Error {
    syn::Error::new(invalid_span, msg)
}
