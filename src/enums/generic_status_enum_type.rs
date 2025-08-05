use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GenericStatusEnumType {
    /// Request has been accepted and will be executed.
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
}

impl TryFrom<String> for GenericStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(GenericStatusEnumType::Accepted),
            "Rejected" => Ok(GenericStatusEnumType::Rejected),
            _ => Err(format!("'{}' is not a valid GenericStatusEnumType", s)),
        }
    }
}

impl Into<String> for GenericStatusEnumType {
    fn into(self) -> String {
        match self {
            GenericStatusEnumType::Accepted => "Accepted".to_string(),
            GenericStatusEnumType::Rejected => "Rejected".to_string(),
        }
    }
}
