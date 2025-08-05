use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum LocationEnumType {
    /// Measurement inside body of Charging Station (e.g. Temperature).
    Body,
    /// Measurement taken from cable between EV and Charging Station.
    Cable,
    /// Measurement taken by EV.
    EV,
    /// For the Charging Station (evseld = 0): measurement at network ("grid") inlet connection of the station. For measurements with evseld > 0, these are measurements taken at the EVSE inlet (This can be useful for a DC charger).
    Inlet,
    /// Measurement at a Connector. Default value.
    Outlet,
    /// Measurement taken from an upstream local grid meter of the premise. This can be useful for charging stations that are connected "behind the meter" of a building, and that are able to read the building energy meter.
    Upstream,
}

impl TryFrom<String> for LocationEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Body" => Ok(LocationEnumType::Body),
            "Cable" => Ok(LocationEnumType::Cable),
            "EV" => Ok(LocationEnumType::EV),
            "Inlet" => Ok(LocationEnumType::Inlet),
            "Outlet" => Ok(LocationEnumType::Outlet),
            "Upstream" => Ok(LocationEnumType::Upstream),
            _ => Err(format!("'{}' is not a valid LocationEnumType", s)),
        }
    }
}

impl Into<String> for LocationEnumType {
    fn into(self) -> String {
        match self {
            LocationEnumType::Body => "Body".to_string(),
            LocationEnumType::Cable => "Cable".to_string(),
            LocationEnumType::EV => "EV".to_string(),
            LocationEnumType::Inlet => "Inlet".to_string(),
            LocationEnumType::Outlet => "Outlet".to_string(),
            LocationEnumType::Upstream => "Upstream".to_string(),
        }
    }
}
