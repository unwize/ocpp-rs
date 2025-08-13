use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Status for when publishing a Firmware.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UnpublishFirmwareStatusEnumType {
    /// Intermediate state. Firmware is being downloaded.
    DownloadOngoing,
    /// There is no published file.
    NoFirmware,
    /// Successful end state. Firmware file no longer being published.
    Unpublished,
}

impl fmt::Display for UnpublishFirmwareStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DownloadOngoing => write!(f, "DownloadOngoing"),
            Self::NoFirmware => write!(f, "NoFirmware"),
            Self::Unpublished => write!(f, "Unpublished"),
        }
    }
}

impl Into<String> for UnpublishFirmwareStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for UnpublishFirmwareStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "DownloadOngoing" => Ok(Self::DownloadOngoing),
            "NoFirmware" => Ok(Self::NoFirmware),
            "Unpublished" => Ok(Self::Unpublished),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "UnpublishFirmwareStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
