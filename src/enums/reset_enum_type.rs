use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Type of reset requested.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResetEnumType {
    /// Immediate reset of the Charging Station or EVSE.
    Immediate,
    /// Delay reset until no more transactions are active.
    OnIdle,
    /// (2.1) Immediate reset and resume transaction(s) afterwards
    ImmediateAndResume,
}

impl fmt::Display for ResetEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Immediate => write!(f, "Immediate"),
            Self::OnIdle => write!(f, "OnIdle"),
            Self::ImmediateAndResume => write!(f, "ImmediateAndResume"),
        }
    }
}

impl Into<String> for ResetEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for ResetEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Immediate" => Ok(Self::Immediate),
            "OnIdle" => Ok(Self::OnIdle),
            "ImmediateAndResume" => Ok(Self::ImmediateAndResume),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "ResetEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
