use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TariffChangeStatusEnumType {
    /// Tariff has been accepted.
    Accepted,
    /// Tariff has been rejected. More info in statusInfo.
    Rejected,
    /// Tariff has too many elements and cannot be processed.
    TooManyElements,
    /// A condition is not supported, or conditions are not supported at all.
    ConditionNotSupported,
    /// Transaction does not exist or has already ended.
    TxNotFound,
    /// Cannot change currency during a transaction.
    NoCurrencyChange,
}

impl fmt::Display for TariffChangeStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::TooManyElements => write!(f, "TooManyElements"),
            Self::ConditionNotSupported => write!(f, "ConditionNotSupported"),
            Self::TxNotFound => write!(f, "TxNotFound"),
            Self::NoCurrencyChange => write!(f, "NoCurrencyChange"),
        }
    }
}

impl Into<String> for TariffChangeStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for TariffChangeStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "TooManyElements" => Ok(Self::TooManyElements),
            "ConditionNotSupported" => Ok(Self::ConditionNotSupported),
            "TxNotFound" => Ok(Self::TxNotFound),
            "NoCurrencyChange" => Ok(Self::NoCurrencyChange),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "TariffChangeStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
