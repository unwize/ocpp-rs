use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum TransactionEventEnumType {
    /// Last event of a transaction
    Ended,
    /// First event of a transaction.
    Started,
    /// Transaction event in between 'Started' and 'Ended'.
    Updated,
}

impl fmt::Display for TransactionEventEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Ended => write!(f, "Ended"),
            Self::Started => write!(f, "Started"),
            Self::Updated => write!(f, "Updated"),
        }
    }
}

impl Into<String> for TransactionEventEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for TransactionEventEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Ended" => Ok(Self::Ended),
            "Started" => Ok(Self::Started),
            "Updated" => Ok(Self::Updated),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "TransactionEventEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
