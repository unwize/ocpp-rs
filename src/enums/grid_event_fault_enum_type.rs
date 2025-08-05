use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GridEventFaultEnumType {
    /// Current imbalance detected
    CurrentImbalance,
    /// A local emergency detected
    LocalEmergency,
    /// Low input power detected
    LowInputPower,
    /// Overcurrent detected
    OverCurrent,
    /// Over frequency detected
    OverFrequency,
    /// Over voltage detected
    OverVoltage,
    /// Phase rotation detected
    PhaseRotation,
    /// A remote emergency detected
    RemoteEmergency,
    /// Under frequency detected
    UnderFrequency,
    /// Under voltage detected
    UnderVoltage,
    /// Voltage imbalance detected
    VoltageImbalance,
}

impl TryFrom<String> for GridEventFaultEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "CurrentImbalance" => Ok(GridEventFaultEnumType::CurrentImbalance),
            "LocalEmergency" => Ok(GridEventFaultEnumType::LocalEmergency),
            "LowInputPower" => Ok(GridEventFaultEnumType::LowInputPower),
            "OverCurrent" => Ok(GridEventFaultEnumType::OverCurrent),
            "OverFrequency" => Ok(GridEventFaultEnumType::OverFrequency),
            "OverVoltage" => Ok(GridEventFaultEnumType::OverVoltage),
            "PhaseRotation" => Ok(GridEventFaultEnumType::PhaseRotation),
            "RemoteEmergency" => Ok(GridEventFaultEnumType::RemoteEmergency),
            "UnderFrequency" => Ok(GridEventFaultEnumType::UnderFrequency),
            "UnderVoltage" => Ok(GridEventFaultEnumType::UnderVoltage),
            "VoltageImbalance" => Ok(GridEventFaultEnumType::VoltageImbalance),
            _ => Err(format!("'{}' is not a valid GridEventFaultEnumType", s)),
        }
    }
}

impl Into<String> for GridEventFaultEnumType {
    fn into(self) -> String {
        match self {
            GridEventFaultEnumType::CurrentImbalance => "CurrentImbalance".to_string(),
            GridEventFaultEnumType::LocalEmergency => "LocalEmergency".to_string(),
            GridEventFaultEnumType::LowInputPower => "LowInputPower".to_string(),
            GridEventFaultEnumType::OverCurrent => "OverCurrent".to_string(),
            GridEventFaultEnumType::OverFrequency => "OverFrequency".to_string(),
            GridEventFaultEnumType::OverVoltage => "OverVoltage".to_string(),
            GridEventFaultEnumType::PhaseRotation => "PhaseRotation".to_string(),
            GridEventFaultEnumType::RemoteEmergency => "RemoteEmergency".to_string(),
            GridEventFaultEnumType::UnderFrequency => "UnderFrequency".to_string(),
            GridEventFaultEnumType::UnderVoltage => "UnderVoltage".to_string(),
            GridEventFaultEnumType::VoltageImbalance => "VoltageImbalance".to_string(),
        }
    }
}
