use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MonitorEnumType {
    /// Triggers an event notice when the actual value of the Variable rises above value.
    UpperThreshold,
    /// Triggers an event notice when the actual value of the Variable drops below value.
    LowerThreshold,
    /// Triggers an event notice when the actual value has changed more than plus or minus value since the time that this monitor was set or since the last time this event notice was sent, whichever was last. For variables that are not numeric, like boolean, string or enumerations, a monitor of type Delta will trigger an event notice whenever the variable changes, regardless of the value of value.
    Delta,
    /// Triggers an event notice every value seconds interval, starting from the time that this monitor was set.
    Periodic,
    /// Triggers an event notice every value seconds interval, starting from the nearest clock-aligned interval after this monitor was set. For example, a value of 900 will trigger event notices at 0, 15, 30 and 45 minutes after the hour, every hour.
    PeriodicClockAligned,
    /// (2.1) Triggers an event notice when the actual value differs from the target value more than plus or minus value since the time that this monitor was set or since the last time this event notice was sent, whichever was last. Behavior of this type of monitor for a variable that is not numeric, is not defined.
    TargetDelta,
    /// (2.1) Triggers an event notice when the actual value differs from the target value more than plus or minus (value * target value) since the time that this monitor was set or since the last time this event notice was sent, whichever was last. Behavior of this type of monitor for a variable that is not numeric, is not defined.
    TargetDeltaRelative,
}

impl fmt::Display for MonitorEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UpperThreshold => write!(f, "UpperThreshold"),
            Self::LowerThreshold => write!(f, "LowerThreshold"),
            Self::Delta => write!(f, "Delta"),
            Self::Periodic => write!(f, "Periodic"),
            Self::PeriodicClockAligned => write!(f, "PeriodicClockAligned"),
            Self::TargetDelta => write!(f, "TargetDelta"),
            Self::TargetDeltaRelative => write!(f, "TargetDeltaRelative"),
        }
    }
}

impl Into<String> for MonitorEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for MonitorEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "UpperThreshold" => Ok(Self::UpperThreshold),
            "LowerThreshold" => Ok(Self::LowerThreshold),
            "Delta" => Ok(Self::Delta),
            "Periodic" => Ok(Self::Periodic),
            "PeriodicClockAligned" => Ok(Self::PeriodicClockAligned),
            "TargetDelta" => Ok(Self::TargetDelta),
            "TargetDeltaRelative" => Ok(Self::TargetDeltaRelative),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MonitorEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}