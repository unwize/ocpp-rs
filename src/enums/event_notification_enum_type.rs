use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum EventNotificationEnumType {
    /// The software implemented by the manufacturer triggered a hardwired notification.
    HardWiredNotification,
    /// Triggered by a monitor, which is hardwired by the manufacturer.
    HardWiredMonitor,
    /// Triggered by a monitor, which is preconfigured by the manufacturer.
    PreconfiguredMonitor,
    /// Triggered by a monitor, which is set with the setvariablemonitoringrequest message by the Charging Station Operator.
    CustomMonitor,
}

impl TryFrom<String> for EventNotificationEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "HardWiredNotification" => Ok(EventNotificationEnumType::HardWiredNotification),
            "HardWiredMonitor" => Ok(EventNotificationEnumType::HardWiredMonitor),
            "PreconfiguredMonitor" => Ok(EventNotificationEnumType::PreconfiguredMonitor),
            "CustomMonitor" => Ok(EventNotificationEnumType::CustomMonitor),
            _ => Err(format!("'{}' is not a valid EventNotificationEnumType", s)),
        }
    }
}

impl Into<String> for EventNotificationEnumType {
    fn into(self) -> String {
        match self {
            EventNotificationEnumType::HardWiredNotification => "HardWiredNotification".to_string(),
            EventNotificationEnumType::HardWiredMonitor => "HardWiredMonitor".to_string(),
            EventNotificationEnumType::PreconfiguredMonitor => "PreconfiguredMonitor".to_string(),
            EventNotificationEnumType::CustomMonitor => "CustomMonitor".to_string(),
        }
    }
}
