use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum DataTransferStatusEnumType {
    /// Message has been accepted and the contained request is accepted.
    #[default]
    Accepted,
    /// Message has been accepted but the contained request is rejected.
    Rejected,
    /// Message could not be interpreted due to unknown messageId string.
    UnknownMessageId,
    /// Message could not be interpreted due to unknown vendorId string.
    UnknownVendorId,
}

impl TryFrom<String> for DataTransferStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(DataTransferStatusEnumType::Accepted),
            "Rejected" => Ok(DataTransferStatusEnumType::Rejected),
            "UnknownMessageId" => Ok(DataTransferStatusEnumType::UnknownMessageId),
            "UnknownVendorId" => Ok(DataTransferStatusEnumType::UnknownVendorId),
            _ => Err(format!("'{}' is not a valid DataTransferStatusEnumType", s)),
        }
    }
}

impl From<DataTransferStatusEnumType> for String {
    fn from(val: DataTransferStatusEnumType) -> Self {
        match val {
            DataTransferStatusEnumType::Accepted => "Accepted".to_string(),
            DataTransferStatusEnumType::Rejected => "Rejected".to_string(),
            DataTransferStatusEnumType::UnknownMessageId => "UnknownMessageId".to_string(),
            DataTransferStatusEnumType::UnknownVendorId => "UnknownVendorId".to_string(),
        }
    }
}
