use proc_macro2::TokenStream;
use quote::quote;

use crate::parsing::common::conversion_endian::ConversionEndian;

/// Generates bits variable endian conversion tokens depending on the
/// configured endian.
pub fn generate_bits_variable_endian_conversion_tokens(endian: ConversionEndian) -> TokenStream {
    match endian {
        ConversionEndian::Little => {
            quote! {
                let bits = bits.swap_bytes();
            }
        },
        ConversionEndian::Big => {
            quote! {}
        },
    }
}
