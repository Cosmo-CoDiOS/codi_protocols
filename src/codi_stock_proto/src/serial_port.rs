//! Module for interfacing with `CoDi` over serial.

use std::time::Duration;

use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};

/// String representation of the `CoDi` message header.
#[allow(dead_code)]
const CODI_MSG_HEADER_DECODED: &str = "X!X!";

/// Raw representation of the `CoDi` message header (in hex).
#[allow(dead_code)]
const CODI_MSG_HEADER_RAW: [u8; 4] = [58, 21, 58, 21];

/// This function opens a serial connection to `CoDi`.
#[allow(dead_code)]
pub(crate) fn open_port() -> Box<dyn SerialPort> {
    serialport::new("/dev/ttyS00", 115_200)
        .data_bits(DataBits::Eight)
        .flow_control(FlowControl::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(10))
        .parity(Parity::None)
        .open()
        .expect("Serial port failed to open!")
}
