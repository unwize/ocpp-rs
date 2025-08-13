use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Status in ReserveNowResponse.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ReserveNowStatusEnumType {
    /// Reservation has been made.
    Accepted,
    /// Reservation has not been made, because evse, connectors or specified connector are in a faulted state.
    Faulted,
    /// Reservation has not been made. The evse or the specified connector is occupied.
    Occupied,
    /// Reservation has not been made. Charging Station is not configured to accept reservations.
    Rejected,
    /// Reservation has not been made, because evse, connectors or specified connector are in an unavailable state.
    Unavailable,
}

impl fmt::Display for ReserveNowStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Faulted => write!(f, "Faulted"),
            Self::Occupied => write!(f, "Occupied"),
            Self::Rejected => write!(f, "Rejected"),
            Self::Unavailable => write!(f, "Unavailable"),
        }
    }
}

impl Into<String> for ReserveNowStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for ReserveNowStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Faulted" => Ok(Self::Faulted),
            "Occupied" => Ok(Self::Occupied),
            "Rejected" => Ok(Self::Rejected),
            "Unavailable" => Ok(Self::Unavailable),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "ReserveNowStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
