use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum EnergyTransferModeEnumType {
    /// AC single phase charging according to IEC 62196.
    AC_single_phase,
    /// AC two phase charging according to IEC 62196.
    AC_two_phase,
    /// AC three phase charging according to IEC 62196.
    AC_three_phase,
    /// DC charging.
    DC,
    /// AC bidirectional (no DER control), ISO 15118-20
    AC_BPT,
    /// AC bidirectional with DER control, ISO 15118-20 (amendment to -20)
    AC_BPT_DER,
    /// AC charging-only with DER control, ISO 15118-20 (amendment to -20) Note: at time of writing (July 2024) not yet defined for ISO 15118-20.
    AC_DER,
    /// DC bidirectional power transfer, ISO 15118-20
    DC_BPT,
    /// DC via ACDP connector (pantograph), ISO 15118-20
    DC_ACDP,
    /// DC bidirectional via ACDP connector (pantograph), ISO 15118-20
    DC_ACDP_BPT,
    /// Wireless power transfer, ISO 15118-20
    WPT,
}

impl TryFrom<String> for EnergyTransferModeEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "AC_single_phase" => Ok(EnergyTransferModeEnumType::AC_single_phase),
            "AC_two_phase" => Ok(EnergyTransferModeEnumType::AC_two_phase),
            "AC_three_phase" => Ok(EnergyTransferModeEnumType::AC_three_phase),
            "DC" => Ok(EnergyTransferModeEnumType::DC),
            "AC_BPT" => Ok(EnergyTransferModeEnumType::AC_BPT),
            "AC_BPT_DER" => Ok(EnergyTransferModeEnumType::AC_BPT_DER),
            "AC_DER" => Ok(EnergyTransferModeEnumType::AC_DER),
            "DC_BPT" => Ok(EnergyTransferModeEnumType::DC_BPT),
            "DC_ACDP" => Ok(EnergyTransferModeEnumType::DC_ACDP),
            "DC_ACDP_BPT" => Ok(EnergyTransferModeEnumType::DC_ACDP_BPT),
            "WPT" => Ok(EnergyTransferModeEnumType::WPT),
            _ => Err(format!("'{}' is not a valid EnergyTransferModeEnumType", s)),
        }
    }
}

impl Into<String> for EnergyTransferModeEnumType {
    fn into(self) -> String {
        match self {
            EnergyTransferModeEnumType::AC_single_phase => "AC_single_phase".to_string(),
            EnergyTransferModeEnumType::AC_two_phase => "AC_two_phase".to_string(),
            EnergyTransferModeEnumType::AC_three_phase => "AC_three_phase".to_string(),
            EnergyTransferModeEnumType::DC => "DC".to_string(),
            EnergyTransferModeEnumType::AC_BPT => "AC_BPT".to_string(),
            EnergyTransferModeEnumType::AC_BPT_DER => "AC_BPT_DER".to_string(),
            EnergyTransferModeEnumType::AC_DER => "AC_DER".to_string(),
            EnergyTransferModeEnumType::DC_BPT => "DC_BPT".to_string(),
            EnergyTransferModeEnumType::DC_ACDP => "DC_ACDP".to_string(),
            EnergyTransferModeEnumType::DC_ACDP_BPT => "DC_ACDP_BPT".to_string(),
            EnergyTransferModeEnumType::WPT => "WPT".to_string(),
        }
    }
}
