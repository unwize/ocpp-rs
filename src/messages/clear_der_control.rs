use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::enums::der_control_enum_type::DERControlEnumType;
use crate::enums::der_control_status_enum_type::DERControlStatusEnumType;
use crate::structures::status_info_type::StatusInfoType;

/// 1.12. ClearDERControl
pub struct ClearDERControl;

impl OcppMessage for ClearDERControl {
    type Request = ClearDERControlRequest;
    type Response = ClearDERControlResponse;
}

/// 1.12.1. ClearDERControlRequest
/// This contains the field definition of the ClearDERControlRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDERControlRequest {
    /// Required. True: clearing default DER controls. False: clearing scheduled controls.
    pub is_default: bool,
    /// Optional. Name of control settings to clear. Not used when `controlId` is provided.
    pub control_type: Option<DERControlEnumType>,
    /// Optional. Id of control setting to clear. When omitted all settings for `controlType` are cleared.
    pub control_id: Option<String>,
}

impl OcppEntity for ClearDERControlRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(control_id) = &self.control_id {
            b.check_cardinality("control_id", 0, 36, &control_id.chars());
        }

        b.build("ClearDERControlRequest")
    }
}

/// 1.12.2. ClearDERControlResponse
/// This contains the field definition of the ClearDERControlResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDERControlResponse {
    /// Required. Result of operation.
    pub status: DERControlStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for ClearDERControlResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("ClearDERControlResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ClearDERControl::request();
        let resp = ClearDERControl::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ClearDERControlRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ClearDERControlRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ClearDERControlResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ClearDERControlResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ClearDERControl::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ClearDERControl::response().validate().is_ok());
    }
}