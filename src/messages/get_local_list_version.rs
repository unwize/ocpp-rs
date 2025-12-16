use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.32. GetLocalListVersion
pub struct GetLocalListVersion;

impl OcppMessage for GetLocalListVersion {
    type Request = GetLocalListVersionRequest;
    type Response = GetLocalListVersionResponse;
}

/// 1.32.1. GetLocalListVersionRequest
/// This contains the field definition of the GetLocalListVersionRequest PDU sent by the CSMS to the Charging Station. No fields are defined.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {}
#[typetag::serde]
impl OcppEntity for GetLocalListVersionRequest {
    fn validate(&self) -> Result<(), OcppError> {
        Ok(())
    }
}

impl OcppRequest for GetLocalListVersionRequest {
    fn get_message_type(&self) -> String {
        String::from("GetLocalListVersion")
    }
}

/// 1.32.2. GetLocalListVersionResponse
/// This contains the field definition of the GetLocalListVersionResponse PDU sent by the Charging Station to CSMS in response to a GetLocalListVersionRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    /// Required. This contains the current version number of the local authorization list in the Charging Station.
    pub version_number: i32,
}
#[typetag::serde]
impl OcppEntity for GetLocalListVersionResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // versionNumber is a required integer, assuming non-negative for a version number
        b.check_bounds("version_number", 0, i32::MAX, self.version_number);

        b.build("GetLocalListVersionResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetLocalListVersion::request();
        let resp = GetLocalListVersion::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetLocalListVersionRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetLocalListVersionRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetLocalListVersionResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetLocalListVersionResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetLocalListVersion::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetLocalListVersion::response().validate().is_ok());
    }
}
