use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use crate::enums::generic_status_enum_type::GenericStatusEnumType;
use crate::structures::periodic_event_stream_params_type::PeriodicEventStreamParamsType;
use crate::structures::status_info_type::StatusInfoType;

/// Represents the `AdjustPeriodicEventStream` message, containing its request and response types.
pub struct AdjustPeriodicEventStream;

impl OcppMessage for AdjustPeriodicEventStream {
    type Request = AdjustPeriodicEventStreamRequestBuilder;
    type Response = AdjustPeriodicEventStreamResponseBuilder;
}

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

/// A builder for `AdjustPeriodicEventStreamRequest`.
#[derive(Default)]
pub struct AdjustPeriodicEventStreamRequestBuilder {
    id: i32,
    params: Option<PeriodicEventStreamParamsType>,
}

impl AdjustPeriodicEventStreamRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn params(&mut self, params: PeriodicEventStreamParamsType) -> &mut Self {
        self.params = Some(params);
        self
    }

    pub fn build(&self) -> Result<AdjustPeriodicEventStreamRequest, OcppError> {
        let request = AdjustPeriodicEventStreamRequest::new(
            self.id,
            self.params.clone().unwrap_or_default(),
        );

        request.validate()?;
        Ok(request)
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

/// A builder for `AdjustPeriodicEventStreamResponse`.
#[derive(Default)]
pub struct AdjustPeriodicEventStreamResponseBuilder {
    status: Option<GenericStatusEnumType>,
    status_info: Option<StatusInfoType>,
}

impl AdjustPeriodicEventStreamResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(&mut self, status: GenericStatusEnumType) -> &mut Self {
        self.status = Some(status);
        self
    }

    pub fn status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
        self
    }

    pub fn build(&self) -> Result<AdjustPeriodicEventStreamResponse, OcppError> {
        let response = AdjustPeriodicEventStreamResponse::new(
            self.status.clone().unwrap_or_default(),
            self.status_info.clone(),
        );

        response.validate()?;
        Ok(response)
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
        let resp = AdjustPeriodicEventStreamResponse::new(GenericStatusEnumType::Accepted, Default::default());
        assert!(resp.validate().is_ok());
    }

    #[test]
    fn test_request_invalid_id() {
        let req = AdjustPeriodicEventStreamRequest::new(-1, Default::default());
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_response_invalid_member() {
        let resp = AdjustPeriodicEventStreamResponse::new(GenericStatusEnumType::Accepted, Some(StatusInfoType {reason_code: "a".repeat(21), additional_info: None }));
        assert!(resp.validate().is_err());
    }

    #[test]
    fn test_builder_ok() {
        let  builder = AdjustPeriodicEventStream::response();
        let  builder = AdjustPeriodicEventStream::request();
    }

    #[test]
    fn test_request_builder_ok() {
        let mut builder = AdjustPeriodicEventStream::request();
        builder.id(1).params(Default::default());
        assert!(builder.build().is_ok());
    }

    #[test]
    fn test_request_builder_invalid_params() {
        let mut builder = AdjustPeriodicEventStream::request();
        builder.id(1).params(PeriodicEventStreamParamsType {interval: Some(-1), ..Default::default()});
        assert!(builder.build().is_err());
    }

    #[test]
    fn test_request_builder_never_set() {
        let mut builder = AdjustPeriodicEventStream::request();
        assert!(builder.build().is_err());
    }
}