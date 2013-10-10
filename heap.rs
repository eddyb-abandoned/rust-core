// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::{abort, free, malloc};

#[lang = "exchange_malloc"]
unsafe fn exchange_malloc(size: uint) -> *mut u8 {
    let ptr = malloc(size);
    if ptr == 0 as *mut u8 {
        abort()
    }
    ptr
}

#[lang = "exchange_free"]
unsafe fn exchange_free(ptr: *mut u8) {
    free(ptr)
}
