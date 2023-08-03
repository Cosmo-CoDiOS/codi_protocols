#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFlashCommand {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFlashResponse {
    #[prost(string, tag = "1")]
    pub firmware_version: ::prost::alloc::string::String,
    #[prost(
        enumeration = "super::protocol_common_variants::CoDiVariant",
        tag = "2"
    )]
    pub firmware_variant: i32,
}
