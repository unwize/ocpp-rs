use crate::enums::certificate_status_source_enum_type::CertificateStatusSourceEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::certificate_hash_data_type::CertificateHashDataType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Data necessary to request the revocation status of a certificate.
/// Used by: GetCertificateChainStatusRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CertificateStatusRequestInfoType {
    /// Required. Source of status: OCSP, CRL
    pub source: CertificateStatusSourceEnumType,
    /// Required. URL(s) of source.
    /// String length: 0..2000
    /// Cardinality 1..5, so represented as a Vec.
    pub urls: Vec<String>,
    /// Required. Hash data of certificate.
    pub certificate_hash_data: CertificateHashDataType,
}

impl Default for CertificateStatusRequestInfoType {
    fn default() -> CertificateStatusRequestInfoType {
        Self {
            source: CertificateStatusSourceEnumType::Crl,
            urls: vec![Default::default()],
            certificate_hash_data: Default::default(),
        }
    }
}
#[typetag::serde]
impl OcppEntity for CertificateStatusRequestInfoType {
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("urls", 1, 5, &self.urls.iter());

        // Manually check cardinality of strings in the `urls` vector as push_iter_member only works
        // for `impl OcppEntity`.
        for i in 0..self.urls.len() {
            e.check_cardinality(format!("urls[{i}").as_str(), 0, 2000, &self.urls[i].chars());
        }

        e.check_member("certificate_hash_data", &self.certificate_hash_data);
        e.build("CertificateRequestInfoType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let cert_status_req = CertificateStatusRequestInfoType {
            source: CertificateStatusSourceEnumType::Ocsp, // Placeholder
            urls: vec![
                "http://example.com/ocsp1".to_string(),
                "http://example.com/ocsp2".to_string(),
            ],
            certificate_hash_data: CertificateHashDataType::default(), // Placeholder
        };

        let serialized = serde_json::to_string(&cert_status_req).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: CertificateStatusRequestInfoType =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(cert_status_req, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let cert_status_req = CertificateStatusRequestInfoType {
            source: CertificateStatusSourceEnumType::Crl,
            urls: vec!["http://example.com/crl".to_string()],
            certificate_hash_data: CertificateHashDataType::default(),
        };
        assert!(cert_status_req.validate().is_ok());

        let cert_status_req_max_urls = CertificateStatusRequestInfoType {
            source: CertificateStatusSourceEnumType::Ocsp,
            urls: vec![
                "a".repeat(2000),
                "b".repeat(2000),
                "c".repeat(2000),
                "d".repeat(2000),
                "e".repeat(2000),
            ],
            certificate_hash_data: CertificateHashDataType::default(),
        };
        assert!(cert_status_req_max_urls.validate().is_ok());
    }

    #[test]
    fn test_validation_no_urls() {
        let cert_status_req = CertificateStatusRequestInfoType {
            source: CertificateStatusSourceEnumType::Ocsp,
            urls: vec![], // No URLs
            certificate_hash_data: CertificateHashDataType::default(),
        };
        assert!(cert_status_req.validate().is_err());
    }

    #[test]
    fn test_validation_too_many_urls() {
        let cert_status_req = CertificateStatusRequestInfoType {
            source: CertificateStatusSourceEnumType::Ocsp,
            urls: vec![
                "u1".to_string(),
                "u2".to_string(),
                "u3".to_string(),
                "u4".to_string(),
                "u5".to_string(),
                "u6".to_string(), // Too many
            ],
            certificate_hash_data: CertificateHashDataType::default(),
        };
        assert!(cert_status_req.validate().is_err());
    }

    #[test]
    fn test_validation_url_too_long() {
        let cert_status_req = CertificateStatusRequestInfoType {
            source: CertificateStatusSourceEnumType::Ocsp,
            urls: vec!["a".repeat(2001)], // URL too long
            certificate_hash_data: CertificateHashDataType::default(),
        };
        assert!(cert_status_req.validate().is_err());
    }

    #[test]
    fn test_validation_default() {
        assert!(
            CertificateStatusRequestInfoType::default()
                .validate()
                .is_ok()
        );
    }
}
