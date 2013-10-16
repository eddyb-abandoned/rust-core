// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::libc::{free, malloc, realloc};
use core::intrinsics::abort;

#[inline]
#[lang = "exchange_free"]
pub unsafe fn free_raw(ptr: *mut u8) {
    free(ptr)
}

#[inline]
#[lang = "exchange_malloc"]
pub unsafe fn malloc_raw(size: uint) -> *mut u8 {
    let ptr = malloc(size);
    if ptr == 0 as *mut u8 {
        out_of_memory()
    }
    ptr
}

#[inline]
pub unsafe fn realloc_raw(ptr: *mut u8, size: uint) -> *mut u8 {
    let ptr = realloc(ptr, size);
    if ptr == 0 as *mut u8 {
        out_of_memory()
    }
    ptr
}

#[inline]
pub unsafe fn out_of_memory() -> ! {
    abort()
}
