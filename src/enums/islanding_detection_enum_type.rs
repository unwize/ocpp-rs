use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum IslandingDetectionEnumType {
    /// No anti-island detection supported
    NoAntiIslandingSupport,
    /// RoCoF - Rate of Change of Frequency
    RoCoF,
    /// Under/over voltage (UVP/OVP)
    UVP_OVP,
    /// Under/over frequency (UFP/OFP)
    UFP_OFP,
    /// Voltage Vector Shift
    VoltageVectorShift,
    /// Zero Crossing Detection
    ZeroCrossingDetection,
    /// Other passive anti-island detection method supported
    OtherPassive,
    /// Impedance measurement
    ImpedanceMeasurement,
    /// Impedance detection at a specific frequency
    ImpedanceAtFrequency,
    /// Slip-mode frequency shift
    SlipModeFrequencyShift,
    /// Frequency bias/Sandia frequency shift
    SandiaFrequencyShift,
    /// Sandia voltage shift
    SandiaVoltageShift,
    /// Frequency jump
    FrequencyJump,
    /// RCL Q factor
    RCLQFactor,
    /// Other active anti-island detection method supported
    OtherActive,
}

impl TryFrom<String> for IslandingDetectionEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "NoAntiIslandingSupport" => Ok(IslandingDetectionEnumType::NoAntiIslandingSupport),
            "RoCoF" => Ok(IslandingDetectionEnumType::RoCoF),
            "UVP_OVP" => Ok(IslandingDetectionEnumType::UVP_OVP),
            "UFP_OFP" => Ok(IslandingDetectionEnumType::UFP_OFP),
            "VoltageVectorShift" => Ok(IslandingDetectionEnumType::VoltageVectorShift),
            "ZeroCrossingDetection" => Ok(IslandingDetectionEnumType::ZeroCrossingDetection),
            "OtherPassive" => Ok(IslandingDetectionEnumType::OtherPassive),
            "ImpedanceMeasurement" => Ok(IslandingDetectionEnumType::ImpedanceMeasurement),
            "ImpedanceAtFrequency" => Ok(IslandingDetectionEnumType::ImpedanceAtFrequency),
            "SlipModeFrequencyShift" => Ok(IslandingDetectionEnumType::SlipModeFrequencyShift),
            "SandiaFrequencyShift" => Ok(IslandingDetectionEnumType::SandiaFrequencyShift),
            "SandiaVoltageShift" => Ok(IslandingDetectionEnumType::SandiaVoltageShift),
            "FrequencyJump" => Ok(IslandingDetectionEnumType::FrequencyJump),
            "RCLQFactor" => Ok(IslandingDetectionEnumType::RCLQFactor),
            "OtherActive" => Ok(IslandingDetectionEnumType::OtherActive),
            _ => Err(format!("'{}' is not a valid IslandingDetectionEnumType", s)),
        }
    }
}

impl Into<String> for IslandingDetectionEnumType {
    fn into(self) -> String {
        match self {
            IslandingDetectionEnumType::NoAntiIslandingSupport => "NoAntiIslandingSupport".to_string(),
            IslandingDetectionEnumType::RoCoF => "RoCoF".to_string(),
            IslandingDetectionEnumType::UVP_OVP => "UVP_OVP".to_string(),
            IslandingDetectionEnumType::UFP_OFP => "UFP_OFP".to_string(),
            IslandingDetectionEnumType::VoltageVectorShift => "VoltageVectorShift".to_string(),
            IslandingDetectionEnumType::ZeroCrossingDetection => "ZeroCrossingDetection".to_string(),
            IslandingDetectionEnumType::OtherPassive => "OtherPassive".to_string(),
            IslandingDetectionEnumType::ImpedanceMeasurement => "ImpedanceMeasurement".to_string(),
            IslandingDetectionEnumType::ImpedanceAtFrequency => "ImpedanceAtFrequency".to_string(),
            IslandingDetectionEnumType::SlipModeFrequencyShift => "SlipModeFrequencyShift".to_string(),
            IslandingDetectionEnumType::SandiaFrequencyShift => "SandiaFrequencyShift".to_string(),
            IslandingDetectionEnumType::SandiaVoltageShift => "SandiaVoltageShift".to_string(),
            IslandingDetectionEnumType::FrequencyJump => "FrequencyJump".to_string(),
            IslandingDetectionEnumType::RCLQFactor => "RCLQFactor".to_string(),
            IslandingDetectionEnumType::OtherActive => "OtherActive".to_string(),
        }
    }
}
