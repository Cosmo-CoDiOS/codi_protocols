/// Holder for metadata of each packet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Self_ {
    /// The origin the packet is going.
    #[prost(enumeration = "PacketOriginDest", tag = "1")]
    pub packet_origin: i32,
    /// The destination the serial packet is coming from.
    #[prost(enumeration = "PacketOriginDest", tag = "2")]
    pub packet_destination: i32,
}
/// The origin/destination of the packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PacketOriginDest {
    /// Stock or Rooted Stock Android.
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
impl PacketOriginDest {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PacketOriginDest::Android => "ANDROID",
            PacketOriginDest::CodiOs => "CODI_OS",
            PacketOriginDest::Gemian => "GEMIAN",
            PacketOriginDest::Lineage => "LINEAGE",
            PacketOriginDest::PmOs => "PM_OS",
            PacketOriginDest::Sailfish => "SAILFISH",
            PacketOriginDest::Ubports => "UBPORTS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANDROID" => Some(Self::Android),
            "CODI_OS" => Some(Self::CodiOs),
            "GEMIAN" => Some(Self::Gemian),
            "LINEAGE" => Some(Self::Lineage),
            "PM_OS" => Some(Self::PmOs),
            "SAILFISH" => Some(Self::Sailfish),
            "UBPORTS" => Some(Self::Ubports),
            _ => None,
        }
    }
}
