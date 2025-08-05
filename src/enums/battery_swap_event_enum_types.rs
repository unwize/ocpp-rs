use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum BatterySwapEventEnumType {
    /// Battery (or set of batteries) is inserted.
    BatteryIn,
    /// Battery (or set of batteries) is removed.
    BatteryOut,
    /// The offered batteries have not been removed within timeout.
    BatteryOutTimeout,
}

impl TryFrom<String> for BatterySwapEventEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "BatteryIn" => Ok(BatterySwapEventEnumType::BatteryIn),
            "BatteryOut" => Ok(BatterySwapEventEnumType::BatteryOut),
            "BatteryOutTimeout" => Ok(BatterySwapEventEnumType::BatteryOutTimeout),
            _ => Err(format!("'{}' is not a valid BatterySwapEventEnumType", s)),
        }
    }
}

impl Into<String> for BatterySwapEventEnumType {
    fn into(self) -> String {
        match self {
            BatterySwapEventEnumType::BatteryIn => "BatteryIn".to_string(),
            BatterySwapEventEnumType::BatteryOut => "BatteryOut".to_string(),
            BatterySwapEventEnumType::BatteryOutTimeout => "BatteryOutTimeout".to_string(),
        }
    }
}
