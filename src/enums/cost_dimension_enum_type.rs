use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum CostDimensionEnumType {
    /// Total amount of energy (dis-)charged during this charging period, defined in Wh (kiloWatt-hours). When negative, more energy was feed into the grid then charged into the EV.
    Energy,
    /// Sum of the maximum current over all phases, reached during this charging period, defined in A (Ampere).
    MaxCurrent,
    /// Sum of the minimum current over all phases, reached during this charging period, when negative, current has flowed from the EV to the grid. Defined in A (Ampere).
    MinCurrent,
    /// Maximum power reached during this charging period: defined in W (Watt).
    MaxPower,
    /// Minimum power reached during this charging period: defined in W (Watt), when negative, the power has flowed from the EV to the grid.
    MinPower,
    /// Time not charging during this charging period: defined in seconds.
    IdleTime,
    /// Time charging during this charging period: defined in seconds.
    ChargingTime,
}

impl TryFrom<String> for CostDimensionEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Energy" => Ok(CostDimensionEnumType::Energy),
            "MaxCurrent" => Ok(CostDimensionEnumType::MaxCurrent),
            "MinCurrent" => Ok(CostDimensionEnumType::MinCurrent),
            "MaxPower" => Ok(CostDimensionEnumType::MaxPower),
            "MinPower" => Ok(CostDimensionEnumType::MinPower),
            "IdleTime" => Ok(CostDimensionEnumType::IdleTime),
            "ChargingTime" => Ok(CostDimensionEnumType::ChargingTime),
            _ => Err(format!("'{}' is not a valid CostDimensionEnumType", s)),
        }
    }
}

impl Into<String> for CostDimensionEnumType {
    fn into(self) -> String {
        match self {
            CostDimensionEnumType::Energy => "Energy".to_string(),
            CostDimensionEnumType::MaxCurrent => "MaxCurrent".to_string(),
            CostDimensionEnumType::MinCurrent => "MinCurrent".to_string(),
            CostDimensionEnumType::MaxPower => "MaxPower".to_string(),
            CostDimensionEnumType::MinPower => "MinPower".to_string(),
            CostDimensionEnumType::IdleTime => "IdleTime".to_string(),
            CostDimensionEnumType::ChargingTime => "ChargingTime".to_string(),
        }
    }
}
