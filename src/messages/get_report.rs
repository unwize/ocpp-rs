use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::enums::component_criterion_enum_type::ComponentCriterionEnumType;
use crate::enums::generic_device_model_status::GenericDeviceModelStatusEnumType;
use crate::structures::component_variable_type::ComponentVariableType;
use crate::structures::status_info_type::StatusInfoType;

/// 1.36. GetReport
pub struct GetReport;

impl OcppMessage for GetReport {
    type Request = GetReportRequest;
    type Response = GetReportResponse;
}

/// 1.36.1. GetReportRequest
/// This contains the field definition of the GetReportRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
    /// Required. The Id of the request.
    pub request_id: i32,
    /// Optional. This field contains criteria for components for which a report is requested.
    pub component_criteria: Option<Vec<ComponentCriterionEnumType>>,
    /// Optional. This field specifies the components and variables for which a report is requested.
    pub component_variable: Option<Vec<ComponentVariableType>>,
}
#[typetag::serde]
impl OcppEntity for GetReportRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(criteria) = &self.component_criteria {
            // Cardinality: 0..4
            b.check_cardinality("component_criteria", 0, 4, &criteria.iter());
        }

        if let Some(variables) = &self.component_variable {
            // Cardinality: 0..*
            b.check_cardinality("component_variable", 0, usize::MAX, &variables.iter());
            // Individual member validation
            b.check_iter_member("component_variable", variables.iter());
        }

        b.build("GetReportRequest")
    }
}

/// 1.36.2. GetReportResponse
/// The response to a GetReportRequest, sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetReportResponse {
    /// Required. This field indicates whether the Charging Station was able to accept the request.
    pub status: GenericDeviceModelStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for GetReportResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetReportResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetReport::request();
        let resp = GetReport::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetReportRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetReportRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetReportResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetReportResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetReport::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetReport::response().validate().is_ok());
    }
}