use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TariffKindEnumType {
    /// Default tariff
    DefaultTariff,
    /// Driver-specific tariff
    DriverTariff,
}

impl fmt::Display for TariffKindEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DefaultTariff => write!(f, "DefaultTariff"),
            Self::DriverTariff => write!(f, "DriverTariff"),
        }
    }
}

impl Into<String> for TariffKindEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for TariffKindEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "DefaultTariff" => Ok(Self::DefaultTariff),
            "DriverTariff" => Ok(Self::DriverTariff),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "TariffKindEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
