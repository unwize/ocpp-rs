use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Phase specifies how a measured value is to be interpreted. Please note that not all values of Phase are applicable to all Measurands.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // This handles L1, L2, L3, N, etc. correctly
pub enum PhaseEnumType {
    /// Measured on L1
    L1,
    /// Measured on L2
    L2,
    /// Measured on L3
    L3,
    /// Measured on Neutral
    N,
    /// Measured on L1 with respect to Neutral conductor
    #[serde(rename = "L1-N")]
    L1N,
    /// Measured on L2 with respect to Neutral conductor
    #[serde(rename = "L2-N")]
    L2N,
    /// Measured on L3 with respect to Neutral conductor
    #[serde(rename = "L3-N")]
    L3N,
    /// Measured between L1 and L2
    #[serde(rename = "L1-L2")]
    L1L2,
    /// Measured between L2 and L3
    #[serde(rename = "L2-L3")]
    L2L3,
    /// Measured between L3 and L1
    #[serde(rename = "L3-L1")]
    L3L1,
}

impl fmt::Display for PhaseEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::L1 => write!(f, "L1"),
            Self::L2 => write!(f, "L2"),
            Self::L3 => write!(f, "L3"),
            Self::N => write!(f, "N"),
            Self::L1N => write!(f, "L1-N"),
            Self::L2N => write!(f, "L2-N"),
            Self::L3N => write!(f, "L3-N"),
            Self::L1L2 => write!(f, "L1-L2"),
            Self::L2L3 => write!(f, "L2-L3"),
            Self::L3L1 => write!(f, "L3-L1"),
        }
    }
}

impl Into<String> for PhaseEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for PhaseEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "L1" => Ok(Self::L1),
            "L2" => Ok(Self::L2),
            "L3" => Ok(Self::L3),
            "N" => Ok(Self::N),
            "L1-N" => Ok(Self::L1N),
            "L2-N" => Ok(Self::L2N),
            "L3-N" => Ok(Self::L3N),
            "L1-L2" => Ok(Self::L1L2),
            "L2-L3" => Ok(Self::L2L3),
            "L3-L1" => Ok(Self::L3L1),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "PhaseEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}