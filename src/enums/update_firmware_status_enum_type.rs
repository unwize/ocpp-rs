use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Generic message response status
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UpdateFirmwareStatusEnumType {
    /// Accepted this firmware update request. This does not mean the firmware update is successful, the Charging Station will now start the firmware update process.
    Accepted,
    /// Firmware update request rejected.
    Rejected,
    /// Accepted this firmware update request, but in doing this has canceled an ongoing firmware update.
    AcceptedCanceled,
    /// The certificate is invalid.
    InvalidCertificate,
    /// Failure end state. The Firmware Signing certificate has been revoked.
    RevokedCertificate,
}

impl fmt::Display for UpdateFirmwareStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::AcceptedCanceled => write!(f, "AcceptedCanceled"),
            Self::InvalidCertificate => write!(f, "InvalidCertificate"),
            Self::RevokedCertificate => write!(f, "RevokedCertificate"),
        }
    }
}

impl From<UpdateFirmwareStatusEnumType> for String {
    fn from(val: UpdateFirmwareStatusEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for UpdateFirmwareStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "AcceptedCanceled" => Ok(Self::AcceptedCanceled),
            "InvalidCertificate" => Ok(Self::InvalidCertificate),
            "RevokedCertificate" => Ok(Self::RevokedCertificate),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "UpdateFirmwareStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
