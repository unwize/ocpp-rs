use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration of OCPP versions.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OCPPVersionEnumType {
    /// OCPP version 1.2
    OCPP12,
    /// OCPP version 1.5
    OCPP15,
    /// OCPP version 1.6, websocket subprotocol: ocpp1.6
    OCPP16,
    /// No longer in use. The OCPP 2.0 release of OCPP has been withdrawn. The value OCPP20 is treated as OCPP2.0.1.
    OCPP20,
    /// OCPP version 2.0.1, websocket subprotocol: ocpp2.0.1
    OCPP201,
    /// (2.1) OCPP version 2.1, websocket subprotocol: ocpp2.1
    OCPP21,
}

impl fmt::Display for OCPPVersionEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OCPP12 => write!(f, "OCPP12"),
            Self::OCPP15 => write!(f, "OCPP15"),
            Self::OCPP16 => write!(f, "OCPP16"),
            Self::OCPP20 => write!(f, "OCPP20"),
            Self::OCPP201 => write!(f, "OCPP201"),
            Self::OCPP21 => write!(f, "OCPP21"),
        }
    }
}

impl Into<String> for OCPPVersionEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for OCPPVersionEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "OCPP12" => Ok(Self::OCPP12),
            "OCPP15" => Ok(Self::OCPP15),
            "OCPP16" => Ok(Self::OCPP16),
            "OCPP20" => Ok(Self::OCPP20),
            "OCPP201" => Ok(Self::OCPP201),
            "OCPP21" => Ok(Self::OCPP21),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "OCPPVersionEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}