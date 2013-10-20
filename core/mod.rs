// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(ctypes, cstack)];

pub mod atomics;
pub mod clone;
pub mod fail;
pub mod intrinsics;
pub mod kinds;
pub mod ops;
pub mod option;
pub mod ptr;
pub mod rc;
pub mod slice;

#[cfg(libc)]
pub mod heap;
#[cfg(libc)]
pub mod libc;
#[cfg(libc)]
pub mod vec;
