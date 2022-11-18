//! This crate defines the JSON-RPC protocol between `codid` and the host ROM, which in turn
//! communicates with either `CoDiOS` or stock `CoDi`.
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
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum OriginDestROMKind {
    Android,
    UBPorts,
    PostmarketOS,
    Gemian,
    Undefined,
}

impl Default for OriginDestROMKind {
    fn default() -> Self {
        Self::Undefined
    }
}

type OriginROMKind = OriginDestROMKind;
type DestROMKind = OriginDestROMKind;

/// Metadata JSON object for RPC requests.
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMetadata {
    pub origin_rom: OriginROMKind,
}

/// Metadata JSON object for RPC responses
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    pub dest_rom: DestROMKind,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum QueueStateKind {
    Paused,
    Rejecting,
    Accepting,
    Undefined,
}

impl Default for QueueStateKind {
    fn default() -> Self {
        Self::Undefined
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct QueueStateChangeMethodRequest {
    #[serde(rename = "_self")]
    /// Metadata struct.
    pub self_metadata: RequestMetadata,
    pub queue_state: QueueStateKind,
}
