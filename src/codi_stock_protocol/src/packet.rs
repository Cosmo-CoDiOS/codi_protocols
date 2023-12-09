//! This module is for the stock `CoDi` packet handling.

use codi_protocol_common::common::PacketDirectionKind;

use crate::StockCoDiPacketCommandKind;

/// This is the header used in the stock `CoDi` protocol.
const CODI_STOCK_PACKET_HEADER: [u8; 4] = [58, 21, 58, 21];

/// This trait (`StockCoDiPacketTrait`) acts as a specification for each
/// individual incoming/outgoing stock `CoDi` serial packet.
pub trait StockCoDiPacketTrait {
    /// Return header as a 4 byte `[u8; 4]` array.
    #[must_use]
    fn get_header() -> [u8; 4] {
        CODI_STOCK_PACKET_HEADER
    }
    /// Return header as a `Vec<u8>`
    #[must_use]
    fn get_header_as_vec() -> Vec<u8> {
        CODI_STOCK_PACKET_HEADER.to_vec()
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
#[derive(Default)]
pub struct StockCoDiPacket {
    cmd: StockCoDiPacketCommandKind,
    payload: Vec<u8>,
    direction: PacketDirectionKind,
}

impl StockCoDiPacket {
    /// This function accepts a 'Command', 'Payload', and the 'Direction' that the `CoDi` packet is
    /// going.
    #[must_use]
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

impl From<StockCoDiPacket> for Vec<u8> {
    fn from(val: StockCoDiPacket) -> Self {
        let mut data: Vec<u8> = Vec::new();

        data.extend(StockCoDiPacket::get_header_as_vec());
        data.push(val.cmd as u8);
        data.extend(&val.payload);

        data
    }
}

impl StockCoDiPacketTrait for StockCoDiPacket {
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
