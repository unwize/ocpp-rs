use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::clear_tarrifs_result_type::ClearTariffsResultType;
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.15. ClearTariffs
pub struct ClearTariffs;

impl OcppMessage for ClearTariffs {
    type Request = ClearTariffsRequest;
    type Response = ClearTariffsResponse;
}

/// 1.15.1. ClearTariffsRequest
/// This contains the field definition of the ClearTariffsRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsRequest {
    /// Optional. List of tariff Ids to clear. When absent clears all tariffs at `evseId`.
    pub tariff_ids: Option<Vec<String>>,
    /// Optional. When present only clear tariffs matching `tariffIds` at EVSE `evseId`.
    pub evse_id: Option<i32>,
}
#[typetag::serde]
impl OcppEntity for ClearTariffsRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(tariff_ids) = &self.tariff_ids {
            for id in tariff_ids {
                b.check_cardinality("tariff_ids", 0, 60, &id.chars());
            }
        }

        if let Some(evse_id) = self.evse_id {
            b.check_bounds("evse_id", 0, i32::MAX, evse_id);
        }

        b.build("ClearTariffsRequest")
    }
}

/// 1.15.2. ClearTariffsResponse
/// This contains the field definition of the ClearTariffsResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsResponse {
    /// Required. Result per tariff.
    pub clear_tariffs_result: Vec<ClearTariffsResultType>,
}
#[typetag::serde]
impl OcppEntity for ClearTariffsResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality(
            "clear_tariffs_result",
            1,
            usize::MAX,
            &self.clear_tariffs_result.iter(),
        );
        b.check_iter_member("clear_tariffs_result", self.clear_tariffs_result.iter());

        b.build("ClearTariffsResponse")
    }
}

impl Default for ClearTariffsResponse {
    fn default() -> Self {
        Self {
            clear_tariffs_result: vec![Default::default()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ClearTariffs::request();
        let resp = ClearTariffs::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ClearTariffsRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ClearTariffsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ClearTariffsResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ClearTariffsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ClearTariffs::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ClearTariffs::response().validate().is_ok());
    }
}
