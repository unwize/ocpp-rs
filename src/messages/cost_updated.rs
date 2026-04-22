use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.18.1. CostUpdatedRequest
/// This contains the field definition of the CostUpdatedRequest PDU sent by the CSMS to the Charging Station. With this request the CSMS can send the current cost of a transaction to a Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedRequest {
    /// Required. Current total cost, based on the information known by the CSMS, of the transaction including taxes. In the currency configured with the configuration Variable: [Currency]
    pub total_cost: f64,
    /// Required. Transaction Id of the transaction the current cost are asked for.
    pub transaction_id: String,
}

impl OcppEntity for CostUpdatedRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality("transaction_id", 0, 36, &self.transaction_id.chars());

        b.build("CostUpdatedRequest")
    }
}

impl OcppRequest for CostUpdatedRequest {
    const NAME: &'static str = "CostUpdated";
    type ResponseType = CostUpdatedResponse;
}

/// 1.18.2. CostUpdatedResponse
/// This contains the field definition of the CostUpdatedResponse PDU sent by the Charging Station to the CSMS in response to CostUpdatedRequest. No fields are defined.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedResponse {}

impl OcppEntity for CostUpdatedResponse {
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = CostUpdatedRequest::default();
        let resp = CostUpdatedResponse::default();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = CostUpdatedRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: CostUpdatedRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = CostUpdatedResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: CostUpdatedResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(CostUpdatedRequest::default().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(CostUpdatedResponse::default().validate().is_ok());
    }
}
