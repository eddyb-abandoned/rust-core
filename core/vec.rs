// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::intrinsics::{move_val_init, offset, size_of, transmute};
use core::heap::{out_of_memory, realloc_raw};
use core::kinds::{Freeze, Send};
use core::ops::Drop;
use core::libc::free;
use core::slice::Slice;
use core::ptr::read_ptr;

pub struct Vec<T> {
    priv len: uint,
    priv cap: uint,
    priv ptr: *mut T
}

impl<T: Send + Freeze> Vec<T> {
    #[inline]
    pub fn new() -> Vec<T> {
        Vec { len: 0, cap: 0, ptr: 0 as *mut T }
    }

    #[inline]
    pub fn len(&self) -> uint {
        self.len
    }

    #[inline]
    pub fn capacity(&self) -> uint {
        self.cap
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        unsafe {
            if self.len == 0 {
                free(self.ptr as *mut u8);
                self.cap = 0;
                self.ptr = 0 as *mut T;
            } else {
                self.cap = self.len;
                self.ptr = realloc_raw(self.ptr as *mut u8, self.cap * size_of::<T>()) as *mut T;
            }
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        unsafe {
            if self.len == self.cap {
                if self.cap == 0 { self.cap += 2 }
                let old_size = self.cap * size_of::<T>();
                self.cap = self.cap * 2;
                let size = old_size * 2;
                if old_size > size { out_of_memory() }
                self.ptr = realloc_raw(self.ptr as *mut u8, size) as *mut T;
            }

            let end = offset(self.ptr as *T, self.len as int) as *mut T;
            move_val_init(&mut *end, value);
            self.len += 1;
        }
    }

    #[inline]
    pub fn as_slice<'r>(&'r self) -> &'r [T] {
        let slice = Slice { data: self.ptr as *T, len: self.len * size_of::<T>()};
        unsafe { transmute(slice) }
    }

    #[inline]
    pub fn as_mut_slice<'r>(&'r mut self) -> &'r mut [T] {
        let slice = Slice { data: self.ptr as *T, len: self.len * size_of::<T>()};
        unsafe { transmute(slice) }
    }
}


#[unsafe_destructor]
impl<T: Send + Freeze> Drop for Vec<T> {
    fn drop(&mut self) {
        unsafe {
            let mut i = 0;
            let len = self.len();
            let xs = self.as_mut_slice();
            while i < len {
                read_ptr(&xs[i]);
                i += 1;
            }
            free(self.ptr as *mut u8)
        }
    }
}
