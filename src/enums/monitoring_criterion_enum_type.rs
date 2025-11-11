use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum MonitoringCriterionEnumType {
    /// Report variables and components with a monitor of type UpperThreshold or LowerThreshold.
    #[default]
    ThresholdMonitoring,
    /// Report variables and components with a monitor of type Delta.
    DeltaMonitoring,
    /// Report variables and components with a monitor of type Periodic or PeriodicClockAligned.
    PeriodicMonitoring,
}

impl fmt::Display for MonitoringCriterionEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ThresholdMonitoring => write!(f, "ThresholdMonitoring"),
            Self::DeltaMonitoring => write!(f, "DeltaMonitoring"),
            Self::PeriodicMonitoring => write!(f, "PeriodicMonitoring"),
        }
    }
}

impl Into<String> for MonitoringCriterionEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for MonitoringCriterionEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ThresholdMonitoring" => Ok(Self::ThresholdMonitoring),
            "DeltaMonitoring" => Ok(Self::DeltaMonitoring),
            "PeriodicMonitoring" => Ok(Self::PeriodicMonitoring),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MonitoringCriterionEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
