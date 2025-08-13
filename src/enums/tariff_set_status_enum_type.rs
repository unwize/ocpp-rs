use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TariffSetStatusEnumType {
    /// Tariff has been accepted.
    Accepted,
    /// Tariff has been rejected. More info in statusInfo.
    Rejected,
    /// Tariff has too many elements and cannot be processed.
    TooManyElements,
    /// A condition is not supported, or conditions are not supported at all.
    ConditionNotSupported,
    /// TariffId already exists in Charging Station.
    DuplicateTariffId,
}

impl fmt::Display for TariffSetStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::TooManyElements => write!(f, "TooManyElements"),
            Self::ConditionNotSupported => write!(f, "ConditionNotSupported"),
            Self::DuplicateTariffId => write!(f, "DuplicateTariffId"),
        }
    }
}

impl Into<String> for TariffSetStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for TariffSetStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "TooManyElements" => Ok(Self::TooManyElements),
            "ConditionNotSupported" => Ok(Self::ConditionNotSupported),
            "DuplicateTariffId" => Ok(Self::DuplicateTariffId),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "TariffSetStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
