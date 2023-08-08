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
    pub fn new(dev: &str, baud: u32) -> Self {
        Self {
            serial: Option::from(serialport::new(dev, baud)
                                .open()
                                .expect("Unable to open a UART connection.")),
        }
    }
}
