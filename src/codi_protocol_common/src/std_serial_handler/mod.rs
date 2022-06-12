//! `std` serial port handling crate.

use std::time;

/// Open serial
pub fn open_serial_conn(device_node: &str, baud_rate: usize) {    
    let ports = serialport::available_ports()
        .expect("No ports found!");

    for p in ports {
        println!("{:?}", p);
    }

    let _port = serialport::new(device_node, baud_rate)
        .timeout(time::Duration::from_secs(10))
        .open()
        .unwrap();
}
