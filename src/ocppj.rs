use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::traits::OcppEntity;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
#[serde(into="i32")]
enum MessageTypeId {
    #[default]
    Call = 2,
    CallResult = 3,
    CallError = 4,
    CallResultError = 5,
    Send = 6
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
            _ => Err(())
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
            _ => Err(())
        }
    }
}

impl Into<String> for MessageTypeId {
    fn into(self) -> String {
        match self {
            MessageTypeId::Call => "CALL".to_string(),
            MessageTypeId::CallResult => "CALLRESULT".to_string(),
            MessageTypeId::CallError => "CALLERROR".to_string(),
            MessageTypeId::Send => "SEND".to_string(),
            MessageTypeId::CallResultError => "CallRESULTERROR".to_string()
        }
    }
}

impl Into<i32> for MessageTypeId {
    fn into(self) -> i32 {
        match self {
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RcpCall {
    message_type_id: MessageTypeId,
    message_id: String,
    action: String,
    payload: Box<dyn OcppEntity>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RcpCallResult {
    message_type_id: MessageTypeId,
    message_id: String,
    payload: Box<dyn OcppEntity>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RcpCallError{
    message_type_id: MessageTypeId,
    message_id: String,
    error_code: String,
    error_description: String,
    error_details: Value
}