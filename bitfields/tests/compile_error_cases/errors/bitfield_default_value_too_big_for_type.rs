use bitfields::bitfield;

#[bitfield(u64)]
pub struct Bitfield {
    #[bits(64, default = -9223372036854775809)]
    a: isize,
}

#[bitfield(u8)]
pub struct Bitfield {
    #[bits(8, default = -129)]
    a: i8,
}

#[bitfield(u16)]
pub struct Bitfield {
    #[bits(16, default = -32769)]
    a: i16,
}

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(32, default = -2147483649)]
    a: i32,
}

#[bitfield(u64)]
pub struct Bitfield {
    #[bits(64, default = -9223372036854775809)]
    a: i64,
}

#[bitfield(u128)]
pub struct Bitfield {
    #[bits(128, default = -170141183460469231731687303715884105729)]
    a: i128,
}

#[bitfield(u64)]
pub struct Bitfield {
    #[bits(64, default = -9223372036854775809)]
    a: isize,
}

#[bitfield(u64)]
pub struct Bitfield {
    #[bits(64, default = 9223372036854775808)]
    a: isize,
}

fn main() {}
