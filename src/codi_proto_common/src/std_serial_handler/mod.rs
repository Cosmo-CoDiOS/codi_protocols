//! `std` serial port handling crate.

/// Open serial
pub fn open_serial_conn(device_node: String) {    
    let ports = serialport::available_ports()
        .expect("No ports found!");

    for p in ports {
        println!("{:?}", p);
    }

    let _port = serialport::new(&device_node, 115_200)
        .timeout(::std::time::Duration::from_millis(1000))
        .open()
        .unwrap();
}
