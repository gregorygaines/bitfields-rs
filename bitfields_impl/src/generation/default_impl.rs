use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::generation::common::{
    BitfieldStructReferenceIdent, generate_bitfield_struct_initialization_tokens,
    generate_setting_fields_default_values_tokens,
};
use crate::parsing::bitfield_attribute::BitfieldAttribute;
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the default implementation.
pub(crate) fn generate_default_implementation_tokens(
    bitfield_struct_name: &Ident,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let setting_fields_default_values_tokens = generate_setting_fields_default_values_tokens(
        &bitfield_attribute.ty,
        fields,
        &BitfieldStructReferenceIdent::SelfReference,
        !ignored_fields.is_empty(),
    );

    let initialize_struct_tokens = generate_bitfield_struct_initialization_tokens(
        ignored_fields,
        &BitfieldStructReferenceIdent::SelfReference,
    );

    quote! {
        impl core::default::Default for #bitfield_struct_name {
            fn default() -> Self {
                let mut this = #initialize_struct_tokens;
                #setting_fields_default_values_tokens
                this
            }
        }
    }
}
