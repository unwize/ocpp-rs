use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::{OcppEntity, OcppMessage};
use serde::{Deserialize, Serialize};
use std::default::Default;
use crate::enums::authorize_certificate_status_enum_type::AuthorizeCertificateStatusEnumType;
use crate::enums::energy_transfer_mode_enum_type::EnergyTransferModeEnumType;
use crate::structures::id_token_info_type::IdTokenInfoType;
use crate::structures::id_token_type::IdTokenType;
use crate::structures::ocsp_request_data_type::OCSPRequestDataType;
use crate::structures::tariff_type::TariffType;

/// 1.3. Authorize
pub struct Authorize;

impl OcppMessage for Authorize {
    type Request = AuthorizeRequest;
    type Response = AuthorizeResponse;
}

/// 1.3.1. AuthorizeRequest
/// This contains the field definition of the AuthorizeRequest PDU sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    /// Optional. (2.1) The X.509 certificate chain presented by EV and encoded in PEM format.
    /// Order of certificates in chain is from leaf up to (but excluding) root certificate.
    /// Only needed in case of central contract validation when Charging Station cannot validate the contract certificate.
    pub certificate: Option<String>,
    /// Required. This contains the identifier that needs to be authorized.
    pub id_token: IdTokenType,
    /// Optional. (2.1) Contains the information needed to verify the EV Contract Certificate via OCSP.
    /// Not needed if certificate is provided.
    pub iso15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>,
}

impl OcppEntity for AuthorizeRequest {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        if let Some(cert) = &self.certificate {
            b.check_cardinality("certificate", 0, 10000, &cert.chars());
        }

        b.check_member("id_token", &self.id_token);

        if let Some(iso15118_certificate_hash_data) = &self.iso15118_certificate_hash_data {
            b.check_cardinality("iso15118_certificate_hash_data", 0, 4, &iso15118_certificate_hash_data.iter());
            b.check_iter_member("iso15118_certificate_hash_data", iso15118_certificate_hash_data.iter());
        }

        b.build("AuthorizeRequest")
    }
}

/// 1.3.2. AuthorizeResponse
/// This contains the field definition of the AuthorizeResponse PDU sent by the CSMS to the Charging Station in response to an
/// AuthorizeRequest.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    /// Optional. Certificate status information.
    /// - if all certificates are valid: return 'Accepted'.
    /// - if one of the certificates was revoked, return 'CertificateRevoked'.
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,
    /// Optional. (2.1) List of allowed energy transfer modes the EV can choose from. If omitted this defaults to charging only.
    pub allowed_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>,
    /// Required. This contains information about authorization status, expiry and group id.
    pub id_token_info: IdTokenInfoType,
    /// Optional. (2.1) Tariff for this IdToken.
    pub tariff: Option<TariffType>,
}

impl OcppEntity for AuthorizeResponse {
    fn validate(&self) -> Result<(), OcppError> {
        let mut b = StructureValidationBuilder::new();

        b.check_member("id_token_info", &self.id_token_info);

        if let Some(tariff) = &self.tariff {
            b.check_member("tariff", tariff);
        }

        b.build("AuthorizeResponse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorize() {
        let req = Authorize::request();
        let resp = Authorize::response();

        assert!(req.validate().is_ok());
        assert!(resp.validate().is_ok());
    }

    #[test]
    fn test_authorize_request_certificate_long() {
        let mut req = Authorize::request();
        req.certificate = Some("a".repeat(10001));
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_authorize_iso15118_certificate_hash_data_long() {
        let mut req = Authorize::request();
        req.iso15118_certificate_hash_data = Some(vec![Default::default(); 5])
    }

    #[test]
    fn test_authorize_request_serialize_deserialize() {
        let req = Authorize::request();
        let serialized = serde_json::to_string(&req).unwrap();
        let deserialized: AuthorizeRequest = serde_json::from_str(&serialized).unwrap();
        assert!(req.validate().is_ok());
        assert_eq!(req, deserialized);
    }

    #[test]
    fn test_authorize_response_serialize_deserialize() {
        let resp = Authorize::response();
        let serialized = serde_json::to_string(&resp).unwrap();
        let deserialized: AuthorizeResponse = serde_json::from_str(&serialized).unwrap();
        assert!(resp.validate().is_ok());
        assert_eq!(resp, deserialized);
    }
}