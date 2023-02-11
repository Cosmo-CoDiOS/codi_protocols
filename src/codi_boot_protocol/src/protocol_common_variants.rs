/// Enum defines the two (currently) different variants of CoDi.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CoDiVariant {
    /// Stock CoDi enum value is an integer.
    Stock = 0,
    /// CoDiOS enum value is an integer.
    Codios = 1,
}
impl CoDiVariant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CoDiVariant::Stock => "STOCK",
            CoDiVariant::Codios => "CODIOS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STOCK" => Some(Self::Stock),
            "CODIOS" => Some(Self::Codios),
            _ => None,
        }
    }
}
