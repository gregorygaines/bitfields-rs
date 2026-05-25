use getset::{CloneGetters, Getters};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};

use crate::parsing::bitfields::bitfield_attribute::bitfield_arguments::BitfieldArguments;
use crate::parsing::bitfields::bits_attribute::bits_arguments::{BitsArguments, FieldAccess};
use crate::parsing::common::spanned_data_type::{DataType, SpannedDataTypeToken};
use crate::parsing::common::visibility::Visibility;

/// Represents the annotated struct that is the source of the bitfield
/// implementation.
#[derive(Getters, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct Bitfield {
    /// The user defined attributes of the bitfield.
    user_attributes_tokens: Vec<TokenStream>,

    /// The visibility of the bitfield.
    visibility: Visibility,

    /// The name of the bitfield.
    name: String,

    /// The type of the bitfield.
    spanned_data_type_token: SpannedDataTypeToken,

    /// The fields of the bitfield.
    fields: Vec<Field>,

    /// The ignored fields of the bitfield.
    ignored_fields: Vec<Field>,

    /// The arguments of the bitfield.
    arguments: BitfieldArguments,
}

impl Bitfield {
    /// Creates a new [`Bitfield`] instance.
    pub const fn new(
        user_attributes_tokens: Vec<TokenStream>,
        visibility: Visibility,
        name: String,
        spanned_data_type_token: SpannedDataTypeToken,
        fields: Vec<Field>,
        ignored_fields: Vec<Field>,
        arguments: BitfieldArguments,
    ) -> Self {
        Self {
            user_attributes_tokens,
            visibility,
            name,
            spanned_data_type_token,
            fields,
            ignored_fields,
            arguments,
        }
    }

    /// Returns if the bitfield has ignored fields.
    pub fn has_ignored_fields(&self) -> bool {
        !self.ignored_fields.is_empty()
    }

    /// Returns the name as tokens.
    pub fn name_tokens(&self) -> TokenStream {
        format_ident!("{}", self.name).to_token_stream()
    }

    pub const fn is_integer_backed(&self) -> bool {
        matches!(self.spanned_data_type_token.data_type(), DataType::Integer(_))
    }
}

/// Represents a bitfield field.
#[derive(Debug, Clone, Getters, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct Field {
    /// The visibility of the field.
    #[getset(skip)]
    visibility: Visibility,

    /// The name of the field.
    name: String,

    /// The type of the field.
    spanned_data_type_token: SpannedDataTypeToken,

    /// The bits of the field.
    bits: u32,

    /// The offset of the field.
    offset: u32,

    /// Whether the field is reserved.
    reserved: bool,

    /// The access of the field.
    access: FieldAccess,

    /// The arguments of the field, if any.
    arguments: Option<BitsArguments>,

    /// Indicates if the field is ignored.
    ignored: bool,
}

impl Field {
    /// Creates a new `[Field]` instance.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        visibility: Visibility,
        name: String,
        spanned_data_type_token: SpannedDataTypeToken,
        bits: u32,
        offset: u32,
        reserved: bool,
        access: FieldAccess,
        arguments: Option<BitsArguments>,
        ignored: bool,
    ) -> Self {
        Self {
            visibility,
            name,
            spanned_data_type_token,
            bits,
            offset,
            reserved,
            access,
            arguments,
            ignored,
        }
    }

    /// Returns the visibility of the field.
    pub fn visibility(&self) -> Visibility {
        let has_public_access =
            matches!(self.access, FieldAccess::ReadWrite | FieldAccess::ReadOnly);
        if !has_public_access {
            return Visibility::Private;
        }

        self.visibility.clone()
    }

    /// Returns the name as tokens.
    pub fn name_tokens(&self) -> TokenStream {
        format_ident!("{}", self.name).to_token_stream()
    }

    /// Returns if the field has a default value.
    pub fn has_default_value(&self) -> bool {
        self.arguments.as_ref().is_some_and(|arguments| arguments.default_value_expr().is_some())
    }

    /// Returns if the field has a setter generated.
    pub const fn has_setter(&self) -> bool {
        if self.reserved {
            return false;
        }

        matches!(self.access, FieldAccess::WriteOnly)
            || matches!(self.access, FieldAccess::ReadWrite)
    }

    /// Returns if the field has a getter generated.
    pub const fn has_getter(&self) -> bool {
        if self.reserved {
            return false;
        }

        matches!(self.access, FieldAccess::ReadOnly)
            || matches!(self.access, FieldAccess::ReadWrite)
    }

    pub const fn has_read_access(&self) -> bool {
        matches!(self.access, FieldAccess::ReadOnly)
            || matches!(self.access, FieldAccess::ReadWrite)
    }

    /// Returns if the field has constants generated.
    pub const fn has_constants(&self) -> bool {
        self.has_getter() || self.has_setter()
    }

    /// Returns if the field is a reserved field.
    pub const fn is_reserved(&self) -> bool {
        self.reserved
    }
}
