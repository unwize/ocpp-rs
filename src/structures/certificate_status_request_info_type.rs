use serde::{Deserialize, Serialize};
use crate::enums::certificate_status_source_enum_type::CertificateStatusSourceEnumType;
use crate::structures::certificate_hash_data_type::CertificateHashDataType;

/// Data necessary to request the revocation status of a certificate.
/// Used by: GetCertificateChainStatusRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateStatusRequestInfoType {
    /// Required. Source of status: OCSP, CRL
    pub source: CertificateStatusSourceEnumType,
    /// Required. URL(s) of source.
    /// String length: 0..2000
    /// Cardinality 1..5, so represented as a Vec.
    pub urls: Vec<String>,
    /// Required. Hash data of certificate.
    pub certificate_hash_data: CertificateHashDataType
}

impl CertificateStatusRequestInfoType {
    /// Validates the fields of CertificateStatusRequestInfoType based on specified constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // No specific validation for 'source' or 'certificate_hash_data' without their enum/struct definitions.

        // Validate urls cardinality (1 to 5) and string length
        if self.urls.is_empty() || self.urls.len() > 5 {
            // println!("Validation failed: urls must contain between 1 and 5 elements.");
            return false;
        }
        for url in &self.urls {
            if url.len() > 2000 {
                // println!("Validation failed: URL length exceeds 2000.");
                return false;
            }
        }

        true
    }
}

// Example usage (optional, for demonstration)
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

        let deserialized: CertificateStatusRequestInfoType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cert_status_req, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let cert_status_req = CertificateStatusRequestInfoType {
            source: CertificateStatusSourceEnumType::Crl,
            urls: vec!["http://example.com/crl".to_string()],
            certificate_hash_data: CertificateHashDataType::default(),
        };
        assert!(cert_status_req.validate());

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
        assert!(cert_status_req_max_urls.validate());
    }

    #[test]
    fn test_validation_no_urls() {
        let cert_status_req = CertificateStatusRequestInfoType {
            source: CertificateStatusSourceEnumType::Ocsp,
            urls: vec![], // No URLs
            certificate_hash_data: CertificateHashDataType::default(),
        };
        assert!(!cert_status_req.validate());
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
        assert!(!cert_status_req.validate());
    }

    #[test]
    fn test_validation_url_too_long() {
        let cert_status_req = CertificateStatusRequestInfoType {
            source: CertificateStatusSourceEnumType::Ocsp,
            urls: vec!["a".repeat(2001)], // URL too long
            certificate_hash_data: CertificateHashDataType::default(),
        };
        assert!(!cert_status_req.validate());
    }
}
