use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::enums::get_display_messages_status_enum_type::GetDisplayMessagesStatusEnumType;
use crate::enums::message_priority_enum_type::MessagePriorityEnumType;
use crate::enums::message_state_enum_type::MessageStateEnumType;
use crate::structures::status_info_type::StatusInfoType;

/// 1.30. GetDisplayMessages
pub struct GetDisplayMessages;

impl OcppMessage for GetDisplayMessages {
    type Request = GetDisplayMessagesRequest;
    type Response = GetDisplayMessagesResponse;
}

/// 1.30.1. GetDisplayMessagesRequest
/// This contains the field definition of the GetDisplayMessagesRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesRequest {
    /// Optional. If provided the Charging Station shall return Display Messages of the given Ids. This field SHALL NOT contain more Ids than set in NumberOfDisplayMessages.maxLimit.
    pub id: Option<Vec<i32>>,
    /// Required. The Id of this request.
    pub request_id: i32,
    /// Optional. If provided the Charging Station shall return Display Messages with the given priority only.
    pub priority: Option<MessagePriorityEnumType>,
    /// Optional. If provided the Charging Station shall return Display Messages with the given state only.
    pub state: Option<MessageStateEnumType>,
}

impl OcppEntity for GetDisplayMessagesRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(ids) = &self.id {
            b.check_cardinality("id", 0, usize::MAX, &ids.iter());
            for &id in ids {
                b.check_bounds("id", 0, i32::MAX, id);
            }
        }

        b.check_bounds("request_id", 0, i32::MAX, self.request_id);
        b.build("GetDisplayMessagesRequest")
    }
}

/// 1.30.2. GetDisplayMessagesResponse
/// This contains the field definition of the GetDisplayMessagesResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesResponse {
    /// Required. Indicates if the Charging Station has Display Messages that match the request criteria in the GetDisplayMessagesRequest.
    pub status: GetDisplayMessagesStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for GetDisplayMessagesResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetDisplayMessagesResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetDisplayMessages::request();
        let resp = GetDisplayMessages::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetDisplayMessagesRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetDisplayMessagesRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetDisplayMessagesResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetDisplayMessagesResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetDisplayMessages::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetDisplayMessages::response().validate().is_ok());
    }
}