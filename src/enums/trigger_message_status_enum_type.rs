use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Status in TriggerMessageResponse.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerMessageStatusEnumType {
    /// Requested message will be sent.
    Accepted,
    /// Requested message will not be sent.
    Rejected,
    /// Requested message cannot be sent because it is either not implemented or unknown.
    NotImplemented,
}

impl fmt::Display for TriggerMessageStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::NotImplemented => write!(f, "NotImplemented"),
        }
    }
}

impl Into<String> for TriggerMessageStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for TriggerMessageStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "NotImplemented" => Ok(Self::NotImplemented),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "TriggerMessageStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
