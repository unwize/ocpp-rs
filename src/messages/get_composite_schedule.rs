use crate::enums::charging_rate_unit_enum_type::ChargingRateUnitEnumType;
use crate::enums::generic_status_enum_type::GenericStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::composite_schedule_type::CompositeScheduleType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.28. GetCompositeSchedule
pub struct GetCompositeSchedule;

impl OcppMessage for GetCompositeSchedule {
    type Request = GetCompositeScheduleRequest;
    type Response = GetCompositeScheduleResponse;
}

/// 1.28.1. GetCompositeScheduleRequest
/// This contains the field definition of the GetCompositeScheduleRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    /// Required. Length of the requested schedule in seconds.
    pub duration: i32,
    /// Optional. Can be used to force a power or current profile.
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,
    /// Required. The ID of the EVSE for which the schedule is requested. When evseId=0, the Charging Station will calculate the expected consumption for the grid connection.
    pub evse_id: i32,
}
#[typetag::serde]
impl OcppEntity for GetCompositeScheduleRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // Assuming duration is a non-negative integer for seconds.
        b.check_bounds("duration", 0, i32::MAX, self.duration);
        // `charging_rate_unit` is an enum, no validation needed.

        // "integer, 0 <= val"
        b.check_bounds("evse_id", 0, i32::MAX, self.evse_id);

        b.build("GetCompositeScheduleRequest")
    }
}

#[typetag::serde]

impl OcppRequest for GetCompositeScheduleRequest {
    fn get_message_type(&self) -> String {
        String::from("GetCompositeSchedule")
    }
}

/// 1.28.2. GetCompositeScheduleResponse
/// This contains the field definition of the GetCompositeScheduleResponse PDU sent by the Charging Station to the CSMS in response to a GetCompositeScheduleRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse {
    /// Required. The Charging Station will indicate if it was able to process the request.
    pub status: GenericStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
    /// Optional. This field contains the calculated composite schedule. It may only be omitted when this message contains status Rejected.
    pub schedule: Option<CompositeScheduleType>,
}
#[typetag::serde]
impl OcppEntity for GetCompositeScheduleResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, no validation is needed.
        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }
        if let Some(schedule) = &self.schedule {
            b.check_member("schedule", schedule);
        }

        b.build("GetCompositeScheduleResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetCompositeSchedule::request();
        let resp = GetCompositeSchedule::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetCompositeScheduleRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetCompositeScheduleRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetCompositeScheduleResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetCompositeScheduleResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetCompositeSchedule::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetCompositeSchedule::response().validate().is_ok());
    }
}
