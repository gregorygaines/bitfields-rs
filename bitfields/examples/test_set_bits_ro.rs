use bitfields::bitfield;

#[bitfield(u32)]
#[derive(Copy, Clone)]
pub struct Bitfield {
    #[bits(default = 0x12)]
    a: u8,
    #[bits(default = 0x34, access = ro)]
    b: u8,
    #[bits(default = 0x56)]
    c: u8,
    #[bits(default = 0x78)]
    _d: u8,
}

fn main() {
    let mut bitfield = Bitfield::new();
    
    println!("Before set_bits:");
    println!("  a: 0x{:x}", bitfield.a());
    println!("  b: 0x{:x}", bitfield.b());
    println!("  c: 0x{:x}", bitfield.c());
    println!("  into_bits: 0x{:x}", bitfield.into_bits());
    
    bitfield.set_bits(0x11223344);
    
    println!("\nAfter set_bits(0x11223344):");
    println!("  a: 0x{:x}", bitfield.a());
    println!("  b: 0x{:x} (should remain 0x34)", bitfield.b());
    println!("  c: 0x{:x}", bitfield.c());
    println!("  into_bits: 0x{:x}", bitfield.into_bits());
}
