use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Status of the log upload process.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum UploadLogStatusEnumType {
    /// A badly formatted packet or other protocol incompatibility was detected.
    BadMessage,
    /// The Charging Station is not uploading a log file. Idle SHALL only be used when the message was triggered by a TriggerMessageRequest.
    Idle,
    /// The server does not support the operation.
    NotSupportedOperation,
    /// Insufficient permissions to perform the operation.
    PermissionDenied,
    /// File has been uploaded successfully.
    Uploaded,
    /// Failed to upload the requested file.
    UploadFailure,
    /// File is being uploaded.
    Uploading,
    /// On-going log upload is canceled and a new request to upload log has been accepted.
    AcceptedCanceled,
}

impl fmt::Display for UploadLogStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BadMessage => write!(f, "BadMessage"),
            Self::Idle => write!(f, "Idle"),
            Self::NotSupportedOperation => write!(f, "NotSupportedOperation"),
            Self::PermissionDenied => write!(f, "PermissionDenied"),
            Self::Uploaded => write!(f, "Uploaded"),
            Self::UploadFailure => write!(f, "UploadFailure"),
            Self::Uploading => write!(f, "Uploading"),
            Self::AcceptedCanceled => write!(f, "AcceptedCanceled"),
        }
    }
}

impl Into<String> for UploadLogStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for UploadLogStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "BadMessage" => Ok(Self::BadMessage),
            "Idle" => Ok(Self::Idle),
            "NotSupportedOperation" => Ok(Self::NotSupportedOperation),
            "PermissionDenied" => Ok(Self::PermissionDenied),
            "Uploaded" => Ok(Self::Uploaded),
            "UploadFailure" => Ok(Self::UploadFailure),
            "Uploading" => Ok(Self::Uploading),
            "AcceptedCanceled" => Ok(Self::AcceptedCanceled),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "UploadLogStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}