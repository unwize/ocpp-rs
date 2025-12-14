use crate::enums::cancel_reservation_status_enum_type::CancelReservationStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.6. CancelReservation
pub struct CancelReservation;

impl OcppMessage for CancelReservation {
    type Request = CancelReservationRequest;
    type Response = CancelReservationResponse;
}

/// 1.6.1. CancelReservationRequest
/// This contains the field definition of the CancelReservationRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationRequest {
    /// Required. Id of the reservation to cancel.
    pub reservation_id: i32,
}
#[typetag::serde]
impl OcppEntity for CancelReservationRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_bounds("reservation_id", 0, i32::MAX, self.reservation_id);

        b.build("CancelReservationRequest")
    }
}

/// 1.6.2. CancelReservationResponse
/// This contains the field definition of the CancelReservationResponse PDU sent by the Charging Station to the CSMS in response to a CancelReservationRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationResponse {
    /// Required. This indicates the success or failure of the canceling of a reservation by CSMS.
    pub status: CancelReservationStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for CancelReservationResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("CancelReservationResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = CancelReservation::request();
        let resp = CancelReservation::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = CancelReservationRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: CancelReservationRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = CancelReservationResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: CancelReservationResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(CancelReservation::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(CancelReservation::response().validate().is_ok());
    }
}
