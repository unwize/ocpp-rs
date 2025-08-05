use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum DERUnitEnumType {
    /// No unit applicable (e.g. for ride-through curves)
    Not_Applicable,
    /// Percentage of configured active power
    PctMaxW,
    /// Percentage of configured reactive power
    PctMaxVar,
    /// Percentage of available reserve active power
    PctWAvail,
    /// Percentage of available reserve reactive power
    PctVarAvail,
    /// Percentage of effective voltage
    PctEffectiveV,
}

impl TryFrom<String> for DERUnitEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Not_Applicable" => Ok(DERUnitEnumType::Not_Applicable),
            "PctMaxW" => Ok(DERUnitEnumType::PctMaxW),
            "PctMaxVar" => Ok(DERUnitEnumType::PctMaxVar),
            "PctWAvail" => Ok(DERUnitEnumType::PctWAvail),
            "PctVarAvail" => Ok(DERUnitEnumType::PctVarAvail),
            "PctEffectiveV" => Ok(DERUnitEnumType::PctEffectiveV),
            _ => Err(format!("'{}' is not a valid DERUnitEnumType", s)),
        }
    }
}

impl Into<String> for DERUnitEnumType {
    fn into(self) -> String {
        match self {
            DERUnitEnumType::Not_Applicable => "Not_Applicable".to_string(),
            DERUnitEnumType::PctMaxW => "PctMaxW".to_string(),
            DERUnitEnumType::PctMaxVar => "PctMaxVar".to_string(),
            DERUnitEnumType::PctWAvail => "PctWAvail".to_string(),
            DERUnitEnumType::PctVarAvail => "PctVarAvail".to_string(),
            DERUnitEnumType::PctEffectiveV => "PctEffectiveV".to_string(),
        }
    }
}
