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
    d: u8,
}

fn main() {
    // Test From trait
    let bitfield: Bitfield = 0x11_22_33_44u32.into();
    
    println!("From trait with value 0x11_22_33_44:");
    println!("  a: 0x{:x}", bitfield.a());
    println!("  b: 0x{:x} (read-only, should be set from bits: 0x33)", bitfield.b());
    println!("  c: 0x{:x}", bitfield.c());
    println!("  d: 0x{:x}", bitfield.d());
    
    assert_eq!(bitfield.a(), 0x44);
    assert_eq!(bitfield.b(), 0x33); // Read-only field should be set by From
    assert_eq!(bitfield.c(), 0x22);
    assert_eq!(bitfield.d(), 0x11);
    
    println!("\n✓ From trait correctly sets read-only fields!");
}
