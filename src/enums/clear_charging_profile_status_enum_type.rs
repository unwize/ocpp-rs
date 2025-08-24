use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum ClearChargingProfileStatusEnumType {
    /// Request has been accepted.
    #[default]
    Accepted,
    /// No Charging Profile(s) were found matching the request.
    Unknown,
}

impl TryFrom<String> for ClearChargingProfileStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(ClearChargingProfileStatusEnumType::Accepted),
            "Unknown" => Ok(ClearChargingProfileStatusEnumType::Unknown),
            _ => Err(format!(
                "'{}' is not a valid ClearChargingProfileStatusEnumType",
                s
            )),
        }
    }
}

impl Into<String> for ClearChargingProfileStatusEnumType {
    fn into(self) -> String {
        match self {
            ClearChargingProfileStatusEnumType::Accepted => "Accepted".to_string(),
            ClearChargingProfileStatusEnumType::Unknown => "Unknown".to_string(),
        }
    }
}
