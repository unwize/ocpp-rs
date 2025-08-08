use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use crate::errors::OcppError;

/// State of the Charging Station during which a message SHALL be displayed.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum MessageStateEnumType {
    /// Message only to be shown while the Charging Station is charging.
    Charging,
    /// Message only to be shown while the Charging Station is in faulted state.
    Faulted,
    /// Message only to be shown while the Charging Station is idle (no transaction active).
    Idle,
    /// Message only to be shown while the Charging Station is in unavailable state.
    Unavailable,
    /// (2.1) Message only to be shown when Charging Station (or EV) has suspending the charging during a transaction.
    Suspended,
    /// (2.1) Message only to be shown while the EV is discharging.
    Discharging,
}

impl fmt::Display for MessageStateEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Charging => write!(f, "Charging"),
            Self::Faulted => write!(f, "Faulted"),
            Self::Idle => write!(f, "Idle"),
            Self::Unavailable => write!(f, "Unavailable"),
            Self::Suspended => write!(f, "Suspended"),
            Self::Discharging => write!(f, "Discharging"),
        }
    }
}

impl Into<String> for MessageStateEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for MessageStateEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Charging" => Ok(Self::Charging),
            "Faulted" => Ok(Self::Faulted),
            "Idle" => Ok(Self::Idle),
            "Unavailable" => Ok(Self::Unavailable),
            "Suspended" => Ok(Self::Suspended),
            "Discharging" => Ok(Self::Discharging),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MessageStateEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}