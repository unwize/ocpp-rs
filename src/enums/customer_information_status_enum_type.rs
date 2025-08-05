use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CustomerInformationStatusEnumType {
    /// The Charging Station accepted the message.
    Accepted,
    /// When the Charging Station is in a state where it cannot process this request.
    Rejected,
    /// In a request to the Charging Station no reference to a customer is included.
    Invalid,
}

impl TryFrom<String> for CustomerInformationStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(CustomerInformationStatusEnumType::Accepted),
            "Rejected" => Ok(CustomerInformationStatusEnumType::Rejected),
            "Invalid" => Ok(CustomerInformationStatusEnumType::Invalid),
            _ => Err(format!("'{}' is not a valid CustomerInformationStatusEnumType", s)),
        }
    }
}

impl Into<String> for CustomerInformationStatusEnumType {
    fn into(self) -> String {
        match self {
            CustomerInformationStatusEnumType::Accepted => "Accepted".to_string(),
            CustomerInformationStatusEnumType::Rejected => "Rejected".to_string(),
            CustomerInformationStatusEnumType::Invalid => "Invalid".to_string(),
        }
    }
}
