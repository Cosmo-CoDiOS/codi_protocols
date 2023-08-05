//! `std` serial port handling crate.

use serialport::SerialPort;

// we can't have Debug or Clone traits on the std-targeted SerialPort. PR
// needed.
#[allow(missing_copy_implementations, missing_debug_implementations)]
/// `SerialPortManager` is a struct that manages the serial port connection,
/// both on `no_std` and `std` environments.
pub struct SerialPortManager {
    /// This field holds the ARM64/x86_64 `serialport` trait.
    pub serial: Option<Box<dyn SerialPort>>,
}

impl SerialPortManager {
    #[allow(dead_code)]
    /// This method returns an instance of `SerialPortManager`, with the
    /// `serial` field populated as `None`.
    pub fn new() -> Self {
        Self { serial: None }
    }
}
