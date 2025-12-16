use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum LogEnumType {
    /// This contains the field definition of a diagnostics log file
    #[default]
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

impl From<LogEnumType> for String {
    fn from(val: LogEnumType) -> Self {
        match val {
            LogEnumType::DiagnosticsLog => "DiagnosticsLog".to_string(),
            LogEnumType::SecurityLog => "SecurityLog".to_string(),
            LogEnumType::DataCollectorLog => "DataCollectorLog".to_string(),
        }
    }
}
