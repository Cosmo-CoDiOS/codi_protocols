//! This crate handles UART communication between the `CoDi` chip and the host ROM.
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![no_std]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;
