//! This crate handles UART communication between the `CoDi` chip and the host ROM.
#![cfg_attr(all(not(feature = "std")), no_std)]
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

/// `SerialPortTrait` acts as a generic way to implement a SerialPortManager struct in both
/// `no_std` and `std` environments.
pub trait SerialPortManagerTrait {
    /// `open_serial_port` assigns a `SerialPort` instance to the `serial` field in a
    /// `SerialPortManager` instance.
    fn open_serial_port(&mut self, dev: &str, baud_rate: u32);
    /// `read_packet` uses the `io::Read` trait to read a packet from the serial connection,
    /// starting with hex `58215821`, or `X!X!`.
    fn read_packet(&self);
}
