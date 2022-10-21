#![allow(missing_docs)]
//! `std` serial port handling crate.

use serialport::SerialPort;

// we can't have Debug or Clone traits on the std-targeted SerialPort. PR needed.
#[allow(missing_copy_implementations, missing_debug_implementations)]
pub struct SerialPortManager {
    serial: Box<dyn SerialPort>,
}

pub trait SerialPortTrait {
    fn open_serial_port(&mut self, dev: &str, baud_rate: u32);
    fn read_packet(&self);
}

impl SerialPortTrait for SerialPortManager {
    fn open_serial_port(&mut self, dev: &str, baud_rate: u32) {
        self.serial = serialport::new(dev, baud_rate).open().unwrap();
    }

    fn read_packet(&self) {
        todo!()
    }
}
