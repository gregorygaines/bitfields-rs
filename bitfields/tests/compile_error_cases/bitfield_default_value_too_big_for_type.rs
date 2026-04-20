use bitfields::bitfield;

#[bitfield(u64)]
pub struct Bitfield {
    #[bits(64, default = -9223372036854775809)] // less than isize::MIN
    a: isize,
}

#[bitfield(u8)]
pub struct Bitfield {
    #[bits(8, default = -129)] // less than i8::MIN (-128)
    a: i8,
}

#[bitfield(u16)]
pub struct Bitfield {
    #[bits(16, default = -32769)] // less than i16::MIN (-32768)
    a: i16,
}

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(32, default = -2147483649)] // less than i32::MIN
    a: i32,
}

#[bitfield(u64)]
pub struct Bitfield {
    #[bits(64, default = -9223372036854775809)] // less than i64::MIN
    a: i64,
}

#[bitfield(u128)]
pub struct Bitfield {
    #[bits(128, default = -170141183460469231731687303715884105729)] // less than i128::MIN
    a: i128,
}

#[bitfield(u64)]
pub struct Bitfield {
    #[bits(64, default = -9223372036854775809)] // abs > isize::MIN (9223372036854775808)
    a: isize,
}

#[bitfield(u64)]
pub struct Bitfield {
    #[bits(64, default = 9223372036854775808)] // > isize::MAX (9223372036854775807)
    a: isize,
}

fn main() {}
