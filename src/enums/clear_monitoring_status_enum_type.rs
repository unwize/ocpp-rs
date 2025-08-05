use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ClearMonitoringStatusEnumType {
    /// Monitor successfully cleared.
    Accepted,
    /// Clearing of monitor rejected.
    Rejected,
    /// Monitor Id is not found.
    NotFound,
}

impl TryFrom<String> for ClearMonitoringStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(ClearMonitoringStatusEnumType::Accepted),
            "Rejected" => Ok(ClearMonitoringStatusEnumType::Rejected),
            "NotFound" => Ok(ClearMonitoringStatusEnumType::NotFound),
            _ => Err(format!("'{}' is not a valid ClearMonitoringStatusEnumType", s)),
        }
    }
}

impl Into<String> for ClearMonitoringStatusEnumType {
    fn into(self) -> String {
        match self {
            ClearMonitoringStatusEnumType::Accepted => "Accepted".to_string(),
            ClearMonitoringStatusEnumType::Rejected => "Rejected".to_string(),
            ClearMonitoringStatusEnumType::NotFound => "NotFound".to_string(),
        }
    }
}
