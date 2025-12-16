use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ChargingStateEnumType {
    /// There is a connection between EV and EVSE, in case the protocol used between EV and the Charging Station can detect a connection, the protocol needs to detect this for the state to become active. The connection can either be wired or wireless. Authorization is required to proceed to state Charging.
    EVConnected,
    /// The contactor of the Connector is closed and energy is flowing to between EVSE and EV.
    Charging,
    /// When the EV is connected to the EVSE and the EVSE is offering energy but the EV is not taking any energy.
    SuspendedEV,
    /// When the EV is connected to the EVSE but the EVSE is not offering energy to the EV, e.g. due to a smart charging restrictions or local supply power constraints.
    SuspendedEVSE,
    /// There is no connection between EV and EVSE.
    Idle,
}

impl TryFrom<String> for ChargingStateEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "EVConnected" => Ok(ChargingStateEnumType::EVConnected),
            "Charging" => Ok(ChargingStateEnumType::Charging),
            "SuspendedEV" => Ok(ChargingStateEnumType::SuspendedEV),
            "SuspendedEVSE" => Ok(ChargingStateEnumType::SuspendedEVSE),
            "Idle" => Ok(ChargingStateEnumType::Idle),
            _ => Err(format!("'{}' is not a valid ChargingStateEnumType", s)),
        }
    }
}

impl From<ChargingStateEnumType> for String {
    fn from(val: ChargingStateEnumType) -> Self {
        match val {
            ChargingStateEnumType::EVConnected => "EVConnected".to_string(),
            ChargingStateEnumType::Charging => "Charging".to_string(),
            ChargingStateEnumType::SuspendedEV => "SuspendedEV".to_string(),
            ChargingStateEnumType::SuspendedEVSE => "SuspendedEVSE".to_string(),
            ChargingStateEnumType::Idle => "Idle".to_string(),
        }
    }
}
