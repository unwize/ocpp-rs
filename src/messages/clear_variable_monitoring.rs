use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::structures::clear_monitoring_result_type::ClearMonitoringResultType;

/// 1.16. ClearVariableMonitoring
pub struct ClearVariableMonitoring;

impl OcppMessage for ClearVariableMonitoring {
    type Request = ClearVariableMonitoringRequest;
    type Response = ClearVariableMonitoringResponse;
}

/// 1.16.1. ClearVariableMonitoringRequest
/// This contains the field definition of the ClearVariableMonitoringRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringRequest {
    /// Required. List of the monitors to be cleared, identified by their Id.
    pub id: Vec<i32>,
}

impl OcppEntity for ClearVariableMonitoringRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality("id", 1, usize::MAX, &self.id.iter());
        for &id in &self.id {
            b.check_bounds("id", 0, i32::MAX, id);
        }

        b.build("ClearVariableMonitoringRequest")
    }
}

impl Default for ClearVariableMonitoringRequest {
    fn default() -> ClearVariableMonitoringRequest {
        Self {
            id: vec![0],
        }
    }
}

/// 1.16.2. ClearVariableMonitoringResponse
/// This contains the field definition of the ClearVariableMonitoringResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearVariableMonitoringResponse {
    /// Required. List of status per monitor.
    pub clear_monitoring_result: Vec<ClearMonitoringResultType>,
}

impl OcppEntity for ClearVariableMonitoringResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality("clear_monitoring_result", 1, usize::MAX, &self.clear_monitoring_result.iter());
        b.check_iter_member("clear_monitoring_result", self.clear_monitoring_result.iter());

        b.build("ClearVariableMonitoringResponse")
    }
}

impl Default for ClearVariableMonitoringResponse {
    fn default() -> Self {
        Self {
            clear_monitoring_result: vec![Default::default()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ClearVariableMonitoring::request();
        let resp = ClearVariableMonitoring::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ClearVariableMonitoringRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ClearVariableMonitoringRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ClearVariableMonitoringResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ClearVariableMonitoringResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ClearVariableMonitoring::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ClearVariableMonitoring::response().validate().is_ok());
    }
}