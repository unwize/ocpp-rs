use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum DERControlEnumType {
    /// Enter Service parameters setting
    EnterService,
    /// Frequency droop settings
    FreqDroop,
    /// Frequency-Watt curve
    FreqWatt,
    /// Fixed power factor when absorbing power setting
    FixedPFAbsorb,
    /// Fixed power factor when injecting power setting
    FixedPFInject,
    /// Fixed reactive power setpoint
    FixedVAr,
    /// Gradient settings
    Gradients,
    /// High Frequency Must Trip curve
    HFMustTrip,
    /// High Frequency May Trip curve (ride-through)
    HFMayTrip,
    /// High Voltage Must Trip curve
    HVMustTrip,
    /// High Voltage Momentary Cessation curve
    HVMomCess,
    /// Limit discharge power to percentage of rated discharge power
    LimitMaxDischarge,
    /// Low Frequency Must Trip curve
    LFMustTrip,
    /// Low Voltage Must Trip curve
    LVMustTrip,
    /// Low Voltage Momentary Cessation curve
    LVMomCess,
    /// Low Voltage May Trip curve (ride-through)
    LVMayTrip,
    /// Power Monitoring curve according to VDE-AR-N 4105 section 5.5.2
    PowerMonitoringMustTrip,
    /// Volt-Var curve
    VoltVar,
    /// Volt-Watt curve
    VoltWatt,
    /// Watt-PowerFactor curve
    WattPF,
    /// Watt-Var curve
    WattVar,
}

impl TryFrom<String> for DERControlEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "EnterService" => Ok(DERControlEnumType::EnterService),
            "FreqDroop" => Ok(DERControlEnumType::FreqDroop),
            "FreqWatt" => Ok(DERControlEnumType::FreqWatt),
            "FixedPFAbsorb" => Ok(DERControlEnumType::FixedPFAbsorb),
            "FixedPFInject" => Ok(DERControlEnumType::FixedPFInject),
            "FixedVAr" => Ok(DERControlEnumType::FixedVAr),
            "Gradients" => Ok(DERControlEnumType::Gradients),
            "HFMustTrip" => Ok(DERControlEnumType::HFMustTrip),
            "HFMayTrip" => Ok(DERControlEnumType::HFMayTrip),
            "HVMustTrip" => Ok(DERControlEnumType::HVMustTrip),
            "HVMomCess" => Ok(DERControlEnumType::HVMomCess),
            "LimitMaxDischarge" => Ok(DERControlEnumType::LimitMaxDischarge),
            "LFMustTrip" => Ok(DERControlEnumType::LFMustTrip),
            "LVMustTrip" => Ok(DERControlEnumType::LVMustTrip),
            "LVMomCess" => Ok(DERControlEnumType::LVMomCess),
            "LVMayTrip" => Ok(DERControlEnumType::LVMayTrip),
            "PowerMonitoringMustTrip" => Ok(DERControlEnumType::PowerMonitoringMustTrip),
            "VoltVar" => Ok(DERControlEnumType::VoltVar),
            "VoltWatt" => Ok(DERControlEnumType::VoltWatt),
            "WattPF" => Ok(DERControlEnumType::WattPF),
            "WattVar" => Ok(DERControlEnumType::WattVar),
            _ => Err(format!("'{}' is not a valid DERControlEnumType", s)),
        }
    }
}

impl Into<String> for DERControlEnumType {
    fn into(self) -> String {
        match self {
            DERControlEnumType::EnterService => "EnterService".to_string(),
            DERControlEnumType::FreqDroop => "FreqDroop".to_string(),
            DERControlEnumType::FreqWatt => "FreqWatt".to_string(),
            DERControlEnumType::FixedPFAbsorb => "FixedPFAbsorb".to_string(),
            DERControlEnumType::FixedPFInject => "FixedPFInject".to_string(),
            DERControlEnumType::FixedVAr => "FixedVAr".to_string(),
            DERControlEnumType::Gradients => "Gradients".to_string(),
            DERControlEnumType::HFMustTrip => "HFMustTrip".to_string(),
            DERControlEnumType::HFMayTrip => "HFMayTrip".to_string(),
            DERControlEnumType::HVMustTrip => "HVMustTrip".to_string(),
            DERControlEnumType::HVMomCess => "HVMomCess".to_string(),
            DERControlEnumType::LimitMaxDischarge => "LimitMaxDischarge".to_string(),
            DERControlEnumType::LFMustTrip => "LFMustTrip".to_string(),
            DERControlEnumType::LVMustTrip => "LVMustTrip".to_string(),
            DERControlEnumType::LVMomCess => "LVMomCess".to_string(),
            DERControlEnumType::LVMayTrip => "LVMayTrip".to_string(),
            DERControlEnumType::PowerMonitoringMustTrip => "PowerMonitoringMustTrip".to_string(),
            DERControlEnumType::VoltVar => "VoltVar".to_string(),
            DERControlEnumType::VoltWatt => "VoltWatt".to_string(),
            DERControlEnumType::WattPF => "WattPF".to_string(),
            DERControlEnumType::WattVar => "WattVar".to_string(),
        }
    }
}
