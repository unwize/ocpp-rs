use crate::enums::generic_device_model_status::GenericDeviceModelStatusEnumType;
use crate::enums::report_base_enum_type::ReportBaseEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.24. GetBaseReport
pub struct GetBaseReport;

impl OcppMessage for GetBaseReport {
    type Request = GetBaseReportRequest;
    type Response = GetBaseReportResponse;
}

/// 1.24.1. GetBaseReportRequest
/// This contains the field definition of the GetBaseReportRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportRequest {
    /// Required. The Id of the request.
    pub request_id: i32,
    /// Required. This field specifies the report base.
    pub report_base: ReportBaseEnumType,
}
#[typetag::serde]
impl OcppEntity for GetBaseReportRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // Assuming request_id is a non-negative integer for IDs
        b.check_bounds("request_id", 0, i32::MAX, self.request_id);
        // `report_base` is an enum, no validation needed.

        b.build("GetBaseReportRequest")
    }
}

impl OcppRequest for GetBaseReportRequest {
    fn get_message_type(&self) -> String {
        String::from("GetBaseReport")
    }
}

/// 1.24.2. GetBaseReportResponse
/// This contains the field definition of the GetBaseReportResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportResponse {
    /// Required. This indicates whether the Charging Station is able to accept this request.
    pub status: GenericDeviceModelStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for GetBaseReportResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, no validation is needed.
        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetBaseReportResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetBaseReport::request();
        let resp = GetBaseReport::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetBaseReportRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetBaseReportRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetBaseReportResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetBaseReportResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetBaseReport::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetBaseReport::response().validate().is_ok());
    }
}
