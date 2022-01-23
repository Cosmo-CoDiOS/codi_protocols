//! `std` serial port handling crate.

use serialport::SerialPortBuilder;

/// Open serial
pub fn open_serial_conn(device_node: String) {    
    let ports = serialport::available_ports()
        .expect("No ports found!");

    for p in ports {
        println!("{:?}", p);
    }

    let mut port = serialport::new("/dev/ttyS1", 115_200)
        .timeout(::std::time::Duration::from_millis(1000))
        .open()
        .unwrap();

    port.write("ACCEPT TRANSMISSION"
               .as_bytes())
        .unwrap();
}
