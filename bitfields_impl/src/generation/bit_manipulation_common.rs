use proc_macro2::TokenStream;
use quote::quote;

/// Return tokens that extract a value from a source value based on the given
/// bit offset and number of bits to extract. Optionally negates the source
/// value before extraction and casts the output value to a different type.
pub(crate) fn generate_extract_value_from_value_tokens(
    bitfield_type: &syn::Type,
    source_value_ident: TokenStream,
    num_bits_to_extract_offset_ident: TokenStream,
    bit_offset_ident: TokenStream,
    output_value_ident: TokenStream,
    type_to_cast_output_value_ident: Option<TokenStream>,
    negate_source_value: bool,
) -> TokenStream {
    let negate_source_value_token = negate_source_value.then(|| quote! { ! });
    let output_value = if type_to_cast_output_value_ident.is_some() {
        quote! {
            let #output_value_ident = (#negate_source_value_token(#source_value_ident >> #bit_offset_ident) & mask) as #type_to_cast_output_value_ident;
        }
    } else {
        quote! {
            let #output_value_ident = #negate_source_value_token(#source_value_ident >> #bit_offset_ident) & mask;
        }
    };

    quote! {
        let mask = #bitfield_type::MAX >> (#bitfield_type::BITS - #num_bits_to_extract_offset_ident);
        #output_value
    }
}

/// Returns tokens that mask bits.
pub(crate) fn generate_mask_implementation_tokens(
    bitfield_type: &syn::Type,
    num_bits_to_extract_offset_ident: TokenStream,
) -> TokenStream {
    quote! {
        let mask: #bitfield_type = #bitfield_type::MAX >> (#bitfield_type::BITS - #num_bits_to_extract_offset_ident);
    }
}

/// Returns implementation tokens to set a bit.
pub(crate) fn generate_set_bit_implementation_tokens(
    source_value_ident: TokenStream,
) -> TokenStream {
    quote! {
        #source_value_ident |= 1 << index;
    }
}

/// Returns implementation tokens to clear a bit.
pub(crate) fn generate_clear_bit_implementation_tokens(
    source_value_ident: TokenStream,
) -> TokenStream {
    quote! {
        #source_value_ident &= !(1 << index);
    }
}
