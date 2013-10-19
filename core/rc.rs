// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::fail::fail_borrowed;
use core::intrinsics::transmute;
use core::ops::{Drop, Eq};
use core::kinds::{Freeze, Send};
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

enum Borrow {
    Mutable,
    Immutable,
    Nothing
}

impl Eq for Borrow {
    #[inline]
    fn eq(&self, other: &Borrow) -> bool {
        match (*self, *other) {
            (Mutable, Mutable) | (Immutable, Immutable) | (Nothing, Nothing) => true,
            _ => false
        }
    }
}

struct RcMutBox<T> {
    value: T,
    count: uint,
    borrow: Borrow
}

#[no_send]
#[no_freeze]
#[unsafe_no_drop_flag]
pub struct RcMut<T> {
    priv ptr: *mut RcMutBox<T>,
}

impl<T: Freeze> RcMut<T> {
    #[inline]
    pub fn new(value: T) -> RcMut<T> {
        unsafe { RcMut::new_unchecked(value) }
    }
}

impl<T: Send> RcMut<T> {
    #[inline]
    pub fn from_send(value: T) -> RcMut<T> {
        unsafe { RcMut::new_unchecked(value) }
    }
}

impl<T> RcMut<T> {
    #[inline]
    pub unsafe fn new_unchecked(value: T) -> RcMut<T> {
        RcMut{ptr: transmute(~RcMutBox{value: value, count: 1, borrow: Nothing})}
    }
}

impl<T> RcMut<T> {
    #[inline]
    pub fn with_borrow<U>(&self, f: &fn(&T) -> U) -> U {
        unsafe {
            if (*self.ptr).borrow == Mutable {
                fail_borrowed()
            }
            let previous = (*self.ptr).borrow;
            (*self.ptr).borrow = Immutable;
            let res = f(&(*self.ptr).value);
            (*self.ptr).borrow = previous;
            res
        }
    }

    #[inline]
    pub fn with_mut_borrow<U>(&self, f: &fn(&mut T) -> U) -> U {
        unsafe {
            if (*self.ptr).borrow != Nothing {
                fail_borrowed()
            }
            (*self.ptr).borrow = Mutable;
            let res = f(&mut (*self.ptr).value);
            (*self.ptr).borrow = Nothing;
            res
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for RcMut<T> {
    fn drop(&mut self) {
        unsafe {
            if self.ptr != 0 as *mut RcMutBox<T> {
                (*self.ptr).count -= 1;
                if (*self.ptr).count == 0 {
                    let _: ~RcMutBox<T> = transmute(self.ptr);
                }
            }
        }
    }
}

impl<T> Clone for RcMut<T> {
    #[inline]
    fn clone(&self) -> RcMut<T> {
        unsafe {
            (*self.ptr).count += 1;
            RcMut{ptr: self.ptr}
        }
    }
}

impl<T: DeepClone> DeepClone for RcMut<T> {
    #[inline]
    fn deep_clone(&self) -> RcMut<T> {
        do self.with_borrow |x| {
            unsafe { RcMut::new_unchecked(x.deep_clone()) }
        }
    }
}
