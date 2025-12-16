use crate::enums::generic_device_model_status::GenericDeviceModelStatusEnumType;
use crate::enums::monitoring_criterion_enum_type::MonitoringCriterionEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_variable_type::ComponentVariableType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.34. GetMonitoringReport
pub struct GetMonitoringReport;

impl OcppMessage for GetMonitoringReport {
    type Request = GetMonitoringReportRequest;
    type Response = GetMonitoringReportResponse;
}

/// 1.34.1. GetMonitoringReportRequest
/// This contains the field definition of the GetMonitoringReportRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportRequest {
    /// Required. The Id of the request.
    pub request_id: i32,
    /// Optional. This field contains criteria for components for which a monitoring report is requested.
    pub monitoring_criteria: Option<Vec<MonitoringCriterionEnumType>>,
    /// Optional. This field specifies the components and variables for which a monitoring report is requested.
    pub component_variable: Option<Vec<ComponentVariableType>>,
}
#[typetag::serde]
impl OcppEntity for GetMonitoringReportRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(criteria) = &self.monitoring_criteria {
            // Cardinality: 0..3
            b.check_cardinality("monitoring_criteria", 0, 3, &criteria.iter());
        }

        if let Some(variables) = &self.component_variable {
            // Cardinality: 0..*
            b.check_cardinality("component_variable", 0, usize::MAX, &variables.iter());
            // Individual member validation
            b.check_iter_member("component_variable", variables.iter());
        }

        b.build("GetMonitoringReportRequest")
    }
}

impl OcppRequest for GetMonitoringReportRequest {
    fn get_message_type(&self) -> String {
        String::from("GetMonitoringReport")
    }
}

/// 1.34.2. GetMonitoringReportResponse
/// This contains the field definition of the GetMonitoringReportResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportResponse {
    /// Required. This field indicates whether the Charging Station was able to accept the request.
    pub status: GenericDeviceModelStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for GetMonitoringReportResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetMonitoringReportResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetMonitoringReport::request();
        let resp = GetMonitoringReport::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetMonitoringReportRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetMonitoringReportRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetMonitoringReportResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetMonitoringReportResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetMonitoringReport::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetMonitoringReport::response().validate().is_ok());
    }
}
