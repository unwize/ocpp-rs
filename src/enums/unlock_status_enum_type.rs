use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Status in response to UnlockConnectorRequest.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnlockStatusEnumType {
    /// Connector has successfully been unlocked.
    Unlocked,
    /// Failed to unlock the connector.
    UnlockFailed,
    /// The connector is not unlocked, because there is still an authorized transaction ongoing.
    OngoingAuthorizedTransaction,
    /// The specified connector is not known by the Charging Station.
    UnknownConnector,
}

impl fmt::Display for UnlockStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Unlocked => write!(f, "Unlocked"),
            Self::UnlockFailed => write!(f, "UnlockFailed"),
            Self::OngoingAuthorizedTransaction => write!(f, "OngoingAuthorizedTransaction"),
            Self::UnknownConnector => write!(f, "UnknownConnector"),
        }
    }
}

impl From<UnlockStatusEnumType> for String {
    fn from(val: UnlockStatusEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for UnlockStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Unlocked" => Ok(Self::Unlocked),
            "UnlockFailed" => Ok(Self::UnlockFailed),
            "OngoingAuthorizedTransaction" => Ok(Self::OngoingAuthorizedTransaction),
            "UnknownConnector" => Ok(Self::UnknownConnector),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "UnlockStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
