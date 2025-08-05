use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ControlModeEnumType {
    /// Scheduled control mode, EVSE provides up to three schedules for EV to choose from. EV follows the selected schedule.
    ScheduledControl,
    /// Dynamic control mode, EVSE executes a single schedule by sending setpoints to EV at every interval.
    DynamicControl,
}

impl TryFrom<String> for ControlModeEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "ScheduledControl" => Ok(ControlModeEnumType::ScheduledControl),
            "DynamicControl" => Ok(ControlModeEnumType::DynamicControl),
            _ => Err(format!("'{}' is not a valid ControlModeEnumType", s)),
        }
    }
}

impl Into<String> for ControlModeEnumType {
    fn into(self) -> String {
        match self {
            ControlModeEnumType::ScheduledControl => "ScheduledControl".to_string(),
            ControlModeEnumType::DynamicControl => "DynamicControl".to_string(),
        }
    }
}
