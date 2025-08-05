use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GetVariableStatusEnumType {
    /// Variable successfully retrieved.
    Accepted,
    /// Request is rejected.
    Rejected,
    /// Component is not known.
    UnknownComponent,
    /// Variable is not known.
    UnknownVariable,
    /// The AttributeType is not supported.
    NotSupportedAttributeType,
}

impl TryFrom<String> for GetVariableStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(GetVariableStatusEnumType::Accepted),
            "Rejected" => Ok(GetVariableStatusEnumType::Rejected),
            "UnknownComponent" => Ok(GetVariableStatusEnumType::UnknownComponent),
            "UnknownVariable" => Ok(GetVariableStatusEnumType::UnknownVariable),
            "NotSupportedAttributeType" => Ok(GetVariableStatusEnumType::NotSupportedAttributeType),
            _ => Err(format!("'{}' is not a valid GetVariableStatusEnumType", s)),
        }
    }
}

impl Into<String> for GetVariableStatusEnumType {
    fn into(self) -> String {
        match self {
            GetVariableStatusEnumType::Accepted => "Accepted".to_string(),
            GetVariableStatusEnumType::Rejected => "Rejected".to_string(),
            GetVariableStatusEnumType::UnknownComponent => "UnknownComponent".to_string(),
            GetVariableStatusEnumType::UnknownVariable => "UnknownVariable".to_string(),
            GetVariableStatusEnumType::NotSupportedAttributeType => "NotSupportedAttributeType".to_string(),
        }
    }
}
