//! This crate defines the protocol for communicating with the new CoDi bootloader (`codi_bootload`)
#![no_std]
#![deny(
    warnings,
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

#[allow(missing_docs)]
pub mod protocol_common_self;

#[allow(missing_docs)]
pub mod protocol_bootload_flashing;

#[allow(missing_docs)]
pub mod protocol_common_variants;
