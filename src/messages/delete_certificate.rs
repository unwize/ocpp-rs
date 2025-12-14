use crate::enums::delete_certificate_status_enum_type::DeleteCertificateStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::certificate_hash_data_type::CertificateHashDataType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.21. DeleteCertificate
pub struct DeleteCertificate;

impl OcppMessage for DeleteCertificate {
    type Request = DeleteCertificateRequest;
    type Response = DeleteCertificateResponse;
}

/// 1.21.1. DeleteCertificateRequest
/// This contains the field definition of the DeleteCertificateRequest PDU sent by the CSMS to the Charging Station to request deletion of an installed certificate.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateRequest {
    /// Required. Indicates the certificate of which deletion is requested.
    pub certificate_hash_data: CertificateHashDataType,
}
#[typetag::serde]
impl OcppEntity for DeleteCertificateRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_member("certificate_hash_data", &self.certificate_hash_data);

        b.build("DeleteCertificateRequest")
    }
}

/// 1.21.2. DeleteCertificateResponse
/// This contains the field definition of the DeleteCertificateResponse PDU sent by the Charging Station to the CSMS in response to a DeleteCertificateRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateResponse {
    /// Required. Charging Station indicates if it can process the request.
    pub status: DeleteCertificateStatusEnumType,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for DeleteCertificateResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, no validation is needed.
        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("DeleteCertificateResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = DeleteCertificate::request();
        let resp = DeleteCertificate::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = DeleteCertificateRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: DeleteCertificateRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = DeleteCertificateResponse::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: DeleteCertificateResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(DeleteCertificate::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(DeleteCertificate::response().validate().is_ok());
    }
}
