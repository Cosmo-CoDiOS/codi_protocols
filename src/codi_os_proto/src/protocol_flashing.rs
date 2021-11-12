#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFlashRequest {
    #[prost(int32, tag="1")]
    pub firmware_version: i32,
    #[prost(enumeration="super::protocol_common_variants::CoDiVariant", tag="2")]
    pub firmware_variant: i32,
}
