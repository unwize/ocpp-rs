use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TariffGetStatusEnumType {
    /// Tariff has been accepted.
    Accepted,
    /// Tariff has been rejected. More info in statusInfo.
    Rejected,
    /// No tariff present on Charging Station or EVSE.
    NoTariff,
}

impl fmt::Display for TariffGetStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::NoTariff => write!(f, "NoTariff"),
        }
    }
}

impl Into<String> for TariffGetStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for TariffGetStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "NoTariff" => Ok(Self::NoTariff),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "TariffGetStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
