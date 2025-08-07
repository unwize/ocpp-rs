use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// ISO 15118-20 service parameter for mobility needs mode.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MobilityNeedsModeEnumType {
    /// Only EV determines min/target SOC and departure time.
    EVCC,
    /// Charging station or CSMS may also update min/target SOC and departure time.
    EVCC_SECC,
}

impl fmt::Display for MobilityNeedsModeEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EVCC => write!(f, "EVCC"),
            Self::EVCC_SECC => write!(f, "EVCC_SECC"),
        }
    }
}

impl Into<String> for MobilityNeedsModeEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for MobilityNeedsModeEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "EVCC" => Ok(Self::EVCC),
            "EVCC_SECC" => Ok(Self::EVCC_SECC),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MobilityNeedsModeEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}