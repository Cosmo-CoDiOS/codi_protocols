/// Notification of missed call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandTelephonyMissedCallNotify {
    /// Metadata of packet.
    #[prost(message, optional, tag = "1")]
    pub self_: ::core::option::Option<super::protocol_common_self::Self_>,
    /// Taken from Android's built-in spam monitoring; if not supported, default to false.
    #[prost(bool, tag = "2")]
    pub is_spam: bool,
    /// Taken from whenever `codid` is aware of registered known contacts..
    #[prost(bool, tag = "3")]
    pub is_known_contact: bool,
    #[prost(bool, tag = "4")]
    pub suppress: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandSmsNewNotify {
    /// Metadata of packet.
    #[prost(message, optional, tag = "1")]
    pub self_: ::core::option::Option<super::protocol_common_self::Self_>,
    #[prost(bool, tag = "2")]
    pub is_spam: bool,
    #[prost(bool, tag = "3")]
    pub is_known_contact: bool,
    #[prost(bool, tag = "4")]
    pub suppress: bool,
}
