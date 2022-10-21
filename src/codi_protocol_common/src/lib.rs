//! This crate handles UART communication between the `CoDi` chip and the host ROM.
#![cfg_attr(all(target_arch = "arm"), no_std)]
#![deny(
    warnings,
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

#[cfg(not(feature = "stm32"))]
pub mod std_serial_handler;
