/// Holder for metadata of each packet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Self_ {
    /// Which direction the serial packet is going in.
    #[prost(enumeration="PacketDirection", tag="1")]
    pub packet_direction: i32,
    /// The origin the packet is going.
    #[prost(enumeration="PacketOriginDest", tag="2")]
    pub packet_origin: i32,
    /// The destination the serial packet is coming from.
    #[prost(enumeration="PacketOriginDest", tag="3")]
    pub packet_destination: i32,
}
/// The direction of the packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PacketDirection {
    /// Packets that go TO the CoDi chip.
    ToCodi = 0,
    /// Packets that come FROM the CoDi chip.
    FromCodi = 1,
}
/// The origin/destination of the packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PacketOriginDest {
    /// Stock or Rooted Android.
    Android = 0,
    /// CoDiOS firmware.
    CodiOs = 1,
    /// Gemian Linux.
    Gemian = 2,
    /// LineageOS custom Android ROM.
    Lineage = 3,
    /// postmarketOS ROM.
    PmOs = 4,
    /// Sailfish ROM.
    Sailfish = 5,
    /// UBPorts ROM.
    Ubports = 6,
}
