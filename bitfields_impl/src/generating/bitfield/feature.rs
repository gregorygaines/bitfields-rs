use proc_macro2::TokenStream;

use crate::parsing::bitfields::bitfield::Bitfield;

/// Represents a bitfield 'feature', meaning simply anything being generated.
pub trait Feature {
    /// Returns the token implementation of the feature.
    fn generate_feature(&self, bitfield: &Bitfield) -> TokenStream;

    /// Returns if the feature is enabled.
    fn enabled(&self, bitfield: &Bitfield) -> bool;

    /// Returns the position of the feature.
    fn feature_position(&self) -> FeaturePosition;

    /// Returns the order priority of the feature.
    ///
    /// The lower the number, the higher the generated feature will be placed
    /// along its peers. I didn't want the generated code to look messy, so I'd
    /// like to have control over where a feature is placed in the final
    /// results.
    /// Numeric ordering for ordering features with the same position.
    fn order_priority(&self) -> u32;
}

/// Represents the position of the features.
///
/// Here is the structure of a generated bitfield:
///
/// ```rust,ignore
/// // Above impl block
/// impl Bitfield {
///   // Inside impl block
/// }
/// // Below impl block
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FeaturePosition {
    /// The feature will be generated above the `impl` block.
    Above,

    /// The feature will be generated inside the `impl` block.
    Inside,

    /// The feature will be generated below the `impl` block.
    Below,
}

pub fn is_bit_ops_feature_enabled(
    bitfield: &Bitfield,
    bit_ops_enabled: bool,
    did_user_set_bit_op_flag: bool,
) -> bool {
    // The global enable bit ops flag is disabled, only if the user explicitly
    // set the bit ops flag to true, then we enable bit ops.
    if !bitfield.arguments().generate_bit_ops() {
        return bit_ops_enabled && did_user_set_bit_op_flag;
    }

    // The global enable bit ops flag is enabled, rely on flag.
    bit_ops_enabled
}
