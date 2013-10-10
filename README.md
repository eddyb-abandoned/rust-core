A stub standard library for Rust. It will provide a baseline level of support
for freestanding Rust, and extended functionality based on the availability of
the standard C library, POSIX and OS-specific features.

The `core` module is meant to be directly included as a module in a top-level
crate making use of the `#[no_std]` attribute. It will make no attempt at
working outside of that use case.

Configuration:

* If the C standard library is available, pass `--cfg libc` to `rustc`
