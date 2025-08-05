use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum DisplayMessageStatusEnumType {
    /// Request to display message accepted.
    Accepted,
    /// None of the formats in the given message are supported.
    NotSupportedMessageFormat,
    /// Request cannot be handled.
    Rejected,
    /// The given MessagePriority not supported for displaying messages by Charging Station.
    NotSupportedPriority,
    /// The given MessageState not supported for displaying messages by Charging Station.
    NotSupportedState,
    /// Given Transaction not known/ongoing.
    UnknownTransaction,
    /// Message contains one or more languages that are not supported by Charging Station.
    LanguageNotSupported,
}

impl TryFrom<String> for DisplayMessageStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(DisplayMessageStatusEnumType::Accepted),
            "NotSupportedMessageFormat" => Ok(DisplayMessageStatusEnumType::NotSupportedMessageFormat),
            "Rejected" => Ok(DisplayMessageStatusEnumType::Rejected),
            "NotSupportedPriority" => Ok(DisplayMessageStatusEnumType::NotSupportedPriority),
            "NotSupportedState" => Ok(DisplayMessageStatusEnumType::NotSupportedState),
            "UnknownTransaction" => Ok(DisplayMessageStatusEnumType::UnknownTransaction),
            "LanguageNotSupported" => Ok(DisplayMessageStatusEnumType::LanguageNotSupported),
            _ => Err(format!("'{}' is not a valid DisplayMessageStatusEnumType", s)),
        }
    }
}

impl Into<String> for DisplayMessageStatusEnumType {
    fn into(self) -> String {
        match self {
            DisplayMessageStatusEnumType::Accepted => "Accepted".to_string(),
            DisplayMessageStatusEnumType::NotSupportedMessageFormat => "NotSupportedMessageFormat".to_string(),
            DisplayMessageStatusEnumType::Rejected => "Rejected".to_string(),
            DisplayMessageStatusEnumType::NotSupportedPriority => "NotSupportedPriority".to_string(),
            DisplayMessageStatusEnumType::NotSupportedState => "NotSupportedState".to_string(),
            DisplayMessageStatusEnumType::UnknownTransaction => "UnknownTransaction".to_string(),
            DisplayMessageStatusEnumType::LanguageNotSupported => "LanguageNotSupported".to_string(),
        }
    }
}
