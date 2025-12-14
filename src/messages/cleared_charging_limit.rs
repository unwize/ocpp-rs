use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.14. ClearedChargingLimit
pub struct ClearedChargingLimit;

impl OcppMessage for ClearedChargingLimit {
    type Request = ClearedChargingLimitRequest;
    type Response = ClearedChargingLimitResponse;
}

/// 1.14.1. ClearedChargingLimitRequest
/// This contains the field definition of the ClearedChargingLimitRequest PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitRequest {
    /// Required. Source of the charging limit. Allowed values defined in Appendix as ChargingLimitSourceEnumStringType.
    pub charging_limit_source: String,
    /// Optional. EVSE Identifier.
    pub evse_id: Option<i32>,
}
#[typetag::serde]
impl OcppEntity for ClearedChargingLimitRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality(
            "charging_limit_source",
            0,
            20,
            &self.charging_limit_source.chars(),
        );

        if let Some(evse_id) = self.evse_id {
            b.check_bounds("evse_id", 0, i32::MAX, evse_id);
        }

        b.build("ClearedChargingLimitRequest")
    }
}

/// 1.14.2. ClearedChargingLimitResponse
/// This contains the field definition of the ClearedChargingLimitResponse PDU sent by the CSMS to the Charging Station. No fields are defined.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitResponse {}
#[typetag::serde]
impl OcppEntity for ClearedChargingLimitResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let b = StructureValidationBuilder::new();
        b.build("ClearedChargingLimitResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ClearedChargingLimit::request();
        let resp = ClearedChargingLimit::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ClearedChargingLimitRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ClearedChargingLimitRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ClearedChargingLimitResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ClearedChargingLimitResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ClearedChargingLimit::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ClearedChargingLimit::response().validate().is_ok());
    }
}
