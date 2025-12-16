use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Possible values of SetNetworkProfileStatus as used in SetNetworkProfileResponse.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SetNetworkProfileStatusEnumType {
    /// Setting new data successful
    Accepted,
    /// Setting new data rejected
    Rejected,
    /// Setting new data failed
    Failed,
}

impl fmt::Display for SetNetworkProfileStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::Failed => write!(f, "Failed"),
        }
    }
}

impl From<SetNetworkProfileStatusEnumType> for String {
    fn from(val: SetNetworkProfileStatusEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for SetNetworkProfileStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "Failed" => Ok(Self::Failed),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "SetNetworkProfileStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
