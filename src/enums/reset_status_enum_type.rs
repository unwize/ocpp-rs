use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Result of ResetRequest.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResetStatusEnumType {
    /// Command will be executed.
    Accepted,
    /// Command will not be executed.
    Rejected,
    /// Reset command is scheduled, Charging Station is busy with a process that cannot be interrupted at the moment. Reset will be executed when process is finished.
    Scheduled,
}

impl fmt::Display for ResetStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::Scheduled => write!(f, "Scheduled"),
        }
    }
}

impl Into<String> for ResetStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for ResetStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "Scheduled" => Ok(Self::Scheduled),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "ResetStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
