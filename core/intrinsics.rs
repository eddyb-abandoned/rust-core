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
        pub fn size_of<T>() -> uint;
    }
}

#[inline]
pub fn abort() -> ! {
    unsafe { detail::abort() }
}

#[inline]
pub fn size_of<T>() -> uint {
    unsafe { detail::size_of::<T>() }
}

extern "rust-intrinsic" {
    pub fn forget<T>(_: T) -> ();
    pub fn transmute<T, U>(thing: T) -> U;

    pub fn init<T>() -> T;
    pub fn uninit<T>() -> T;

    pub fn offset<T>(dst: *T, offset: int) -> *T;

    pub fn memcpy32<T>(dst: *mut T, src: *T, count: u32);
    pub fn memcpy64<T>(dst: *mut T, src: *T, count: u64);

    pub fn memmove32<T>(dst: *mut T, src: *T, count: u32);
    pub fn memmove64<T>(dst: *mut T, src: *T, count: u64);

    pub fn memset32<T>(dst: *mut T, val: u8, count: u32);
    pub fn memset64<T>(dst: *mut T, val: u8, count: u64);

    pub fn move_val_init<T>(dst: &mut T, src: T);

    pub fn sqrtf32(x: f32) -> f32;
    pub fn sqrtf64(x: f64) -> f64;

    pub fn powif32(a: f32, x: i32) -> f32;
    pub fn powif64(a: f64, x: i32) -> f64;

    pub fn sinf32(x: f32) -> f32;
    pub fn sinf64(x: f64) -> f64;

    pub fn cosf32(x: f32) -> f32;
    pub fn cosf64(x: f64) -> f64;

    pub fn powf32(a: f32, x: f32) -> f32;
    pub fn powf64(a: f64, x: f64) -> f64;

    pub fn expf32(x: f32) -> f32;
    pub fn expf64(x: f64) -> f64;

    pub fn exp2f32(x: f32) -> f32;
    pub fn exp2f64(x: f64) -> f64;

    pub fn logf32(x: f32) -> f32;
    pub fn logf64(x: f64) -> f64;

    pub fn log10f32(x: f32) -> f32;
    pub fn log10f64(x: f64) -> f64;

    pub fn log2f32(x: f32) -> f32;
    pub fn log2f64(x: f64) -> f64;

    pub fn fmaf32(a: f32, b: f32, c: f32) -> f32;
    pub fn fmaf64(a: f64, b: f64, c: f64) -> f64;

    pub fn fabsf32(x: f32) -> f32;
    pub fn fabsf64(x: f64) -> f64;

    pub fn floorf32(x: f32) -> f32;
    pub fn floorf64(x: f64) -> f64;

    pub fn ceilf32(x: f32) -> f32;
    pub fn ceilf64(x: f64) -> f64;

    pub fn truncf32(x: f32) -> f32;
    pub fn truncf64(x: f64) -> f64;

    pub fn ctpop8(x: i8) -> i8;
    pub fn ctpop16(x: i16) -> i16;
    pub fn ctpop32(x: i32) -> i32;
    pub fn ctpop64(x: i64) -> i64;

    pub fn ctlz8(x: i8) -> i8;
    pub fn ctlz16(x: i16) -> i16;
    pub fn ctlz32(x: i32) -> i32;
    pub fn ctlz64(x: i64) -> i64;

    pub fn cttz8(x: i8) -> i8;
    pub fn cttz16(x: i16) -> i16;
    pub fn cttz32(x: i32) -> i32;
    pub fn cttz64(x: i64) -> i64;

    pub fn bswap16(x: i16) -> i16;
    pub fn bswap32(x: i32) -> i32;
    pub fn bswap64(x: i64) -> i64;

    pub fn i8_add_with_overflow(x: i8, y: i8) -> (i8, bool);
    pub fn i16_add_with_overflow(x: i16, y: i16) -> (i16, bool);
    pub fn i32_add_with_overflow(x: i32, y: i32) -> (i32, bool);
    pub fn i64_add_with_overflow(x: i64, y: i64) -> (i64, bool);

    pub fn u8_add_with_overflow(x: u8, y: u8) -> (u8, bool);
    pub fn u16_add_with_overflow(x: u16, y: u16) -> (u16, bool);
    pub fn u32_add_with_overflow(x: u32, y: u32) -> (u32, bool);
    pub fn u64_add_with_overflow(x: u64, y: u64) -> (u64, bool);

    pub fn i8_sub_with_overflow(x: i8, y: i8) -> (i8, bool);
    pub fn i16_sub_with_overflow(x: i16, y: i16) -> (i16, bool);
    pub fn i32_sub_with_overflow(x: i32, y: i32) -> (i32, bool);
    pub fn i64_sub_with_overflow(x: i64, y: i64) -> (i64, bool);

    pub fn u8_sub_with_overflow(x: u8, y: u8) -> (u8, bool);
    pub fn u16_sub_with_overflow(x: u16, y: u16) -> (u16, bool);
    pub fn u32_sub_with_overflow(x: u32, y: u32) -> (u32, bool);
    pub fn u64_sub_with_overflow(x: u64, y: u64) -> (u64, bool);

    pub fn i8_mul_with_overflow(x: i8, y: i8) -> (i8, bool);
    pub fn i16_mul_with_overflow(x: i16, y: i16) -> (i16, bool);
    pub fn i32_mul_with_overflow(x: i32, y: i32) -> (i32, bool);
    pub fn i64_mul_with_overflow(x: i64, y: i64) -> (i64, bool);

    pub fn u8_mul_with_overflow(x: u8, y: u8) -> (u8, bool);
    pub fn u16_mul_with_overflow(x: u16, y: u16) -> (u16, bool);
    pub fn u32_mul_with_overflow(x: u32, y: u32) -> (u32, bool);
    pub fn u64_mul_with_overflow(x: u64, y: u64) -> (u64, bool);
}
