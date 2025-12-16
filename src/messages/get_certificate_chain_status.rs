use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::certificate_status_request_info_type::CertificateStatusRequestInfoType;
use crate::structures::certificate_status_type::CertificateStatusType;
use crate::traits::{OcppEntity, OcppMessage, OcppRequest};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.25. GetCertificateChainStatus
pub struct GetCertificateChainStatus;

impl OcppMessage for GetCertificateChainStatus {
    type Request = GetCertificateChainStatusRequest;
    type Response = GetCertificateChainStatusResponse;
}

/// 1.25.1. GetCertificateChainStatusRequest
/// This contains the field definition of the GetCertificateChainStatusRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusRequest {
    /// Required. Certificate to check revocation status for.
    pub certificate_status_requests: Vec<CertificateStatusRequestInfoType>,
}
#[typetag::serde]
impl OcppEntity for GetCertificateChainStatusRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality(
            "certificate_status_requests",
            1,
            4,
            &self.certificate_status_requests.iter(),
        );
        b.check_iter_member(
            "certificate_status_requests",
            self.certificate_status_requests.iter(),
        );

        b.build("GetCertificateChainStatusRequest")
    }
}

#[typetag::serde]
impl OcppRequest for GetCertificateChainStatusRequest {
    fn get_message_type(&self) -> String {
        String::from("GetCertificateChainStatus")
    }
}

/// 1.25.2. GetCertificateChainStatusResponse
/// This contains the field definition of the GetCertificateChainStatusResponse PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusResponse {
    /// Required. Status of the certificate revocation check.
    pub certificate_status: Vec<CertificateStatusType>,
}
#[typetag::serde]
impl OcppEntity for GetCertificateChainStatusResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality("certificate_status", 1, 4, &self.certificate_status.iter());
        b.check_iter_member("certificate_status", self.certificate_status.iter());

        b.build("GetCertificateChainStatusResponse")
    }
}

impl Default for GetCertificateChainStatusResponse {
    fn default() -> GetCertificateChainStatusResponse {
        Self {
            certificate_status: vec![Default::default()],
        }
    }
}

impl Default for GetCertificateChainStatusRequest {
    fn default() -> GetCertificateChainStatusRequest {
        Self {
            certificate_status_requests: vec![Default::default()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetCertificateChainStatus::request();
        let resp = GetCertificateChainStatus::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetCertificateChainStatusRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetCertificateChainStatusRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetCertificateChainStatusResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetCertificateChainStatusResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetCertificateChainStatus::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetCertificateChainStatus::response().validate().is_ok());
    }
}
