use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Type, Visibility};

/// Generates the tokens for the bitfield tuple struct.
pub(crate) fn generate_tuple_struct_tokens(name: Ident, vis: Visibility, ty: Type) -> TokenStream {
    let documentation = "Represents a bitfield.";
    quote! {
        #[doc = #documentation]
        #vis struct #name (#ty);
    }
}
