//! This crate handles UART communication between the `CoDi` chip and the host
//! ROM.
#![cfg_attr(target_arch = "arm", no_std)]
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

#[cfg_attr(target_arch = "arm", path = "embedded_serial.rs")]
#[cfg_attr(
    any(target_arch = "aarch64", target_arch = "x86_64"),
    path = "std_serial.rs"
)]
pub mod serial;

// Compiles with both `no_std` and `std`.
pub mod common;

pub mod reexports {
    //! Reexports for external crates.
    pub use super::common;
    pub use super::serial;

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    pub use serialport;
}
