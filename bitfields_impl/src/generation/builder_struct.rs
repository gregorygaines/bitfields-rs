use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Visibility;

use crate::generation::common::{
    does_field_have_setter, generate_setting_fields_default_values_tokens,
    generate_setting_fields_to_zero_tokens,
};
use crate::parsing::bitfield_field::BitfieldField;

/// Generates the builder implementation.
pub(crate) fn generate_builder_tokens(
    vis: Visibility,
    bitfield_type: &syn::Type,
    bitfield_struct_name: Ident,
    fields: &[BitfieldField],
    ignored_fields: &[BitfieldField],
) -> TokenStream {
    let builder_name = format_ident!("{}Builder", bitfield_struct_name);

    let builder_setter_tokens = fields
        .iter()
        .filter(|field| !field.padding)
        .filter(|field| does_field_have_setter(field))
        .map(|field| {
            let field_name = field.name.clone();
            let field_name_with_builder_setter_ident = format_ident!("with_{}", field_name);
            let checked_field_name_with_builder_setter_ident = format_ident!("checked_with_{}", field_name);
            let field_offset_setter_ident = format_ident!("set_{}", field_name);
            let checked_field_offset_setter_ident = format_ident!("checked_set_{}", field_name);
            let field_type = field.ty.clone();

            let field_bits = field.bits;
            let field_offset = field.offset;
            let field_bits_end = field_offset + field_bits;

            let with_field_documentation = format!("Sets builder bits [{}..={}].", field_offset, field_bits_end);
            let checked_with_field_documentation = format!("Sets builder bits [{}..={}]. Returns an error if the value is too big to fit within the field bits.", field_offset, field_bits_end);

            quote! {
                #[doc = #with_field_documentation]
                #vis fn #field_name_with_builder_setter_ident(mut self, bits: #field_type) -> Self {
                    self.this.#field_offset_setter_ident(bits);
                    self
                }

                #[doc = #checked_with_field_documentation]
                #vis fn #checked_field_name_with_builder_setter_ident(mut self, bits: #field_type) -> Result<Self, &'static str> {
                    self.this.#checked_field_offset_setter_ident(bits)?;
                    Ok(self)
                }
            }
        });

    let setting_fields_default_values_tokens = generate_setting_fields_default_values_tokens(
        bitfield_type,
        fields,
        Some(quote! { #bitfield_struct_name }),
        !ignored_fields.is_empty(),
    );
    let setting_fields_to_zero_tokens = generate_setting_fields_to_zero_tokens(
        bitfield_type,
        fields,
        Some(quote! { #bitfield_struct_name }),
        !ignored_fields.is_empty(),
    );

    let ignored_fields_defaults = ignored_fields.iter().map(|field| {
        let field_name = &field.name;
        let field_ty = &field.ty;
        quote! {
            #field_name: <#field_ty>::default(),
        }
    });

    let initialize_struct_tokens = if !ignored_fields.is_empty() {
        quote! {
            #bitfield_struct_name {
                val: 0,
                #( #ignored_fields_defaults )*
            }
        }
    } else {
        quote! {
            #bitfield_struct_name(0)
        }
    };

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
