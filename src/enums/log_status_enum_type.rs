use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum LogStatusEnumType {
    /// Accepted this log upload. This does not mean the log file is uploaded is successfully, the Charging Station will now start the log file upload.
    #[default]
    Accepted,
    /// Log update request rejected.
    Rejected,
    /// Accepted this log upload, but in doing this has canceled an ongoing log file upload.
    AcceptedCanceled,
}

impl TryFrom<String> for LogStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(LogStatusEnumType::Accepted),
            "Rejected" => Ok(LogStatusEnumType::Rejected),
            "AcceptedCanceled" => Ok(LogStatusEnumType::AcceptedCanceled),
            _ => Err(format!("'{}' is not a valid LogStatusEnumType", s)),
        }
    }
}

impl Into<String> for LogStatusEnumType {
    fn into(self) -> String {
        match self {
            LogStatusEnumType::Accepted => "Accepted".to_string(),
            LogStatusEnumType::Rejected => "Rejected".to_string(),
            LogStatusEnumType::AcceptedCanceled => "AcceptedCanceled".to_string(),
        }
    }
}
