use crate::enums::clear_charging_profile_status_enum_type::ClearChargingProfileStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::clear_charging_profile_type::ClearChargingProfileType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.11. ClearChargingProfile
pub struct ClearChargingProfile;

impl OcppMessage for ClearChargingProfile {
    type Request = ClearChargingProfileRequest;
    type Response = ClearChargingProfileResponse;
}

/// 1.11.1. ClearChargingProfileRequest
/// This contains the field definition of the ClearChargingProfileRequest PDU sent by the CSMS to the Charging Station. The CSMS can use this message to clear (remove) either a specific charging profile (denoted by id) or a selection of charging profiles that match with the values of the optional `evse`, `stackLevel` and `chargingProfilePurpose` fields.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    /// Optional. The Id of the charging profile to clear.
    pub charging_profile_id: Option<i32>,
    /// Optional. Specifies the charging profile.
    pub charging_profile_criteria: Option<ClearChargingProfileType>,
}
#[typetag::serde]
impl OcppEntity for ClearChargingProfileRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(charging_profile_criteria) = &self.charging_profile_criteria {
            b.check_member("charging_profile_criteria", charging_profile_criteria);
        }

        b.build("ClearChargingProfileRequest")
    }
}

impl OcppRequest for ClearChargingProfileRequest {
    fn get_message_type(&self) -> String {
        String::from("ClearChargingProfile")
    }
}

/// 1.11.2. ClearChargingProfileResponse
/// This contains the field definition of the ClearChargingProfileResponse PDU sent by the Charging Station to the CSMS in response to a ClearChargingProfileRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileResponse {
    /// Required. Indicates if the Charging Station was able to execute the request.
    pub status: ClearChargingProfileStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for ClearChargingProfileResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("ClearChargingProfileResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ClearChargingProfile::request();
        let resp = ClearChargingProfile::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ClearChargingProfileRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ClearChargingProfileRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ClearChargingProfileResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ClearChargingProfileResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ClearChargingProfile::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ClearChargingProfile::response().validate().is_ok());
    }
}
