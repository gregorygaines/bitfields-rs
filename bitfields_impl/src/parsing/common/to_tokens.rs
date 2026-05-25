use proc_macro2::TokenStream;

/// Converts `self` into token stream.
pub trait ToTokens {
    /// Returns `self` as a token stream.
    fn to_tokens(&self) -> TokenStream;
}
