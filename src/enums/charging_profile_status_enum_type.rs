use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ChargingProfileStatusEnumType {
    /// Request has been accepted and will be executed.
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
}

impl TryFrom<String> for ChargingProfileStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(ChargingProfileStatusEnumType::Accepted),
            "Rejected" => Ok(ChargingProfileStatusEnumType::Rejected),
            _ => Err(format!(
                "'{}' is not a valid ChargingProfileStatusEnumType",
                s
            )),
        }
    }
}

impl From<ChargingProfileStatusEnumType> for String {
    fn from(val: ChargingProfileStatusEnumType) -> Self {
        match val {
            ChargingProfileStatusEnumType::Accepted => "Accepted".to_string(),
            ChargingProfileStatusEnumType::Rejected => "Rejected".to_string(),
        }
    }
}
