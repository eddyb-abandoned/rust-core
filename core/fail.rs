// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod detail {
    extern "rust-intrinsic" {
        pub fn abort() -> !;
    }
}

#[inline]
pub fn abort() -> ! {
    unsafe { detail::abort() }
}

#[inline]
#[lang="fail_bounds_check"]
pub fn fail_bounds_check(_: *u8, _: uint, _: uint, _: uint) -> ! {
    abort()
}

#[inline]
#[lang="fail_"]
pub fn fail_(_: *u8, _: *u8, _: uint) -> ! {
    abort()
}
