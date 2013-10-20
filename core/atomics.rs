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

mod detail {
    extern "rust-intrinsic" {
        /// Atomic compare and exchange, sequentially consistent.
        pub fn atomic_cxchg(dst: &mut int, old: int, src: int) -> int;
        /// Atomic compare and exchange, acquire ordering.
        pub fn atomic_cxchg_acq(dst: &mut int, old: int, src: int) -> int;
        /// Atomic compare and exchange, release ordering.
        pub fn atomic_cxchg_rel(dst: &mut int, old: int, src: int) -> int;

        pub fn atomic_cxchg_acqrel(dst: &mut int, old: int, src: int) -> int;
        pub fn atomic_cxchg_relaxed(dst: &mut int, old: int, src: int) -> int;

        /// Atomic load, sequentially consistent.
        pub fn atomic_load(src: &int) -> int;
        /// Atomic load, acquire ordering.
        pub fn atomic_load_acq(src: &int) -> int;

        pub fn atomic_load_relaxed(src: &int) -> int;

        /// Atomic store, sequentially consistent.
        pub fn atomic_store(dst: &mut int, val: int);
        /// Atomic store, release ordering.
        pub fn atomic_store_rel(dst: &mut int, val: int);

        pub fn atomic_store_relaxed(dst: &mut int, val: int);

        /// Atomic exchange, sequentially consistent.
        pub fn atomic_xchg(dst: &mut int, src: int) -> int;
        /// Atomic exchange, acquire ordering.
        pub fn atomic_xchg_acq(dst: &mut int, src: int) -> int;
        /// Atomic exchange, release ordering.
        pub fn atomic_xchg_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_xchg_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_xchg_relaxed(dst: &mut int, src: int) -> int;

        /// Atomic addition, sequentially consistent.
        pub fn atomic_xadd(dst: &mut int, src: int) -> int;
        /// Atomic addition, acquire ordering.
        pub fn atomic_xadd_acq(dst: &mut int, src: int) -> int;
        /// Atomic addition, release ordering.
        pub fn atomic_xadd_rel(dst: &mut int, src: int) -> int;
        pub fn atomic_xadd_acqrel(dst: &mut int, src: int) -> int;
        pub fn atomic_xadd_relaxed(dst: &mut int, src: int) -> int;

        /// Atomic subtraction, sequentially consistent.
        pub fn atomic_xsub(dst: &mut int, src: int) -> int;
        /// Atomic subtraction, acquire ordering.
        pub fn atomic_xsub_acq(dst: &mut int, src: int) -> int;
        /// Atomic subtraction, release ordering.
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
pub unsafe fn atomic_compare_and_swap<T>(dst: &mut T, old: T, new: T) -> T {
    let dst = intrinsics::transmute(dst);
    let old = intrinsics::transmute(old);
    let new = intrinsics::transmute(new);

    intrinsics::transmute(detail::atomic_cxchg(dst, old, new))
}

