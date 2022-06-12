#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFlashRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFlashResponse {
    #[prost(message, optional, tag="1")]
    pub self_: ::core::option::Option<super::protocol_common_self::Self_>,
    #[prost(string, tag="2")]
    pub firmware_version: ::prost::alloc::string::String,
    #[prost(enumeration="super::protocol_common_variants::CoDiVariant", tag="3")]
    pub firmware_variant: i32,
}
/// Initial 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitFlashingRequest {
}
