//! This crate defines the protocol for `codid` and other tooling to communicate
//! with stock `CoDi`.
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
#![allow(dead_code, unused_variables)] // FIXME: Temporary.

mod commands;
mod connection;
mod packet;
mod serial_manager;

pub use commands::StockCoDiPacketCommandKind;
pub use connection::*;
pub use packet::*;
pub use serial_manager::*;
