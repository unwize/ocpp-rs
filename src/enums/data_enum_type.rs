use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum DataEnumType {
    /// This variable is of the type string.
    String,
    /// This variable is of the type decimal.
    Decimal,
    /// This variable is of the type integer.
    Integer,
    /// DateTime following the [RFC3339] specification.
    DateTime,
    /// This variable is of the type boolean.
    Boolean,
    /// Supported/allowed values for a single choice, enumerated, text variable.
    OptionList,
    /// Supported/allowed values for an ordered sequence variable.
    SequenceList,
    /// Supported/allowed values for a mathematical set variable.
    MemberList,
}

impl TryFrom<String> for DataEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "string" => Ok(DataEnumType::String),
            "decimal" => Ok(DataEnumType::Decimal),
            "integer" => Ok(DataEnumType::Integer),
            "dateTime" => Ok(DataEnumType::DateTime),
            "boolean" => Ok(DataEnumType::Boolean),
            "OptionList" => Ok(DataEnumType::OptionList),
            "SequenceList" => Ok(DataEnumType::SequenceList),
            "MemberList" => Ok(DataEnumType::MemberList),
            _ => Err(format!("'{}' is not a valid DataEnumType", s)),
        }
    }
}

impl Into<String> for DataEnumType {
    fn into(self) -> String {
        match self {
            DataEnumType::String => "string".to_string(),
            DataEnumType::Decimal => "decimal".to_string(),
            DataEnumType::Integer => "integer".to_string(),
            DataEnumType::DateTime => "dateTime".to_string(),
            DataEnumType::Boolean => "boolean".to_string(),
            DataEnumType::OptionList => "OptionList".to_string(),
            DataEnumType::SequenceList => "SequenceList".to_string(),
            DataEnumType::MemberList => "MemberList".to_string(),
        }
    }
}
