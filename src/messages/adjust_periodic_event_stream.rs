use crate::enums::generic_status_enum_type::GenericStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::periodic_event_stream_params_type::PeriodicEventStreamParamsType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Represents the `AdjustPeriodicEventStreamRequest` message.
///
/// This message is used by the CSMS to tell the Charging Station that it wants to change the reporting
/// rate of a Periodic Event Stream.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AdjustPeriodicEventStreamRequest {
    /// The ID of the event stream to be adjusted.
    pub id: i32,
    /// Updated rate of sending data.
    pub params: PeriodicEventStreamParamsType,
}

impl AdjustPeriodicEventStreamRequest {
    /// Creates a new `AdjustPeriodicEventStreamRequest`.
    pub fn new(id: i32, params: PeriodicEventStreamParamsType) -> Self {
        Self { id, params }
    }
}

impl OcppEntity for AdjustPeriodicEventStreamRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut builder = StructureValidationBuilder::new();

        builder.check_bounds("id", 0, i32::MAX, self.id);
        builder.check_member("params", &self.params);

        builder.build("AdjustPeriodicEventStreamRequest")
    }
}

/// Represents the `AdjustPeriodicEventStreamResponse` message.
///
/// This message is sent by the Charging Station to the CSMS to indicate the success or failure of the
/// `AdjustPeriodicEventStreamRequest`.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AdjustPeriodicEventStreamResponse {
    /// The status of the operation.
    pub status: GenericStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl AdjustPeriodicEventStreamResponse {
    /// Creates a new `AdjustPeriodicEventStreamResponse`.
    pub fn new(status: GenericStatusEnumType, status_info: Option<StatusInfoType>) -> Self {
        Self {
            status,
            status_info,
        }
    }
}

impl OcppEntity for AdjustPeriodicEventStreamResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut builder = StructureValidationBuilder::new();

        if let Some(si) = &self.status_info {
            builder.check_member("statusInfo", si);
        }

        builder.build("AdjustPeriodicEventStreamResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_response_default_ok() {
        let resp = AdjustPeriodicEventStreamResponse::default().validate();
        assert!(resp.is_ok());
    }

    #[test]
    fn test_request_default_ok() {
        let req = AdjustPeriodicEventStreamRequest::default().validate();
        assert!(req.is_ok());
    }

    #[test]
    fn test_request_new_ok() {
        let req = AdjustPeriodicEventStreamRequest::new(1, Default::default());
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_response_new_ok() {
        let resp = AdjustPeriodicEventStreamResponse::new(
            GenericStatusEnumType::Accepted,
            Default::default(),
        );
        assert!(resp.validate().is_ok());
    }

    #[test]
    fn test_request_invalid_id() {
        let req = AdjustPeriodicEventStreamRequest::new(-1, Default::default());
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_response_invalid_member() {
        let resp = AdjustPeriodicEventStreamResponse::new(
            GenericStatusEnumType::Accepted,
            Some(StatusInfoType {
                reason_code: "a".repeat(21),
                additional_info: None,
            }),
        );
        assert!(resp.validate().is_err());
    }
}
