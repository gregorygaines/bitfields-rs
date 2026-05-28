use generating::bitfield::bitfield_generator::generate_bitfield;
use parsing::bitfields::bitfield::Bitfield;
use parsing::bitfields::bitfield_parser::parse_bitfield_struct;

use crate::generating::bitflag::bitflag_generator::generate_bitflag;
use crate::parsing::bitflags::bitflag_parser::parse_bitflag_enum;

mod generating;
mod parsing;

/// An error message to display when an internal error occurs.
const INTERNAL_ERROR_MESSAGE: &str = "A major unexpected error has occurred. If possible, please file an issue with code reproducing the error to https://github.com/gregorygaines/bitfields-rs/issues";

#[rustfmt::skip]
/// <!-- rust-docs-start -->
/// ### Bitfield Types
///
/// A primitive bitfield can represent the unsigned types (`u8`, `u16`, `u32`,
/// `u64`, `u128`), the max being 128-bits. The bitfield field bits must add
/// up to the exact number of bits of the bitfield type.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u8)]
/// struct BitfieldU8 {
///     a: u8,
/// }
///
/// #[bitfield(u32)]
/// struct BitfieldMultipleFields {
///     a: u8,
///     b: u8,
///     c: u8,
///     d: u8,
/// }
/// ```
///
/// ### Array Backed Bitfield
///
/// A bitfield can also be backed by an `[u8;N]` array type, which allows for
/// bitfields larger than `u128`. Just like primitive bitfields, the bitfield
/// field bits must add up to the exact number of bits of the bitfield type.
///
/// If you have an array backed bitfield that may overflow the stack, you can pass
/// the optional argument `#[bitfield(array_heap = true)]` to the bitfield, which
/// will box the array on the heap instead of the stack. Keep in mind that you
/// **lose constant memory, zero-allocation, and no_std guarantees when using heap
/// array bitfields**.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield([u8; 17])] /// 136 bits.
/// struct ArrayBitfield {
///     a: u128,
///     b: u8,
/// }
///
/// #[bitfield([u8; 96], array_heap = true
/// )] /// Allocated on the heap, 768 bits (96 bytes).
/// struct HeapArrayBitfield {
///     a: u128,
///     b: u128,
///     c: u128,
///     d: u128,
///     e: u128,
///     f: u128,
/// }
///
/// fn main() {
///     let array_bitfield = ArrayBitfield::new();
///     let heap_array_bitfield = HeapArrayBitfield::new();
/// }
/// ```
///
/// ### Constructing a Bitfield
///
/// #### Bitfield Constructor
///
/// A bitfield can be initialized with the `new` and `new_without_defaults`
/// constructors. The former initializes the bitfield to zero, but sets
/// default values, while the latter initializes the bitfield to zero without
/// setting any default values, except for reserved fields with default values,
/// which always keep their defaults.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///
///     /// Reserved field, keeps its default value even in `new_without_defaults`.
///     #[bits(default = 0x78)]
///     _d: u8,
/// }
///
/// fn main() {
///     let bitfield = Bitfield::new();
///     assert_eq!(bitfield.a(), 0x12);
///     assert_eq!(bitfield.b(), 0x34);
///     assert_eq!(bitfield.c(), 0x56);
///     assert_eq!(bitfield.into_bits(), 0x78563412);
///
///     let bitfield_without_defaults = Bitfield::new_without_defaults();
///     assert_eq!(bitfield_without_defaults.a(), 0);
///     assert_eq!(bitfield_without_defaults.b(), 0);
///     assert_eq!(bitfield_without_defaults.c(), 0);
///     assert_eq!(bitfield_without_defaults.into_bits(), 0x78000000);
/// }
/// ```
///
/// #### Bitfield Builder
///
/// A bitfield can also be constructed using a fluent builder pattern using the
/// `<Bitfield>Builder::new` and `<Bitfield>Builder::new_without_defaults`
/// constructors. They operate the same as the `new` and `new_without_defaults`,
/// but allow you to set fields using a fluent builder pattern.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     _reserved: u8,
/// }
///
/// fn main() {
///     let builder = BitfieldBuilder::new()
///         .with_a(0x12)
///         .with_b(0x34)
///         .with_c(0x56)
///         // with_reserved() /// Compile error, reserved fields are inaccessible.
///         .build();
///     assert_eq!(builder.a(), 0x12);
///     assert_eq!(builder.b(), 0x34);
///     assert_eq!(builder.c(), 0x56);
///     assert_eq!(builder.into_bits(), 0x78563412);
/// }
/// ```
///
/// ### Bitfield Field Types
///
/// A bitfield field can be any unsigned (`u8`, `u16`, `u32`, `u64`, `u128`).
///
/// Fields can be annotated with the `#[bits]` attribute to specify the number of
/// bits the field occupies, a default value, or access. If the `#[bits]` attribute
/// is omitted, the field will occupy the number of bits of its type. For example,
/// a `u8` field will occupy 8 bits.
///
/// Fields can have a default value, which must fit in the field type or the
/// specified bits of the field. A default value must be a const variable or
/// a const function. Just be aware that const function and variable 
/// defaults lose their compile-time field size checks, so it's
/// up to you to make sure they fit.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// /// A constant variable can be used as a default value.
/// const CONST_VAR: u8 = 0x2;
///
/// /// A const function can be used as a default value.
/// const fn answer_to_life() -> u8 {
///     0x1
/// }
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     /// No bits attribute, so it defaults to the size of the field type,
///     /// which is 8 bits for a `u8` type.
///     #[bits(default = 0xFF)]
///     a: u8,
///
///     /// ❌ No compile time default value size checks for const variables.
///     #[bits(2, default = CONST_VAR)]
///     const_var_default: u8,
///
///     /// ❌ No compile time default value size checks for const functions.
///     #[bits(2, default = answer_to_life())]
///     const_fn_default: u8,
///
///     #[bits(20)]
///     _reserved: u32,
/// }
///
/// fn main() {
///     let bitfield = Bitfield::new();
///     assert_eq!(bitfield.a(), 0xFF);
///     assert_eq!(bitfield.const_var_default(), 0x2);
///     assert_eq!(bitfield.const_fn_default(), 0x1);
/// }
/// ```
///
/// #### Signed Bitfield Fields
///
/// A bitfield can have signed (`i8`, `i16`, `i32`, `i64`, `i128`) types. Signed
/// types are treated as 2's complement data types, where the most significant bit
/// representing the sign bit. For example, if you had a field with 5 bits, the value
/// range would be `-16` to `15`. The more bits you include, the larger the range!
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// struct Bitfield {
///     #[bits(default = -127)]
///     a: i8,
///
///     /// Sign-extended by the most significant bit of 4 bits. Also treated as 2's
///     /// complement, meaning this field with 4 bits has the value range of
///     /// `-8` to `7`. You can add more bits to increase this range!
///     #[bits(4, default = 9)]
///     b_sign_extended: i8,
///
///     #[bits(4)]
///     _reserved: u8,
/// }
///
/// fn main() {
///     let bitfield = Bitfield::new();
///     assert_eq!(bitfield.a(), -127);
///     assert_eq!(bitfield.b_sign_extended(), -7);
/// }
/// ```
///
/// #### Array Bitfield Fields
///
/// Bitfield fields can also be `[u8;N]` array types, which are useful
/// for representing fields that are larger than 128 bits.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Packet {
///     header: [u8; 2],
///     #[bits(16)]
///     payload: [u8; 16], // 128 bits
/// }
///
/// fn main() {
///     let mut packet = Packet::new();
///     packet.set_header([0u8; 2]);
///     packet.set_payload([0xFFu8; 16]);
///
///     assert_eq!(packet.header(), [0u8; 2]);
///     assert_eq!(packet.payload(), [0xFFu8, 0xFFu8, 0, 0, 0,
///         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
/// }
/// ```
///
/// #### Checked Setters
///
/// Normally, when fields are set, the provided value is truncated to the number of
/// bits of the field. Fields also have checked setters that returns an error if
/// the value overflows the bits of the field.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// struct Bitfield {
///     a: u8,
///     #[bits(4)]
///     b: u8,
///     #[bits(4)]
///     _reserved: u8,
/// }
///
/// fn main() {
///     let mut bitfield = Bitfield::new();
///     bitfield.set_a(0xFF);
///     bitfield.set_b(0x12); // Truncated to 4 bits.
///     assert_eq!(bitfield.a(), 0xFF);
///     assert_eq!(bitfield.b(), 0x2);
///
///     let res = bitfield.checked_set_b(0x12); // Error, value overflows bits.
///     assert!(res.is_err());
/// }
/// ```
///
/// #### Field Access
///
/// Field access can be controlled by specifying the `#[bits(access = N)]` arg on a
/// field. There are four access levels:
///
/// - `rw` - Read and write access (default).
///
/// - `ro` - Read-only access, only set during construction or from bits conversion.
///
/// - `wo` - Write-only access, can only be set but not read. A use case for
///   write-only
///   fields is for fields that trigger an action when set, but their value is not
///   stored.
///   You can write your own setters for `wo` fields to trigger the action you want
///   when
///   the field is set.
///
/// - `na` - No access, the field is inaccessible, functioning similarly to reserved
///   fields. But use a **reserved field** when you want to pad/fill bits that have
///   no user-facing identity (hardware reserved bits, alignment gaps). Use
///   **`access = na`** when the field is a real, named field that you want to
///   completely lock down from the API while still keeping it distinguishable 
///   by name. Later on, if you want to expose the field, you can just change
///   the access level without having to change the field name.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     read_write: u8,
///
///     #[bits(access = ro)]
///     read_only: u8,
///
///     #[bits(access = wo)]
///     write_only: u8,
///
///     #[bits(default = 0xFF, access = na)]
///     no_access: u8,
/// }
///
/// impl Bitfield {
///     /// Custom setter for the write-only field that triggers an action when set.
///     fn custom_set_write_only(&mut self, value: u8) {
///         // Trigger some action here when the write-only field is set.
///         println!("Write-only field set to: {}", value);
///
///         // Using the generated constants, you can manipulate the bits of the 
///         // write-only field even though it's inaccessible from the API.
///         self.0 = (self.0 & !(0xFF << Self::WRITE_ONLY_OFFSET)) |
///             ((value as u32) << Self::WRITE_ONLY_OFFSET);
///     }
/// }
///
/// fn main() {
///     let mut bitfield = BitfieldBuilder::new()
///         .with_read_write(0x12)
///         .with_read_only(0x34) // Read-only fields only set during construction or from bits.
///         .with_write_only(0x56)
///         // .with_no_access(0x78) // Compile error, no-access field can't be set.
///         .build();
///     bitfield.set_read_write(0x12);
///     // bitfield.set_read_only(1); // Compile error, read-only field can't be set.
///     bitfield.set_write_only(0x56);
///     // bitfield.set_no_access(0x78); // Compile error, no-access field can't be set.
///
///     assert_eq!(bitfield.read_write(), 0x12);
///     assert_eq!(bitfield.read_only(), 0x34);
///     // assert_eq!(bitfield.write_only(), 0x56); // Compile error, write-only can't be read.
///     // assert_eq!(bitfield.no_access(), 0xFF); // Compile error, no-access field can't be accessed.
///     assert_eq!(bitfield.into_bits(), 0xFF563412); // All fields exposed when converted to bits.
///
///     bitfield.custom_set_write_only(0xAB); // Custom setter for write-only field that triggers an action when set.
///     assert_eq!(bitfield.into_bits(), 0xFFAB3412); // All fields exposed when converted to bits, 
///     // including the write-only field that was set
///     // using a custom setter.
/// }
/// ```
///
/// #### Custom Types
///
/// A bitfield field can be a user-defined custom types, but the bits of the field
/// must be specified since the macro has no way of knowing how many bits the custom
/// type occupies. To interface with a bitfield, the custom types must implement the
/// const functions `from_bits` and `into_bits`.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitflag;
/// use bitfields::bitfield;
///
/// const DEFAULT_PLAYER_STATE: PlayerState = PlayerState {
///     health: 15,
///     mana: 3,
///     stamina: 1,
/// };
///
/// #[bitfield(u16)]
/// struct GameStatus {
///     /// ❌ No compile time default value size checks for const variables.
///     #[bits(8, default = DEFAULT_PLAYER_STATE)]
///     player_state: PlayerState,
///
///     /// ❌ No compile time default value size checks for enum types.
///     #[bits(8, default = GameState::Playing)]
///     game_state: GameState
/// }
///
/// /// A struct can be a custom type, but must implement the `from_bits` 
/// /// and `into_bits` const functions to convert to and from bits.
/// #[derive(Debug, PartialEq)]
/// struct PlayerState {
///     health: u8,
///     mana: u8,
///     stamina: u8,
/// }
///
/// impl PlayerState {
///     /// The from_bits takes in the bits of the field and converts 
///     /// it into the custom type.
///     const fn from_bits(bits: u8) -> Self {
///         Self {
///             health: bits & 0b0000_1111, // First 4 bits for health
///             mana: (bits >> 4) & 0b0000_0011, // Next 2 bits for mana
///             stamina: (bits >> 6) & 0b0000_0001, // Last bit for stamina
///         }
///     }
///
///     /// The into_bits converts the custom type into a packed bits
///     /// representation. The only requirement is that the return type
///     /// is a primitive type.
///     const fn into_bits(self) -> u8 {
///         (self.health & 0b0000_1111) |
///             ((self.mana & 0b0000_0011) << 4) |
///             ((self.stamina & 0b0000_0001) << 6)
///     }
/// }
///
/// /// Enums can also be custom types as long as they implement the 
/// /// `from_bits` and `into_bits` const functions.
/// ///
/// /// Instead of writing custom enum types by hand, you can also
/// /// use the `#[bitflag]` attribute to generate these functions
/// /// for you.
/// #[derive(Debug, PartialEq)]
/// enum GameState {
///     Playing = 0,
///     Paused = 1,
///     GameOver = 2,
/// }
///
/// impl GameState {
///     /// The from_bits takes in the bits of the field and converts
///     /// it into the custom type.
///     const fn from_bits(bits: u8) -> Self {
///         match bits {
///             0 => Self::Playing,
///             1 => Self::Paused,
///             2 => Self::GameOver,
///             _ => unreachable!(),
///         }
///     }
///
///     /// The into_bits converts the custom type into a packed bits'
///     /// representation. The only requirement is that the return
///     /// type is a primitive type.
///     const fn into_bits(self) -> u8 {
///         self as u8
///     }
/// }
///
/// /// Bitflags are enums but the `from_bits` and `into_bits` functions
/// /// are generated for you. The only requirement is that one of the
/// /// variants must be annotated with `#[base]` which represents the 
/// /// base value of the bitflag.
/// #[bitflag(u8)]
/// #[derive(Debug, PartialEq)]
/// enum GameStateBitflag {
///     #[base]
///     Playing = 0,
///     Paused = 1,
///     GameOver = 2,
/// }
///
/// /// The code that's generated for the `GameStateBitflag` by the
/// /// `#[bitflag]` attribute saving you the trouble of writing 
/// /// the `from_bits` and `into_bits` functions yourself.
/// // impl GameState {
/// //     const fn from_bits(bits: u8) -> Self {
/// //         match bits {
/// //             0 => Self::Playing,
/// //             1 => Self::Paused,
/// //             2 => Self::GameOver,
/// //             _ => unreachable!(),
/// //         }
/// //     }
/// //
/// //     const fn into_bits(self) -> u8 {
/// //         self as u8
/// //     }
/// // }
///
/// fn main() {
///     let game_status = GameStatus::new();
///     assert_eq!(game_status.player_state(), DEFAULT_PLAYER_STATE);
///     assert_eq!(game_status.player_state().health, 15);
///     assert_eq!(game_status.player_state().mana, 3);
///     assert_eq!(game_status.player_state().stamina, 1);
///     assert_eq!(game_status.game_state(), GameState::Playing);
/// }
/// ```
///
/// #### Nested Bitfields
///
/// Bitfields can be nested within other bitfields, but the bits of the nested
/// bitfield must be specified since the macro has no way of knowing how many bits
/// the nested bitfield occupies.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct GameWorld {
///     /// ❌ No compile time default value size checks for const functions.
///     #[bits(32, default = GameMap::new())]
///     game_map: GameMap,
/// }
///
/// #[bitfield(u32)]
/// struct GameMap {
///     #[bits(default = 0x11)]
///     num_trees: u8,
///     #[bits(default = 0x22)]
///     num_players: u8,
///     #[bits(default = 0x33)]
///     num_monsters: u8,
///     #[bits(default = 0x44)]
///     num_items: u8,
/// }
///
/// fn main() {
///     let game_world = GameWorld::new();
///     assert_eq!(game_world.game_map().num_items(), 0x44);
///     assert_eq!(game_world.game_map().num_monsters(), 0x33);
///     assert_eq!(game_world.game_map().num_players(), 0x22);
///     assert_eq!(game_world.game_map().num_trees(), 0x11);
///     assert_eq!(game_world.into_bits(), 0x44332211);
/// }
/// ```
///
/// #### Reserved Fields
///
/// Fields prefixed with an underscore `_` are reserved fields, which are
/// inaccessible. Meaning the field is always 0, false, or a default value.
/// They are useful for reserving the bits of the bitfield, padding fields,
/// or for values that are always constant.
///
/// If you don't want to give the reserved fields a name, you can use
/// `__` as the name for all reserved fields.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// struct Bitfield {
///     a: u8,
///
///     /// Fills the remaining bits of the u16.
///     #[bits(default = 0xFF)]
///     _reserved: u8,
/// }
///
/// #[bitfield(u16)]
/// struct ReservedBitfield {
///     a: u8,
///
///     /// Fills the middle bits of the u16.
///     #[bits(4, default = 0xF)]
///     __: u8,
///     
///     /// Fils the end bits of the u16.
///     #[bits(4, default = 0xF)]
///     __: u8,
/// }
///
/// fn main() {
///     let bitfield = Bitfield::new();
///     assert_eq!(bitfield.a(), 0);
///     // assert_eq!(bitfield._reserved(), 0xFF00); // Compile error, reserved inaccessible.
///     // bitfield.set__reserved(0xFF); // Compile error, reserved fields are inaccessible.
///     assert_eq!(bitfield.into_bits(), 0xFF00); // All fields exposed when converted 
///     // to bits.
///     
///     let reserved_bitfield = ReservedBitfield::new();
///     assert_eq!(reserved_bitfield.a(), 0);
///     // assert_eq!(reserved_bitfield.__(), 0xFF0); // Compile error, reserved inaccessible.
///     // reserved_bitfield.set__(0xFF); // Compile error, reserved fields are inaccessible.
///     assert_eq!(reserved_bitfield.into_bits(), 0xFF00); // All fields exposed when 
///     // converted to bits.
/// }
/// ```
///
/// <!-- rust-bitflags-docs-start -->
///
/// ### Bitflags
///
/// There are times when you just want to define a bitflag, which are just enums
/// that map to bits. Instead of defining a custom type, you can take advantage of
/// the `#[bitflag]` attribute which generates `from_bits` and `into_bits` for enums
/// automatically.
///
/// Bitflags only supports unsigned types (`u8`, `u16`, `u32`, `u64`, `u128`) and
/// the one of the variants must be annotated with `#[base]` or `#[default]` which represents the
/// base value of the bitflag. If `#[base]` and `#[default]` are both present, `#[base]` takes precedence.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
/// use bitfields::bitflag;
///
/// /// Annotate an enum with the `#[bitflag]` attribute to automatically generate
/// /// the `from_bits` and `into_bits` functions for you! One variant must be annotated
/// /// with `#[base]` which represents the base value of the bitflag.
/// #[bitflag(u8)]
/// #[derive(Debug, PartialEq)]
/// enum RenderMode {
///     #[base]
///     Normal = 0,
///     Mirror = 1,
///     Flip = 2,
///     // #[default] - Default can be specified but base takes 
///     // precedence if both are present.
///     Hidden = 3,
/// }
///
/// /// The code that's generated for the `RenderMode` by the
/// /// `#[bitflag]` attribute saving you the trouble of writing 
/// /// the `from_bits` and `into_bits` functions yourself.
/// // impl RenderMode {
/// //     const fn from_bits(bits: u8) -> Self {
/// //         match bits {
/// //             0 => Self::Normal,
/// //             1 => Self::Mirror,
/// //             2 => Self::Flip,
/// //             3 => Self::Hidden,
/// //             _ => unreachable!(),
/// //         }
/// //     }
/// //
/// //     const fn into_bits(self) -> u8 {
/// //         self as u8
/// //     }
/// // }
///
/// /// Annotate an enum with the `#[bitflag]` attribute to automatically generate
/// /// the `from_bits` and `into_bits` functions for you! One variant must be annotated
/// /// with `#[base]` which represents the base value of the bitflag.
/// #[bitflag(u8)]
/// #[derive(Debug, PartialEq)]
/// enum AudioMode {
///     #[base]
///     Stereo = 0,
///     Mono = 1,
///     Mute = 2,
///     Surround = 3,
/// }
///
/// #[bitfield(u8)]
/// struct DisplayControl {
///     /// Must have the `#[bits]` attribute since the macro has 
///     /// no way of knowing how many bits the custom type occupies.
///     #[bits(4, default = RenderMode::Normal)]
///     render_mode: RenderMode,
///     /// Must have the `#[bits]` attribute since the macro has 
///     /// no way of knowing how many bits the custom type occupies.
///     #[bits(4, default = AudioMode::Stereo)]
///     audio_mode: AudioMode,
/// }
///
/// fn main() {
///     let display = DisplayControlBuilder::new()
///         .with_render_mode(RenderMode::Mirror)
///         .with_audio_mode(AudioMode::Mute)
///         .build();
///
///     assert_eq!(display.render_mode(), RenderMode::Mirror);
///     assert_eq!(display.audio_mode(), AudioMode::Mute);
/// }
/// ```
///
/// #### Bitflag Configuration
///
/// Bitflags can be configured with arguments passed to the `#[bitflag(...)]` attribute (the first argument is always the backing unsigned integer type):
///
/// | Argument         | Values                            | Default  | Description                                                                                          |
/// |------------------|-----------------------------------|----------|------------------------------------------------------------------------------------------------------|
/// | `<backing type>` | `u8`, `u16`, `u32`, `u64`, `u128` | Required | The storage used by the generated bitflag. Primitive backing types support bitfields up to 128 bits. |
/// | `from_endian`    | `big`, `little`                   | `big`    | Default endianness used by the generated `from_bits` function.                                       |
/// | `into_endian`    | `big`, `little`                   | `big`    | Default endianness used by the generated `into_bits` function.                                       |
/// | `copy`           | `true`, `false`                   | `true`   | Determines whether to derive `Copy` and `Clone` automatically for the enum.                          |
///
/// <!-- rust-bitflags-docs-end -->
///
/// ### Field Constants
///
/// Fields with read or write access have constants generated for their number of
/// bits and offset in the bitfield.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     d: u8,
/// }
///
/// fn main() {
///     assert_eq!(Bitfield::A_BITS, 8); // Number of bits of the a field.
///     assert_eq!(Bitfield::A_OFFSET, 0); // The offset of the a field in the bitfield.
///     assert_eq!(Bitfield::B_BITS, 8); // Number of bits of the b field.
///     assert_eq!(Bitfield::B_OFFSET, 8); // The offset of the b field in the bitfield.
///     assert_eq!(Bitfield::C_BITS, 8); // Number of bits of the c field.
///     assert_eq!(Bitfield::C_OFFSET, 16); // The offset of the c field in the bitfield.
///     assert_eq!(Bitfield::D_BITS, 8); // Number of bits of the d field.
///     assert_eq!(Bitfield::D_OFFSET, 24); // The offset of the d field in the bitfield.
/// }
/// ```
///
/// ### Field Order
///
/// The order of the bitfield determines whether from top to bottom struct fields
/// are ordered from the least significant bit (lsb) to the most significant bit (
/// msb). By default, fields are ordered from the least significant bit (lsb) to the
/// most significant bit (msb).
///
/// The order can be changed by specifying the `#[bitfield(order = N)]` arg on the
/// bitfield struct, with the options `lsb` or `msb`.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// /// Field layout (LSB → MSB):
/// ///
/// /// | 31    24 | 23   16 | 15        8 | 7       0 |
/// /// +----------+---------+-------------+-----------+
/// /// | checksum | reading | status_code | sensor_id |
/// /// +----------+---------+-------------+-----------+
/// ///
/// /// Field mapping:
/// /// - sensor_id:   bits 7..=0
/// /// - status_code: bits 15..=8
/// /// - reading:     bits 23..=16
/// /// - checksum:    bits 31..=24
/// #[bitfield(u32)] // LSB by default.
/// struct TelemetryFrameLsb {
///     #[bits(default = 0x12)] /// sensor_id occupies bits 7..=0 (LSB).
///     sensor_id: u8,
///     #[bits(default = 0x34)] /// status_code occupies bits 15..=8.
///     status_code: u8,
///     #[bits(default = 0x56)] /// reading occupies bits 23..=16.
///     reading: u8,
///     #[bits(default = 0x78)] /// checksum occupies bits 31..=24 (MSB).
///     checksum: u8,
/// }
///
/// /// Bit layout (MSB → LSB):
/// ///
/// /// | 31    24 | 23          16 | 15      8 | 7       0 |
/// /// +----------+---------------+-----------+------------+
/// /// | sensor_id|  status_code  |  reading  |  checksum  |
/// /// +----------+---------------+-----------+------------+
/// ///
/// /// Field mapping:
/// /// - sensor_id:   bits 31..=24
/// /// - status_code: bits 23..=16
/// /// - reading:     bits 15..=8
/// /// - checksum:    bits 7..=0
/// #[bitfield(u32, order = msb)]
/// struct TelemetryFrameMsb {
///     #[bits(default = 0x12)]
///     sensor_id: u8,
///     #[bits(default = 0x34)]
///     status_code: u8,
///     #[bits(default = 0x56)]
///     reading: u8,
///     #[bits(default = 0x78)]
///     checksum: u8,
/// }
///
/// fn main() {
///     let frame_lsb = TelemetryFrameLsb::new();
///     assert_eq!(frame_lsb.sensor_id(), 0x12);
///     assert_eq!(frame_lsb.status_code(), 0x34);
///     assert_eq!(frame_lsb.reading(), 0x56);
///     assert_eq!(frame_lsb.checksum(), 0x78);
///     let val = frame_lsb.into_bits();
///
///     //                .- checksum
///     //                |  .- reading
///     //                |  |  .- status_code
///     //                |  |  |  .- sensor_id
///     assert_eq!(val, 0x78_56_34_12);
///     assert_eq!(TelemetryFrameLsb::SENSOR_ID_OFFSET, 0); // Offset in LSB order.
///
///     let frame_msb = TelemetryFrameMsb::new();
///     assert_eq!(frame_msb.sensor_id(), 0x12);
///     assert_eq!(frame_msb.status_code(), 0x34);
///     assert_eq!(frame_msb.reading(), 0x56);
///     assert_eq!(frame_msb.checksum(), 0x78);
///     let val = frame_msb.into_bits();
///
///     //                .- sensor_id
///     //                |  .- status_code
///     //                |  |  .- reading
///     //                |  |  |  .- checksum
///     assert_eq!(val, 0x12_34_56_78);
///     assert_eq!(TelemetryFrameMsb::SENSOR_ID_OFFSET, 24); // Offset in MSB order.
/// }
/// ```
///
/// ### Bitfield Conversions
///
/// A bitfield can be converted into and from bits using multiple functions.
///
/// #### From Bits
///
/// A bitfield can be converted from bits using the following from APIs:
///
/// **Primitive Bitfield**:
///
/// | Method                                | Endianness                                   | Description                                                                                                  |
/// |---------------------------------------|----------------------------------------------|--------------------------------------------------------------------------------------------------------------|
/// | `from_bits(bits: N)`                  | Determined by `#[bitfield(from_endian = N)]` | Creates a new bitfield instance from the given raw bits, ignoring field defaults. (Default is `big`).        |
/// | `from_bits_with_defaults(bits: N)`    | Determined by `#[bitfield(from_endian = N)]` | Creates a new bitfield instance from the given raw bits, while respecting/applying field defaults.           |
/// | `from_le_bits(bits: N)`               | Little-endian                                | Creates a new bitfield instance from the given little-endian bits, ignoring field defaults.                  |
/// | `from_le_bits_with_defaults(bits: N)` | Little-endian                                | Creates a new bitfield instance from the given little-endian bits, while respecting/applying field defaults. |
/// | `from_be_bits(bits: N)`               | Big-endian                                   | Creates a new bitfield instance from the given big-endian bits, ignoring field defaults.                     |
/// | `from_be_bits_with_defaults(bits: N)` | Big-endian                                   | Creates a new bitfield instance from the given big-endian bits, while respecting/applying field defaults.    |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     _reserved: u8,
/// }
///
/// fn main() {
///     // from_bits - Creates a bitfield from raw bits, ignoring defaults. 
///     // Default endian is big.
///     let from_bits = Bitfield::from_bits(0x11_22_33_44);
///     assert_eq!(from_bits.a(), 0x44);
///     assert_eq!(from_bits.b(), 0x33);
///     assert_eq!(from_bits.c(), 0x22);
///     assert_eq!(from_bits.into_bits(), 0x11223344);
///
///     // from_bits_with_defaults - Creates a bitfield from raw bits, respecting 
///     // default values. Fields with defaults are set to their default values.
///     let from_bits_with_defaults = Bitfield::from_bits_with_defaults(0x11_22_33_44);
///     assert_eq!(from_bits_with_defaults.a(), 0x12);
///     assert_eq!(from_bits_with_defaults.b(), 0x34);
///     assert_eq!(from_bits_with_defaults.c(), 0x56);
///     assert_eq!(from_bits_with_defaults.into_bits(), 0x78563412);
///
///     // from_le_bits - Creates a bitfield from bits assumed to be in 
///     // little-endian order, ignoring defaults.
///     let from_le_bits = Bitfield::from_le_bits(0x11_22_33_44);
///     assert_eq!(from_le_bits.into_bits(), 0x44_33_22_11);
///
///     // from_le_bits_with_defaults - Creates a bitfield from bits assumed to be 
///     // in little-endian order, respecting default values.
///     let from_le_bits_with_defaults = Bitfield::from_le_bits_with_defaults(0x11_22_33_44);
///     assert_eq!(
///         from_le_bits_with_defaults.into_bits(),
///         0x78_56_34_12
///     );
///
///     // from_be_bits - Creates a bitfield from bits assumed to be in 
///     // big-endian order, ignoring defaults.
///     let from_be_bits = Bitfield::from_be_bits(0x11_22_33_44);
///     assert_eq!(from_be_bits.into_bits(), 0x11_22_33_44);
///
///     // from_be_bits_with_defaults - Creates a bitfield from bits assumed to be 
///     // in big-endian order, respecting default values.
///     let from_be_bits_with_defaults = Bitfield::from_be_bits_with_defaults(0x11_22_33_44);
///     assert_eq!(
///         from_be_bits_with_defaults.into_bits(),
///         0x78_56_34_12,
///     );
/// }
/// ```
///
/// **Array Backed Bitfield**:
///
/// | Method                                                                    | Endianness                                   | Description                                                                                                                                            |
/// |---------------------------------------------------------------------------|----------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | `from_bytes(bytes: [u8; N])`                                              | Determined by `#[bitfield(from_endian = N)]` | Creates a new bitfield instance from the given bytes, ignoring field defaults. (Default is `big`).                                                     |
/// | `from_bytes_with_defaults(bytes: [u8; N])`                                | Determined by `#[bitfield(from_endian = N)]` | Creates a new bitfield instance from the given bytes, while respecting/applying field defaults.                                                        |
/// | `from_le_bytes(bytes: [u8; N])`                                           | Little-endian                                | Creates a new bitfield instance from the given little-endian bytes, ignoring field defaults.                                                           |
/// | `from_le_bytes_with_defaults(bytes: [u8; N])`                             | Little-endian                                | Creates a new bitfield instance from the given little-endian bytes, while respecting/applying field defaults.                                          |
/// | `from_be_bytes(bytes: [u8; N])`                                           | Big-endian                                   | Creates a new bitfield instance from the given big-endian bytes, ignoring field defaults.                                                              |
/// | `from_be_bytes_with_defaults(bytes: [u8; N])`                             | Big-endian                                   | Creates a new bitfield instance from the given big-endian bytes, while respecting/applying field defaults.                                             |
/// | `from_slice(slice: &[u8])`                                                | Determined by `#[bitfield(from_endian = N)]` | Creates a new bitfield instance from a byte slice. Shorter slices are padded with zero.                                                                |
/// | `from_slice_with_defaults(slice: &[u8])`                                  | Determined by `#[bitfield(from_endian = N)]` | Creates a new bitfield instance from a byte slice, while respecting/applying field defaults. Shorter slices are padded with zero.                      |
/// | `checked_from_slice(slice: &[u8]) -> Result<Self, &str>`                  | Determined by `#[bitfield(from_endian = N)]` | Creates a new bitfield instance from a byte slice. Returns an error if the slice is too small to fill the bitfield.                                    |
/// | `checked_from_slice_with_defaults(slice: &[u8]) -> Result<Self, &str>`    | Determined by `#[bitfield(from_endian = N)]` | Creates a new bitfield instance from a byte slice, while respecting/applying field defaults. Returns an error if the slice is too small.               |
/// | `from_le_slice(slice: &[u8])`                                             | Little-endian                                | Creates a new bitfield instance from a little-endian byte slice. Shorter slices are padded with zero.                                                  |
/// | `from_le_slice_with_defaults(slice: &[u8])`                               | Little-endian                                | Creates a new bitfield instance from a little-endian byte slice, while respecting/applying field defaults. Shorter slices are padded with zero.        |
/// | `checked_from_le_slice(slice: &[u8]) -> Result<Self, &str>`               | Little-endian                                | Creates a new bitfield instance from a little-endian byte slice. Returns an error if the slice is too small to fill the bitfield.                      |
/// | `checked_from_le_slice_with_defaults(slice: &[u8]) -> Result<Self, &str>` | Little-endian                                | Creates a new bitfield instance from a little-endian byte slice, while respecting/applying field defaults. Returns an error if the slice is too small. |
/// | `from_be_slice(slice: &[u8])`                                             | Big-endian                                   | Creates a new bitfield instance from a big-endian byte slice. Shorter slices are padded with zero.                                                     |
/// | `from_be_slice_with_defaults(slice: &[u8])`                               | Big-endian                                   | Creates a new bitfield instance from a big-endian byte slice, while respecting/applying field defaults. Shorter slices are padded with zero.           |
/// | `checked_from_be_slice(slice: &[u8]) -> Result<Self, &str>`               | Big-endian                                   | Creates a new bitfield instance from a big-endian byte slice. Returns an error if the slice is too small to fill the bitfield.                         |
/// | `checked_from_be_slice_with_defaults(slice: &[u8]) -> Result<Self, &str>` | Big-endian                                   | Creates a new bitfield instance from a big-endian byte slice, while respecting/applying field defaults. Returns an error if the slice is too small.    |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield([u8; 4])]
/// struct Packet {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     _reserved: u8,
/// }
///
/// fn main() {
///     // from_bytes — default endian is big, ignoring field defaults.
///     // In big-endian layout the last byte maps to the lowest bits (field `a`).
///     let p = Packet::from_bytes([0x11, 0x22, 0x33, 0x44]);
///     assert_eq!(p.a(), 0x44);
///     assert_eq!(p.b(), 0x33);
///     assert_eq!(p.c(), 0x22);
///     assert_eq!(p.into_bytes(), [0x11, 0x22, 0x33, 0x44]);
///
///     // from_bytes_with_defaults — fields that have defaults keep their defaults.
///     let p_defaults = Packet::from_bytes_with_defaults([0x11, 0x22, 0x33, 0x44]);
///     assert_eq!(p_defaults.a(), 0x12);
///     assert_eq!(p_defaults.b(), 0x34);
///     assert_eq!(p_defaults.c(), 0x56);
///
///     // from_le_bytes — treats the byte array as little-endian.
///     let p_le = Packet::from_le_bytes([0x44, 0x33, 0x22, 0x11]);
///     assert_eq!(p_le.a(), 0x44);
///     assert_eq!(p_le.b(), 0x33);
///     assert_eq!(p_le.c(), 0x22);
///
///     // from_be_bytes — treats the byte array as big-endian (same as from_bytes default).
///     let p_be = Packet::from_be_bytes([0x11, 0x22, 0x33, 0x44]);
///     assert_eq!(p_be.a(), 0x44);
///
///     // from_slice — shorter slices are zero-padded; same endian rules as from_bytes.
///     let p_slice = Packet::from_slice(&[0x11, 0x22, 0x33, 0x44]);
///     assert_eq!(p_slice.a(), 0x44);
///
///     // checked_from_slice — returns an error when the slice is too small.
///     assert!(Packet::checked_from_slice(&[0x11, 0x22]).is_err());
///     assert!(Packet::checked_from_slice(&[0x11, 0x22, 0x33, 0x44]).is_ok());
/// }
/// ```
///
/// #### Into Bits
///
/// A bitfield can be converted into bits using the following APIs:
///
/// **Primitive Bitfield**:
///
/// | Method                | Endianness                                   | Description                                                            |
/// |-----------------------|----------------------------------------------|------------------------------------------------------------------------|
/// | `into_bits() -> N`    | Determined by `#[bitfield(into_endian = N)]` | Returns the raw bits of the bitfield (default `into_endian` is `big`). |
/// | `into_le_bits() -> N` | Little-endian                                | Returns the bits of the bitfield in little-endian order.               |
/// | `into_be_bits() -> N` | Big-endian                                   | Returns the bits of the bitfield in big-endian order.                  |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     d: u8,
/// }
///
/// fn main() {
///     let bitfield = Bitfield::new();
///
///     // into_bits: default into_endian (big) — most-significant field at the top byte.
///     assert_eq!(bitfield.into_bits(), 0x78_56_34_12);
///
///     // into_le_bits: byte-swaps the raw bits into little-endian order.
///     assert_eq!(bitfield.into_le_bits(), 0x12_34_56_78);
///
///     // into_be_bits: big-endian order (same as into_bits when into_endian defaults to big).
///     assert_eq!(bitfield.into_be_bits(), 0x78_56_34_12);
/// }
/// ```
///
/// **Array Backed Bitfield**:
///
/// | Method                                                        | Endianness                                   | Description                                                                                                           |
/// |---------------------------------------------------------------|----------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
/// | `into_bytes() -> [u8; N]`                                     | Determined by `#[bitfield(into_endian = N)]` | Returns the bytes of the bitfield as a `[u8; N]` array. Default is `big`.                                             |
/// | `into_le_bytes() -> [u8; N]`                                  | Little-endian                                | Returns the bytes of the bitfield in little-endian order.                                                             |
/// | `into_be_bytes() -> [u8; N]`                                  | Big-endian                                   | Returns the bytes of the bitfield in big-endian order.                                                                |
/// | `into_slice(slice: &mut [u8])`                                | Determined by `#[bitfield(into_endian = N)]` | Writes the bitfield bytes into the provided slice. If the slice is shorter, only the bytes that fit are written.      |
/// | `checked_into_slice(slice: &mut [u8]) -> Result<(), &str>`    | Determined by `#[bitfield(into_endian = N)]` | Writes the bitfield bytes into the provided slice. Returns an error if the slice is too small.                        |
/// | `into_le_slice(slice: &mut [u8])`                             | Little-endian                                | Writes the bitfield bytes in little-endian order into the provided slice.                                             |
/// | `checked_into_le_slice(slice: &mut [u8]) -> Result<(), &str>` | Little-endian                                | Writes the bitfield bytes in little-endian order into the provided slice. Returns an error if the slice is too small. |
/// | `into_be_slice(slice: &mut [u8])`                             | Big-endian                                   | Writes the bitfield bytes in big-endian order into the provided slice.                                                |
/// | `checked_into_be_slice(slice: &mut [u8]) -> Result<(), &str>` | Big-endian                                   | Writes the bitfield bytes in big-endian order into the provided slice. Returns an error if the slice is too small.    |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield([u8; 4])]
/// struct Packet {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     d: u8,
/// }
///
/// fn main() {
///     let packet = Packet::new();
///
///     // into_bytes: default into_endian (big) — most-significant byte first.
///     assert_eq!(packet.into_bytes(), [0x78, 0x56, 0x34, 0x12]);
///
///     // into_le_bytes: least-significant byte first.
///     assert_eq!(packet.into_le_bytes(), [0x12, 0x34, 0x56, 0x78]);
///
///     // into_be_bytes: most-significant byte first (same as default).
///     assert_eq!(packet.into_be_bytes(), [0x78, 0x56, 0x34, 0x12]);
///
///     // into_slice: writes up to slice.len() bytes.
///     let mut buf = [0u8; 4];
///     packet.into_slice(&mut buf);
///     assert_eq!(buf, [0x78, 0x56, 0x34, 0x12]);
///
///     // checked_into_slice: returns Err when the slice is too small.
///     let mut small = [0u8; 2];
///     assert!(packet.checked_into_slice(&mut small).is_err());
///
///     // into_le_slice / into_be_slice work the same but with explicit endian.
///     let mut le_buf = [0u8; 4];
///     packet.into_le_slice(&mut le_buf);
///     assert_eq!(le_buf, [0x12, 0x34, 0x56, 0x78]);
///
///     let mut be_buf = [0u8; 4];
///     packet.into_be_slice(&mut be_buf);
///     assert_eq!(be_buf, [0x78, 0x56, 0x34, 0x12]);
/// }
/// ```
///
/// #### Conversion Endian
///
/// Sometimes the outside world is outside our control, like the endianness of how
/// systems export data. Luckily, the endianness of the bitfield conversions can
/// be controlled by specifying the `#[bitfield(from_endian = N, into_endian = N)]`
/// args. The possible endianness options are `little` or `big`. By default, the endianness
/// of both is `big`.
///
/// This arg controls the endianness of the `from`, `into`, and `From` trait
/// conversions.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32, from_endian = little, into_endian = big)]
/// struct Header {
///     version: u8,
///     flags: u8,
///     size: u8,
///     tag: u8,
/// }
///
/// fn main() {
///     // Device gave us bytes in little-endian order:
///     // [0x11, 0x22, 0x33, 0x44]
///     let h = Header::from_bits(0x1122_3344);
///
///     // from_endian = little means fields decode as:
///     assert_eq!(h.version(), 0x11);
///     assert_eq!(h.flags(), 0x22);
///     assert_eq!(h.size(), 0x33);
///     assert_eq!(h.tag(), 0x44);
///
///     // into_endian = big means output is big-endian representation.
///     assert_eq!(h.into_bits(), 0x4433_2211);
///
///     // Explicit conversion helpers still work:
///     let x = Header::from_le_bits(0xAABB_CCDD);
///     assert_eq!(x.version(), 0xAA);
///     assert_eq!(x.flags(), 0xBB);
///     assert_eq!(x.size(), 0xCC);
///     assert_eq!(x.tag(), 0xDD);
///
///     assert_eq!(x.into_be_bits(), 0xDDCC_BBAA);
///     assert_eq!(x.into_le_bits(), 0xAABB_CCDD);
/// }
/// ```
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield([u8; 4], from_endian = little, into_endian = big)]
/// struct Header {
///     version: u8,
///     flags: u8,
///     size: u8,
///     tag: u8,
/// }
///
/// fn main() {
///     // Device gave us bytes in little-endian order:
///     // [0x11, 0x22, 0x33, 0x44]
///     let h = Header::from_bytes([0x11, 0x22, 0x33, 0x44]);
///
///     // from_endian = little means fields decode as:
///     assert_eq!(h.version(), 0x11);
///     assert_eq!(h.flags(), 0x22);
///     assert_eq!(h.size(), 0x33);
///     assert_eq!(h.tag(), 0x44);
///
///     // into_endian = big means output is big-endian representation.
///     assert_eq!(h.into_bytes(), [0x44, 0x33, 0x22, 0x11]);
///
///     // Explicit conversion helpers still work:
///     let x = Header::from_le_bytes([0xDD, 0xCC, 0xBB, 0xAA]);
///     assert_eq!(x.version(), 0xDD);
///     assert_eq!(x.flags(), 0xCC);
///     assert_eq!(x.size(), 0xBB);
///     assert_eq!(x.tag(), 0xAA);
///
///     assert_eq!(x.into_be_bytes(), [0xAA, 0xBB, 0xCC, 0xDD]);
///     assert_eq!(x.into_le_bytes(), [0xDD, 0xCC, 0xBB, 0xAA]);
/// }
/// ```
///
/// ### Bit Operations
///
/// The bitfield generates bitwise operations that make it easy to manipulate,
/// read, reset, and invert the backing bits without manually writing masks and
/// shifts.
///
/// #### Write Bits
///
/// Write bits allows you to write data to the writable bits of
/// the bitfield.
///
/// **Primitive Bitfield**:
///
/// | Function                               | Endianness                                    | Description                                                    |
/// |----------------------------------------|-----------------------------------------------|----------------------------------------------------------------|
/// | `write_bits(bits: N)`                  | Determined by `#[bitfield(write_endian = N)]` | Writes raw bits using `write_endian` (default is `big`).       |
/// | `write_bits_with_defaults(bits: N)`    | Determined by `#[bitfield(write_endian = N)]` | Writes raw bits, then reapplies defaults.                      |
/// | `write_le_bits(bits: N)`               | Little-endian                                 | Writes explicitly little-endian bits.                          |
/// | `write_le_bits_with_defaults(bits: N)` | Little-endian                                 | Writes explicitly little-endian bits, then reapplies defaults. |
/// | `write_be_bits(bits: N)`               | Big-endian                                    | Writes explicitly big-endian bits.                             |
/// | `write_be_bits_with_defaults(bits: N)` | Big-endian                                    | Writes explicitly big-endian bits, then reapplies defaults.    |
/// | `write_defaults()`                     | N/A                                           | Reapplies field defaults without replacing the whole bitfield. |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Register {
///     #[bits(default = 0x12)]
///     low: u8,
///     data: u8,
///     #[bits(default = 0x78)]
///     _reserved: u8,
///     high: u8,
/// }
///
/// fn main() {
///     let mut reg = Register::new();
///     assert_eq!(reg.into_bits(), 0x00780012);
///
///     // write_bits — replaces all writable bits; reserved/read-only fields are preserved.
///     reg.write_bits(0xAA_BB_CC_DD);
///     assert_eq!(reg.low(), 0xDD);
///     assert_eq!(reg.data(), 0xCC);
///     assert_eq!(reg.high(), 0xAA);
///     // layout: high=0xAA | _reserved=0x78 | data=0xCC | low=0xDD
///     assert_eq!(reg.into_bits(), 0xAA78CCDD);
///
///     // write_bits_with_defaults — writes the new bits then reapplies field defaults.
///     reg.write_bits_with_defaults(0xAA_BB_CC_DD);
///     assert_eq!(reg.low(), 0x12); // default restored
///     assert_eq!(reg.data(), 0xCC); // no default, comes from the written value
///     // layout: high=0xAA | _reserved=0x78 | data=0xCC | low=0x12
///     assert_eq!(reg.into_bits(), 0xAA78CC12);
///
///     // write_le_bits — writes bits interpreted as little-endian.
///     reg.write_le_bits(0x11_22_33_44);
///     assert_eq!(reg.low(), 0x11);
///     assert_eq!(reg.data(), 0x22);
///     assert_eq!(reg.high(), 0x44);
///
///     // write_defaults — reapplies only field defaults without overwriting other fields.
///     reg.write_defaults();
///     assert_eq!(reg.low(), 0x12);
/// }
/// ```
///
/// **Array Backed Bitfield**:
///
/// | Function                                       | Endianness                                    | Description                                                     |
/// |------------------------------------------------|-----------------------------------------------|-----------------------------------------------------------------|
/// | `write_bytes(bytes: [u8; N])`                  | Determined by `#[bitfield(write_endian = N)]` | Writes raw bytes using `write_endian` (default is `big`).       |
/// | `write_bytes_with_defaults(bytes: [u8; N])`    | Determined by `#[bitfield(write_endian = N)]` | Writes raw bytes, then reapplies defaults.                      |
/// | `write_le_bytes(bytes: [u8; N])`               | Little-endian                                 | Writes explicitly little-endian bytes.                          |
/// | `write_le_bytes_with_defaults(bytes: [u8; N])` | Little-endian                                 | Writes explicitly little-endian bytes, then reapplies defaults. |
/// | `write_be_bytes(bytes: [u8; N])`               | Big-endian                                    | Writes explicitly big-endian bytes.                             |
/// | `write_be_bytes_with_defaults(bytes: [u8; N])` | Big-endian                                    | Writes explicitly big-endian bytes, then reapplies defaults.    |
/// | `write_defaults()`                             | N/A                                           | Reapplies field defaults without replacing the whole bitfield.  |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Register {
///     #[bits(default = 0x12)]
///     low: u8,
///     #[bits(default = 0x34, access = ro)]
///     status: u8,
///     data: u8,
///     #[bits(default = 0x78)]
///     _reserved: u8,
/// }
///
/// fn main() {
///     let mut register = Register::new();
///     assert_eq!(register.into_bits(), 0x78003412);
///
///     register.write_bits(0x11223344);
///
///     // Writable fields are updated from the incoming bits.
///     assert_eq!(register.low(), 0x44);
///     assert_eq!(register.data(), 0x22);
///
///     // Read-only and reserved fields keep their original/default values.
///     assert_eq!(register.status(), 0x34);
///     assert_eq!(register.into_bits(), 0x78223444);
///
///     // `write_bits_with_defaults` writes first, then restores fields that have
///     // defaults. `low`, `status`, and `_reserved` all keep their defaults.
///     register.write_bits_with_defaults(0xAABBCCDD);
///     assert_eq!(register.low(), 0x12);
///     assert_eq!(register.status(), 0x34);
///     assert_eq!(register.data(), 0xBB);
///     assert_eq!(register.into_bits(), 0x78BB3412);
/// }
/// ```
///
/// Use `write_endian` to configure the default endian used by `write_bits` or
/// `write_bytes`. The explicit helpers always use the endian in their name.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32, write_endian = little)]
/// struct Header {
///     version: u8,
///     flags: u8,
///     size: u8,
///     tag: u8,
/// }
///
/// fn main() {
///     let mut header = Header::new();
///
///     // `write_endian = little` makes `write_bits` interpret the input as
///     // little-endian. This is equivalent to calling `write_le_bits`.
///     header.write_bits(0x1122_3344);
///     assert_eq!(header.version(), 0x11);
///     assert_eq!(header.flags(), 0x22);
///     assert_eq!(header.size(), 0x33);
///     assert_eq!(header.tag(), 0x44);
///
///     // Explicit endian helpers ignore the configured default.
///     header.write_be_bits(0xAABB_CCDD);
///     assert_eq!(header.version(), 0xDD);
///     assert_eq!(header.flags(), 0xCC);
///     assert_eq!(header.size(), 0xBB);
///     assert_eq!(header.tag(), 0xAA);
/// }
/// ```
///
/// #### Get/Set Bits
///
/// Single-bit helpers let you read or write one bit at a time. These helpers behave
/// the same for both primitive and array-backed bitfields.
///
/// **Primitive & Array Backed Bitfields**:
///
/// | Function                                                      | Checked / Unchecked | Description                                                                                                       |
/// |---------------------------------------------------------------|---------------------|-------------------------------------------------------------------------------------------------------------------|
/// | `get_bit(offset: u32) -> bool`                                | Unchecked           | Reads a single bit at the given offset. Returns false if offset is out of bounds or field is inaccessible.        |
/// | `checked_get_bit(offset: u32) -> Result<bool, &str>`          | Checked             | Reads a single bit at the given offset. Returns an error if the offset is out of bounds or field is inaccessible. |
/// | `set_bit(offset: u32, bit: bool)`                             | Unchecked           | Writes a single bit at the given offset. No-op if offset is out of bounds or field is protected.                  |
/// | `checked_set_bit(offset: u32, bit: bool) -> Result<(), &str>` | Checked             | Writes a single bit at the given offset. Returns an error if the offset is out of bounds or field is protected.   |
///
/// Unchecked helpers are convenient for quick probing. Checked helpers are better
/// when invalid offsets or inaccessible fields should be treated as errors.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u8, bit_ops = true)]
/// struct Control {
///     #[bits(2, default = 0b11)]
///     mode: u8,
///
///     #[bits(2)]
///     flags: u8,
///
///     #[bits(2, default = 0b10, access = wo)]
///     command: u8,
///
///     #[bits(2, default = 0b01)]
///     _reserved: u8,
/// }
///
/// fn main() {
///     let mut control = Control::new();
///
///     assert!(control.get_bit(Control::MODE_OFFSET));
///     assert!(control.get_bit(Control::MODE_OFFSET + 1));
///     assert!(!control.get_bit(Control::FLAGS_OFFSET));
///
///     // `command` is write-only, so unchecked reads return false and checked
///     // reads return an error.
///     assert!(!control.get_bit(Control::COMMAND_OFFSET));
///     assert!(control.checked_get_bit(Control::COMMAND_OFFSET).is_err());
///
///     control.set_bit(Control::FLAGS_OFFSET, true);
///     control.set_bit(Control::FLAGS_OFFSET + 1, true);
///     assert_eq!(control.flags(), 0b11);
///
///     // Reserved bits are not writable. Unchecked writes are no-ops, while
///     // checked writes return an error.
///     control.set_bit(6, false);
///     assert!(control.get_bit(6));
///     assert!(control.checked_set_bit(6, false).is_err());
///
///     // Out-of-bounds unchecked reads return false; checked reads return an error.
///     assert!(!control.get_bit(99));
///     assert!(control.checked_get_bit(99).is_err());
/// }
/// ```
///
/// **Primitive Bitfield range helpers**:
///
/// | Function                                                                      | Checked / Unchecked | Description                                                                                                 |
/// |-------------------------------------------------------------------------------|---------------------|-------------------------------------------------------------------------------------------------------------|
/// | `get_bits_range(offset: u32, len: u32) -> N`                                  | Unchecked           | Reads a range of bits and shifts the result to bit 0. Returns 0 if invalid.                                 |
/// | `checked_get_bits_range(offset: u32, len: u32) -> Result<N, &str>`            | Checked             | Reads a range of bits and shifts them to bit 0. Returns an error on out of bounds.                          |
/// | `set_bits_range(offset: u32, len: u32, value: N)`                             | Unchecked           | Sets a range of bits to the given shifted value. No-op if out of bounds or protected.                       |
/// | `checked_set_bits_range(offset: u32, len: u32, value: N) -> Result<(), &str>` | Checked             | Sets a range of bits to the given shifted value. Returns an error if any bit is out of bounds or protected. |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16, bit_ops = true)]
/// struct Flags {
///     #[bits(4)]
///     mode: u8,
///
///     #[bits(4, default = 0xA)]
///     status: u8,
///
///     #[bits(4, access = ro)]
///     control: u8,
///
///     #[bits(4)]
///     extra: u8,
/// }
///
/// fn main() {
///     let mut flags = Flags::new();
///
///     // set_bits_range — sets `len` bits starting at `offset` to the given value.
///     flags.set_bits_range(Flags::MODE_OFFSET, Flags::MODE_BITS, 0x5);
///     assert_eq!(flags.mode(), 0x5);
///
///     // get_bits_range — reads `len` bits starting at `offset`, shifted to bit 0.
///     assert_eq!(flags.get_bits_range(Flags::MODE_OFFSET, Flags::MODE_BITS), 0x5);
///     assert_eq!(flags.get_bits_range(Flags::STATUS_OFFSET, Flags::STATUS_BITS), 0xA);
///
///     // Unchecked range writes silently skip protected (read-only) bits.
///     flags.set_bits_range(Flags::CONTROL_OFFSET, 8, 0xFF);
///     assert_eq!(flags.extra(), 0xF); // extra bits written
///     assert_eq!(flags.control(), 0x0); // read-only bits are unchanged
///
///     // checked_set_bits_range — returns an error if any bit in the range is protected.
///     assert!(flags.checked_set_bits_range(Flags::CONTROL_OFFSET, 8, 0xFF).is_err());
///
///     // checked_get_bits_range — returns an error if the range is out of bounds.
///     assert!(flags.checked_get_bits_range(12, 8).is_err());
/// }
/// ```
///
/// **Array Backed Bitfield range helpers**:
///
/// | Function                                                                             | Checked / Unchecked | Description                                                                                                   |
/// |--------------------------------------------------------------------------------------|---------------------|---------------------------------------------------------------------------------------------------------------|
/// | `get_bytes_range(offset: u32, len: u32) -> [u8; N]`                                  | Unchecked           | Reads a range of bits and shifts the result into a byte array starting at bit 0.                              |
/// | `checked_get_bytes_range(offset: u32, len: u32) -> Result<[u8; N], &str>`            | Checked             | Reads a range of bits and shifts them into a byte array starting at bit 0. Returns an error on out of bounds. |
/// | `set_bytes_range(offset: u32, len: u32, value: [u8; N])`                             | Unchecked           | Sets a range of bits using values starting at bit 0 of the input array.                                       |
/// | `checked_set_bytes_range(offset: u32, len: u32, value: [u8; N]) -> Result<(), &str>` | Checked             | Sets a range of bits using the input array. Returns an error if any bit is out of bounds or protected.        |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16, bit_ops = true)]
/// struct PacketWord {
///     low: u8,
///
///     #[bits(4)]
///     counter: u8,
///
///     #[bits(4, default = 0xF, access = ro)]
///     status: u8,
/// }
///
/// fn main() {
///     let mut packet = PacketWord::new();
///
///     packet.set_bits_range(PacketWord::LOW_OFFSET, PacketWord::LOW_BITS, 0xAB);
///     packet.set_bits_range(PacketWord::COUNTER_OFFSET, PacketWord::COUNTER_BITS, 0x5);
///
///     assert_eq!(packet.low(), 0xAB);
///     assert_eq!(packet.counter(), 0x5);
///     assert_eq!(packet.get_bits_range(PacketWord::LOW_OFFSET, PacketWord::LOW_BITS), 0xAB);
///     assert_eq!(packet.into_bits(), 0xF5AB);
///
///     // Unchecked range writes skip protected bits.
///     packet.set_bits_range(PacketWord::COUNTER_OFFSET, 8, 0x00);
///     assert_eq!(packet.counter(), 0x0);
///     assert_eq!(packet.status(), 0xF);
///
///     // Checked range writes fail if any bit in the range is protected.
///     assert!(packet.checked_set_bits_range(PacketWord::COUNTER_OFFSET, 8, 0x00).is_err());
/// }
/// ```
///
/// For array-backed bitfields, range values are packed starting at bit `0` of the
/// provided or returned array.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield([u8; 2], bit_ops = true)]
/// struct PacketBytes {
///     low: u8,
///     high: u8,
/// }
///
/// fn main() {
///     let mut packet = PacketBytes::new();
///
///     packet.set_bytes_range(PacketBytes::LOW_OFFSET, 12, [0xBC, 0x0A]);
///
///     assert_eq!(packet.low(), 0xBC);
///     assert_eq!(packet.high(), 0x0A);
///     assert_eq!(packet.get_bytes_range(PacketBytes::LOW_OFFSET, 12), [0xBC, 0x0A]);
///     assert_eq!(packet.into_le_bytes(), [0xBC, 0x0A]);
/// }
/// ```
///
/// #### Clear Bits
///
/// Clear helpers reset either the whole bitfield or individual writable fields.
///
/// **Primitive Bitfield**:
///
/// | Function                     | Description                                                                                |
/// |------------------------------|--------------------------------------------------------------------------------------------|
/// | `clear_bits()`               | Resets the bitfield to its zero representation (except reserved fields with defaults).     |
/// | `clear_bits_with_defaults()` | Resets the bitfield and then restores all field defaults.                                  |
/// | `clear_<field>()`            | Clears the specific writable field to zero. (Generated per writable field)                 |
/// | `clear_<field>_to_default()` | Restores the specific writable field's default value. (Generated per field with a default) |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Flags {
///     #[bits(default = 0x12)]
///     enabled: u8,
///
///     count: u8,
///
///     #[bits(default = 0x56)]
///     priority: u8,
///
///     #[bits(default = 0x78)]
///     _reserved: u8,
/// }
///
/// fn main() {
///     let mut flags = FlagsBuilder::new()
///         .with_enabled(0xFF)
///         .with_count(0x34)
///         .with_priority(0xAB)
///         .build();
///     assert_eq!(flags.into_bits(), 0x78AB34FF);
///
///     // clear_<field> — zeroes out a single writable field.
///     flags.clear_count();
///     assert_eq!(flags.count(), 0);
///
///     // clear_<field>_to_default — restores a single field to its default value.
///     flags.clear_enabled_to_default();
///     assert_eq!(flags.enabled(), 0x12);
///
///     // clear_bits — zeroes all writable bits; reserved fields keep their defaults.
///     flags.clear_bits();
///     assert_eq!(flags.enabled(), 0);
///     assert_eq!(flags.count(), 0);
///     assert_eq!(flags.priority(), 0);
///     assert_eq!(flags.into_bits(), 0x78000000); // _reserved keeps its default
///
///     // clear_bits_with_defaults — zeroes everything then restores field defaults.
///     flags.set_count(0x99);
///     flags.clear_bits_with_defaults();
///     assert_eq!(flags.enabled(), 0x12);
///     assert_eq!(flags.count(), 0);
///     assert_eq!(flags.priority(), 0x56);
///     assert_eq!(flags.into_bits(), 0x78560012);
/// }
/// ```
///
/// **Array Backed Bitfield**:
///
/// | Function                      | Description                                                                                |
/// |-------------------------------|--------------------------------------------------------------------------------------------|
/// | `clear_bytes()`               | Resets the bitfield bytes to zero (except reserved fields with defaults).                  |
/// | `clear_bytes_with_defaults()` | Resets the bitfield bytes and then restores all field defaults.                            |
/// | `clear_<field>()`             | Clears the specific writable field to zero. (Generated per writable field)                 |
/// | `clear_<field>_to_default()`  | Restores the specific writable field's default value. (Generated per field with a default) |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield([u8; 4])]
/// struct Flags {
///     #[bits(default = 0x12)]
///     enabled: u8,
///
///     count: u8,
///
///     #[bits(default = 0x56, access = ro)]
///     status: u8,
///
///     #[bits(default = 0x78)]
///     _reserved: u8,
/// }
///
/// fn main() {
///     let mut flags = FlagsBuilder::new().with_count(0x34).build();
///     assert_eq!(flags.into_bytes(), [0x78, 0x56, 0x34, 0x12]);
///
///     flags.clear_enabled();
///     assert_eq!(flags.enabled(), 0);
///
///     flags.clear_enabled_to_default();
///     assert_eq!(flags.enabled(), 0x12);
///
///     flags.clear_bytes();
///     assert_eq!(flags.enabled(), 0);
///     assert_eq!(flags.count(), 0);
///     assert_eq!(flags.status(), 0);
///     assert_eq!(flags.into_bytes(), [0x78, 0, 0, 0]);
///
///     flags.clear_bytes_with_defaults();
///     assert_eq!(flags.enabled(), 0x12);
///     assert_eq!(flags.count(), 0);
///     assert_eq!(flags.status(), 0x56);
///     assert_eq!(flags.into_bytes(), [0x78, 0x56, 0, 0x12]);
/// }
/// ```
///
/// #### Invert Bits
///
/// Invert helpers flip writable bits while preserving protected bits. They are
/// useful for toggling flags, building masks, and inspecting the complement of a
/// field without manually calculating a field-sized mask.
///
/// **Primitive Bitfield**:
///
/// | Function                  | Description                                                                                                   |
/// |---------------------------|---------------------------------------------------------------------------------------------------------------|
/// | `invert_bits()`           | Flips every writable bit in the primitive bitfield (read-only and reserved fields are untouched).             |
/// | `invert_<field>()`        | Mutates a writable field by flipping only its bits. (Generated per writable field)                            |
/// | `<field>_inverted() -> T` | Returns the field value with its bits inverted, without mutating the bitfield. (Generated per readable field) |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16, bit_ops = true)]
/// struct Control {
///     #[bits(4, default = 0b0101)]
///     mode: u8,
///
///     #[bits(4, default = 0b1010)]
///     flags: u8,
///
///     #[bits(8, default = 0xA5, access = ro)]
///     checksum: u8,
/// }
///
/// fn main() {
///     let mut control = Control::new();
///
///     // <field>_inverted() — returns the inverted value without mutating the bitfield.
///     assert_eq!(control.mode_inverted(), 0b1010); // inverted within 4 bits
///     assert_eq!(control.flags_inverted(), 0b0101);
///     assert_eq!(control.mode(), 0b0101); // original value unchanged
///
///     // invert_<field>() — flips only the bits of a single writable field.
///     control.invert_mode();
///     assert_eq!(control.mode(), 0b1010);
///
///     // invert_bits() — flips all writable bits; read-only fields are preserved.
///     control.invert_bits();
///     assert_eq!(control.mode(), 0b0101);   // back to original
///     assert_eq!(control.flags(), 0b0101);  // also inverted
///     assert_eq!(control.checksum(), 0xA5); // read-only, untouched
/// }
/// ```
///
/// **Array Backed Bitfield**:
///
/// | Function                  | Description                                                                                                   |
/// |---------------------------|---------------------------------------------------------------------------------------------------------------|
/// | `invert_bytes()`          | Flips every writable bit in the array-backed bitfield (read-only and reserved fields are untouched).          |
/// | `invert_<field>()`        | Mutates a writable field by flipping only its bits. (Generated per writable field)                            |
/// | `<field>_inverted() -> T` | Returns the field value with its bits inverted, without mutating the bitfield. (Generated per readable field) |
///
/// The inversion is limited to the field width. For example, inverting a 5-bit
/// field value `0b01100` returns `0b10011`, not an 8-bit `0b11110011`.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield([u8; 2], bit_ops = true)]
/// struct Mask {
///     #[bits(5, default = 0b0_1100)]
///     pattern: u8,
///
///     #[bits(1, default = true)]
///     enabled: bool,
///
///     #[bits(2)]
///     flags: u8,
///
///     #[bits(8, default = 0xA5, access = ro)]
///     checksum: u8,
/// }
///
/// fn main() {
///     let mut mask = Mask::new();
///
///     assert_eq!(mask.pattern(), 0b0_1100);
///     assert_eq!(mask.pattern_inverted(), 0b1_0011);
///     assert!(!mask.enabled_inverted());
///
///     mask.invert_pattern();
///     assert_eq!(mask.pattern(), 0b1_0011);
///
///     mask.invert_enabled();
///     assert!(!mask.enabled());
///
///     mask.invert_bytes();
///
///     // Writable fields are inverted.
///     assert_eq!(mask.pattern(), 0b0_1100);
///     assert!(mask.enabled());
///     assert_eq!(mask.flags(), 0b11);
///
///     // Read-only fields are preserved.
///     assert_eq!(mask.checksum(), 0xA5);
/// }
/// ```
///
/// ### Passing Attributes
///
/// Attributes below the `#[bitfield]` attribute are passed to the generated struct.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// #[derive(Hash)]
/// struct Bitfield {
///     a: u32,
/// }
/// ```
///
/// ### Ignored Fields
///
/// Fields with the `#[bits(ignore = true)]` are ignored and not included
/// in the bitfield. This is useful for when you are building a custom
/// bitfield, but want to wrap bitfield in a parent struct to add additional
/// non-bitfield fields.
///
/// All ignored fields must implement the `Default` trait. Ignored fields
/// are accessible, and you can control their visibility like normal fields.
///
/// Take note that using ignored fields removes some constant guarantees.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// struct Bitfield {
///     a: u8,
///     b: u8,
///     #[bits(ignore = true)] /// Ignored field.
///     field_id: u8,
///     #[bits(ignore = true)] /// Ignored field.
///     pub(crate) custom_type: CustomType,
/// }
///
/// #[derive(Debug, Default, Clone, Copy, PartialEq)]
/// enum CustomType {
///     #[default]
///     A,
///     B,
/// }
///
/// fn main() {
///     let bitfield = Bitfield::new();
///
///     assert_eq!(bitfield.field_id, 0); // Ignored fields can be accessed directly.
///     assert_eq!(bitfield.custom_type, CustomType::A); // Ignored fields can be
///     // accessed directly.
/// }
/// ```
///
/// ### Visibility
///
/// Visibility applied to structs and fields determine the visibility of the
/// generated struct and field accessors.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// pub struct Bitfield { // The struct is public, so `Bitfield` is public.
///     pub a: u8, // `a` is public, so `a()` is public.
///     b: u8,     // `b` is private, so `b()` is private.
/// }
/// ```
///
/// ### Default Implementations
///
/// #### Debug Trait
///
/// A debug implementation is generated for the bitfield, which prints the fields
/// and their values.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u32)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     #[bits(default = 0x34)]
///     b: u8,
///     #[bits(default = 0x56)]
///     c: u8,
///     #[bits(default = 0x78)]
///     d: u8,
/// }
///
/// fn main() {
///     let bitfield = Bitfield::new();
///
///     assert_eq!(format!("{:?}", bitfield), "Bitfield { a: 18, b: 52, c: 86, d: 120 }");
/// }
/// ```
///
/// #### Default Trait
///
/// A default implementation is generated for the bitfield, which initializes the
/// bitfield to zero and sets default values. This can be disabled by specifying
/// `#[bitfield(default = false)]` on the bitfield.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// struct Bitfield {
///     #[bits(default = 0x12)]
///     a: u8,
///     b: u8,
/// }
///
/// fn main() {
///     let bitfield = Bitfield::default();
///
///     assert_eq!(bitfield.a(), 0x12);
///     assert_eq!(bitfield.b(), 0);
/// }
/// ```
///
/// #### From/Into Trait
///
/// From and Into trait implementations are generated for the bitfield, which
/// convert to and from the bitfield type. This can be disabled by specifying
/// `#[bitfield(from_traits = false)]` on the bitfield.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16)] /// From<u16> and Into<u16> are generated for this bitfield.
/// struct Bitfield {
///     low: u8,
///     high: u8,
/// }
///
/// fn main() {
///     // `From<u16> for Bitfield`
///     let bitfield = Bitfield::from(0xABCD_u16);
///     assert_eq!(bitfield.low(), 0xCD);
///     assert_eq!(bitfield.high(), 0xAB);
///
///     // `From<Bitfield> for u16`
///     let bits = u16::from(bitfield);
///     assert_eq!(bits, 0xABCD);
///
///     // Equivalent `Into` usage
///     let bitfield_from_into: Bitfield = 0x1234_u16.into();
///     let bits_from_into: u16 = bitfield_from_into.into();
///     assert_eq!(bits_from_into, 0x1234);
/// }
/// ```
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield([u8; 2]
/// )] /// From<[u8; 2]> and Into<[u8; 2]> are generated for this bitfield.
/// struct Bitfield {
///     low: u8,
///     high: u8,
/// }
///
/// fn main() {
///     // `From<[u8; 2]> for Bitfield`
///     let bitfield = Bitfield::from([0xAB, 0xCD]);
///     assert_eq!(bitfield.low(), 0xCD);
///     assert_eq!(bitfield.high(), 0xAB);
///
///     // `From<Bitfield> for [u8; 2]`
///     let bytes = <[u8; 2]>::from(bitfield);
///     assert_eq!(bytes, [0xAB, 0xCD]);
///
///     // Equivalent `Into` usage
///     let bitfield_from_into: Bitfield = [0x12, 0x34].into();
///     let bytes_from_into: [u8; 2] = bitfield_from_into.into();
///     assert_eq!(bytes_from_into, [0x12, 0x34]);
/// }
/// ```
///
/// ### Bitfield Internal Value
///
/// The internal value of the bitfield is stored as either a tuple struct where it's
/// the first parameter, or as a field named `val` if ignored fields are present.
/// This allows you to access the internal value of the bitfield directly if needed,
/// but it is recommended to use the provided APIs to access and modify the
/// bitfield.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(u16)]
/// struct TupleBitfield {
///     low: u8,
///     high: u8,
/// }
///
/// #[bitfield(u16)]
/// struct ValBitfield {
///     low: u8,
///     high: u8,
///     #[bits(ignore = true)]
///     tag: u8,
/// }
///
/// fn main() {
///     let tuple_bf = TupleBitfield::new();
///     println!("{}", tuple_bf.0);
///
///     let val_bf = ValBitfield::new();
///     println!("{:?}", val_bf.val);
/// }
/// ```
///
/// ### Configuration
///
/// Bitfields can be configured with arguments passed to the `#[bitfield(...)]`
/// attribute. The first argument is always the backing storage type; every other
/// argument is named.
///
/// Boolean arguments accept `true` or `false`. The defaults below are the defaults
/// when no Cargo feature overrides are enabled. Per-bitfield attribute arguments
/// override Cargo feature defaults.
///
/// ```rust,ignore
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// #[bitfield(
///     u32,
///     order = lsb,
///     from_endian = big,
///     into_endian = big,
///     write_endian = big,
///     new = true,
///     from_into_bits = true,
///     from_traits = true,
///     default = true,
///     debug = true,
///     copy = true,
///     builder = true,
///     bit_ops = true,
///     write_bit_ops = true,
///     clear_bit_ops = true,
///     set_get_bit_ops = true,
///     invert_bit_ops = true,
///     toggle_bit_ops = true,
///     array_heap = false,
/// )]
/// struct Example {
///     a: u32,
/// }
/// ```
///
/// | Argument          | Values                                          | Default  | Description                                                                                                                                                                                                                                                                                                      |
/// |-------------------|-------------------------------------------------|----------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | `<backing type>`  | `u8`, `u16`, `u32`, `u64`, `u128`, or `[u8; N]` | Required | The storage used by the generated bitfield. Primitive backing types support bitfields up to 128 bits. `[u8; N]` creates an array-backed bitfield for larger layouts. Field bit widths, excluding ignored fields, must add up exactly to the backing storage size.                                                |
/// | `order`           | `lsb`, `msb`                                    | `lsb`    | Controls how struct fields are assigned to bit offsets. `lsb` assigns the first non-ignored field to the least-significant bits. `msb` assigns the first non-ignored field to the most-significant bits.                                                                                                         |
/// | `from_endian`     | `big`, `little`                                 | `big`    | Default endian used by `from_bits`, `from_bytes`, `from_slice`, and `From<Backing> for Bitfield`. Explicit helpers such as `from_le_bits` and `from_be_bytes` ignore this setting.                                                                                                                               |
/// | `into_endian`     | `big`, `little`                                 | `big`    | Default endian used by `into_bits`, `into_bytes`, `into_slice`, and `From<Bitfield> for Backing`. Explicit helpers such as `into_le_bits` and `into_be_bytes` ignore this setting.                                                                                                                               |
/// | `write_endian`    | `big`, `little`                                 | `big`    | Default endian used by whole-bitfield write helpers such as `write_bits` and `write_bytes`. Explicit helpers such as `write_le_bits` and `write_be_bytes` ignore this setting.                                                                                                                                   |
/// | `new`             | `true`, `false`                                 | `true`   | Generates `new()` and `new_without_defaults()` constructors. Other generated features that need construction logic, such as `Default` and the builder, still inline equivalent initialization logic when this is disabled.                                                                                       |
/// | `from_into_bits`  | `true`, `false`                                 | `true`   | Generates backing-data conversion functions. Primitive bitfields get `from_bits`, `from_bits_with_defaults`, endian-specific `from_*_bits` helpers, `into_bits`, and endian-specific `into_*_bits` helpers. Array-backed bitfields get the corresponding `bytes` and `slice` APIs.                               |
/// | `from_traits`     | `true`, `false`                                 | `true`   | Generates `From<Backing> for Bitfield` and `From<Bitfield> for Backing`. These conversions use `from_endian` and `into_endian`.                                                                                                                                                                                  |
/// | `default`         | `true`, `false`                                 | `true`   | Generates `Default` for the bitfield. The default value is equivalent to `new()`: zero-initialized storage with field defaults applied.                                                                                                                                                                          |
/// | `debug`           | `true`, `false`                                 | `true`   | Generates `core::fmt::Debug` for the bitfield. The implementation prints readable fields and their values.                                                                                                                                                                                                       |
/// | `copy`            | `true`, `false`                                 | `true`   | Derives `Copy` and `Clone` for primitive and stack array-backed bitfields. Heap array-backed bitfields derive `Clone` only because `Box<[u8; N]>` is not `Copy`.                                                                                                                                                 |
/// | `builder`         | `true`, `false`                                 | `true`   | Generates the `<Bitfield>Builder` type, `new`, `new_without_defaults`, `with_<field>`, `checked_with_<field>`, and `build`. Reserved fields do not get builder setters.                                                                                                                                          |
/// | `bit_ops`         | `true`, `false`                                 | `true`   | Master switch for bit operation groups. When `false`, all bit operation groups are disabled unless a specific bit operation group is explicitly set to `true`.                                                                                                                                                   |
/// | `write_bit_ops`   | `true`, `false`                                 | `true`   | Generates whole-bitfield write helpers such as `write_bits`, `write_bits_with_defaults`, `write_le_bits`, `write_be_bits`, and `write_defaults` for primitive bitfields, or the corresponding `bytes` helpers for array-backed bitfields.                                                                        |
/// | `clear_bit_ops`   | `true`, `false`                                 | `true`   | Generates whole-bitfield clear helpers such as `clear_bits` / `clear_bytes`, `clear_bits_with_defaults` / `clear_bytes_with_defaults`, plus per-field helpers like `clear_<field>()` and `clear_<field>_to_default()`.                                                                                           |
/// | `set_get_bit_ops` | `true`, `false`                                 | `true`   | Generates individual bit helpers (`get_bit`, `checked_get_bit`, `set_bit`, `checked_set_bit`) and range helpers (`get_bits_range` / `set_bits_range` for primitive bitfields, `get_bytes_range` / `set_bytes_range` for array-backed bitfields, plus checked variants).                                          |
/// | `invert_bit_ops`  | `true`, `false`                                 | `true`   | Generates inversion helpers such as `invert_bits` / `invert_bytes`, per-field `invert_<field>()`, and readable-field `<field>_inverted()` getters.                                                                                                                                                               |
/// | `toggle_bit_ops`  | `true`, `false`                                 | `true`   | Accepted as a bit-operation group flag for configuration compatibility. In this version, there are no separate `toggle_*` APIs; use the generated invert helpers to toggle bits.                                                                                                                                 |
/// | `array_heap`      | `true`, `false`                                 | `false`  | For array-backed bitfields only, stores the backing `[u8; N]` in a `Box` instead of inline in the struct. This helps avoid large stack values but requires heap allocation and therefore gives up the zero-allocation and `no_std` guarantees for that bitfield. It has no effect on primitive-backed bitfields. |
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
///
/// // Demonstrates a selection of configuration arguments.
/// #[bitfield(
///     u32,
///     order = msb,           // first struct field occupies the most-significant bits
///     from_endian = little,  // from_bits / From<u32> treat raw bits as little-endian
///     into_endian = big,     // into_bits / From<Bitfield> emit big-endian bits
///     default = true,        // generate Default impl
///     debug = true,          // generate Debug impl
///     builder = true,        // generate <Struct>Builder
///     bit_ops = false,       // disable all bit-operation helpers
/// )]
/// struct Config {
///     #[bits(default = 0x12)]
///     tag: u8,
///     #[bits(default = 0x34)]
///     flags: u8,
///     #[bits(default = 0x56)]
///     size: u8,
///     #[bits(default = 0x78)]
///     checksum: u8,
/// }
///
/// fn main() {
///     // With order = msb, `tag` occupies bits 31..=24 and `checksum` occupies bits 7..=0.
///     let cfg = Config::new();
///     assert_eq!(cfg.tag(), 0x12);
///     assert_eq!(cfg.flags(), 0x34);
///     assert_eq!(cfg.size(), 0x56);
///     assert_eq!(cfg.checksum(), 0x78);
///
///     // into_endian = big: the raw u32 is big-endian (tag at top byte).
///     assert_eq!(cfg.into_bits(), 0x12_34_56_78);
///
///     // from_endian = little: raw bits are interpreted as little-endian before decoding.
///     let cfg2 = Config::from_bits(0x12_34_56_78);
///     assert_eq!(cfg2.checksum(), 0x12); // 0x12 is now the LSB (checksum field in msb order)
/// }
/// ```
///
/// #### Bit Operations Config
///
/// The `bit_ops` argument is a master switch for the bit-operation groups:
///
/// - `#[bitfield(u32, bit_ops = false)]` disables all bit operations.
/// - `#[bitfield(u32, bit_ops = true, set_get_bit_ops = false)]` enables bit
///   operations except the set/get bit group.
/// - `#[bitfield(u32, bit_ops = false, set_get_bit_ops = true)]` disables the
///   master switch but explicitly opts the set/get bit group back in.
///
/// This lets you generate only the bit-operation APIs you need.
///
/// #### Global Cargo Feature Flags
///
/// If you find yourself applying the same configuration arguments to many bitfields
/// in your codebase, you can set those defaults globally by **disabling default 
/// features** and enabling the corresponding Cargo features:
///
/// - Constructors: `generate_new` / `disable_new`
/// - From/into functions: `generate_from_into_bits` /
///   `disable_from_into_bits`
/// - From traits: `generate_from_traits` / `disable_from_traits`
/// - Default trait: `generate_default` / `disable_default`
/// - Debug trait: `generate_debug` / `disable_debug`
/// - Copy/Clone derives: `derive_copy` / `disable_copy`
/// - Builder: `generate_builder` / `disable_builder`
/// - All bit operations: `generate_bit_ops` / `disable_bit_ops`
/// - Write bit operations: `generate_write_bit_ops` /
///   `disable_write_bit_ops`
/// - Clear bit operations: `generate_clear_bit_ops` /
///   `disable_clear_bit_ops`
/// - Set/get bit operations: `generate_set_get_bit_ops` /
///   `disable_set_get_bit_ops`
/// - Invert bit operations: `generate_invert_bit_ops` /
///   `disable_invert_bit_ops`
/// - Toggle bit operations: `generate_toggle_bit_ops` /
///   `disable_toggle_bit_ops`
/// - Array heap storage: `enable_array_heap` / `disable_array_heap`
///
/// Endian and order defaults have dedicated feature names:
///
/// - Bitfield field order: `order_lsb` / `order_msb`
/// - Bitfield From operations endian: `from_endian_little` / `from_endian_big`
/// - Bitfield Into operations endian: `into_endian_little` / `into_endian_big`
/// - Bitfield Write endian: `write_endian_little` / `write_endian_big`
///
/// Bitflags can also be configured globally with the following defaults:
///
/// - from_bits endian: `bitflag_from_endian_little` / `bitflag_from_endian_big`
/// - into_bits endian: `bitflag_into_endian_little` / `bitflag_into_endian_big`
/// - Copy and Clone: `bitflag_derive_copy` / `bitflag_disable_copy`
///
/// ```toml
/// [dependencies]
/// bitfields = {
///     version = "2.0.5",
///     # Default features must be disabled.
///     default-features = false,
///     features = [
///         "generate_builder",
///         "generate_set_get_bit_ops",
///         "order_msb",
///         "from_endian_little",
///         "into_endian_big"
///     ]
/// }
/// ```
/// <!-- rust-docs-end -->
#[proc_macro_attribute]
pub fn bitfield(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    catch_panics(|| start_bitfield_generation(args, input))
}

/// Entry for starting the bitfield generation.
fn start_bitfield_generation(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_bitfield = match parse_bitfield_struct(args.into(), input.into()) {
        Ok(bitfield) => bitfield,
        Err(err) => return err.into_compile_error().into(),
    };
    check_force_panic(&parsed_bitfield);
    generate_bitfield(&parsed_bitfield).into()
}

/// Forces a panic if argument is enabled.
fn check_force_panic(bitfield: &Bitfield) {
    assert!(!bitfield.arguments().force_panic(), "Forced panic for testing purposes");
}

#[rustfmt::skip]
/// <!-- rust-bitflags-docs-start -->
/// ### Bitflags
///
/// There are times when you just want to define a bitflag, which are just enums
/// that map to bits. Instead of defining a custom type, you can take advantage of
/// the `#[bitflag]` attribute which generates `from_bits` and `into_bits` for enums
/// automatically.
///
/// Bitflags only supports unsigned types (`u8`, `u16`, `u32`, `u64`, `u128`) and
/// the one of the variants must be annotated with `#[base]` or `#[default]` which represents the
/// base value of the bitflag. If `#[base]` and `#[default]` are both present, `#[base]` takes precedence.
///
/// ```rust
/// # use bitfields_impl as bitfields;
/// use bitfields::bitfield;
/// use bitfields::bitflag;
///
/// /// Annotate an enum with the `#[bitflag]` attribute to automatically generate
/// /// the `from_bits` and `into_bits` functions for you! One variant must be annotated
/// /// with `#[base]` which represents the base value of the bitflag.
/// #[bitflag(u8)]
/// #[derive(Debug, PartialEq)]
/// enum RenderMode {
///     #[base]
///     Normal = 0,
///     Mirror = 1,
///     Flip = 2,
///     // #[default] - Default can be specified but base takes 
///     // precedence if both are present.
///     Hidden = 3,
/// }
///
/// /// The code that's generated for the `RenderMode` by the
/// /// `#[bitflag]` attribute saving you the trouble of writing 
/// /// the `from_bits` and `into_bits` functions yourself.
/// // impl RenderMode {
/// //     const fn from_bits(bits: u8) -> Self {
/// //         match bits {
/// //             0 => Self::Normal,
/// //             1 => Self::Mirror,
/// //             2 => Self::Flip,
/// //             3 => Self::Hidden,
/// //             _ => unreachable!(),
/// //         }
/// //     }
/// //
/// //     const fn into_bits(self) -> u8 {
/// //         self as u8
/// //     }
/// // }
///
/// /// Annotate an enum with the `#[bitflag]` attribute to automatically generate
/// /// the `from_bits` and `into_bits` functions for you! One variant must be annotated
/// /// with `#[base]` which represents the base value of the bitflag.
/// #[bitflag(u8)]
/// #[derive(Debug, PartialEq)]
/// enum AudioMode {
///     #[base]
///     Stereo = 0,
///     Mono = 1,
///     Mute = 2,
///     Surround = 3,
/// }
///
/// #[bitfield(u8)]
/// struct DisplayControl {
///     /// Must have the `#[bits]` attribute since the macro has 
///     /// no way of knowing how many bits the custom type occupies.
///     #[bits(4, default = RenderMode::Normal)]
///     render_mode: RenderMode,
///     /// Must have the `#[bits]` attribute since the macro has 
///     /// no way of knowing how many bits the custom type occupies.
///     #[bits(4, default = AudioMode::Stereo)]
///     audio_mode: AudioMode,
/// }
///
/// fn main() {
///     let display = DisplayControlBuilder::new()
///         .with_render_mode(RenderMode::Mirror)
///         .with_audio_mode(AudioMode::Mute)
///         .build();
///
///     assert_eq!(display.render_mode(), RenderMode::Mirror);
///     assert_eq!(display.audio_mode(), AudioMode::Mute);
/// }
/// ```
///
/// #### Bitflag Configuration
///
/// Bitflags can be configured with arguments passed to the `#[bitflag(...)]` attribute (the first argument is always the backing unsigned integer type):
///
/// | Argument         | Values                            | Default  | Description                                                                                          |
/// |------------------|-----------------------------------|----------|------------------------------------------------------------------------------------------------------|
/// | `<backing type>` | `u8`, `u16`, `u32`, `u64`, `u128` | Required | The storage used by the generated bitflag. Primitive backing types support bitfields up to 128 bits. |
/// | `from_endian`    | `big`, `little`                   | `big`    | Default endianness used by the generated `from_bits` function.                                       |
/// | `into_endian`    | `big`, `little`                   | `big`    | Default endianness used by the generated `into_bits` function.                                       |
/// | `copy`           | `true`, `false`                   | `true`   | Determines whether to derive `Copy` and `Clone` automatically for the enum.                          |
/// <!-- rust-bitflags-docs-end -->
#[proc_macro_attribute]
pub fn bitflag(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    catch_panics(|| start_bitflag_generation(args, input))
}

/// Entry for starting the bitflag generation.
fn start_bitflag_generation(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed_bitflag = match parse_bitflag_enum(args.into(), input.into()) {
        Ok(bitflag) => bitflag,
        Err(err) => return err.into_compile_error().into(),
    };

    generate_bitflag(&parsed_bitflag).into()
}

/// Wraps and converts any unexpected panics into a compile error at the call
/// site.
fn catch_panics(macro_fn: impl FnOnce() -> proc_macro::TokenStream) -> proc_macro::TokenStream {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(macro_fn)).unwrap_or_else(|panic| {
        let panic_msg =
            panic.downcast_ref::<&str>().copied().unwrap_or("unable to parse panic payload");
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("{INTERNAL_ERROR_MESSAGE}: {panic_msg}"),
        )
        .to_compile_error()
        .into()
    })
}
