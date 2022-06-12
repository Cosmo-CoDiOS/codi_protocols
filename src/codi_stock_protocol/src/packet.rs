//! This module is for the stock CoDi packet handling.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

use alloc::vec::Vec;

use crate::stock_commands::StockCoDiPacketCommand;

const CODI_STOCK_PACKET_HEADER: [u8; 4] = [58, 21, 58, 21];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PacketSendError {
    UartNotAvailable,
    MiscError,
}

/// This trait (`StockCoDiPacketTrait`) acts as a specificiation for each indvidiual incoming/outgoing stock CoDi serial packet.
pub trait StockCoDiPacketTrait {
    fn send_packet(&self) -> Result<(), PacketSendError>;
    fn get_header(&self) -> [u8; 4];
    fn get_command(&self) -> StockCoDiPacketCommand;
    fn get_payload(&self) -> Vec<u8>;
    fn set_payload(&mut self, payload: Vec<u8>);
}

pub struct StockCoDiPacket {
    header: [u8; 4],
    cmd: StockCoDiPacketCommand,
    payload: Vec<u8>,
}

impl StockCoDiPacket {
    #[allow(dead_code)] // temporary fix
    pub fn new(cmd: StockCoDiPacketCommand, payload: Vec<u8>) -> Self {
        Self {
            header: CODI_STOCK_PACKET_HEADER,
            cmd: cmd,
            payload: payload,
        }
    }
}

impl Default for StockCoDiPacket {
    fn default() -> Self {
        Self {
            header: CODI_STOCK_PACKET_HEADER,
            cmd: StockCoDiPacketCommand::default(),
            payload: Vec::new()
        }
    }
}

impl StockCoDiPacketTrait for StockCoDiPacket {
    fn send_packet(&self) -> Result<(), PacketSendError> {
        Ok(())
    }

    fn get_command(&self) -> StockCoDiPacketCommand {
        self.cmd
    }

    fn get_header(&self) -> [u8; 4] {
        self.header
    }

    fn get_payload(&self) -> Vec<u8> {
        self.payload.clone()
    }

    fn set_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }
}

#[cfg(test)]
mod tests {
    use super::{StockCoDiPacket, StockCoDiPacketCommand, StockCoDiPacketTrait};

    #[test]
    fn test_default_packet_representation() {
        let packet = StockCoDiPacket::default();

        assert_eq!(packet.get_header(), [58, 21, 58, 21]);
        assert_eq!(packet.get_command(), StockCoDiPacketCommand::UNDEFINED);
        assert_eq!(packet.get_payload(), []);
    }
}
