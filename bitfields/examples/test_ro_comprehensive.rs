use bitfields::bitfield;

#[bitfield(u32)]
pub struct TestBitfield {
    #[bits(default = 0x12)]
    rw_field: u8,
    #[bits(default = 0x34, access = ro)]
    ro_field: u8,
    no_default_field: u8,
    #[bits(default = 0x78)]
    another_field: u8,
}

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

#[bitfield(u64)]
pub struct TestBitfieldMultipleRO {
    #[bits(8, access = ro)]
    ro1: u8,
    #[bits(8)]
    rw1: u8,
    #[bits(8, access = ro)]
    ro2: u8,
    #[bits(8)]
    rw2: u8,
    #[bits(32)]
    large: u32,
}

fn main() {
    println!("Test 1: Basic read-only field");
    let bf1 = TestBitfield::from_bits(0x11_22_33_44);
    assert_eq!(bf1.rw_field(), 0x44, "rw_field should be 0x44");
    assert_eq!(bf1.ro_field(), 0x33, "ro_field should be 0x33 (NOT 0)");
    assert_eq!(bf1.no_default_field(), 0x22, "no_default_field should be 0x22");
    assert_eq!(bf1.another_field(), 0x11, "another_field should be 0x11");
    assert_eq!(bf1.into_bits(), 0x11_22_33_44, "into_bits should preserve all fields");
    println!("✓ Basic read-only field test passed");

    println!("\nTest 2: Read-only bool field");
    // Bitfield layout (LSB first):
    // bits 0-3: a (4 bits)
    // bit 4: ro_bool (1 bit)
    // bits 5-7: b (3 bits)
    // bits 8-15: c (8 bits)
    let bf2 = TestBitfieldWithBool::from_bits(0xFFFF);
    assert_eq!(bf2.a(), 0xF, "a should be 0xF");
    assert_eq!(bf2.ro_bool(), true, "ro_bool should be true");
    assert_eq!(bf2.b(), 0x7, "b should be 0x7");
    assert_eq!(bf2.c(), 0xFF, "c should be 0xFF");
    println!("✓ Read-only bool field test passed");
    
    let bf2_false = TestBitfieldWithBool::from_bits(0xFFEF); // bit 4 is 0
    assert_eq!(bf2_false.ro_bool(), false, "ro_bool should be false");
    println!("✓ Read-only bool field false test passed");

    println!("\nTest 3: Multiple read-only fields");
    let bf3 = TestBitfieldMultipleRO::from_bits(0x1122334455667788);
    assert_eq!(bf3.ro1(), 0x88, "ro1 should be 0x88");
    assert_eq!(bf3.rw1(), 0x77, "rw1 should be 0x77");
    assert_eq!(bf3.ro2(), 0x66, "ro2 should be 0x66");
    assert_eq!(bf3.rw2(), 0x55, "rw2 should be 0x55");
    assert_eq!(bf3.large(), 0x11223344, "large should be 0x11223344");
    println!("✓ Multiple read-only fields test passed");

    println!("\nTest 4: from_bits_with_defaults respects defaults");
    let bf4 = TestBitfield::from_bits_with_defaults(0x11_22_33_44);
    assert_eq!(bf4.rw_field(), 0x12, "rw_field should use default 0x12");
    assert_eq!(bf4.ro_field(), 0x34, "ro_field should use default 0x34");
    assert_eq!(bf4.no_default_field(), 0x22, "no_default_field should be 0x22 (no default, use from bits)");
    assert_eq!(bf4.another_field(), 0x78, "another_field should use default 0x78");
    println!("✓ from_bits_with_defaults test passed");

    println!("\n✓✓✓ All tests passed! ✓✓✓");
}
