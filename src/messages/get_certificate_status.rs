use crate::enums::get_certificate_status_enum_type::GetCertificateStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::ocsp_request_data_type::OCSPRequestDataType;
use crate::structures::status_info_type::StatusInfoType;
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;

/// 1.26. GetCertificateStatus
pub struct GetCertificateStatus;

impl OcppMessage for GetCertificateStatus {
    type Request = GetCertificateStatusRequest;
    type Response = GetCertificateStatusResponse;
}

/// 1.26.1. GetCertificateStatusRequest
/// This contains the field definition of the GetCertificateStatusRequest PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusRequest {
    /// Required. Indicates the certificate of which the status is requested.
    pub ocsp_request_data: OCSPRequestDataType,
}
#[typetag::serde]
impl OcppEntity for GetCertificateStatusRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_member("ocsp_request_data", &self.ocsp_request_data);

        b.build("GetCertificateStatusRequest")
    }
}

/// 1.26.2. GetCertificateStatusResponse
/// This contains the field definition of the GetCertificateStatusResponse PDU sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusResponse {
    /// Required. This indicates whether the charging station was able to retrieve the OCSP certificate status.
    pub status: GetCertificateStatusEnumType,
    /// Optional. (2.1) OCSPResponse class as defined in IETF RFC 6960, DER encoded (as defined in IETF RFC 6960), and then base64 encoded. MAY only be omitted when status is not Accepted.
    pub ocsp_result: Option<String>,
    /// Optional. Detailed status information.
    pub status_info: Option<StatusInfoType>,
}
#[typetag::serde]
impl OcppEntity for GetCertificateStatusResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        // `status` is an enum, no validation is needed.
        if let Some(ocsp_result) = &self.ocsp_result {
            b.check_cardinality("ocsp_result", 0, 18000, &ocsp_result.chars());
        }

        if let Some(status_info) = &self.status_info {
            b.check_member("status_info", status_info);
        }

        b.build("GetCertificateStatusResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg() {
        let req = GetCertificateStatus::request();
        let resp = GetCertificateStatus::response();
    }

    #[test]
    fn test_request_serialize_deserialize() {
        let req = GetCertificateStatusRequest::default();
        let json = serde_json::to_string(&req).unwrap();
        let deserialized: GetCertificateStatusRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_response_serialize_deserialize() {
        let resp = GetCertificateStatusRequest::default();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: GetCertificateStatusRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }

    #[test]
    fn test_request_validate() {
        assert!(GetCertificateStatus::request().validate().is_ok());
    }

    #[test]
    fn test_response_validate() {
        assert!(GetCertificateStatus::response().validate().is_ok());
    }
}
