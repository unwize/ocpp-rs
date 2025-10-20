use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Enumeration.
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ReportBaseEnumType {
    /// Required. A (configuration) report that lists all Components/Variables that can be set by the operator.
    #[default]
    ConfigurationInventory,
    /// Required. A (full) report that lists everything except monitoring settings.
    FullInventory,
    /// A (summary) report that lists Components/Variables relating to the Charging Station's current charging availability, and to any existing problem conditions.
    ///
    /// For the Charging Station Component:
    /// - AvailabilityState.
    /// For each EVSE Component:
    /// - AvailabilityState.
    /// For each Connector Component:
    /// - AvailabilityState (if known and different from EVSE).
    /// For all Components in an abnormal State:
    /// - Active (Problem, Tripped, Overload, Fallback) variables.
    /// - Any other diagnostically relevant Variables of the Components.
    /// - Include TechCode and TechInfo where available.
    /// All monitored Component.Variables in Critical or Alert state shall also be included.
    /// - Charging Stations that do not have Monitoring implemented are NOT REQUIRED to include Connector
    /// Availability, monitoring alerts, and MAY limit problem reporting detail to just the active Problem boolean
    /// Variable.
    SummaryInventory,
}

impl fmt::Display for ReportBaseEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConfigurationInventory => write!(f, "ConfigurationInventory"),
            Self::FullInventory => write!(f, "FullInventory"),
            Self::SummaryInventory => write!(f, "SummaryInventory"),
        }
    }
}

impl Into<String> for ReportBaseEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for ReportBaseEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ConfigurationInventory" => Ok(Self::ConfigurationInventory),
            "FullInventory" => Ok(Self::FullInventory),
            "SummaryInventory" => Ok(Self::SummaryInventory),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "ReportBaseEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
