# 🌻 Bitfields

<p>
  <a href="https://github.com/gregorygaines/bitfields-rs/actions/workflows/ci.yml"><img src="https://github.com/gregorygaines/bitfields-rs/actions/workflows/ci.yml/badge.svg" alt="Continuous integration"></a>
  <a href="https://crates.io/crates/bitfields"><img src="https://img.shields.io/crates/v/bitfields" alt="Version"></a>
  <a href="https://docs.rs/bitfields"><img src="https://docs.rs/bitfields/badge.svg" alt="Docs"></a>
  <a href="https://github.com/gregorygaines/bitfields-rs"><img src="https://img.shields.io/badge/github-gregorygaines/bitfields--rs-8da0cb?labelColor=555555&logo=github" alt="GitHub repo" /></a>
  <a href="#%EF%B8%8F-license"><img src="https://img.shields.io/github/license/Naereen/StrapDown.js.svg" alt="License"></a>
  <a href="https://ko-fi.com/T6T07SXPV"><img src="https://img.shields.io/badge/Ko--fi-FF5E5B?logo=kofi&logoColor=fff&style=flat"></a>
</p>

A Rust create that provides a procedural macro for generating bitfields from structs or
custom types, which is useful for defining schemas when working with low-level environments 
or concepts (e.g. embedded or writing an emulator).

- Efficient and safe code like you would write by hand.
- Fully flexible and customizable, you can choose what gets generated.
- No unsafe code, zero-allocations, const functions, and no runtime dependencies.
- Constant memory usage regardless of fields as the size of the bitfield is constant to its type.
- Usable in `no_std` environments.
- Compile-time checks for fields, types, and bits bounds checking.
- Supports most primitive and user-defined custom types.
- Supports endian conversion (little, big) and field order (msb or lsb).
- Signed fields are treated as 2's complement data types.
- Generates checked versions of field setters to catch overflows.
- Generates bit operations to get or set bits.
- Ability to ignore fields.

## 🔧 Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
bitfields = "0.9.0"
```

## 🚀 Getting Started

You're emulating the N1nt3nd0 GameChild and come across the Display
Control Register (DISPCNT) which is an 8-bit register:

```text
  Bit   Expl.
  0-1   BG Mode                    (0-7=Video Mode)
  2-3   Display BG (0-1)           (0=BGx Off, 1=BGx On)
  4     OBJ Character VRAM Mapping (0=Two dimensional, 1=One dimensional)
  5-7   Always 0x3                 Padding
```

In table form, the bits are as follows:

<table>
  <tr>
    <td>7</td>
    <td>6</td>
    <td>5</td>
    <td>4</td>
    <td>3</td>
    <td>2</td>
    <td>1</td>
    <td>0</td>
  </tr>
  <tr>
    <td colspan="3">Always 0x3</td>
    <td>OBJ</td>
    <td colspan="2">Display BG</td>
    <td colspan="2">BG Mode</td>
  </tr>
</table>


You can define the register as follows:

```rust
use bitfields::bitfield;

/// Create a struct annotated with the #[bitfield] attribute.
/// By default, the field order is the least significant bit
/// to most significant bit.
#[bitfield(u8)]
#[derive(Copy, Clone)] // Attributes are passed to the struct.
struct DisplayControl {
    /// We specify gg mode occupies the first 2 bits (0-1) of the bitfield
    /// using the `#[bits]` attribute.
    #[bits(2)]
    bg_mode: u8,
    /// Custom type fields must implement the `from_bits` and `into_bits`
    /// functions and declare its size using the `#[bits]` attribute.
    #[bits(2)]
    display_mode: DisplayMode,
    /// We can omit the `#[bits]` attribute for non-custom types, the macro
    /// will assume the number of bits is the size of the field type. Here,
    /// its 1 bit for a `bool` type.
    obj_char_vram_mapping: bool,
    /// Prefixing a field with "_" makes it as a padding field which
    /// is inaccessible. Padding fields are 0 by default, unless a default value
    /// is provided.
    #[bits(3, default = 0x3)]
    _always_0x3_padding: u8,
}

/// Define a custom type that represents the display mode.
struct DisplayMode {
    bg0_on: bool,
    bg1_on: bool,
}

/// Implement the `from_bits` and `into_bits` funcs for the custom type.
impl DisplayMode {
    /// Convert bits to the custom type.
    const fn from_bits(bits: u8) -> Self {
        Self {
            bg0_on: bits & 0b001 != 0,
            bg1_on: bits & 0b010 != 0,
        }
    }

    /// Convert the custom type into bits.
    const fn into_bits(self) -> u8 {
        (self.bg0_on as u8) | (self.bg1_on as u8) << 1
    }
}

// Creating the display mode custom type.
let display_mode = DisplayMode {
    bg0_on: true,
    bg1_on: false,
};

// Building the display control.
let display_control = DisplayControlBuilder::new()
    .with_bg_mode(0b1)
    .with_display_mode(display_mode)
    .with_obj_char_vram_mapping(true)
    .build();

// Converting into bits.
let val = display_control.into_bits();
assert_eq!(val, 0b01110101);
```

## 🤔 What Other Features Does Bitfields Offer?

Bitfields offers a wide range of features to help you define and work with bitfields.

```rust
use bitfields::bitfield;

/// All fields in the bitfield must sum up to the number of bits of the bitfield type.
#[bitfield(u64)]
pub struct Bitfield {
    /// Fields without bits specified default to the size of the field type.
    /// 8 bits.
    u8int: u8,
    /// A field can have specified bits, but the bits must be greater than zero
    /// and fit in the field type.
    #[bitfield(4)] // u8 is 8 bits, so 4 bits is valid.
    small_u8int: u8,
    /// A field that is signed, will be sign-extended by the most significant 
    /// bit of its type.
    signed_int: i8,
    /// If you specify bits, the field will be sign-extended by the most significant
    /// bit of the specified bits. In this case, the most significant bit of 4 bits.
    /// Also signed fields are 2's complement, meaning this field with 4 bits has
    /// the value range of `-8` to `7`. You can add more bits to increase this
    /// range!
    #[bits(4)]
    small_signed_int: i8,
    /// A field can be a bool type.
    bool_field: bool,
    /// A field can have a default value, which must fit in the field type.
    #[bits(default = 0x1F)]
    field_with_default: u8,
    /// A field can have a default value and specified bits. The default value
    /// must fit in the specified bits or a compile error will occur.
    #[bits(4, default = 0xF)] // Default fits in 4 bits.
    field_with_bits_default: u8,
    /// By default, all functions share the same visibility as the bitfield struct.
    /// Fields can have their getters and setters visibility overridden by specifying
    /// the visibility of the field.
    pub pub_field: u8, // Getter and setter are public.
    /// Nested bitfields are supported, but must have their bits specified.
    #[bits(3)]
    nested_field: NestedBitfield,
    /// Custom types are supported, but must have their bits specified and
    /// implement the `from_bits` and `into_bits` functions.
    #[bits(3)]
    custom_type: CustomType,
    /// Fields can have their access restricted. `ro` means read-only, meaning
    /// the field can be read but not written.
    #[bits(5, access = ro)] // Read-only field, no setter.
    read_only: u8,
    /// Fields prefixed with "_" are padding fields, which are inaccessible.
    #[bits(4, default = 0x3)]
    _padding: u8,
    /// Fields with the ignore attribute are ignored.
    #[bits(99, ignore = true)]
    ignore_me: u128,
}

#[bitfield(u8)]
struct NestedBitfield {
    field: u8
}

/// Custom types must have 2 const functions, `from_bits` and `into_bits` to convert
/// the type to and from bits functions.
#[derive(Default)]
struct CustomType {
    a: u8,
}

impl CustomType {
    /// Make sure the parameter type can fit the specified number of bits. Also,
    /// must be const, we need that extra compile time safety.
    const fn from_bits(bits: u8) -> Self {
        Self {
            a: bits,
        }
    }

    /// Make sure the return type can fit the specified number of bits. Also,
    /// must be const, we need that extra compile time safety.
    const fn into_bits(self) -> u8 {
        self.a
    }
}

// Usage:
// Creates a new bitfield using a builder pattern, unset fields default to 0 
// or their provided default value.
let mut bitfield = BitfieldBuilder::new()
    .with_u8int(5)
    .with_small_u8int(0xF)
    .with_custom_type(CustomType::from_bits(0x3))
    // .with_custom_type(CustomType::default()) // Can pass a [`CustomType`] instance.
    // .with_read_only(0x3) // Compile error, read-only field can't be set.
    // .with__padding(0x3) // Compile error, padding fields are inaccessible.
    .with_signed_int(-5)
    .with_small_signed_int(0xF)
    .build();

// let bitfield = Bitfield::new(); // Bitfield with default values.
// let bitfield = Bitfield::new_without_defaults(); // Bitfield without default values.
// let bitfield = BitfieldBuilder::new_without_defaults(); // Builder without defaults. 

// Accessing fields:
let u8int = bitfield.u8int(); // Getters
let small_u8int = bitfield.small_u8int(); // Signed-types are sign-extended.
bitfield.ignore_me; // Ignored fields can be accessed directly.

// Setting fields:
bitfield.set_u8int(0x3); // Setters
bitfield.checked_set_small_u8int(0xF); // Checked setter, error if value overflow bits.

// Converting to bits:
let bits = bitfield.into_bits();

// Converting from bits:
let bitfield = Bitfield::from_bits(0x3); // Converts from bits
// let bitfield = Bitfield::from_bits_with_defaults(0x3); // Converts, respects defaults.

// Constants:
assert_eq!(Bitfield::U8INT_BITS, 8); // Number of bits of the field.
assert_eq!(Bitfield::U8INT_OFFSET, 0); // The offset of the field in the bitfield.
```

### Bitfield Types

A bitfield can represent unsigned types (`u8`, `u16`, `u32`, `u64`, `u128`) up to
128-bits, because Rust was weak and stopped at `u128`. The field bits of a bitfield 
must add up to the number of bits of the bitfield type.

```rust
use bitfields::bitfield;

#[bitfield(u8)]
struct BitFieldU8 {
    a: u8,
}

#[bitfield(u32)]
struct BitFieldU32 {
    a: u32,
}

#[bitfield(u128)]
struct BitFieldU128 {
    a: u128,
}

```

### Bitfield Field Types

A bitfield field can be any unsigned (`u8`, `u16`, `u32`, `u64`, `u128`), signed
type (`i8`, `i16`, `i32`, `i64`, `i128`), or a custom type that implements the
const functions `from_bits` and `into_bits`. A default value can also be a const
variable or a const function. Just be aware that const function and variables defaults
lose their compile-time field bits checking.

Signed types are treated as 2's complement data types, meaning the most significant
represents the sign bit. For example, if you had a field with 5 bits, the value range
would be `-16` to `15`. The more bits you include, the larger the value range.

```rust
use bitfields::bitfield;

const CONST_VAR: u8 = 0x2;

const fn provide_val() -> u8 {
    0x1
}

#[bitfield(u32)]
struct Bitfield {
    #[bits(default = 0xFF)]
    a: u8,
    #[bits(default = -127)]
    b: i8,
    /// Sign-extended by the most significant bit of 4 bits. Also treated as 2's
    /// complement, meaning this field with 4 bits has the value range of
    /// `-8` to `7`. You can add more bits to increase this range!
    #[bits(4, default = 9)]
    c_sign_extended: i8,
    #[bits(2, default = CONST_VAR)] // No compile time checks for const variables.
    const_var_default: u8,
    #[bits(2, default = provide_val())] // No compile time checks for const functions.
    const_fn_default: u8, // No compile time checks for const functions.
    #[bits(8, default = CustomType::C)]
    custom_type: CustomType
}

#[derive(Debug, PartialEq)]
enum CustomType {
    A = 0,
    B = 1,
    C = 2,
}

impl CustomType {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            _ => unreachable!(),
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

let bitfield = Bitfield::new();
assert_eq!(bitfield.a(), 0xFF);
assert_eq!(bitfield.b(), -127);
assert_eq!(bitfield.c_sign_extended(), -7);
assert_eq!(bitfield.const_var_default(), 0x2);
assert_eq!(bitfield.const_fn_default(), 0x1);
assert_eq!(bitfield.custom_type(), CustomType::C);
```

### Constructing a Bitfield

A bitfield can be constructed using the `new` and `new_without_defaults` constructors. The former initializes 
the bitfield with default values, while the latter initializes the bitfield without default values, 
except for padding fields which always keep their default value or 0.

```rust
use bitfields::bitfield;

#[bitfield(u32)]
struct Bitfield {
    #[bits(default = 0x12)]
    a: u8,
    #[bits(default = 0x34)]
    b: u8,
    #[bits(default = 0x56)]
    c: u8,
    #[bits(default = 0x78)]
    _d: u8,
}

let bitfield = Bitfield::new();
assert_eq!(bitfield.a(), 0x12);
assert_eq!(bitfield.b(), 0x34);
assert_eq!(bitfield.c(), 0x56);
assert_eq!(bitfield.into_bits(), 0x78563412);

let bitfield_without_defaults = Bitfield::new_without_defaults();
assert_eq!(bitfield_without_defaults.a(), 0);
assert_eq!(bitfield_without_defaults.b(), 0);
assert_eq!(bitfield_without_defaults.c(), 0);
assert_eq!(bitfield_without_defaults.into_bits(), 0x78000000);
```

### Bitfield Conversions

A bitfield can be converted from bits using the `from_bits` or `from_bits_with_defaults` functions. The former
ignores default values, while the latter respects them. Padding fields are always 0 or their default value. The
bitfield can also be converted to bits using the `into_bits` function. The `From` trait is also implemented
between the bitfield and the bitfield type and operates the same as `from_bits`.

```rust
use bitfields::bitfield;

#[bitfield(u32)]
#[derive(Copy, Clone)]
struct Bitfield {
    #[bits(default = 0x12)]
    a: u8,
    #[bits(8)]
    b: CustomType,
    c: u8,
    #[bits(default = 0x78)]
    _d: u8,
}

#[derive(Debug, PartialEq)]
enum CustomType {
    A = 0,
    B = 1,
    C = 2,
}

impl CustomType {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            _ => Self::A,
        }
    }
    
    const fn into_bits(self) -> u8 {
        self as u8
    }
}

let bitfield = Bitfield::from_bits(0x11223344);
assert_eq!(bitfield.a(), 0x44);
assert_eq!(bitfield.b(), CustomType::A);
assert_eq!(bitfield.c(), 0x22);
let val = bitfield.into_bits();
assert_eq!(val, 0x78220044);

let bitfield_respect_defaults = Bitfield::from_bits_with_defaults(0x11223344);
assert_eq!(bitfield_respect_defaults.a(), 0x12); // Default value respected
assert_eq!(bitfield_respect_defaults.b(), CustomType::A);
assert_eq!(bitfield_respect_defaults.c(), 0x22);
let val = bitfield_respect_defaults.into_bits();
assert_eq!(val, 0x78220012);

// From trait
let val: u32 = bitfield.into();
assert_eq!(val, 0x78220044);
let bitfield: Bitfield = val.into();
assert_eq!(bitfield.into_bits(), 0x78220044);
```

### Conversion Endianess

Sometimes the outside world is outside our control, like how systems store or expect data endian. Luckily, the endian
of the bitfield conversions can be controlled by specifying the `#[bitfield(from_endian = x, into_endian = x)]` args. 
The possible endians are `little` or `big`. By default, the endian of both is `big`.

````rust
use bitfields::bitfield;

// We are working with a system that stores data in little-endian, we
// set the from_endian to little for the proper representation.
//
// The system expects the data it stores in big-endian, we set the 
// into_endian to big-endian for converting into the proper representation.
#[bitfield(u32, from_endian = little, into_endian = big)]
pub struct Bitfield {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

// The host device stored the data 0x12345678 in little-endian memory 
// as [0x78, 0x56, 0x34, 0x12].
let bitfield = Bitfield::from_bits(0x78563412);

assert_eq!(bitfield.a(), 0x78);
assert_eq!(bitfield.b(), 0x56);
assert_eq!(bitfield.c(), 0x34);
assert_eq!(bitfield.d(), 0x12);
assert_eq!(bitfield.into_bits(), 0x12345678);
````

### Field Order

By default, fields are ordered from the least significant bit (lsb) to the most significant bit (msb).
The order can be changed by specifying the `#[bitfield(order = x)]` arg on the bitfield struct.
There are two field orderings, `lsb` and `msb`.

```rust
use bitfields::bitfield;

#[bitfield(u32, order = msb)]
struct Bitfield {
    #[bits(default = 0x12)]
    a: u8,
    #[bits(default = 0x34)]
    b: u8,
    #[bits(default = 0x56)]
    c: u8,
    #[bits(default = 0x78)]
    d: u8,
}

let bitfield = Bitfield::new();
assert_eq!(bitfield.a(), 0x12);
assert_eq!(bitfield.b(), 0x34);
assert_eq!(bitfield.c(), 0x56);
assert_eq!(bitfield.d(), 0x78);
let val = bitfield.into_bits();

//                .- a
//                |    .- b
//                |    | .- c
//                |    | |  .- d
assert_eq!(val, 0x12_34_56_78);
assert_eq!(Bitfield::A_OFFSET, 24); // Offset of the a field in the bitfield.
```

### Field Access

Field access can be controlled by specifying the `#[bits(access = x)]` arg on a field. There are four accesses:
- `rw` - Read and write access (default)
- `ro` - Read-only access.
- `wo` - Write-only access.
- `none` - No access.

```rust
use bitfields::bitfield;

#[bitfield(u32)]
struct Bitfield {
    read_write: u8,
    #[bits(access = ro)]
    read_only: u8,
    #[bits(access = wo)]
    write_only: u8,
    #[bits(default = 0xFF, access = none)]
    none: u8,
}

let mut bitfield = BitfieldBuilder::new()
    .with_read_write(0x12)
    // .with_read_only(0x34) // Compile error, read-only field can't be set.
    .with_write_only(0x56)
    // .with_none(0x78) // Compile error, none field can't be set.
    .build();
bitfield.set_read_write(0x12);
// bitfield.set_read_only(0x34); // Compile error, read-only field can't be set.
bitfield.set_write_only(0x56);
// bitfield.set_none(0x78); // Compile error, none field can't be set.

assert_eq!(bitfield.read_write(), 0x12);
assert_eq!(bitfield.read_only(), 0);
// assert_eq!(bitfield.write_only(), 0x56); // Compile error, write-only can't be read.
// assert_eq!(bitfield.none(), 0xFF); // Compile error, none field can't be accessed.
assert_eq!(bitfield.into_bits(), 0xFF560012); // All fields exposed when converted to bits.
```

### Checked Setters

Normally, when a field is set, the value is truncated to the number of bits of the field. Fields also 
have checked setters that returns an error if the value overflows the number of bits of the field.

```rust
use bitfields::bitfield;

#[bitfield(u16)]
struct Bitfield {
    a: u8,
    #[bits(4)]
    b: u8,
    #[bits(4)]
    _padding: u8,
}

let mut bitfield = Bitfield::new();
bitfield.set_a(0xFF);
bitfield.set_b(0x12); // Truncated to 4 bits.
assert_eq!(bitfield.a(), 0xFF);
assert_eq!(bitfield.b(), 0x2);

let res = bitfield.checked_set_b(0x12); // Error, value overflows bits.
assert!(res.is_err());
```

### Bit Operations

Individual bits can be get or set using the `get_bit` and `set_bit` functions. They can be enabled using
the bitfield attribute arg For `get_bit`, if the bit is  out-of-bounds or the field doesn't have write access,
`false` is returned. There is a checked version `checked_get_bit` that return an error instead. Similarly, 
for `set_bit`, if the bit is out-of-bounds or the  field doesn't have write access, the operation is no-op. 
There is a checked version `checked_set_bit` that returns an error instead.

```rust
use bitfields::bitfield;

#[bitfield(u8, bit_ops = true)]
#[derive(Copy, Clone)]
pub struct Bitfield {
    #[bits(2, default = 0b11)]
    a: u8,
    #[bits(2, default = 0b00)]
    b: u8,
    #[bits(2, default = 0b10, access = wo)]
    c: u8,
    #[bits(2, default = 0b01)]
    _d: u8,
}

let bitfield = Bitfield::new();

assert!(bitfield.get_bit(0));
assert!(bitfield.get_bit(1));
assert!(!bitfield.get_bit(2));
assert!(!bitfield.get_bit(3));
assert!(!bitfield.get_bit(4)); // No write access, false is returned.
assert!(!bitfield.get_bit(5)); // No write access, false is returned.
assert!(bitfield.checked_get_bit(4).is_err()); // No write access, err.
assert!(bitfield.checked_get_bit(5).is_err()); // No write access, err.
assert!(bitfield.get_bit(6));
assert!(!bitfield.get_bit(7));
assert!(!bitfield.get_bit(50)); // Out-of-bounds, false is returned.
assert!(bitfield.checked_get_bit(50).is_err()); // Out-of-bounds, err.
```

```rust
use bitfields::bitfield;

#[bitfield(u8, bit_ops = true)]
#[derive(Copy, Clone)]
pub struct Bitfield {
    #[bits(2)]
    a: u8,
    #[bits(2, default = 0b11)]
    b: u8,
    #[bits(2, default = 0b11, access = ro)]
    c: u8,
    #[bits(2, default = 0b00)]
    _d: u8,
}

let mut bitfield = Bitfield::new();

bitfield.set_bit(0, true);
bitfield.set_bit(1, true);
bitfield.set_bit(2, false);
bitfield.set_bit(3, false);
bitfield.set_bit(4, false); // No-op, no write access.
bitfield.set_bit(5, false); // No-op, no write access.
assert!(bitfield.checked_set_bit(4, false).is_err()); // Error, no write access.
assert!(bitfield.checked_set_bit(5, false).is_err()); // Error, no write access.
bitfield.set_bit(6, true); // No-op, padding.
bitfield.set_bit(7, true); // No-op, padding.
assert!(bitfield.checked_set_bit(4, false).is_err()); // Error, padding.
assert!(bitfield.checked_set_bit(5, false).is_err()); // Error, padding..
assert_eq!(bitfield.into_bits(), 0b110011);
```

### Padding Fields

Fields prefixed with an underscore `_` are padding fields, which are inaccessible. Meaning the field is always
0/false or a default value. They are useful for padding the bits of the bitfield.

```rust
use bitfields::bitfield;

#[bitfield(u16)]
struct Bitfield {
    a: u8,
    #[bits(default = 0xFF)]
    _padding: u8, // Fills the remaining bits of the u16.
}

let bitfield = Bitfield::new();
assert_eq!(bitfield.a(), 0);
// assert_eq!(bitfield._padding(), 0xFF00); // Compile error, padding inaccessible.
// bitfield.set__padding(0xFF); // Compile error, padding fields are inaccessible.
assert_eq!(bitfield.into_bits(), 0xFF00); // All fields exposed when converted to bits.
```

### Ignored Fields

Fields with the `#[bits(ignore = true)` attribute are ignored and not included in the bitfield. This is useful for
when you are building a custom bitfield, but want to include certain fields that aren't a part of the bitfield without
wrapping having to wrap bitfield is a parent struct. All ignored fields must implement the `Default` trait. Ignored fields
are accessible directly like normal struct fields.

```rust
use bitfields::bitfield;

#[bitfield(u16)]
struct Bitfield {
    a: u8,
    b: u8,
    #[bits(ignore = true)] // Ignored field.
    field_id: u8,
    #[bits(ignore = true)] // Ignored field.
    field_custom: CustomType,
}

#[derive(Debug, Default, PartialEq)]
enum CustomType {
    #[default]
    A,
    B,
}

let bitfield = Bitfield::new();

assert_eq!(bitfield.field_id, 0); // Ignored fields can be accessed directly.
assert_eq!(bitfield.field_custom, CustomType::A); // Ignored fields can be accessed directly.
```

### Field Constants 

Fields with read or write access have constants generated for their number of bits and offset in the bitfield.

```rust
use bitfields::bitfield;

#[bitfield(u32)]
struct Bitfield {
    #[bits(default = 0x12)]
    a: u8,
    #[bits(default = 0x34)]
    b: u8,
    #[bits(default = 0x56)]
    c: u8,
    #[bits(default = 0x78)]
    d: u8,
}

assert_eq!(Bitfield::A_BITS, 8); // Number of bits of the  afield.
assert_eq!(Bitfield::A_OFFSET, 0); // The offset of the a field in the bitfield.
assert_eq!(Bitfield::B_BITS, 8); // Number of bits of the b field.
assert_eq!(Bitfield::B_OFFSET, 8); // The offset of the b field in the bitfield.
assert_eq!(Bitfield::C_BITS, 8); // Number of bits of c the field.
assert_eq!(Bitfield::C_OFFSET, 16); // The offset of the c field in the bitfield.
assert_eq!(Bitfield::D_BITS, 8); // Number of bits of the d field.
assert_eq!(Bitfield::D_OFFSET, 24); // The offset of the d field in the bitfield.
```

### Debug Implementation

A debug implementation is generated for the bitfield, which prints the fields and their values.

```rust
use bitfields::bitfield;

#[bitfield(u32)]
struct Bitfield {
    #[bits(default = 0x12)]
    a: u8,
    #[bits(default = 0x34)]
    b: u8,
    #[bits(default = 0x56)]
    c: u8,
    #[bits(default = 0x78)]
    d: u8,
}

let bitfield = Bitfield::new();

assert_eq!(format!("{:?}", bitfield), "Bitfield { d: 120, c: 86, b: 52, a: 18 }");
```

### Passing Attributes

Attributes below the `#[bitfield]` attribute are passed to the generated struct.

```rust
use bitfields::bitfield;

#[bitfield(u32)]
#[derive(Copy, Clone)]
struct Bitfield {
    a: u32,
}
```

### Complete Generation Control

You have complete control over what gets generated by the bitfield macro. When your deploying to a resource-constrained
environment, you can generate only the necessary functions or implementations. You can disable generation by passing 
`false` to its attribute arg.

The `#[bitfield]` args that control generation are:

- `#[bitfield(new = true)]` - Generates the `new` and `new_without_defaults` constructor.
- `#[bitfield(from_bits = true)]` - Generates the `from_bits` and `from_bits_with_defaults` functions.
- `#[bitfield(into_bits = true)]` - Generates the `into_bits` function.
- `#[bitfield(from = true)]` - Generates the `From` trait implementation.
- `#[bitfield(debug = true)]` - Generates the `Debug` trait implementation.
- `#[bitfield(default = true)]` - Generates the `Default` trait implementation
- `#[bitfield(builder = true)]` - Generates the builder implementation.
- `#[bitfield(bit_ops = true)]` - Generates the bit operations implementation.

## ⚖️ License

Distributed under the MIT License. See [LICENSE](/LICENSE) for more information.

## 🤝 Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MIT license, shall be licensed as
above, without any additional terms or conditions.

## 🥶 Employment Disclaimer

As of Dec 2024, I am a Google employee; "Bitfields" is my own work, not
affiliated with Google, its subsidiaries, nor endorsing any Google-owned
products or tools. "Bitfields" was written without any proprietary knowledge, 
tools, or resources of Google.

## 💯 Acknowledgments

- [Johan Mickos](https://github.com/johanmickos) - Awesome dude who gave me the 'bitfields' crate name.
- [dtolnay/proc-macro-workshop](https://github.com/dtolnay/proc-macro-workshop)
- [wrenger/bitfield-struct-rs](https://github.com/wrenger/bitfield-struct-rs)
- [hawkw/mycelium](https://crates.io/crates/mycelium-bitfield)
- [dzamlo/rust-bitfield](https://github.com/dzamlo/rust-bitfield)
