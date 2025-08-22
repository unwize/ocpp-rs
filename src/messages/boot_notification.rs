use crate::enums::boot_reason_enum_type::BootReasonEnumType;
use crate::enums::registration_status_enum_type::RegistrationStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::charging_station_type::ChargingStationType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.5. BootNotification
pub struct BootNotification;

impl OcppMessage for BootNotification {
    type Request = BootNotificationRequest;
    type Response = BootNotificationResponse;
}

/// 1.5.1. BootNotificationRequest
/// This contains the field definition of the BootNotificationRequest PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    /// Required. This contains the reason for sending this message to the CSMS.
    pub reason: BootReasonEnumType,
    /// Required. Identifies the Charging Station
    pub charging_station: ChargingStationType,
}

impl OcppEntity for BootNotificationRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_member("charging_station", &self.charging_station);

        b.build("BootNotificationRequest")
    }
}

/// 1.5.2. BootNotificationResponse
/// This contains the field definition of the BootNotificationResponse PDU sent by the CSMS to the Charging Station in response to a BootNotificationRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    /// Required. This contains the CSMS's current time.
    pub current_time: DateTime<Utc>,
    /// Required. When Status is Accepted, this contains the heartbeat interval in seconds.
    /// If the CSMS returns something other than Accepted, the value of the interval field indicates the minimum wait time before sending a next BootNotification request.
    pub interval: i32,
    /// Required. This contains whether the Charging Station has been registered within the CSMS.
    pub status: RegistrationStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for BootNotificationResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("BootNotificationResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = BootNotification::request();
        let resp = BootNotification::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = BootNotificationRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: BootNotificationRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = BootNotificationResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: BootNotificationResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(BootNotification::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(BootNotification::response().validate().is_ok());
    }
}
