use crate::enums::log_enum_type::LogEnumType;
use crate::enums::log_status_enum_type::LogStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::log_parameters_type::LogParametersType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.33. GetLog
pub struct GetLog;

impl OcppMessage for GetLog {
    type Request = GetLogRequest;
    type Response = GetLogResponse;
}

/// 1.33.1. GetLogRequest
/// This contains the field definition of the GetLogRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRequest {
    /// Required. This contains the type of log file that the Charging Station should send.
    pub log_type: LogEnumType,
    /// Required. The Id of this request.
    pub request_id: i32,
    /// Optional. This specifies how many times the Charging Station must retry to upload the log before giving up. If this field is not present, it is left to Charging Station to decide how many times it wants to retry. If the value is 0, it means: no retries.
    pub retries: Option<i32>,
    /// Optional. The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    pub retry_interval: Option<i32>,
    /// Required. This field specifies the requested log and the location to which the log should be sent.
    pub log: LogParametersType,
}
#[typetag::serde]
impl OcppEntity for GetLogRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(retries) = self.retries {
            // retries is integer, 0 <= val
            b.check_bounds("retries", 0, i32::MAX, retries);
        }

        if let Some(retry_interval) = self.retry_interval {
            // retryInterval is integer, assuming non-negative for a time interval
            b.check_bounds("retry_interval", 0, i32::MAX, retry_interval);
        }

        b.check_member("log", &self.log);

        b.build("GetLogRequest")
    }
}

/// 1.33.2. GetLogResponse
/// This contains the field definition of the GetLogResponse PDU sent by the Charging Station to the CSMS in response to a GetLogRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLogResponse {
    /// Required. This field indicates whether the Charging Station was able to accept the request.
    pub status: LogStatusEnumType,
    /// Optional. This contains the name of the log file that will be uploaded. This field is not present when no logging information is available.
    pub filename: Option<String>,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for GetLogResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(filename) = &self.filename {
            b.check_cardinality("filename", 0, 255, &filename.chars());
        }

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetLogResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetLog::request();
        let resp = GetLog::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetLogRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetLogRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetLogResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetLogResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetLog::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetLog::response().validate().is_ok());
    }
}
