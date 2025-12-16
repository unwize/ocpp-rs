use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum ChargingRateUnitEnumType {
    /// Watts (power). This is the TOTAL allowed charging power. If used for AC Charging, the phase current should be calculated via: Current per phase = Power / (Line Voltage * Number of Phases). The "Line Voltage" used in the calculation is not the measured voltage, but the set voltage for the area (hence, 230 of 110 volt). The "Number of Phases" is the numberPhases from the ChargingSchedulePeriod. It is usually more convenient to use this for DC charging. Note that if numberPhases in a ChargingSchedulePeriod is absent, 3 SHALL be assumed.
    W,
    /// Amperes (current). The amount of Ampere per phase, not the sum of all phases. It is usually more convenient to use this for AC charging.
    A,
}

impl TryFrom<String> for ChargingRateUnitEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "W" => Ok(ChargingRateUnitEnumType::W),
            "A" => Ok(ChargingRateUnitEnumType::A),
            _ => Err(format!("'{}' is not a valid ChargingRateUnitEnumType", s)),
        }
    }
}

impl From<ChargingRateUnitEnumType> for String {
    fn from(val: ChargingRateUnitEnumType) -> Self {
        match val {
            ChargingRateUnitEnumType::W => "W".to_string(),
            ChargingRateUnitEnumType::A => "A".to_string(),
        }
    }
}
