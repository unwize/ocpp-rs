use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ClearMessageStatusEnumType {
    /// Request successfully executed: message cleared.
    Accepted,
    /// Given message (based on the id) not known.
    Unknown,
    /// Request could not be executed.
    Rejected,
}

impl TryFrom<String> for ClearMessageStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(ClearMessageStatusEnumType::Accepted),
            "Unknown" => Ok(ClearMessageStatusEnumType::Unknown),
            "Rejected" => Ok(ClearMessageStatusEnumType::Rejected),
            _ => Err(format!("'{}' is not a valid ClearMessageStatusEnumType", s)),
        }
    }
}

impl Into<String> for ClearMessageStatusEnumType {
    fn into(self) -> String {
        match self {
            ClearMessageStatusEnumType::Accepted => "Accepted".to_string(),
            ClearMessageStatusEnumType::Unknown => "Unknown".to_string(),
            ClearMessageStatusEnumType::Rejected => "Rejected".to_string(),
        }
    }
}
