use bitfields::bitfield;

#[bitfield(u16)]
pub struct TestBitfieldWithBool {
    #[bits(4)]
    a: u8,
    #[bits(access = ro)]
    ro_bool: bool,
    #[bits(3)]
    b: u8,
    #[bits(8)]
    c: u8,
}

fn main() {
    // Bitfield layout (LSB first):
    // bits 0-3: a (4 bits)
    // bit 4: ro_bool (1 bit)  
    // bits 5-7: b (3 bits)
    // bits 8-15: c (8 bits)
    
    // Test with ro_bool = 1
    let value_with_true = 0b1111_1111_111_1_1111u16;
    println!("Testing with value: 0x{:04x} (binary: {:016b})", value_with_true, value_with_true);
    println!("  Expected ro_bool bit (bit 4): {}", (value_with_true >> 4) & 1);
    
    let bf2 = TestBitfieldWithBool::from_bits(value_with_true);
    println!("  a: 0x{:x} (expected 0xF)", bf2.a());
    println!("  ro_bool: {} (expected true)", bf2.ro_bool());
    println!("  b: 0x{:x} (expected 0x7)", bf2.b());
    println!("  c: 0x{:x} (expected 0xFF)", bf2.c());
    println!("  into_bits: 0x{:04x}", bf2.into_bits());
    
    // Test with ro_bool = 0
    let value_with_false = 0b1111_1111_111_0_1111u16;
    println!("\nTesting with value: 0x{:04x} (binary: {:016b})", value_with_false, value_with_false);
    println!("  Expected ro_bool bit (bit 4): {}", (value_with_false >> 4) & 1);
    
    let bf3 = TestBitfieldWithBool::from_bits(value_with_false);
    println!("  a: 0x{:x} (expected 0xF)", bf3.a());
    println!("  ro_bool: {} (expected false)", bf3.ro_bool());
    println!("  b: 0x{:x} (expected 0x7)", bf3.b());
    println!("  c: 0x{:x} (expected 0xFF)", bf3.c());
    println!("  into_bits: 0x{:04x}", bf3.into_bits());
}
