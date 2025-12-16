use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Status for when publishing a Firmware.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PublishFirmwareStatusEnumType {
    /// Intermediate state.
    Idle,
    /// Intermediate state. Downloading of new firmware has been scheduled.
    DownloadScheduled,
    /// Intermediate state. Firmware is being downloaded.
    Downloading,
    /// Intermediate state. New firmware has been downloaded by Charging Station.
    Downloaded,
    /// The firmware has been successfully published.
    Published,
    /// Failure end state. Charging Station failed to download firmware.
    DownloadFailed,
    /// Intermediate state. Downloading has been paused.
    DownloadPaused,
    /// Failure end state. The firmware checksum is not matching.
    InvalidChecksum,
    /// Intermediate state. The Firmware checksum is successfully verified.
    ChecksumVerified,
    /// Publishing the new firmware has failed.
    PublishFailed,
}

impl fmt::Display for PublishFirmwareStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::DownloadScheduled => write!(f, "DownloadScheduled"),
            Self::Downloading => write!(f, "Downloading"),
            Self::Downloaded => write!(f, "Downloaded"),
            Self::Published => write!(f, "Published"),
            Self::DownloadFailed => write!(f, "DownloadFailed"),
            Self::DownloadPaused => write!(f, "DownloadPaused"),
            Self::InvalidChecksum => write!(f, "InvalidChecksum"),
            Self::ChecksumVerified => write!(f, "ChecksumVerified"),
            Self::PublishFailed => write!(f, "PublishFailed"),
        }
    }
}

impl From<PublishFirmwareStatusEnumType> for String {
    fn from(val: PublishFirmwareStatusEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for PublishFirmwareStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Idle" => Ok(Self::Idle),
            "DownloadScheduled" => Ok(Self::DownloadScheduled),
            "Downloading" => Ok(Self::Downloading),
            "Downloaded" => Ok(Self::Downloaded),
            "Published" => Ok(Self::Published),
            "DownloadFailed" => Ok(Self::DownloadFailed),
            "DownloadPaused" => Ok(Self::DownloadPaused),
            "InvalidChecksum" => Ok(Self::InvalidChecksum),
            "ChecksumVerified" => Ok(Self::ChecksumVerified),
            "PublishFailed" => Ok(Self::PublishFailed),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "PublishFirmwareStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
