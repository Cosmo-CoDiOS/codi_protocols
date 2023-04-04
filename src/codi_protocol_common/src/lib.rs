//! This crate handles UART communication between the `CoDi` chip and the host ROM.
#![cfg_attr(all(feature = "std"), no_std)]
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

#[cfg(not(feature = "std"))]
pub mod no_std_serial_port_manager;

#[cfg(feature = "std")]
pub mod std_serial_port_manager;

// Compiles with both `no_std` and `std`.

use serialport::SerialPort;

pub trait SerialPortTrait {
    fn open_serial_port(&mut self, dev: &str, baud_rate: u32);
    fn read_packet(&self);
}
