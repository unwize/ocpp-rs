use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GenericDeviceModelStatusEnumType {
    /// Request has been accepted and will be executed.
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
    /// The content of the request message is not supported.
    NotSupported,
    /// If the combination of received criteria result in an empty result set.
    EmptyResultSet,
}

impl TryFrom<String> for GenericDeviceModelStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(GenericDeviceModelStatusEnumType::Accepted),
            "Rejected" => Ok(GenericDeviceModelStatusEnumType::Rejected),
            "NotSupported" => Ok(GenericDeviceModelStatusEnumType::NotSupported),
            "EmptyResultSet" => Ok(GenericDeviceModelStatusEnumType::EmptyResultSet),
            _ => Err(format!("'{}' is not a valid GenericDeviceModelStatusEnumType", s)),
        }
    }
}

impl Into<String> for GenericDeviceModelStatusEnumType {
    fn into(self) -> String {
        match self {
            GenericDeviceModelStatusEnumType::Accepted => "Accepted".to_string(),
            GenericDeviceModelStatusEnumType::Rejected => "Rejected".to_string(),
            GenericDeviceModelStatusEnumType::NotSupported => "NotSupported".to_string(),
            GenericDeviceModelStatusEnumType::EmptyResultSet => "EmptyResultSet".to_string(),
        }
    }
}
