use crate::enums::clear_cache_status_enum_type::ClearCacheStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.10. ClearCache
pub struct ClearCache;

impl OcppMessage for ClearCache {
    type Request = ClearCacheRequest;
    type Response = ClearCacheResponse;
}

/// 1.10.1. ClearCacheRequest
/// This contains the field definition of the ClearCacheRequest PDU sent by the CSMS to the Charging Station. No fields are defined.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheRequest {}
#[typetag::serde]
impl OcppEntity for ClearCacheRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let b = StructureValidationBuilder::new();
        b.build("ClearCacheRequest")
    }
}

#[typetag::serde]
impl OcppRequest for ClearCacheRequest {
    fn get_message_type(&self) -> String {
        String::from("ClearCache")
    }
}

/// 1.10.2. ClearCacheResponse
/// This contains the field definition of the ClearCacheResponse PDU sent by the Charging Station to the CSMS in response to a ClearCacheRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCacheResponse {
    /// Required. Accepted if the Charging Station has executed the request, otherwise rejected.
    pub status: ClearCacheStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for ClearCacheResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("ClearCacheResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = ClearCache::request();
        let resp = ClearCache::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = ClearCacheRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: ClearCacheRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = ClearCacheResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: ClearCacheResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(ClearCache::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(ClearCache::response().validate().is_ok());
    }
}
