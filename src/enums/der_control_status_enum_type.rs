use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum DERControlStatusEnumType {
    /// Operation successful.
    #[default]
    Accepted,
    /// Operation failed.
    Rejected,
    /// Type of DER setting or curve is not supported.
    NotSupported,
    /// Type or Id in clear/get request was not found.
    NotFound,
}

impl TryFrom<String> for DERControlStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(DERControlStatusEnumType::Accepted),
            "Rejected" => Ok(DERControlStatusEnumType::Rejected),
            "NotSupported" => Ok(DERControlStatusEnumType::NotSupported),
            "NotFound" => Ok(DERControlStatusEnumType::NotFound),
            _ => Err(format!("'{}' is not a valid DERControlStatusEnumType", s)),
        }
    }
}

impl Into<String> for DERControlStatusEnumType {
    fn into(self) -> String {
        match self {
            DERControlStatusEnumType::Accepted => "Accepted".to_string(),
            DERControlStatusEnumType::Rejected => "Rejected".to_string(),
            DERControlStatusEnumType::NotSupported => "NotSupported".to_string(),
            DERControlStatusEnumType::NotFound => "NotFound".to_string(),
        }
    }
}
