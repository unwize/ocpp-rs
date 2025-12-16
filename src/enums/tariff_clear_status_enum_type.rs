use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TariffClearStatusEnumType {
    /// Clearing tariff has been accepted.
    Accepted,
    /// Clearing tariff has been rejected. More info in statusInfo.
    Rejected,
    /// No tariff for EVSE of IdToken
    NoTariff,
}

impl fmt::Display for TariffClearStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::NoTariff => write!(f, "NoTariff"),
        }
    }
}

impl From<TariffClearStatusEnumType> for String {
    fn from(val: TariffClearStatusEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for TariffClearStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "NoTariff" => Ok(Self::NoTariff),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "TariffClearStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
