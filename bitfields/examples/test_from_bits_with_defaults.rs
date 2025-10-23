use bitfields::bitfield;

#[bitfield(u32)]
pub struct TestBitfield {
    #[bits(default = 0x12)]
    rw_field: u8,
    #[bits(default = 0x34, access = ro)]
    ro_field: u8,
    #[bits(default = 0x56)]
    normal_field: u8,
    #[bits(default = 0x78)]
    another_field: u8,
}

fn main() {
    println!("Test from_bits_with_defaults");
    let bf = TestBitfield::from_bits_with_defaults(0x11_22_33_44);
    println!("  rw_field: 0x{:x} (default 0x12)", bf.rw_field());
    println!("  ro_field: 0x{:x} (default 0x34)", bf.ro_field());
    println!("  normal_field: 0x{:x} (default 0x56)", bf.normal_field());
    println!("  another_field: 0x{:x} (default 0x78)", bf.another_field());
    println!("  into_bits: 0x{:x}", bf.into_bits());
}
