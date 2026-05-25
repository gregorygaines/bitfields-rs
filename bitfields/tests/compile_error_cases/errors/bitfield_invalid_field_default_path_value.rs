use bitfields::bitfield;

#[bitfield(u32)]
pub struct BitfieldA {
    #[bits(default = create::hello::+=322)]
    a: u32,
}

#[bitfield(u32)]
pub struct BitfieldB {
    #[bits(default = create::hello::::)]
    a: u32,
}

#[bitfield(u32)]
pub struct BitfieldC {
    #[bits(default = 3create::hello::::)]
    a: u32,
}

#[bitfield(u32)]
pub struct BitfieldD {
    #[bits(default = foo::<T>)]
    a: u32,
}

#[bitfield(u32)]
pub struct BitfieldE {
    #[bits(default = "foo::💥")]
    a: u32,
}

fn main() {}
