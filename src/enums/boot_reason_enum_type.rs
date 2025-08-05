use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum BootReasonEnumType {
    /// The Charging Station rebooted due to an application error.
    ApplicationReset,
    /// The Charging Station rebooted due to a firmware update.
    FirmwareUpdate,
    /// The Charging Station rebooted due to a local reset command.
    LocalReset,
    /// The Charging Station powered up and registers itself with the CSMS.
    PowerUp,
    /// The Charging Station rebooted due to a remote reset command.
    RemoteReset,
    /// The Charging Station rebooted due to a scheduled reset command.
    ScheduledReset,
    /// Requested by the CSMS via a TriggerMessage
    Triggered,
    /// The boot reason is unknown.
    Unknown,
    /// The Charging Station rebooted due to an elapsed watchdog timer.
    Watchdog,
}

impl TryFrom<String> for BootReasonEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "ApplicationReset" => Ok(BootReasonEnumType::ApplicationReset),
            "FirmwareUpdate" => Ok(BootReasonEnumType::FirmwareUpdate),
            "LocalReset" => Ok(BootReasonEnumType::LocalReset),
            "PowerUp" => Ok(BootReasonEnumType::PowerUp),
            "RemoteReset" => Ok(BootReasonEnumType::RemoteReset),
            "ScheduledReset" => Ok(BootReasonEnumType::ScheduledReset),
            "Triggered" => Ok(BootReasonEnumType::Triggered),
            "Unknown" => Ok(BootReasonEnumType::Unknown),
            "Watchdog" => Ok(BootReasonEnumType::Watchdog),
            _ => Err(format!("'{}' is not a valid BootReasonEnumType", s)),
        }
    }
}

impl Into<String> for BootReasonEnumType {
    fn into(self) -> String {
        match self {
            BootReasonEnumType::ApplicationReset => "ApplicationReset".to_string(),
            BootReasonEnumType::FirmwareUpdate => "FirmwareUpdate".to_string(),
            BootReasonEnumType::LocalReset => "LocalReset".to_string(),
            BootReasonEnumType::PowerUp => "PowerUp".to_string(),
            BootReasonEnumType::RemoteReset => "RemoteReset".to_string(),
            BootReasonEnumType::ScheduledReset => "ScheduledReset".to_string(),
            BootReasonEnumType::Triggered => "Triggered".to_string(),
            BootReasonEnumType::Unknown => "Unknown".to_string(),
            BootReasonEnumType::Watchdog => "Watchdog".to_string(),
        }
    }
}
