use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum EventTriggerEnumType {
    /// Monitored variable has passed a Lower or Upper Threshold. Also used as trigger type for a HardwiredNotification.
    Alerting,
    /// Delta Monitored Variable value has changed by more than specified amount
    Delta,
    /// Periodic Monitored Variable has been sampled for reporting at the specified interval
    Periodic,
}

impl TryFrom<String> for EventTriggerEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Alerting" => Ok(EventTriggerEnumType::Alerting),
            "Delta" => Ok(EventTriggerEnumType::Delta),
            "Periodic" => Ok(EventTriggerEnumType::Periodic),
            _ => Err(format!("'{}' is not a valid EventTriggerEnumType", s)),
        }
    }
}

impl From<EventTriggerEnumType> for String {
    fn from(val: EventTriggerEnumType) -> Self {
        match val {
            EventTriggerEnumType::Alerting => "Alerting".to_string(),
            EventTriggerEnumType::Delta => "Delta".to_string(),
            EventTriggerEnumType::Periodic => "Periodic".to_string(),
        }
    }
}
