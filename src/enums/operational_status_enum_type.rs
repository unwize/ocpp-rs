use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Requested availability change.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum OperationalStatusEnumType {
    /// Charging Station is not available for charging.
    #[default]
    Inoperative,
    /// Charging Station is available for charging.
    Operative,
}

impl fmt::Display for OperationalStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Inoperative => write!(f, "Inoperative"),
            Self::Operative => write!(f, "Operative"),
        }
    }
}

impl Into<String> for OperationalStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for OperationalStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Inoperative" => Ok(Self::Inoperative),
            "Operative" => Ok(Self::Operative),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "OperationalStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
