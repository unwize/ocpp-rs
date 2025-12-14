use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::enums::tariff_get_status_enum_type::TariffGetStatusEnumType;
use crate::structures::status_info_type::StatusInfoType;
use crate::structures::tariff_assignment_type::TariffAssignmentType;

/// 1.37. GetTariffs
pub struct GetTariffs;

impl OcppMessage for GetTariffs {
    type Request = GetTariffsRequest;
    type Response = GetTariffsResponse;
}

/// 1.37.1. GetTariffsRequest
/// This contains the field definition of the GetTariffsRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsRequest {
    /// Required. EVSE id to get tariff from. When evseId = 0, this gets tariffs from all EVSEs.
    pub evse_id: i32,
}
#[typetag::serde]
impl OcppEntity for GetTariffsRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // evseId is integer, 0 <= val
        b.check_bounds("evse_id", 0, i32::MAX, self.evse_id);

        b.build("GetTariffsRequest")
    }
}

/// 1.37.2. GetTariffsResponse
/// The response to a GetTariffsRequest, sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsResponse {
    /// Required. Status of operation.
    pub status: TariffGetStatusEnumType,
    /// Optional. Installed default and user-specific tariffs per EVSE.
    pub tariff_assignments: Option<Vec<TariffAssignmentType>>,
    /// Optional. Details status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for GetTariffsResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();


        if let Some(assignments) = &self.tariff_assignments {
            // Cardinality: 0..*
            b.check_cardinality("tariff_assignments", 0, usize::MAX, &assignments.iter());
            // Individual member validation
            b.check_iter_member("tariff_assignments", assignments.iter());
        }

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetTariffsResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetTariffs::request();
        let resp = GetTariffs::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetTariffsRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetTariffsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetTariffsResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetTariffsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetTariffs::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetTariffs::response().validate().is_ok());
    }
}