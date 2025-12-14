use crate::enums::certificate_signed_status_enum_type::CertificateSignedStatusEnumType;
use crate::enums::certificate_signing_use_enum_type::CertificateSigningUseEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.7. CertificateSigned
pub struct CertificateSigned;

impl OcppMessage for CertificateSigned {
    type Request = CertificateSignedRequest;
    type Response = CertificateSignedResponse;
}

/// 1.7.1. CertificateSignedRequest
/// This contains the field definition of the CertificateSignedRequest PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedRequest {
    /// Required. The signed PEM encoded X.509 certificate. This SHALL also contain the necessary sub CA certificates, when applicable. The order of the bundle follows the certificate chain, starting from the leaf certificate. The Configuration Variable `MaxCertificateChainSize` can be used to limit the maximum size of this field.
    pub certificate_chain: String,
    /// Optional. Indicates the type of the signed certificate that is returned. When omitted the certificate is used for both the 15118 connection (if implemented) and the Charging Station to CSMS connection. This field is required when a `typeOfCertificate` was included in the `SignCertificateRequest` that requested this certificate to be signed AND both the 15118 connection and the Charging Station connection are implemented.
    pub certificate_type: Option<CertificateSigningUseEnumType>,
    /// Optional. (2.1) RequestId to correlate this message with the `SignCertificateRequest`.
    pub request_id: Option<i32>,
}
#[typetag::serde]
impl OcppEntity for CertificateSignedRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_cardinality(
            "certificate_chain",
            0,
            10000,
            &self.certificate_chain.chars(),
        );

        b.build("CertificateSignedRequest")
    }
}

/// 1.7.2. CertificateSignedResponse
/// This contains the field definition of the CertificateSignedResponse PDU sent by the Charging Station to the CSMS in response to a CancelReservationRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedResponse {
    /// Required. Returns whether certificate signing has been accepted, otherwise rejected.
    pub status: CertificateSignedStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for CertificateSignedResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("CertificateSignedResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = CertificateSigned::request();
        let resp = CertificateSigned::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = CertificateSignedRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: CertificateSignedRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = CertificateSignedResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: CertificateSignedResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(CertificateSigned::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(CertificateSigned::response().validate().is_ok());
    }
}
