use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Result of registration in response to BootNotificationRequest.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RegistrationStatusEnumType {
    /// Charging Station is accepted by the CSMS.
    Accepted,
    /// CSMS is not yet ready to accept the Charging Station. CSMS may send messages to retrieve information or prepare the Charging Station.
    Pending,
    /// Charging Station is not accepted by CSMS. This may happen when the Charging Station id is not known by CSMS.
    Rejected,
}

impl fmt::Display for RegistrationStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Pending => write!(f, "Pending"),
            Self::Rejected => write!(f, "Rejected"),
        }
    }
}

impl Into<String> for RegistrationStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for RegistrationStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Pending" => Ok(Self::Pending),
            "Rejected" => Ok(Self::Rejected),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "RegistrationStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}