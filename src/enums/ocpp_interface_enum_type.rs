use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration of network interfaces.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum OCPPInterfaceEnumType {
    /// Use wired connection 0
    Wired0,
    /// Use wired connection 1
    Wired1,
    /// Use wired connection 2
    Wired2,
    /// Use wired connection 3
    Wired3,
    /// Use wireless connection 0
    Wireless0,
    /// Use wireless connection 1
    Wireless1,
    /// Use wireless connection 2
    Wireless2,
    /// Use wireless connection 3
    Wireless3,
    /// (2.1) Use any interface.
    Any,
}

impl fmt::Display for OCPPInterfaceEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Wired0 => write!(f, "Wired0"),
            Self::Wired1 => write!(f, "Wired1"),
            Self::Wired2 => write!(f, "Wired2"),
            Self::Wired3 => write!(f, "Wired3"),
            Self::Wireless0 => write!(f, "Wireless0"),
            Self::Wireless1 => write!(f, "Wireless1"),
            Self::Wireless2 => write!(f, "Wireless2"),
            Self::Wireless3 => write!(f, "Wireless3"),
            Self::Any => write!(f, "Any"),
        }
    }
}

impl Into<String> for OCPPInterfaceEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for OCPPInterfaceEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Wired0" => Ok(Self::Wired0),
            "Wired1" => Ok(Self::Wired1),
            "Wired2" => Ok(Self::Wired2),
            "Wired3" => Ok(Self::Wired3),
            "Wireless0" => Ok(Self::Wireless0),
            "Wireless1" => Ok(Self::Wireless1),
            "Wireless2" => Ok(Self::Wireless2),
            "Wireless3" => Ok(Self::Wireless3),
            "Any" => Ok(Self::Any),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "OCPPInterfaceEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}