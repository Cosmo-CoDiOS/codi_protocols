//! This module is for the stock `CoDi` `Connection`

use crate::StockCoDiPacket;
use crate::serial_manager::StockCoDiSerialPortManager;

#[allow(missing_debug_implementations)]
/// `StockCoDiConnection` holds packets from/to `CoDi`, and the `StockCoDiSerialPortManager`.
pub struct StockCoDiConnection {
    packets: Vec<StockCoDiPacket>,
    serial: StockCoDiSerialPortManager,
}
