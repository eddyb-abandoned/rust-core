// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub trait Clone {
    fn clone(&self) -> Self;
}

impl<T: Clone> Clone for ~T {
    #[inline]
    fn clone(&self) -> ~T { ~(**self).clone() }
}

impl<'self, T> Clone for &'self T {
    #[inline]
    fn clone(&self) -> &'self T { *self }
}

impl<'self, T> Clone for &'self [T] {
    #[inline]
    fn clone(&self) -> &'self [T] { *self }
}

impl<'self> Clone for &'self str {
    #[inline]
    fn clone(&self) -> &'self str { *self }
}

impl<T> Clone for *T {
    #[inline]
    fn clone(&self) -> *T { *self }
}


impl<T> Clone for *mut T {
    #[inline]
    fn clone(&self) -> *mut T { *self }
}

impl Clone for int {
    #[inline]
    fn clone(&self) -> int { *self }
}

impl Clone for i8 {
    #[inline]
    fn clone(&self) -> i8 { *self }
}

impl Clone for i16 {
    #[inline]
    fn clone(&self) -> i16 { *self }
}

impl Clone for i32 {
    #[inline]
    fn clone(&self) -> i32 { *self }
}

impl Clone for i64 {
    #[inline]
    fn clone(&self) -> i64 { *self }
}

impl Clone for uint {
    #[inline]
    fn clone(&self) -> uint { *self }
}

impl Clone for u8 {
    #[inline]
    fn clone(&self) -> u8 { *self }
}

impl Clone for u16 {
    #[inline]
    fn clone(&self) -> u16 { *self }
}


impl Clone for u32 {
    #[inline]
    fn clone(&self) -> u32 { *self }
}

impl Clone for u64 {
    #[inline]
    fn clone(&self) -> u64 { *self }
}

impl Clone for f32 {
    #[inline]
    fn clone(&self) -> f32 { *self }
}

impl Clone for f64 {
    #[inline]
    fn clone(&self) -> f64 { *self }
}

impl Clone for () {
    #[inline]
    fn clone(&self) -> () { *self }
}

impl Clone for bool {
    #[inline]
    fn clone(&self) -> bool { *self }
}

impl Clone for char {
    #[inline]
    fn clone(&self) -> char { *self }
}

pub trait DeepClone {
    fn deep_clone(&self) -> Self;
}

impl<T: DeepClone> DeepClone for ~T {
    #[inline]
    fn deep_clone(&self) -> ~T { ~(**self).deep_clone() }
}

impl DeepClone for int {
    #[inline]
    fn deep_clone(&self) -> int { *self }
}

impl DeepClone for i8 {
    #[inline]
    fn deep_clone(&self) -> i8 { *self }
}

impl DeepClone for i16 {
    #[inline]
    fn deep_clone(&self) -> i16 { *self }
}

impl DeepClone for i32 {
    #[inline]
    fn deep_clone(&self) -> i32 { *self }
}

impl DeepClone for i64 {
    #[inline]
    fn deep_clone(&self) -> i64 { *self }
}

impl DeepClone for uint {
    #[inline]
    fn deep_clone(&self) -> uint { *self }
}

impl DeepClone for u8 {
    #[inline]
    fn deep_clone(&self) -> u8 { *self }
}

impl DeepClone for u16 {
    #[inline]
    fn deep_clone(&self) -> u16 { *self }
}


impl DeepClone for u32 {
    #[inline]
    fn deep_clone(&self) -> u32 { *self }
}

impl DeepClone for u64 {
    #[inline]
    fn deep_clone(&self) -> u64 { *self }
}

impl DeepClone for f32 {
    #[inline]
    fn deep_clone(&self) -> f32 { *self }
}

impl DeepClone for f64 {
    #[inline]
    fn deep_clone(&self) -> f64 { *self }
}

impl DeepClone for () {
    #[inline]
    fn deep_clone(&self) -> () { *self }
}

impl DeepClone for bool {
    #[inline]
    fn deep_clone(&self) -> bool { *self }
}

impl DeepClone for char {
    #[inline]
    fn deep_clone(&self) -> char { *self }
}
