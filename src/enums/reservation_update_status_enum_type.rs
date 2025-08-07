use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReservationUpdateStatusEnumType {
    /// The reservation is expired.
    Expired,
    /// The reservation is removed.
    Removed,
    /// (2.1) The reservation was used, but no transaction was started.
    NoTransaction,
}

impl fmt::Display for ReservationUpdateStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Expired => write!(f, "Expired"),
            Self::Removed => write!(f, "Removed"),
            Self::NoTransaction => write!(f, "NoTransaction"),
        }
    }
}

impl Into<String> for ReservationUpdateStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for ReservationUpdateStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Expired" => Ok(Self::Expired),
            "Removed" => Ok(Self::Removed),
            "NoTransaction" => Ok(Self::NoTransaction),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "ReservationUpdateStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}