use proc_macro2::TokenStream;
use quote::quote;
use syn::Visibility;

use crate::generation::common::{
    BitfieldStructReferenceIdent, generate_bitfield_struct_initialization_tokens,
    generate_setting_fields_default_values_tokens, generate_setting_fields_to_zero_tokens,
    get_const_modifier_tokens,
};
use crate::parsing::bitfield_attribute::BitfieldAttribute;
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the `new` and `new_without_defaults` functions for the bitfield.
///
/// Output:
///
/// ```ignore
/// fn new() -> Self { ... }
/// fn new_without_defaults() -> Self { ... }
/// ```
pub(crate) fn generate_new_function_tokens(
    vis: &Visibility,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let has_ignored_fields = !ignored_fields.is_empty();
    let setting_fields_default_values_tokens = generate_setting_fields_default_values_tokens(
        &bitfield_attribute.ty,
        fields,
        &BitfieldStructReferenceIdent::SelfReference,
        has_ignored_fields,
    );
    let initialize_struct_tokens = generate_bitfield_struct_initialization_tokens(
        ignored_fields,
        &BitfieldStructReferenceIdent::SelfReference,
    );
    let setting_fields_to_zero_tokens = generate_setting_fields_to_zero_tokens(
        &bitfield_attribute.ty,
        fields,
        &BitfieldStructReferenceIdent::SelfReference,
        has_ignored_fields,
    );

    let const_modifier_tokens = (!has_ignored_fields).then(get_const_modifier_tokens);
    quote! {
        #[doc = "Creates a new bitfield instance."]
        #vis #const_modifier_tokens fn new() -> Self {
            let mut this = #initialize_struct_tokens;
            #setting_fields_default_values_tokens
            this
        }

        #[doc = "Creates a new bitfield instance without setting any default values."]
        #vis #const_modifier_tokens fn new_without_defaults() -> Self {
            let mut this = #initialize_struct_tokens;
            #setting_fields_to_zero_tokens
            this
        }
    }
}
