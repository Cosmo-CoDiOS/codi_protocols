use codi_protocol_common::common::{SerialPortManagerError, SerialPortManagerTrait};
use codi_protocol_common::reexports::serialport;
use codi_protocol_common::serial::SerialPortManager;
use core2::io::{Read, Write};

#[allow(missing_debug_implementations)]
pub struct StockCoDiSerialPortManager(SerialPortManager);

impl SerialPortManagerTrait for StockCoDiSerialPortManager {
    fn open_serial_port(
        &mut self,
        dev_node: Option<&str>,
        baud_rate: u32,
    ) -> Result<(), SerialPortManagerError> {
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
    ) -> Result<Option<Vec<u8>>, SerialPortManagerError>
    where
        R: Read,
    {
        Ok(Some(Default::default()))
    }

    fn write_packet<W>(
        &mut self,
        _writer: W,
        _payload: Vec<u8>,
    ) -> Result<usize, SerialPortManagerError>
    where
        W: Write,
    {
        Ok(Default::default())
    }
}
