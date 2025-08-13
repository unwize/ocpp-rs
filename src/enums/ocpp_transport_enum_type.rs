use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration of OCPP transport mechanisms. SOAP is currently not a valid value for OCPP 2.0.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum OCPPTransportEnumType {
    /// Use SOAP for transport of OCPP PDU's
    SOAP,
    /// Use JSON over WebSockets for transport of OCPP PDU's
    JSON,
}

impl fmt::Display for OCPPTransportEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SOAP => write!(f, "SOAP"),
            Self::JSON => write!(f, "JSON"),
        }
    }
}

impl Into<String> for OCPPTransportEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for OCPPTransportEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "SOAP" => Ok(Self::SOAP),
            "JSON" => Ok(Self::JSON),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "OCPPTransportEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
