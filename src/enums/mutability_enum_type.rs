use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum MutabilityEnumType {
    /// This variable is read-only.
    ReadOnly,
    /// This variable is write-only.
    WriteOnly,
    /// This variable is read-write.
    ReadWrite,
}

impl fmt::Display for MutabilityEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReadOnly => write!(f, "ReadOnly"),
            Self::WriteOnly => write!(f, "WriteOnly"),
            Self::ReadWrite => write!(f, "ReadWrite"),
        }
    }
}

impl Into<String> for MutabilityEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for MutabilityEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ReadOnly" => Ok(Self::ReadOnly),
            "WriteOnly" => Ok(Self::WriteOnly),
            "ReadWrite" => Ok(Self::ReadWrite),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MutabilityEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
