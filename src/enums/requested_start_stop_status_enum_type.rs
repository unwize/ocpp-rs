use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// The result of a RequestStartTransactionRequest or RequestStopTransactionRequest.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RequestStartStopStatusEnumType {
    /// Command will be executed.
    Accepted,
    /// Command will not be executed.
    Rejected,
}

impl fmt::Display for RequestStartStopStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
        }
    }
}

impl Into<String> for RequestStartStopStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for RequestStartStopStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "RequestStartStopStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
