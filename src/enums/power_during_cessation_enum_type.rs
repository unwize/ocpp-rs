use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PowerDuringCessationEnumType {
    /// Active power
    Active,
    /// Reactive power
    Reactive,
}

impl fmt::Display for PowerDuringCessationEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Active => write!(f, "Active"),
            Self::Reactive => write!(f, "Reactive"),
        }
    }
}

impl Into<String> for PowerDuringCessationEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for PowerDuringCessationEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Active" => Ok(Self::Active),
            "Reactive" => Ok(Self::Reactive),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "PowerDuringCessationEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}