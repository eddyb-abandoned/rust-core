// Compile with --cfg has_heap_impl

#[no_std];

use core::vec::Vec;

mod core;

mod core_heap_impl {
    use core::heap::LinearAllocator;

    static STATIC_HEAP_SIZE: uint = 4 * 1024 * 1024; // 4MB.
    pub static mut allocator : LinearAllocator<[u8, ..STATIC_HEAP_SIZE]> = LinearAllocator {
        offset: 0,
        data: [0, ..STATIC_HEAP_SIZE]
    };
}

#[start]
fn main(_: int, _: **u8) -> int {
    let mut xs = Vec::new();
    let mut i = 0;
    while i < 100 {
        xs.push(i);
        i += 1;
    }
    0
}
