#[bitfield(u8)]
struct Bitfield {
    #[bits(4)]
    rw: u8,
    #[bits(4, access = ro)]
    ro: u8,
}

fn main() {
    let b = BitfieldBuilder::new().with_ro(0xF).build();
    assert_eq!(0xF, b.ro());
}
