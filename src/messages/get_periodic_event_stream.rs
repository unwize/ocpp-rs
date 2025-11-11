use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::structures::constant_stream_data_type::ConstantStreamDataType;

/// 1.35. GetPeriodicEventStream
pub struct GetPeriodicEventStream;

impl OcppMessage for GetPeriodicEventStream {
    type Request = GetPeriodicEventStreamRequest;
    type Response = GetPeriodicEventStreamResponse;
}

/// 1.35.1. GetPeriodicEventStreamRequest
/// This contains the field definition of the GetPeriodicEventStreamRequest PDU sent by the CSMS to the Charging Station. No fields are defined.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetPeriodicEventStreamRequest {}

impl OcppEntity for GetPeriodicEventStreamRequest {
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

/// 1.35.2. GetPeriodicEventStreamResponse
/// This contains the field definition of the GetPeriodicEventStreamResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetPeriodicEventStreamResponse {
    /// Optional. List of constant part of streams
    pub constant_stream_data: Option<Vec<ConstantStreamDataType>>,
}

impl OcppEntity for GetPeriodicEventStreamResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(stream_data) = &self.constant_stream_data {
            // Cardinality: 0..*
            b.check_cardinality("constant_stream_data", 0, usize::MAX, &stream_data.iter());
            // Individual member validation
            b.check_iter_member("constant_stream_data", stream_data.iter());
        }

        b.build("GetPeriodicEventStreamResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetPeriodicEventStream::request();
        let resp = GetPeriodicEventStream::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetPeriodicEventStreamRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetPeriodicEventStreamRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetPeriodicEventStreamResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetPeriodicEventStreamResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetPeriodicEventStream::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetPeriodicEventStream::response().validate().is_ok());
    }
}