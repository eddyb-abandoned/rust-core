// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::heap::{Allocator, out_of_memory};

extern {
    pub fn malloc(size: uint) -> *mut u8;
    pub fn realloc(ptr: *mut u8, size: uint) -> *mut u8;
    pub fn free(ptr: *mut u8);
}

struct LibCAllocator;

impl Allocator for LibCAllocator {
    fn alloc(&mut self, size: uint) -> *mut u8 {
        let ptr = unsafe { malloc(size) };
        if ptr == 0 as *mut u8 {
            unsafe { out_of_memory() }
        }
        ptr
    }

    fn free(&mut self, ptr: *mut u8) {
        unsafe { free(ptr) }
    }

    fn realloc(&mut self, ptr: *mut u8, size: uint) -> *mut u8 {
        let ptr = unsafe { realloc(ptr, size) };
        if ptr == 0 as *mut u8 {
            unsafe { out_of_memory() }
        }
        ptr
    }
}

pub static mut allocator : LibCAllocator = LibCAllocator;
