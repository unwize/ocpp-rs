use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ChargingProfilePurposeEnumType {
    /// Additional constraints from an external source (e.g. an EMS) that will be incorporated into a local power schedule. When applied to evse.Id = 0 it sets a limit to the entire Charging Station. Note: In OCPP 2.0.1 this purpose was only allowed on evse.Id = 0. In OCPP 2.1 it can be set to an individual EVSE.
    ChargingStationExternalConstraints,
    /// Configuration for the maximum power or current available for an entire Charging Station.
    ChargingStationMaxProfile,
    /// Default profile that can be configured in the Charging Station. When a new transaction is started, this profile SHALL be used, unless it was a transaction that was started by a RequestStartTransactionRequest with a ChargingProfile that is accepted by the Charging Station.
    TxDefaultProfile,
    /// Profile with constraints to be imposed by the Charging Station on the current transaction, or on a new transaction when this is started via a RequestStartTransactionRequest with a ChargingProfile. A profile with this purpose SHALL cease to be valid when the transaction terminates.
    TxProfile,
    /// This profile is used in place of a Tx(Default)Profile, when priority charging is requested, either locally on Charging Station or via a request from CSMS.
    PriorityCharging,
    /// This profile adds capacity from local generation. Its capacity is added on top of other charging profiles.
    LocalGeneration,
}

impl TryFrom<String> for ChargingProfilePurposeEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "ChargingStationExternalConstraints" => Ok(ChargingProfilePurposeEnumType::ChargingStationExternalConstraints),
            "ChargingStationMaxProfile" => Ok(ChargingProfilePurposeEnumType::ChargingStationMaxProfile),
            "TxDefaultProfile" => Ok(ChargingProfilePurposeEnumType::TxDefaultProfile),
            "TxProfile" => Ok(ChargingProfilePurposeEnumType::TxProfile),
            "PriorityCharging" => Ok(ChargingProfilePurposeEnumType::PriorityCharging),
            "LocalGeneration" => Ok(ChargingProfilePurposeEnumType::LocalGeneration),
            _ => Err(format!("'{}' is not a valid ChargingProfilePurposeEnumType", s)),
        }
    }
}

impl Into<String> for ChargingProfilePurposeEnumType {
    fn into(self) -> String {
        match self {
            ChargingProfilePurposeEnumType::ChargingStationExternalConstraints => "ChargingStationExternalConstraints".to_string(),
            ChargingProfilePurposeEnumType::ChargingStationMaxProfile => "ChargingStationMaxProfile".to_string(),
            ChargingProfilePurposeEnumType::TxDefaultProfile => "TxDefaultProfile".to_string(),
            ChargingProfilePurposeEnumType::TxProfile => "TxProfile".to_string(),
            ChargingProfilePurposeEnumType::PriorityCharging => "PriorityCharging".to_string(),
            ChargingProfilePurposeEnumType::LocalGeneration => "LocalGeneration".to_string(),
        }
    }
}
