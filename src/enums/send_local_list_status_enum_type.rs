use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Type of update for SendLocalListRequest.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SendLocalListStatusEnumType {
    /// Local Authorization List successfully updated.
    Accepted,
    /// Failed to update the Local Authorization List.
    Failed,
    /// Version number in the request for a differential update is less or equal then version number of current list.
    VersionMismatch,
}

impl fmt::Display for SendLocalListStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Failed => write!(f, "Failed"),
            Self::VersionMismatch => write!(f, "VersionMismatch"),
        }
    }
}

impl Into<String> for SendLocalListStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for SendLocalListStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Failed" => Ok(Self::Failed),
            "VersionMismatch" => Ok(Self::VersionMismatch),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "SendLocalListStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
