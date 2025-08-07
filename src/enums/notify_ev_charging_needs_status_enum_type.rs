use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Status result of a NotifyAllowedEnergyTransferRequest
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum NotifyEVChargingNeedsStatusEnumType {
    /// A schedule will be provided momentarily.
    Accepted,
    /// (2.1) Service not available. No charging profile can be provided. For an ISO 15118-20 session this is used to convey that the requested energy transfer type is not possible.
    Rejected,
    /// The CSMS is gathering information to provide a schedule.
    Processing,
    /// (2.1) CSMS will not provide a charging profile at this time. CS should not wait for it. For an ISO 15118-20 session this value is used instead of Rejected to differentiate between the situation where no charging profile is available (NoChargingProfile) and requested energy transfer type is not available (Rejected).
    NoChargingProfile,
}

impl fmt::Display for NotifyEVChargingNeedsStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::Processing => write!(f, "Processing"),
            Self::NoChargingProfile => write!(f, "NoChargingProfile"),
        }
    }
}

impl Into<String> for NotifyEVChargingNeedsStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for NotifyEVChargingNeedsStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "Processing" => Ok(Self::Processing),
            "NoChargingProfile" => Ok(Self::NoChargingProfile),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "NotifyEVChargingNeedsStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}