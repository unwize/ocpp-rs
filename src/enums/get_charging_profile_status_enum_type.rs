use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GetChargingProfileStatusEnumType {
    /// Normal successful completion (no errors).
    Accepted,
    /// No ChargingProfiles found that match the information in the GetChargingProfilesRequest.
    NoProfiles,
}

impl TryFrom<String> for GetChargingProfileStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(GetChargingProfileStatusEnumType::Accepted),
            "NoProfiles" => Ok(GetChargingProfileStatusEnumType::NoProfiles),
            _ => Err(format!("'{}' is not a valid GetChargingProfileStatusEnumType", s)),
        }
    }
}

impl Into<String> for GetChargingProfileStatusEnumType {
    fn into(self) -> String {
        match self {
            GetChargingProfileStatusEnumType::Accepted => "Accepted".to_string(),
            GetChargingProfileStatusEnumType::NoProfiles => "NoProfiles".to_string(),
        }
    }
}
