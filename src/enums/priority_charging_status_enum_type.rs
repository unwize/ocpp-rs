use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// (2.1) Status of a UsePriorityChargingRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PriorityChargingStatusEnumType {
    /// Request has been accepted.
    Accepted,
    /// Request has been rejected.
    Rejected,
    /// No priority charging profile present.
    NoProfile,
}

impl fmt::Display for PriorityChargingStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::NoProfile => write!(f, "NoProfile"),
        }
    }
}

impl From<PriorityChargingStatusEnumType> for String {
    fn from(val: PriorityChargingStatusEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for PriorityChargingStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "NoProfile" => Ok(Self::NoProfile),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "PriorityChargingStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
