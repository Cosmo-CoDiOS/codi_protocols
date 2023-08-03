//! This module is for the stock CoDi packet handling.

use codi_protocol_common::common::PacketDirectionKind;
use codi_protocol_common::serial::SerialPortManager;

use crate::StockCoDiPacketCommandKind;

const CODI_STOCK_PACKET_HEADER: [u8; 4] = [58, 21, 58, 21];

#[allow(missing_debug_implementations)]
pub struct StockCoDiConnection {
    packets: Vec<StockCoDiPacket>,
    serial: SerialPortManager,
}

/// This trait (`StockCoDiPacketTrait`) acts as a specification for each
/// individual incoming/outgoing stock `CoDi` serial packet.
pub trait StockCoDiPacketTrait {
    fn get_header(&self) -> [u8; 4];
    fn get_command(&self) -> StockCoDiPacketCommandKind;
    fn set_command(&mut self, command: StockCoDiPacketCommandKind);
    fn get_payload(&self) -> Vec<u8>;
    fn set_payload(&mut self, payload: Vec<u8>);
}

#[derive(Debug)]
pub struct StockCoDiPacket {
    header: [u8; 4],
    cmd: StockCoDiPacketCommandKind,
    payload: Vec<u8>,
    direction: PacketDirectionKind,
}

impl StockCoDiPacket {
    pub fn new(
        cmd: StockCoDiPacketCommandKind,
        payload: Vec<u8>,
        direction: PacketDirectionKind,
    ) -> Self {
        Self {
            header: CODI_STOCK_PACKET_HEADER,
            cmd,
            payload,
            direction,
        }
    }
}

impl Default for StockCoDiPacket {
    fn default() -> Self {
        Self {
            header: CODI_STOCK_PACKET_HEADER,
            cmd: Default::default(),
            payload: Vec::new(),
            direction: Default::default(),
        }
    }
}

impl StockCoDiPacketTrait for StockCoDiPacket {
    fn get_header(&self) -> [u8; 4] {
        self.header.clone()
    }
    fn get_command(&self) -> StockCoDiPacketCommandKind {
        self.cmd
    }
    fn set_command(&mut self, command: StockCoDiPacketCommandKind) {
        self.cmd = command;
    }
    fn get_payload(&self) -> Vec<u8> {
        self.payload.clone()
    }
    fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }
}
