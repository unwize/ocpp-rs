use crate::traits::{OcppEntity, OcppRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use uuid::uuid;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
#[serde(into = "i32")]
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
    type Error = ();
    fn try_from(i: i32) -> Result<Self, Self::Error> {
        match i {
            2 => Ok(MessageTypeId::Call),
            3 => Ok(MessageTypeId::CallResult),
            4 => Ok(MessageTypeId::CallError),
            5 => Ok(MessageTypeId::CallResultError),
            6 => Ok(MessageTypeId::Send),
            _ => Err(()),
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
#[derive(Clone, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RcpCall {
    pub message_type_id: MessageTypeId,
    pub message_id: String,
    pub action: String,
    pub payload: Box<dyn OcppRequest>,
}

impl RcpCall {

    /// Create a new RCP-spec CALL.
    pub fn new(message_type_id: MessageTypeId, payload: Box<dyn OcppRequest>) -> Self {
        Self {
            message_type_id,
            message_id: uuid::Uuid::new_v4().to_string(),
            action: payload.get_message_type(),
            payload,
        }
    }
}

#[derive(Clone, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RcpCallResult {
    pub message_type_id: MessageTypeId,
    pub message_id: String,
    pub payload: Box<dyn OcppEntity>,
}

#[derive(Clone, Debug, Serialize_tuple, Deserialize_tuple)]
pub struct RcpCallError {
    pub message_type_id: MessageTypeId,
    pub message_id: String,
    pub error_code: String,
    pub error_description: String,
    pub error_details: Value,
}