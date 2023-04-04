//! `std` serial port handling crate.

pub use crate::SerialPortManagerTrait;
use serialport::SerialPort;

// we can't have Debug or Clone traits on the std-targeted SerialPort. PR needed.
#[allow(missing_copy_implementations, missing_debug_implementations)]
/// `SerialPortManager` is a struct that manages the serial port connection, both on `no_std` and
/// `std` environments.
pub struct SerialPortManager {
    serial: Box<dyn SerialPort>,
}

impl SerialPortManagerTrait for SerialPortManager {
    fn open_serial_port(&mut self, dev: &str, baud_rate: u32) {
        self.serial = serialport::new(dev, baud_rate).open().unwrap();
    }

    fn read_packet(&self) {
        todo!()
    }
}
