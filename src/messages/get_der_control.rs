use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::enums::der_control_enum_type::DERControlEnumType;
use crate::enums::der_control_status_enum_type::DERControlStatusEnumType;
use crate::structures::status_info_type::StatusInfoType;

/// 1.29. GetDERControl
pub struct GetDERControl;

impl OcppMessage for GetDERControl {
    type Request = GetDERControlRequest;
    type Response = GetDERControlResponse;
}

/// 1.29.1. GetDERControlRequest
/// This contains the field definition of the GetDERControlRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDERControlRequest {
    /// Required. RequestId to be used in ReportDERControlRequest.
    pub request_id: i32,
    /// Optional. True: get a default DER control. False: get a scheduled control.
    pub is_default: Option<bool>,
    /// Optional. Type of control settings to retrieve. Not used when `controlId` is provided.
    pub control_type: Option<DERControlEnumType>,
    /// Optional. Id of setting to get. When omitted all settings for `controlType` are retrieved.
    pub control_id: Option<String>,
}

impl OcppEntity for GetDERControlRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // Assuming request_id is a non-negative integer for IDs
        b.check_bounds("request_id", 0, i32::MAX, self.request_id);

        if let Some(control_id) = &self.control_id {
            b.check_cardinality("control_id", 0, 36, &control_id.chars());
        }

        b.build("GetDERControlRequest")
    }
}

/// 1.29.2. GetDERControlResponse
/// This contains the field definition of the GetDERControlResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetDERControlResponse {
    /// Required. Result of operation.
    pub status: DERControlStatusEnumType,
    /// Optional. Detailed status info.
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for GetDERControlResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetDERControlResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetDERControl::request();
        let resp = GetDERControl::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetDERControlRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetDERControlRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetDERControlResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetDERControlResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetDERControl::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetDERControl::response().validate().is_ok());
    }
}