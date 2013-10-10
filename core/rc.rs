// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::intrinsics::transmute;
use core::ops::Drop;
use core::kinds::Freeze;
use core::clone::{Clone, DeepClone};

struct RcBox<T> {
    value: T,
    count: uint
}

#[unsafe_no_drop_flag]
#[no_send]
pub struct Rc<T> {
    priv ptr: *mut RcBox<T>
}

impl<T: Freeze> Rc<T> {
    #[inline]
    pub fn new(value: T) -> Rc<T> {
        unsafe {
            Rc::new_unchecked(value)
        }
    }
}

impl<T> Rc<T> {
    #[inline]
    pub unsafe fn new_unchecked(value: T) -> Rc<T> {
        Rc{ptr: transmute(~RcBox{value: value, count: 1})}
    }
}

impl<T> Rc<T> {
    #[inline]
    pub fn borrow<'r>(&'r self) -> &'r T {
        unsafe { &(*self.ptr).value }
    }
}

impl<T> Clone for Rc<T> {
    #[inline]
    fn clone(&self) -> Rc<T> {
        unsafe {
            (*self.ptr).count += 1;
            Rc{ptr: self.ptr}
        }
    }
}

impl<T: DeepClone> DeepClone for Rc<T> {
    #[inline]
    fn deep_clone(&self) -> Rc<T> {
        unsafe { Rc::new_unchecked(self.borrow().deep_clone()) }
    }
}

#[unsafe_destructor]
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            if self.ptr != 0 as *mut RcBox<T> {
                (*self.ptr).count -= 1;
                if (*self.ptr).count == 0 {
                    let _: ~RcBox<T> = transmute(self.ptr);
                }
            }
        }
    }
}
