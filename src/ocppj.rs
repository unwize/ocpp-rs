use crate::ocpp_message::OcppMessage;
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
#[serde(into = "i32", try_from = "i32")]
pub enum MessageTypeId {
    #[default]
    Call = 2,
    CallResult = 3,
    CallError = 4,
    CallResultError = 5,
    Send = 6,
}

impl TryFrom<&str> for MessageTypeId {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "CALL" => Ok(MessageTypeId::Call),
            "CALLRESULT" => Ok(MessageTypeId::CallResult),
            "CALLERROR" => Ok(MessageTypeId::CallError),
            "CALLRESULTERROR" => Ok(MessageTypeId::CallResultError),
            "SEND" => Ok(MessageTypeId::Send),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for MessageTypeId {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.as_str().try_into()
    }
}

impl TryFrom<i32> for MessageTypeId {
    type Error = String;
    fn try_from(i: i32) -> Result<Self, Self::Error> {
        match i {
            2 => Ok(MessageTypeId::Call),
            3 => Ok(MessageTypeId::CallResult),
            4 => Ok(MessageTypeId::CallError),
            5 => Ok(MessageTypeId::CallResultError),
            6 => Ok(MessageTypeId::Send),
            _ => Err(format!("Invalid MessageTypeId: {}", i)),
        }
    }
}

impl From<MessageTypeId> for String {
    fn from(val: MessageTypeId) -> Self {
        match val {
            MessageTypeId::Call => "CALL".to_string(),
            MessageTypeId::CallResult => "CALLRESULT".to_string(),
            MessageTypeId::CallError => "CALLERROR".to_string(),
            MessageTypeId::Send => "SEND".to_string(),
            MessageTypeId::CallResultError => "CallRESULTERROR".to_string(),
        }
    }
}

impl From<MessageTypeId> for i32 {
    fn from(val: MessageTypeId) -> Self {
        match val {
            MessageTypeId::Call => 2,
            MessageTypeId::CallResult => 3,
            MessageTypeId::CallError => 4,
            MessageTypeId::CallResultError => 5,
            MessageTypeId::Send => 6,
        }
    }
}

/// A struct containing all the info required to send an ocpp message in a way that complies with
/// OCPP-J. Messages strictly adhere to RCP standards.
#[derive(Clone, Debug, Serialize_tuple)]
pub struct RcpCall {
    pub message_type_id: MessageTypeId,
    pub message_id: String,
    pub action: String,
    pub payload: OcppMessage,
}

impl<'de> Deserialize<'de> for RcpCall {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RcpCallVisitor;

        impl<'de> Visitor<'de> for RcpCallVisitor {
            type Value = RcpCall;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a tuple representing an RcpCall")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<RcpCall, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let message_type_id = seq
                    .next_element::<MessageTypeId>()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let message_id = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let action = seq
                    .next_element::<String>()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let payload_value = seq
                    .next_element::<Value>()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                let payload = OcppMessage::parse_request(&action, payload_value)
                    .map_err(de::Error::custom)?;

                Ok(RcpCall {
                    message_type_id,
                    message_id,
                    action,
                    payload,
                })
            }
        }

        deserializer.deserialize_seq(RcpCallVisitor)
    }
}

impl RcpCall {
    /// Create a new RCP-spec CALL.
    pub fn new(message_id: &str, payload: OcppMessage) -> Self {
        Self {
            message_type_id: MessageTypeId::Call,
            message_id: String::from(message_id),
            action: payload.get_message_type().to_string(),
            payload,
        }
    }
}

#[derive(Clone, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RcpCallResult {
    pub message_type_id: MessageTypeId,
    pub message_id: String,
    pub payload: OcppMessage,
}

#[derive(Clone, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RcpCallError {
    pub message_type_id: MessageTypeId,
    pub message_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: Value,
}

#[cfg(test)]
mod test {
    use crate::messages::adjust_periodic_event_stream::AdjustPeriodicEventStreamRequest;
    use super::*;
    #[test]
    fn test_serialize_rpc_call() {
        let call = RcpCall::new(
            "12345",
            OcppMessage::AdjustPeriodicEventStreamRequest(AdjustPeriodicEventStreamRequest::default()),
        );

        let serialized = serde_json::to_string_pretty(&call).unwrap();
        println!("{}", serialized);
        let deserialized: RcpCall = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.message_type_id, MessageTypeId::Call);
    }
}
