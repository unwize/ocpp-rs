use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FirmwareStatusEnumType {
    /// Intermediate state. New firmware has been downloaded by Charging Station.
    Downloaded,
    /// Failure end state. Charging Station failed to download firmware.
    DownloadFailed,
    /// Intermediate state. Firmware is being downloaded.
    Downloading,
    /// Intermediate state. Downloading of new firmware has been scheduled.
    DownloadScheduled,
    /// Intermediate state. Downloading has been paused.
    DownloadPaused,
    /// Charging Station is not performing firmware update related tasks. Status Idle SHALL only be used as in a FirmwareStatusNotificationRequest that was triggered by TriggerMessageRequest.
    Idle,
    /// Failure end state. Installation of new firmware has failed.
    InstallationFailed,
    /// Intermediate state. Firmware is being installed.
    Installing,
    /// Successful end state. New firmware has successfully been installed in Charging Station.
    Installed,
    /// Intermediate state. If sent before installing the firmware, it indicates the Charging Station is about to reboot to start installing new firmware. If sent after installing the new firmware, it indicates the Charging Station has finished installing, but requires a reboot to activate the new firmware, which will be done automatically when idle. This status MAY be omitted if a reboot is an integral part of the installation and cannot be reported separately.
    InstallRebooting,
    /// Intermediate state. Installation of the downloaded firmware is scheduled to take place on installDateTime given in UpdateFirmware request.
    InstallScheduled,
    /// Failure end state. Verification of the new firmware (e.g. using a checksum or some other means) has failed and installation will not proceed. (Final failure state)
    InstallVerificationFailed,
    /// Failure end state. The firmware signature is not valid.
    InvalidSignature,
    /// Intermediate state. Provide signature successfully verified.
    SignatureVerified,
}

impl TryFrom<String> for FirmwareStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Downloaded" => Ok(FirmwareStatusEnumType::Downloaded),
            "DownloadFailed" => Ok(FirmwareStatusEnumType::DownloadFailed),
            "Downloading" => Ok(FirmwareStatusEnumType::Downloading),
            "DownloadScheduled" => Ok(FirmwareStatusEnumType::DownloadScheduled),
            "DownloadPaused" => Ok(FirmwareStatusEnumType::DownloadPaused),
            "Idle" => Ok(FirmwareStatusEnumType::Idle),
            "InstallationFailed" => Ok(FirmwareStatusEnumType::InstallationFailed),
            "Installing" => Ok(FirmwareStatusEnumType::Installing),
            "Installed" => Ok(FirmwareStatusEnumType::Installed),
            "InstallRebooting" => Ok(FirmwareStatusEnumType::InstallRebooting),
            "InstallScheduled" => Ok(FirmwareStatusEnumType::InstallScheduled),
            "InstallVerificationFailed" => Ok(FirmwareStatusEnumType::InstallVerificationFailed),
            "InvalidSignature" => Ok(FirmwareStatusEnumType::InvalidSignature),
            "SignatureVerified" => Ok(FirmwareStatusEnumType::SignatureVerified),
            _ => Err(format!("'{}' is not a valid FirmwareStatusEnumType", s)),
        }
    }
}

impl Into<String> for FirmwareStatusEnumType {
    fn into(self) -> String {
        match self {
            FirmwareStatusEnumType::Downloaded => "Downloaded".to_string(),
            FirmwareStatusEnumType::DownloadFailed => "DownloadFailed".to_string(),
            FirmwareStatusEnumType::Downloading => "Downloading".to_string(),
            FirmwareStatusEnumType::DownloadScheduled => "DownloadScheduled".to_string(),
            FirmwareStatusEnumType::DownloadPaused => "DownloadPaused".to_string(),
            FirmwareStatusEnumType::Idle => "Idle".to_string(),
            FirmwareStatusEnumType::InstallationFailed => "InstallationFailed".to_string(),
            FirmwareStatusEnumType::Installing => "Installing".to_string(),
            FirmwareStatusEnumType::Installed => "Installed".to_string(),
            FirmwareStatusEnumType::InstallRebooting => "InstallRebooting".to_string(),
            FirmwareStatusEnumType::InstallScheduled => "InstallScheduled".to_string(),
            FirmwareStatusEnumType::InstallVerificationFailed => "InstallVerificationFailed".to_string(),
            FirmwareStatusEnumType::InvalidSignature => "InvalidSignature".to_string(),
            FirmwareStatusEnumType::SignatureVerified => "SignatureVerified".to_string(),
        }
    }
}
