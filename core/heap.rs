// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::atomics;
use core::fail::abort;
use core::intrinsics::size_of;
use core::ptr;

#[cfg(libc)]
use core::libc::allocator;

#[cfg(has_heap_impl)]
use core_heap_impl::allocator;

pub trait Allocator {
    fn alloc(&mut self, size: uint) -> *mut u8;

    fn free(&mut self, ptr: *mut u8);

    fn realloc(&mut self, ptr: *mut u8, size: uint) -> *mut u8 {
        let (min, max) = self.alloc_size_bounds(ptr);

        if min <= size && size <= max {
            return ptr; // In-place zero-copy realloc.
        }

        let new = self.alloc(size);
        //min(size, max)
        let copy_size = if size > max { max } else { size };
        unsafe {
            ptr::copy_nonoverlapping_memory(new, ptr as *u8, copy_size);
        }
        self.free(ptr);
        new
    }

    // [min; max] (inclusive) region in which the allocation size is valid for ptr.
    // This can be provided for allocators with size classes, where min != max.
    fn alloc_size_bounds(&mut self, _ptr: *mut u8) -> (uint, uint) {
        abort();
    }
}


// Simple linear allocator operating on a static heap.
pub struct LinearAllocator<N> {
    offset: uint,
    data: N
}

/*
macro_rules! LinearAllocator (
    ($N:expr) => (
        core::heap::LinearAllocator {
            data: [0u8, ..($N)],
            offset: 0
        }
    )
)
*/

impl<N> Allocator for LinearAllocator<N> {
    fn alloc(&mut self, size: uint) -> *mut u8 {
        let ptr = &self.data as *N as uint + unsafe {
            let used_size = size + size_of::<uint>();
            let offset = atomics::add(&mut self.offset, used_size);
            if offset + used_size > size_of::<N>() {
                out_of_memory();
            }
            offset
        };
        unsafe {
            // Set the allocation size.
            *(ptr as *mut uint) = size;
        }
        (ptr + size_of::<uint>()) as *mut u8
    }

    fn free(&mut self, _ptr: *mut u8) {}

    fn alloc_size_bounds(&mut self, ptr: *mut u8) -> (uint, uint) {
        let size = unsafe {
            // Get the allocation size.
            *((ptr as uint - size_of::<uint>()) as *uint)
        };
        (size, size)
    }
}

#[inline(always)]
#[lang = "exchange_free"]
pub unsafe fn free_raw(ptr: *mut u8) {
    allocator.free(ptr)
}

#[inline(always)]
#[lang = "exchange_malloc"]
pub unsafe fn malloc_raw(size: uint) -> *mut u8 {
    allocator.alloc(size)
}

#[inline(always)]
pub unsafe fn realloc_raw(ptr: *mut u8, size: uint) -> *mut u8 {
    allocator.realloc(ptr, size)
}

#[inline]
pub unsafe fn out_of_memory() -> ! {
    abort()
}
