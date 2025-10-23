use bitfields::bitfield;

#[bitfield(u32)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u8,
    #[bits(default = 0x34, access = ro)]
    b: u8,
    #[bits(default = 0x56)]
    c: u8,
    #[bits(default = 0x78)]
    d: u8,
}

fn main() {
    // Test from_bits
    let bitfield = Bitfield::from_bits(0x11_22_33_44);
    
    println!("from_bits(0x11_22_33_44):");
    println!("  a: 0x{:x}", bitfield.a());
    println!("  b: 0x{:x}", bitfield.b());
    println!("  c: 0x{:x}", bitfield.c());
    println!("  d: 0x{:x}", bitfield.d());
    println!("  into_bits: 0x{:x}", bitfield.into_bits());
}
