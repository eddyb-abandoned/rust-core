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

mod detail {
    extern "rust-intrinsic" {
        pub fn atomic_cxchg(dst: &mut int, old: int, src: int) -> int;
        pub fn atomic_cxchg_acq(dst: &mut int, old: int, src: int) -> int;
        pub fn atomic_cxchg_rel(dst: &mut int, old: int, src: int) -> int;
        pub fn atomic_cxchg_acqrel(dst: &mut int, old: int, src: int) -> int;
        pub fn atomic_cxchg_relaxed(dst: &mut int, old: int, src: int) -> int;

        pub fn atomic_load(src: &int) -> int;
        pub fn atomic_load_acq(src: &int) -> int;
        pub fn atomic_load_relaxed(src: &int) -> int;

        pub fn atomic_store(dst: &mut int, val: int);
        pub fn atomic_store_rel(dst: &mut int, val: int);
        pub fn atomic_store_relaxed(dst: &mut int, val: int);

        pub fn atomic_xchg(dst: &mut int, src: int) -> int;
        pub fn atomic_xchg_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_xchg_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_xchg_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_xchg_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_xadd(dst: &mut int, src: int) -> int;
        pub fn atomic_xadd_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_xadd_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_xadd_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_xadd_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_xsub(dst: &mut int, src: int) -> int;
        pub fn atomic_xsub_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_xsub_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_xsub_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_xsub_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_and(dst: &mut int, src: int) -> int;
        pub fn atomic_and_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_and_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_and_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_and_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_nand(dst: &mut int, src: int) -> int;
        pub fn atomic_nand_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_nand_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_nand_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_nand_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_or(dst: &mut int, src: int) -> int;
        pub fn atomic_or_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_or_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_or_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_or_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_xor(dst: &mut int, src: int) -> int;
        pub fn atomic_xor_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_xor_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_xor_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_xor_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_max(dst: &mut int, src: int) -> int;
        pub fn atomic_max_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_max_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_max_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_max_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_min(dst: &mut int, src: int) -> int;
        pub fn atomic_min_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_min_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_min_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_min_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_umin(dst: &mut int, src: int) -> int;
        pub fn atomic_umin_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_umin_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_umin_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_umin_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_umax(dst: &mut int, src: int) -> int;
        pub fn atomic_umax_acq(dst: &mut int, src: int) -> int;
        pub fn atomic_umax_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_umax_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_umax_relaxed(dst: &mut int, src: int) -> int;

        pub fn atomic_fence();
        pub fn atomic_fence_acq();
        pub fn atomic_fence_rel();
        pub fn atomic_fence_acqrel();
    }
}

#[inline]
pub unsafe fn compare_and_swap<T>(dst: &mut T, old: T, new: T) -> T {
    let dst = transmute(dst);
    let old = transmute(old);
    let new = transmute(new);

    transmute(detail::atomic_cxchg(dst, old, new))
}

#[inline]
pub unsafe fn load<T>(dst: &T) -> T {
    transmute(detail::atomic_load(transmute(dst)))
}

#[inline]
pub unsafe fn store<T>(dst: &mut T, val: T) {
    detail::atomic_store(transmute(dst), transmute(val))
}

#[inline]
pub unsafe fn swap<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_xchg(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn add<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_xadd(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn sub<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_xsub(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn and<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_and(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn nand<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_nand(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn or<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_or(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn xor<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_xor(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn min<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_min(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn max<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_max(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn umin<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_min(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn umax<T>(dst: &mut T, val: T) -> T {
    transmute(detail::atomic_max(transmute(dst), transmute(val)))
}

#[inline]
pub unsafe fn fence() {
    detail::atomic_fence()
}
