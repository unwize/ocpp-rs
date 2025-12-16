use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Status result of a NotifyAllowedEnergyTransferRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NotifyAllowedEnergyTransferStatusEnumType {
    /// Request has been accepted.
    Accepted,
    /// Request has been rejected. Should not occur, unless there are some technical problems.
    Rejected,
}

impl fmt::Display for NotifyAllowedEnergyTransferStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
        }
    }
}

impl From<NotifyAllowedEnergyTransferStatusEnumType> for String {
    fn from(val: NotifyAllowedEnergyTransferStatusEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for NotifyAllowedEnergyTransferStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "NotifyAllowedEnergyTransferStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
