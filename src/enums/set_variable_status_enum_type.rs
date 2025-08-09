use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Result of setting the variable.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum SetVariableStatusEnumType {
    /// Variable successfully set.
    Accepted,
    /// Request is rejected.
    Rejected,
    /// Component is not known.
    UnknownComponent,
    /// Variable is not known.
    UnknownVariable,
    /// The AttributeType is not supported.
    NotSupportedAttributeType,
    /// A reboot is required.
    RebootRequired,
}

impl fmt::Display for SetVariableStatusEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
            Self::UnknownComponent => write!(f, "UnknownComponent"),
            Self::UnknownVariable => write!(f, "UnknownVariable"),
            Self::NotSupportedAttributeType => write!(f, "NotSupportedAttributeType"),
            Self::RebootRequired => write!(f, "RebootRequired"),
        }
    }
}

impl Into<String> for SetVariableStatusEnumType {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for SetVariableStatusEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Accepted" => Ok(Self::Accepted),
            "Rejected" => Ok(Self::Rejected),
            "UnknownComponent" => Ok(Self::UnknownComponent),
            "UnknownVariable" => Ok(Self::UnknownVariable),
            "NotSupportedAttributeType" => Ok(Self::NotSupportedAttributeType),
            "RebootRequired" => Ok(Self::RebootRequired),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "SetVariableStatusEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}