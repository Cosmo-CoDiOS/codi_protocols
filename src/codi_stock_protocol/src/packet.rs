//! This module is for the stock `CoDi` packet handling.

use codi_protocol_common::common::PacketDirectionKind;

use crate::StockCoDiPacketCommandKind;

/// This is the header used in the stock `CoDi` protocol.
const CODI_STOCK_PACKET_HEADER: [u8; 4] = [58, 21, 58, 21];

/// This trait (`StockCoDiPacketTrait`) acts as a specification for each
/// individual incoming/outgoing stock `CoDi` serial packet.
pub trait StockCoDiPacketTrait {
    /// This returns the header of each 'stock' `CoDi` packet
    fn get_header(&self) -> [u8; 4] {
        CODI_STOCK_PACKET_HEADER
    }
    /// This method returns the command used in the packet.
    fn get_command(&self) -> StockCoDiPacketCommandKind;
    /// This method sets, and returns no value, the command used in the packet.
    fn set_command(&mut self, command: StockCoDiPacketCommandKind);
    /// This method returns the payload used in the packet.
    fn get_payload(&self) -> Vec<u8>;
    /// This method sets, and returns no value, the payload used in the packet.
    fn set_payload(&mut self, payload: Vec<u8>);
}

#[derive(Debug)]
/// This struct is an *individual* container of each `CoDi` packet.
pub struct StockCoDiPacket {
    cmd: StockCoDiPacketCommandKind,
    payload: Vec<u8>,
    direction: PacketDirectionKind,
}

impl StockCoDiPacket {
    /// This function accepts a 'Command', 'Payload', and the 'Direction' that the `CoDi` packet is
    /// going.
    pub fn new(
        cmd: StockCoDiPacketCommandKind,
        payload: Vec<u8>,
        direction: PacketDirectionKind,
    ) -> Self {
        Self {
            cmd,
            payload,
            direction,
        }
    }
}

impl Into<Vec<u8>> for StockCoDiPacket {
    fn into(self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        data.extend(CODI_STOCK_PACKET_HEADER.to_vec());
        data.push(self.cmd as u8);
        data.extend(&self.payload);

        data
    }
}

impl Default for StockCoDiPacket {
    fn default() -> Self {
        Self {
            cmd: Default::default(),
            payload: Vec::new(),
            direction: Default::default(),
        }
    }
}

impl StockCoDiPacketTrait for StockCoDiPacket {
    fn get_header(&self) -> [u8; 4] {
        CODI_STOCK_PACKET_HEADER
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
