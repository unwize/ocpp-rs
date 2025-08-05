use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ConnectorStatusEnumType {
    /// When a Connector becomes available for a new User (Operative).
    Available,
    /// When a Connector becomes occupied, so it is not available for a new EV driver. (Operative).
    Occupied,
    /// When a Connector becomes reserved as a result of ReserveNow command (Operative).
    Reserved,
    /// When a Connector becomes unavailable as the result of a Change Availability command or an event upon which the Charging Station transitions to unavailable at its discretion. Upon receipt of ChangeAvailability message command, the status MAY change immediately or the change MAY be scheduled. When scheduled, StatusNotification SHALL be send when the availability change becomes effective (Inoperative).
    Unavailable,
    /// When a Connector (or the EVSE or the entire Charging Station it belongs to) has reported an error and is not available for energy delivery. (Inoperative).
    Faulted,
}

impl TryFrom<String> for ConnectorStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Available" => Ok(ConnectorStatusEnumType::Available),
            "Occupied" => Ok(ConnectorStatusEnumType::Occupied),
            "Reserved" => Ok(ConnectorStatusEnumType::Reserved),
            "Unavailable" => Ok(ConnectorStatusEnumType::Unavailable),
            "Faulted" => Ok(ConnectorStatusEnumType::Faulted),
            _ => Err(format!("'{}' is not a valid ConnectorStatusEnumType", s)),
        }
    }
}

impl Into<String> for ConnectorStatusEnumType {
    fn into(self) -> String {
        match self {
            ConnectorStatusEnumType::Available => "Available".to_string(),
            ConnectorStatusEnumType::Occupied => "Occupied".to_string(),
            ConnectorStatusEnumType::Reserved => "Reserved".to_string(),
            ConnectorStatusEnumType::Unavailable => "Unavailable".to_string(),
            ConnectorStatusEnumType::Faulted => "Faulted".to_string(),
        }
    }
}
