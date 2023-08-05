//! This module is for the stock `CoDi` `SerialPortManager`

use codi_protocol_common::common::{SerialPortManagerResult, SerialPortManagerTrait};
use codi_protocol_common::reexports::serialport;
use codi_protocol_common::serial::SerialPortManager;
use core2::io::{Read, Write};

#[allow(missing_debug_implementations)]
/// This is a wrapper around `SerialPortManager`, as Rust doesn't allow `impl`s of external types.
pub struct StockCoDiSerialPortManager(SerialPortManager);

impl SerialPortManagerTrait for StockCoDiSerialPortManager {
    fn open_serial_port(
        &mut self,
        dev_node: Option<&str>,
        baud_rate: u32,
    ) -> SerialPortManagerResult<()> {
        self.0.serial = Some(
            serialport::new(dev_node.unwrap(), baud_rate)
                .open()
                .unwrap(),
        );
        Ok(())
    }

    fn read_packet<R>(
        &self,
        _reader: R,
        _buf_size: usize,
    ) -> SerialPortManagerResult<Option<Vec<u8>>>
    where
        R: Read,
    {
        Ok(Some(Default::default()))
    }

    fn write_packet<W>(
        &mut self,
        _writer: W,
        _payload: Vec<u8>,
    ) -> SerialPortManagerResult<usize>
    where
        W: Write,
    {
        Ok(Default::default())
    }
}
