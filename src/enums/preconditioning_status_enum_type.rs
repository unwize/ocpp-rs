use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// (2.1) Preconditioning status of the battery
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PreconditioningStatusEnumType {
    /// No information available on the status of preconditioning
    Unknown,
    /// The battery is preconditioned and ready to react directly on a given setpoint for charging (and discharging when available).
    Ready,
    /// Busy with preconditioning the BMS. When done will move to status Ready.
    NotReady,
    /// The battery is not preconditioned and not able to directly react to given setpoint.
    Preconditioning,
}

impl fmt::Display for PreconditioningStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Ready => write!(f, "Ready"),
            Self::NotReady => write!(f, "NotReady"),
            Self::Preconditioning => write!(f, "Preconditioning"),
        }
    }
}

impl Into<String> for PreconditioningStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for PreconditioningStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Unknown" => Ok(Self::Unknown),
            "Ready" => Ok(Self::Ready),
            "NotReady" => Ok(Self::NotReady),
            "Preconditioning" => Ok(Self::Preconditioning),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "PreconditioningStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
