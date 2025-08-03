use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ChangeAvailabilityStatusEnumType {
    /// Request has been accepted and will be executed.
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
    /// Request has been accepted and will be executed when transaction(s) in progress have finished.
    Scheduled,
}

impl TryFrom<String> for ChangeAvailabilityStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(ChangeAvailabilityStatusEnumType::Accepted),
            "Rejected" => Ok(ChangeAvailabilityStatusEnumType::Rejected),
            "Scheduled" => Ok(ChangeAvailabilityStatusEnumType::Scheduled),
            _ => Err(format!("'{}' is not a valid ChangeAvailabilityStatusEnumType", s)),
        }
    }
}

impl Into<String> for ChangeAvailabilityStatusEnumType {
    fn into(self) -> String {
        match self {
            ChangeAvailabilityStatusEnumType::Accepted => "Accepted".to_string(),
            ChangeAvailabilityStatusEnumType::Rejected => "Rejected".to_string(),
            ChangeAvailabilityStatusEnumType::Scheduled => "Scheduled".to_string(),
        }
    }
}
