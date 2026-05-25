#![no_std]
#![warn(incomplete_features)]

/// Creates a bitfield from the attributed struct.
pub use bitfields_impl::bitfield;
/// Creates a bitfield from the attributed enum.
pub use bitfields_impl::bitflag;
