//! This crate defines the protocol for `codid`, and other tooling, to communicate with `CoDiOS`.
#![deny(
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

#[allow(missing_docs)]
pub mod protocol_common_self;

#[allow(missing_docs)]
pub mod protocol_command_telephony;
