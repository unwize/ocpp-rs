use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LogEnumType {
    /// This contains the field definition of a diagnostics log file
    DiagnosticsLog,
    /// Sent by the CSMS to the Charging Station to request that the Charging Station uploads the security log.
    SecurityLog,
    /// The log of sampled measurements from the DataCollector component.
    DataCollectorLog,
}

impl TryFrom<String> for LogEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "DiagnosticsLog" => Ok(LogEnumType::DiagnosticsLog),
            "SecurityLog" => Ok(LogEnumType::SecurityLog),
            "DataCollectorLog" => Ok(LogEnumType::DataCollectorLog),
            _ => Err(format!("'{}' is not a valid LogEnumType", s)),
        }
    }
}

impl Into<String> for LogEnumType {
    fn into(self) -> String {
        match self {
            LogEnumType::DiagnosticsLog => "DiagnosticsLog".to_string(),
            LogEnumType::SecurityLog => "SecurityLog".to_string(),
            LogEnumType::DataCollectorLog => "DataCollectorLog".to_string(),
        }
    }
}
