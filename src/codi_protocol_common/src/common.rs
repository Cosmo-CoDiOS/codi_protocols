//! This crate handles UART communication between the `CoDi` chip and the host
//! ROM.

// Compiles with both `no_std` and `std`.

use core2::io::{Error as IoError, Read, Write};

#[allow(dead_code)]
#[derive(Debug)]
/// This enum is a plain Enum currently.
/// At some point, we will use an `no_std`-compatible error handling crate.
pub enum SerialPortManagerError {
    /// This type of error is when an I/O problem occurs when reading or writing
    /// to the UART peripheral.
    IoError(IoError),
    /// This error occurs when the UART peripheral cannot be opened.
    PortOpenError,
}

/// `SerialPortManagerTrait` acts as a generic way to implement a
/// SerialPortManager struct in both `no_std` and `std` environments.
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
    ) -> Result<(), SerialPortManagerError>;

    /// This method is quite generic. It reads a certain amount into a buffer
    /// (based on the value of the `buf_size` parameter), and returns a
    /// `Option<Vec<u8>>`, wrapped in a `Result`. Calling functions should
    /// test for `Error`s, react on those, and store the packet in the struct.
    ///
    /// Later revisions of the `Connection` struct will use a
    /// `core::cell::RefCell` to perform GC on stored packets, and act as a
    /// continual 'grim reaper' of processed packets.
    fn read_packet<R>(
        &self,
        _reader: R,
        _buf_size: usize,
    ) -> Result<Option<Vec<u8>>, SerialPortManagerError>
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
    fn write_packet<W>(
        &mut self,
        _writer: W,
        _payload: Vec<u8>,
    ) -> Result<usize, SerialPortManagerError>
    where
        W: Write;
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum PacketSendError {
    UartNotAvailable,
    UartBusy,
    #[default]
    MiscError,
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum PacketDirectionKind {
    ToCoDi,
    FromCoDi,
    #[default]
    Unknown,
}
