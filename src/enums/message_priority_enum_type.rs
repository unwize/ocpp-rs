use crate::errors::OcppError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

/// Priority with which a message should be displayed on a Charging Station.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum MessagePriorityEnumType {
    /// Show this message always in front. Highest priority, don't cycle with other messages. When a newer message with this MessagePriority is received, this message is replaced. No Charging Station own message may override this message.
    AlwaysFront,
    /// Show this message in front of the normal cycle of messages. When more messages with this priority are to be shown, they SHALL be cycled.
    InFront,
    /// Show this message in the cycle of messages.
    NormalCycle,
}

impl fmt::Display for MessagePriorityEnumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AlwaysFront => write!(f, "AlwaysFront"),
            Self::InFront => write!(f, "InFront"),
            Self::NormalCycle => write!(f, "NormalCycle"),
        }
    }
}

impl From<MessagePriorityEnumType> for String {
    fn from(val: MessagePriorityEnumType) -> Self {
        val.to_string()
    }
}

impl TryFrom<&str> for MessagePriorityEnumType {
    type Error = OcppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AlwaysFront" => Ok(Self::AlwaysFront),
            "InFront" => Ok(Self::InFront),
            "NormalCycle" => Ok(Self::NormalCycle),
            _ => Err(OcppError::InvalidEnumValueError {
                enum_name: "MessagePriorityEnumType".to_string(),
                value: value.to_string(),
            }),
        }
    }
}
