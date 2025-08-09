use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum SetMonitoringStatusEnumType {
    /// Monitor successfully set.
    Accepted,
    /// Component is not known.
    UnknownComponent,
    /// Variable is not known.
    UnknownVariable,
    /// Requested monitor type is not supported.
    UnsupportedMonitorType,
    /// Request is rejected.
    Rejected,
    /// A monitor already exists for the given type/severity combination.
    Duplicate,
}

impl fmt::Display for SetMonitoringStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::UnknownComponent => write!(f, "UnknownComponent"),
            Self::UnknownVariable => write!(f, "UnknownVariable"),
            Self::UnsupportedMonitorType => write!(f, "UnsupportedMonitorType"),
            Self::Rejected => write!(f, "Rejected"),
            Self::Duplicate => write!(f, "Duplicate"),
        }
    }
}

impl Into<String> for SetMonitoringStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for SetMonitoringStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "UnknownComponent" => Ok(Self::UnknownComponent),
            "UnknownVariable" => Ok(Self::UnknownVariable),
            "UnsupportedMonitorType" => Ok(Self::UnsupportedMonitorType),
            "Rejected" => Ok(Self::Rejected),
            "Duplicate" => Ok(Self::Duplicate),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "SetMonitoringStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}