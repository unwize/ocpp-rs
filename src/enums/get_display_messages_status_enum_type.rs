use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum GetDisplayMessagesStatusEnumType {
    /// Request accepted, there are Display Messages found that match all the requested criteria. The Charging Station will send NotifyDisplayMessagesRequest messages to report the requested Display Messages.
    #[default]
    Accepted,
    /// No messages found that match the given criteria.
    Unknown,
}

impl TryFrom<String> for GetDisplayMessagesStatusEnumType {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Accepted" => Ok(GetDisplayMessagesStatusEnumType::Accepted),
            "Unknown" => Ok(GetDisplayMessagesStatusEnumType::Unknown),
            _ => Err(format!(
                "'{}' is not a valid GetDisplayMessagesStatusEnumType",
                s
            )),
        }
    }
}

impl From<GetDisplayMessagesStatusEnumType> for String {
    fn from(val: GetDisplayMessagesStatusEnumType) -> Self {
        match val {
            GetDisplayMessagesStatusEnumType::Accepted => "Accepted".to_string(),
            GetDisplayMessagesStatusEnumType::Unknown => "Unknown".to_string(),
        }
    }
}
