use crate::enums::firmware_status_enum_type::FirmwareStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.22. FirmwareStatusNotification
pub struct FirmwareStatusNotification;

impl OcppMessage for FirmwareStatusNotification {
    type Request = FirmwareStatusNotificationRequest;
    type Response = FirmwareStatusNotificationResponse;
}

/// 1.22.1. FirmwareStatusNotificationRequest
/// This contains the field definition of the FirmwareStatusNotificationRequest PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    /// Required. This contains the progress status of the firmware installation.
    pub status: FirmwareStatusEnumType,
    /// Optional. The request id that was provided in the UpdateFirmwareRequest that started this firmware update. This field is mandatory, unless the message was triggered by a TriggerMessageRequest AND there is no firmware update ongoing.
    pub request_id: Option<i32>,
    /// Optional. Detailed status info.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for FirmwareStatusNotificationRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(request_id) = self.request_id {
            b.check_bounds("request_id", 0, i32::MAX, request_id);
        }

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("FirmwareStatusNotificationRequest")
    }
}

#[typetag::serde]
impl OcppRequest for FirmwareStatusNotificationRequest {
    fn get_message_type(&self) -> String {
        String::from("FirmwareStatusNotification")
    }
}

/// 1.22.2. FirmwareStatusNotificationResponse
/// This contains the field definition of the FirmwareStatusNotificationResponse PDU sent by the CSMS to the Charging Station in response to a FirmwareStatusNotificationRequest. No fields are defined.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {}
#[typetag::serde]
impl OcppEntity for FirmwareStatusNotificationResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let b = StructureValidationBuilder::new();
        b.build("FirmwareStatusNotificationResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = FirmwareStatusNotification::request();
        let resp = FirmwareStatusNotification::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = FirmwareStatusNotificationRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: FirmwareStatusNotificationRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = FirmwareStatusNotificationResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: FirmwareStatusNotificationResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(FirmwareStatusNotification::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(FirmwareStatusNotification::response().validate().is_ok());
    }
}
