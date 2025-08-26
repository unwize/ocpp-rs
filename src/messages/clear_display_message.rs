use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::enums::clear_message_status_enum_type::ClearMessageStatusEnumType;
use crate::structures::status_info_type::StatusInfoType;

/// 1.13. ClearDisplayMessage
pub struct ClearDisplayMessage;

impl OcppMessage for ClearDisplayMessage {
    type Request = ClearDisplayMessageRequest;
    type Response = ClearDisplayMessageResponse;
}

/// 1.13.1. ClearDisplayMessageRequest
/// This contains the field definition of the ClearDisplayMessageRequest PDU sent by the CSMS to the Charging Station. The CSMS asks the Charging Station to clear a display message that has been configured in the Charging Station to be cleared/removed.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageRequest {
    /// Required. Id of the message that SHALL be removed from the Charging Station.
    pub id: i32,
}

impl OcppEntity for ClearDisplayMessageRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_bounds("id", 0, i32::MAX, self.id);

        b.build("ClearDisplayMessageRequest")
    }
}

/// 1.13.2. ClearDisplayMessageResponse
/// This contains the field definition of the ClearDisplayMessageResponse PDU sent by the Charging Station to the CSMS in response to a ClearDisplayMessageRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayMessageResponse {
    /// Required. Returns whether the Charging Station has been able to remove the message.
    pub status: ClearMessageStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for ClearDisplayMessageResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, so no validation is needed.
        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("ClearDisplayMessageResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ClearDisplayMessage::request();
        let resp = ClearDisplayMessage::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ClearDisplayMessageRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ClearDisplayMessageRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ClearDisplayMessageResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ClearDisplayMessageResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ClearDisplayMessage::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ClearDisplayMessage::response().validate().is_ok());
    }

    #[test]
    fn test_request_invalid_id() {
        let mut req = ClearDisplayMessage::request();
        req.id = -1;
        assert!(req.validate().is_err());
    }
}