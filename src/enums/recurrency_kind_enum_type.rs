use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RecurrencyKindEnumType {
    /// The schedule restarts every 24 hours, at the same time as in the startSchedule.
    Daily,
    /// The schedule restarts every 7 days, at the same time and day-of-the-week as in the startSchedule.
    Weekly,
}

impl fmt::Display for RecurrencyKindEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Daily => write!(f, "Daily"),
            Self::Weekly => write!(f, "Weekly"),
        }
    }
}

impl Into<String> for RecurrencyKindEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for RecurrencyKindEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Daily" => Ok(Self::Daily),
            "Weekly" => Ok(Self::Weekly),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "RecurrencyKindEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}