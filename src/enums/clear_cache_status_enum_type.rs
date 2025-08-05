use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ClearCacheStatusEnumType {
    /// Command has been executed.
    Accepted,
    /// Command has not been executed.
    Rejected,
}

impl TryFrom<String> for ClearCacheStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(ClearCacheStatusEnumType::Accepted),
            "Rejected" => Ok(ClearCacheStatusEnumType::Rejected),
            _ => Err(format!("'{}' is not a valid ClearCacheStatusEnumType", s)),
        }
    }
}

impl Into<String> for ClearCacheStatusEnumType {
    fn into(self) -> String {
        match self {
            ClearCacheStatusEnumType::Accepted => "Accepted".to_string(),
            ClearCacheStatusEnumType::Rejected => "Rejected".to_string(),
        }
    }
}
