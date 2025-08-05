use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum EvseKindEnumType {
    /// AC current EVSE
    AC,
    /// DC current EVSE
    DC,
}

impl TryFrom<String> for EvseKindEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "AC" => Ok(EvseKindEnumType::AC),
            "DC" => Ok(EvseKindEnumType::DC),
            _ => Err(format!("'{}' is not a valid EvseKindEnumType", s)),
        }
    }
}

impl Into<String> for EvseKindEnumType {
    fn into(self) -> String {
        match self {
            EvseKindEnumType::AC => "AC".to_string(),
            EvseKindEnumType::DC => "DC".to_string(),
        }
    }
}
