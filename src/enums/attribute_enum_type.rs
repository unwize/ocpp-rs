use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum AttributeEnumType {
    /// The actual value of the variable.
    Actual,
    /// The target value for this variable.
    Target,
    /// The minimal allowed value for this variable
    MinSet,
    /// The maximum allowed value for this variable
    MaxSet,
}

impl TryFrom<String> for AttributeEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Actual" => Ok(AttributeEnumType::Actual),
            "Target" => Ok(AttributeEnumType::Target),
            "MinSet" => Ok(AttributeEnumType::MinSet),
            "MaxSet" => Ok(AttributeEnumType::MaxSet),
            _ => Err(format!("'{}' is not a valid AttributeEnumType", s)),
        }
    }
}

impl From<AttributeEnumType> for String {
    fn from(val: AttributeEnumType) -> Self {
        match val {
            AttributeEnumType::Actual => "Actual".to_string(),
            AttributeEnumType::Target => "Target".to_string(),
            AttributeEnumType::MinSet => "MinSet".to_string(),
            AttributeEnumType::MaxSet => "MaxSet".to_string(),
        }
    }
}
