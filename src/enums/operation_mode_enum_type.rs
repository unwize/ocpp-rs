use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// (2.1) Operation mode for (bi-)directional charging during a charging schedule period.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum OperationModeEnumType {
    /// Minimize energy consumption by having the EV either on standby or in sleep.
    Idle,
    /// Classic charging or smart charging mode. (default)
    ChargingOnly,
    /// Control of setpoint by CSMS or some secondary actor that relays through the CSMS.
    CentralSetpoint,
    /// Control of setpoint by an external actor directly on the Charging Station.
    ExternalSetpoint,
    /// Control of (dis)charging limits by an external actor on the Charging Station.
    ExternalLimits,
    /// Frequency support with control by CSMS or some secondary actor that relays through the CSMS.
    CentralFrequency,
    /// Frequency support with control in the Charging Station.
    LocalFrequency,
    /// Load-balancing performed by the Charging Station.
    LocalLoadBalancing,
}

impl fmt::Display for OperationModeEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::ChargingOnly => write!(f, "ChargingOnly"),
            Self::CentralSetpoint => write!(f, "CentralSetpoint"),
            Self::ExternalSetpoint => write!(f, "ExternalSetpoint"),
            Self::ExternalLimits => write!(f, "ExternalLimits"),
            Self::CentralFrequency => write!(f, "CentralFrequency"),
            Self::LocalFrequency => write!(f, "LocalFrequency"),
            Self::LocalLoadBalancing => write!(f, "LocalLoadBalancing"),
        }
    }
}

impl Into<String> for OperationModeEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for OperationModeEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Idle" => Ok(Self::Idle),
            "ChargingOnly" => Ok(Self::ChargingOnly),
            "CentralSetpoint" => Ok(Self::CentralSetpoint),
            "ExternalSetpoint" => Ok(Self::ExternalSetpoint),
            "ExternalLimits" => Ok(Self::ExternalLimits),
            "CentralFrequency" => Ok(Self::CentralFrequency),
            "LocalFrequency" => Ok(Self::LocalFrequency),
            "LocalLoadBalancing" => Ok(Self::LocalLoadBalancing),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "OperationModeEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
