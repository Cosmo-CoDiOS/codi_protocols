//! This module is for the stock CoDi packet handling.

use alloc::vec::Vec;

use crate::stock_commands::StockCoDiPacketCommand;

const CODI_STOCK_PACKET_HEADER: [u8; 4] = [58, 21, 58, 21];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PacketSendError {
    UartNotAvailable,
    MiscError,
}

impl Default for PacketSendError {
    fn default() -> Self {
        Self::MiscError
    }
}

/// This trait (`StockCoDiPacketTrait`) acts as a specification for each individual incoming/outgoing stock `CoDi` serial packet.
pub trait StockCoDiPacketTrait {
    fn send_packet(&self) -> Result<(), PacketSendError>;
    fn get_header(&self) -> [u8; 4];
    fn get_command(&self) -> StockCoDiPacketCommand;
    fn set_command(&mut self, command: StockCoDiPacketCommand);
    fn get_payload(&self) -> Vec<u8>;
    fn set_payload(&mut self, payload: Vec<u8>);
}
#[derive(Debug)]
pub struct StockCoDiPacket {
    header: [u8; 4],
    cmd: StockCoDiPacketCommand,
    payload: Vec<u8>,
}

impl StockCoDiPacket {
    pub fn new(cmd: StockCoDiPacketCommand, payload: Vec<u8>) -> Self {
        Self {
            header: CODI_STOCK_PACKET_HEADER,
            cmd,
            payload,
        }
    }
}

impl Default for StockCoDiPacket {
    fn default() -> Self {
        Self {
            header: CODI_STOCK_PACKET_HEADER,
            cmd: StockCoDiPacketCommand::default(),
            payload: Vec::new(),
        }
    }
}

impl StockCoDiPacketTrait for StockCoDiPacket {
    fn send_packet(&self) -> Result<(), PacketSendError> {
        Ok(())
    }

    fn get_header(&self) -> [u8; 4] {
        self.header
    }

    fn get_command(&self) -> StockCoDiPacketCommand {
        self.cmd
    }

    fn set_command(&mut self, command: StockCoDiPacketCommand) {
        self.cmd = command;
    }

    fn get_payload(&self) -> Vec<u8> {
        self.payload.clone()
    }

    fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }
}
