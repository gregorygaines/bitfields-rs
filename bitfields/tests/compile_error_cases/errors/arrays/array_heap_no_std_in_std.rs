#![no_std]

use bitfields::bitfield;
use core::panic::PanicInfo;
extern crate alloc;
use dummy_alloc::DummyAllocator;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;

#[bitfield([u8; 4], array_heap_no_std = true)]
struct ArrayHeapBitfield {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
}

fn main() {
    let bitfield = ArrayHeapBitfield::from_slice(&[0x11, 0x22, 0x33, 0x44]);
    assert_eq!(bitfield.a(), 0x44);
    assert_eq!(bitfield.b(), 0x33);
    assert_eq!(bitfield.c(), 0x22);
    assert_eq!(bitfield.d(), 0x11);
}
