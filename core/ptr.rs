// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::intrinsics;

#[inline]
#[cfg(target_word_size = "32")]
pub unsafe fn set_memory<T>(dst: *mut T, c: u8, count: uint) {
    intrinsics::memset32(dst, c, count as u32);
}

#[inline]
#[cfg(target_word_size = "64")]
pub unsafe fn set_memory<T>(dst: *mut T, c: u8, count: uint) {
    intrinsics::memset64(dst, c, count as u64);
}
