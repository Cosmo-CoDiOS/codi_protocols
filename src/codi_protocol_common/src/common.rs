//! This crate handles UART communication between the `CoDi` chip and the host
//! ROM.

// Compiles with both `no_std` and `std`.

#[cfg(target_arch = "arm")]
use alloc::string::String;

#[cfg(target_arch = "arm")]
use alloc::vec::Vec;

#[cfg(target_arch = "arm")]
use core2::io::{Read, Write};

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
use std::io::{Read, Write};

use self::errors::SerialPortManagerError;

pub mod errors {
    //! This is an collection of 'errors' that *could* be encountered by `SerialPortManager`.
    use core2::io::Error as IoError;
    use thiserror_no_std::Error;

    #[allow(dead_code)]
    #[derive(Default, Debug, Error)]
    /// This enum is a plain Enum currently.
    /// At some point, we will use an `no_std`-compatible error handling crate.
    pub enum SerialPortManagerError {
        /// This type of error is when an I/O problem occurs when reading or writing
        /// to the UART peripheral.
        #[error("Error performing I/O on UART.")]
        IoError(#[from] IoError),
        /// This error occurs when the UART peripheral cannot be opened.
        #[error("UART could not be opened: {uart_port:?}, busy?: {busy:?}")]
        PortOpenError {
            /// A String representation of the UART port ID/device node.
            uart_port: String,
            /// `bool` that represents if the UART port is busy.
            busy: bool,
        },
        #[default]
        /// Unknown error, avoid using this.
        #[error("Unknown error with SerialPortManager.")]
        Unknown,
    }
}

/// This is a custom type alias for `anyhow` in the context of `SerialPortManager`.
pub type SerialPortManagerResult<T> = anyhow::Result<T, SerialPortManagerError>;

/// `SerialPortManagerTrait` acts as a generic way to implement a
/// `SerialPortManager` struct in both `no_std` and `std` environments.
pub trait SerialPortManagerTrait {
    /// This method opens the UART device (incl. `no_std`), and assigns it (via
    /// a mutable reference) to the struct. It accepts a *optional* device
    /// node (for use on the Cosmo itself, and development machines), and a baud
    /// rate.
    ///
    /// It returns a `Result<(), SerialPortManagerError`. Calling functions
    /// should react on `Error`s returned, and assume a empty `Result` has
    /// connected to the UART line.
    fn open_serial_port(
        &mut self,
        dev_node: Option<&str>,
        baud_rate: u32,
    ) -> SerialPortManagerResult<()>;

    #[allow(missing_docs)]
    fn open_serial_upload_mode(&mut self, dev_node: Option<&str>) -> SerialPortManagerResult<()>;

    #[allow(missing_docs)]
    fn open_serial_cmd_mode(&mut self, dev_node: Option<&str>) -> SerialPortManagerResult<()>;

    /// This method is quite generic. It reads a certain amount into a buffer
    /// (based on the value of the `buf_size` parameter), and returns a
    /// `Option<Vec<u8>>`, wrapped in a `Result`. Calling functions should
    /// test for `Error`s, react on those, and store the packet in the struct.
    ///
    /// Later revisions of the `Connection` struct will use a
    /// `core::cell::RefCell` to perform GC on stored packets, and act as a
    /// continual 'grim reaper' of processed packets.
    fn read_packet<R>(
        &mut self,
        _reader: R,
        _buf_size: usize,
    ) -> SerialPortManagerResult<Option<Vec<u8>>>
    where
        R: Read;

    /// This method is fairly simple. It accepts a object that implements
    /// `core2::io::Write`, and a 'payload' (of type: `Vec<u8>`) for sending
    /// over UART. The payload can be anything, as long as it's a Vec of
    /// `u8`. It does *not* perform any checks on the payload, and sends as-is.
    ///
    /// It returns a `usize` of the payload, wrapped in a `Result<usize,
    /// SerialPortManagerError`.
    ///
    /// Calling functions should test for `Error`s, react on those, and continue
    /// the flow of execution.
    fn write_packet<W>(&mut self, _writer: W, _payload: Vec<u8>) -> SerialPortManagerResult<usize>
    where
        W: Write;
}

/// `PacketDirectionKind` defines the direction of each `CoDi` packet.
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum PacketDirectionKind {
    /// `ToCoDi` variant is when the packet is being sent via UART to the STM32.
    ToCoDi,
    /// `FromCoDi` variant is when the packet is being sent via UART to the STM32.
    FromCoDi,
    #[default]
    /// This is the default variant, and should be avoided.
    Unknown,
}
