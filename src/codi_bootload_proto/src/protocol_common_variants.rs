/// Enum defines the two (currently) different variants of CoDi.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CoDiVariant {
    /// Stock CoDi enum value is an integer.
    Stock = 0,
    /// CoDiOS enum value is an integer.
    Codios = 1,
}
