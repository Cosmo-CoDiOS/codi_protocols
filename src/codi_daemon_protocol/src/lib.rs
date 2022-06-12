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

use serde::{Serialize, Deserialize};

/// Metadata JSON object for RPC requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetadata {
    /// Timestamp of the message *to* the daemon.
    #[serde(rename = "timeStamp")]
    pub time_stamp: String
}


/// Metadata JSON object for RPC responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Timestamp of the message *from* the daemon.
    #[serde(rename = "timeStamp")]
    pub time_stamp: String
}
