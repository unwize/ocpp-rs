use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ChargingProfileKindEnumType {
    /// Schedule periods are relative to a fixed point in time defined in the schedule. This requires that `startSchedule` is set to a starting point in time.
    Absolute,
    /// The schedule restarts periodically at the first schedule period. To be most useful, this requires that `startSchedule` is set to a starting point in time.
    Recurring,
    /// Charging schedule periods start when the EVSE is ready to deliver energy. i.e. when the EV driver is authorized and the EV is connected. When a ChargingProfile is received for a transaction that is already charging, then the charging schedule periods remain relative to the PowerPathClosed moment. No value for `startSchedule` must be supplied.
    Relative,
    /// The schedule consists of only one charging schedule period, which is updated dynamically by CSMS.
    Dynamic,
}

impl TryFrom<String> for ChargingProfileKindEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Absolute" => Ok(ChargingProfileKindEnumType::Absolute),
            "Recurring" => Ok(ChargingProfileKindEnumType::Recurring),
            "Relative" => Ok(ChargingProfileKindEnumType::Relative),
            "Dynamic" => Ok(ChargingProfileKindEnumType::Dynamic),
            _ => Err(format!(
                "'{}' is not a valid ChargingProfileKindEnumType",
                s
            )),
        }
    }
}

impl From<ChargingProfileKindEnumType> for String {
    fn from(val: ChargingProfileKindEnumType) -> Self {
        match val {
            ChargingProfileKindEnumType::Absolute => "Absolute".to_string(),
            ChargingProfileKindEnumType::Recurring => "Recurring".to_string(),
            ChargingProfileKindEnumType::Relative => "Relative".to_string(),
            ChargingProfileKindEnumType::Dynamic => "Dynamic".to_string(),
        }
    }
}
