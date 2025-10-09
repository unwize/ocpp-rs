use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.17. ClosePeriodicEventStream
pub struct ClosePeriodicEventStream;

impl OcppMessage for ClosePeriodicEventStream {
    type Request = ClosePeriodicEventStreamRequest;
    type Response = ClosePeriodicEventStreamResponse;
}

/// 1.17.1. ClosePeriodicEventStreamRequest
/// This contains the field definition of the ClosePeriodicEventStreamRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ClosePeriodicEventStreamRequest {
    /// Required. Id of stream to close.
    pub id: i32,
}

impl OcppEntity for ClosePeriodicEventStreamRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_bounds("id", 0, i32::MAX, self.id);

        b.build("ClosePeriodicEventStreamRequest")
    }
}

/// 1.17.2. ClosePeriodicEventStreamResponse
/// This contains the field definition of the ClosePeriodicEventStreamResponse PDU sent by the Charging Station to the CSMS. No fields are defined in the visible part of the specification.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ClosePeriodicEventStreamResponse {}

impl OcppEntity for ClosePeriodicEventStreamResponse {
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ClosePeriodicEventStream::request();
        let resp = ClosePeriodicEventStream::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ClosePeriodicEventStreamRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ClosePeriodicEventStreamRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ClosePeriodicEventStreamResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ClosePeriodicEventStreamResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ClosePeriodicEventStream::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ClosePeriodicEventStream::response().validate().is_ok());
    }
}