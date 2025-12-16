use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Indicates how the Local Authorization List must be updated.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateEnumType {
    /// Indicates that the current Local Authorization List must be updated with the values in this message.
    Differential,
    /// Indicates that the current Local Authorization List must be replaced by the values in this message.
    Full,
}

impl fmt::Display for UpdateEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Differential => write!(f, "Differential"),
            Self::Full => write!(f, "Full"),
        }
    }
}

impl From<UpdateEnumType> for String {
    fn from(val: UpdateEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for UpdateEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Differential" => Ok(Self::Differential),
            "Full" => Ok(Self::Full),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "UpdateEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
