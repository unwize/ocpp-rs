use crate::enums::change_availability_status_enum_type::ChangeAvailabilityStatusEnumType;
use crate::enums::operational_status_enum_type::OperationalStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::evse_type::EVSEType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.8. ChangeAvailability
pub struct ChangeAvailability;

impl OcppMessage for ChangeAvailability {
    type Request = ChangeAvailabilityRequest;
    type Response = ChangeAvailabilityResponse;
}

/// 1.8.1. ChangeAvailabilityRequest
/// This contains the field definition of the ChangeAvailabilityRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    /// Required. This contains the type of availability change that the Charging Station should perform.
    pub operational_status: OperationalStatusEnumType,
    /// Optional. Contains Id's to designate a specific EVSE/connector by index numbers. When omitted, the message refers to the Charging Station as a whole.
    pub evse: Option<EVSEType>,
}

impl OcppEntity for ChangeAvailabilityRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `operational_status` is an enum, so no validation is needed.
        if let Some(evse) = &self.evse {
            b.check_member("evse", evse);
        }

        b.build("ChangeAvailabilityRequest")
    }
}

/// 1.8.2. ChangeAvailabilityResponse
/// This contains the field definition of the ChangeAvailabilityResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    /// Required. This indicates whether the Charging Station is able to perform the availability change.
    pub status: ChangeAvailabilityStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}

impl OcppEntity for ChangeAvailabilityResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, so no validation is needed.
        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("ChangeAvailabilityResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ChangeAvailability::request();
        let resp = ChangeAvailability::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ChangeAvailabilityRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ChangeAvailabilityRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ChangeAvailabilityResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ChangeAvailabilityResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ChangeAvailability::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ChangeAvailability::response().validate().is_ok());
    }
}
