// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern "rust-intrinsic" {
    pub fn forget<T>(_: T) -> ();
    pub fn transmute<T, U>(thing: T) -> U;

    pub fn size_of<T>() -> uint;

    pub fn init<T>() -> T;
    pub fn uninit<T>() -> T;

    pub fn offset<T>(dst: *T, offset: int) -> *T;

    pub fn memcpy32<T>(dst: *mut T, src: *T, count: u32);
    pub fn memcpy64<T>(dst: *mut T, src: *T, count: u64);

    pub fn memmove32<T>(dst: *mut T, src: *T, count: u32);
    pub fn memmove64<T>(dst: *mut T, src: *T, count: u64);

    pub fn memset32<T>(dst: *mut T, val: u8, count: u32);
    pub fn memset64<T>(dst: *mut T, val: u8, count: u64);
}
