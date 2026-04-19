use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Visibility;

use crate::generation::common::{
    BitfieldStructReferenceIdent, does_field_have_setter,
    generate_bitfield_struct_initialization_tokens, generate_setter_impl_tokens,
    generate_setting_fields_default_values_tokens, generate_setting_fields_to_zero_tokens,
    get_documentation_field_bits_order, get_field_checked_setter_method_identifier,
    get_field_setter_method_identifier,
};
use crate::parsing::bitfield_attribute::BitfieldAttribute;
use crate::parsing::bitfield_field::{BitfieldField, FieldAccess};

/// Generates the builder implementation.
pub(crate) fn generate_builder_tokens(
    vis: &Visibility,
    bitfield_struct_name: &Ident,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
    bitfield_attribute: &BitfieldAttribute,
) -> TokenStream {
    let builder_name = format_ident!("{}Builder", bitfield_struct_name);
    let has_ignored_fields = !ignored_fields.is_empty();

    let builder_setter_tokens = fields
        .iter()
        .filter(|field| !field.padding)
        .filter(|field| does_field_have_setter(field) || field.access == FieldAccess::ReadOnly)
        .map(|field| {
            generate_builder_field_setter(
                vis,
                bitfield_struct_name,
                field,
                bitfield_attribute,
                has_ignored_fields,
            )
        });

    let setting_fields_default_values_tokens = generate_setting_fields_default_values_tokens(
        &bitfield_attribute.ty,
        fields,
        &BitfieldStructReferenceIdent::NameReference(bitfield_struct_name.to_string()),
        has_ignored_fields,
    );
    let setting_fields_to_zero_tokens = generate_setting_fields_to_zero_tokens(
        &bitfield_attribute.ty,
        fields,
        &BitfieldStructReferenceIdent::NameReference(bitfield_struct_name.to_string()),
        has_ignored_fields,
    );
    let initialize_struct_tokens = generate_bitfield_struct_initialization_tokens(
        ignored_fields,
        &BitfieldStructReferenceIdent::NameReference(bitfield_struct_name.to_string()),
    );

    quote! {
        #[doc = "A builder for the bitfield."]
        #vis struct #builder_name {
            this: #bitfield_struct_name,
        }

        impl Default for #builder_name {
            fn default() -> Self {
                let mut this = #initialize_struct_tokens;
                #setting_fields_default_values_tokens
                Self {
                    this,
                }
            }
        }

        impl #builder_name {
            #[doc = "Creates a new bitfield builder instance."]
            #vis fn new() -> Self {
                let mut this = #initialize_struct_tokens;
                #setting_fields_default_values_tokens
                Self {
                    this,
                }
            }

            #[doc = "Creates a new bitfield builder instance without setting any default values."]
            #vis fn new_without_defaults() -> Self {
                let mut this = #initialize_struct_tokens;
                #setting_fields_to_zero_tokens
                Self {
                    this,
                }
            }

           #( #builder_setter_tokens )*

            #[doc = "Builds a bitfield instance from the builder."]
            #vis fn build(self) -> #bitfield_struct_name {
                self.this
            }
        }
    }
}

fn generate_builder_field_setter(
    vis: &Visibility,
    bitfield_struct_name: &Ident,
    field: &BitfieldField,
    bitfield_attribute: &BitfieldAttribute,
    has_ignored_fields: bool,
) -> TokenStream {
    let field_name = &field.name;
    let field_name_with_builder_setter_ident = format_ident!("with_{}", field_name);
    let checked_field_name_with_builder_setter_ident = format_ident!("checked_with_{}", field_name);
    let field_offset_setter_ident = get_field_setter_method_identifier(&field.name.to_string());
    let checked_field_offset_setter_ident =
        get_field_checked_setter_method_identifier(&field.name.to_string());
    let field_type = &field.ty;

    let with_field_setter_documentation = get_builder_setter_documentation(
        bitfield_attribute,
        field,
        /* checked_setter= */ false,
    );
    let checked_with_field_setter_documentation = get_builder_setter_documentation(
        bitfield_attribute,
        field,
        /* checked_setter= */ true,
    );
    if does_field_have_setter(field) {
        quote! {
            #[doc = #with_field_setter_documentation]
            #vis fn #field_name_with_builder_setter_ident(mut self, bits: #field_type) -> Self {
                self.this.#field_offset_setter_ident(bits);
                self
            }

            #[doc = #checked_with_field_setter_documentation]
            #vis fn #checked_field_name_with_builder_setter_ident(mut self, bits: #field_type) -> ::core::result::Result<Self, &'static str> {
                self.this.#checked_field_offset_setter_ident(bits)?;
                Ok(self)
            }
        }
    } else {
        let setter_impl_tokens = generate_setter_impl_tokens(
            &bitfield_attribute.ty,
            field,
            &BitfieldStructReferenceIdent::NameReference(bitfield_struct_name.to_string()),
            quote! { bits },
            /* check_value_bit_size= */ false,
            get_bitfield_struct_internal_value_for_builder_identifier_tokens(has_ignored_fields),
        );
        let checked_setter_impl_tokens = generate_setter_impl_tokens(
            &bitfield_attribute.ty,
            field,
            &BitfieldStructReferenceIdent::NameReference(bitfield_struct_name.to_string()),
            quote! { bits },
            /* check_value_bit_size= */ true,
            get_bitfield_struct_internal_value_for_builder_identifier_tokens(has_ignored_fields),
        );
        quote! {
            #[doc = #with_field_setter_documentation]
            #vis fn #field_name_with_builder_setter_ident(mut self, bits: #field_type) -> Self {
                #setter_impl_tokens
                self
            }

            #[doc = #checked_with_field_setter_documentation]
            #vis fn #checked_field_name_with_builder_setter_ident(mut self, bits: #field_type) -> ::core::result::Result<Self, &'static str> {
                #checked_setter_impl_tokens
                Ok(self)
            }
        }
    }
}

fn get_builder_setter_documentation(
    bitfield_attribute: &BitfieldAttribute,
    field: &BitfieldField,
    checked_setter: bool,
) -> String {
    let field_offset = field.offset;

    let with_field_documentation = if field.bits == 1 {
        format!("Sets builder bit `{field_offset}`.")
    } else {
        let (documentation_bits_start, documentation_bits_end) =
            get_documentation_field_bits_order(field, bitfield_attribute.bit_order);
        format!("Sets builder bits `{documentation_bits_start}..={documentation_bits_end}`.")
    };

    if checked_setter {
        return if field.bits == 1 {
            format!(
                "{with_field_documentation}. Returns an error if the value is too big to fit within the field bit."
            )
        } else {
            format!(
                "{with_field_documentation}. Returns an error if the value is too big to fit within the field bits."
            )
        };
    }

    with_field_documentation
}

/// Generates the to builder implementation.
pub(crate) fn generate_to_builder_tokens(
    vis: &Visibility,
    bitfield_struct_name: &Ident,
) -> TokenStream {
    let builder_name = format_ident!("{}Builder", bitfield_struct_name);

    quote! {
        #vis fn to_builder(&self) -> #builder_name {
            #builder_name {
                this: self.clone(),
            }
        }
    }
}

/// Returns the internal value identifier tokens for the builder struct.
pub(crate) fn get_bitfield_struct_internal_value_for_builder_identifier_tokens(
    has_ignored_fields: bool,
) -> TokenStream {
    if has_ignored_fields {
        quote! { self.this.val }
    } else {
        quote! { self.this.0 }
    }
}
