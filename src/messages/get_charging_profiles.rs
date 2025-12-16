use crate::enums::get_charging_profile_status_enum_type::GetChargingProfileStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::charging_profile_criterion_type::ChargingProfileCriterionType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.27. GetChargingProfiles
pub struct GetChargingProfiles;

impl OcppMessage for GetChargingProfiles {
    type Request = GetChargingProfilesRequest;
    type Response = GetChargingProfilesResponse;
}

/// 1.27.1. GetChargingProfilesRequest
/// The message GetChargingProfilesRequest can be used by the CSMS to request installed charging profiles from the Charging Station. The charging profiles will then be reported by the Charging Station via ReportChargingProfilesRequest messages.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesRequest {
    /// Required. Reference identification that is to be used by the Charging Station in the ReportChargingProfilesRequest when provided.
    pub request_id: i32,
    /// Optional. For which EVSE installed charging profiles SHALL be reported. If 0, only charging profiles installed on the Charging Station itself (the grid connection) SHALL be reported. If omitted, all installed charging profiles SHALL be reported.
    pub evse_id: Option<i32>,
    /// Required. Specifies the charging profile.
    pub charging_profile: ChargingProfileCriterionType,
}
#[typetag::serde]
impl OcppEntity for GetChargingProfilesRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // Assuming request_id is a non-negative integer for IDs
        b.check_bounds("request_id", 0, i32::MAX, self.request_id);

        if let Some(evse_id) = self.evse_id {
            // "integer, 0 <= val"
            b.check_bounds("evse_id", 0, i32::MAX, evse_id);
        }

        b.check_member("charging_profile", &self.charging_profile);

        b.build("GetChargingProfilesRequest")
    }
}

impl OcppRequest for GetChargingProfilesRequest {
    fn get_message_type(&self) -> String {
        String::from("GetChargingProfiles")
    }
}

/// 1.27.2. GetChargingProfilesResponse
/// This contains the field definition of the GetChargingProfilesResponse PDU sent by the Charging Station to the CSMS in response to a GetChargingProfilesRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesResponse {
    /// Required. This indicates whether the Charging Station is able to process this request and will send ReportChargingProfilesRequest messages.
    pub status: GetChargingProfileStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for GetChargingProfilesResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, no validation is needed.
        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetChargingProfilesResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetChargingProfiles::request();
        let resp = GetChargingProfiles::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetChargingProfilesRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetChargingProfilesRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetChargingProfilesResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetChargingProfilesResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetChargingProfiles::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetChargingProfiles::response().validate().is_ok());
    }
}
