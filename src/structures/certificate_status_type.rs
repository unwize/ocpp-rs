use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::enums::certificate_status_enum_type::CertificateStatusEnumType;
use crate::enums::certificate_status_source_enum_type::CertificateStatusSourceEnumType;
use crate::errors::OcppError;
use crate::structures::certificate_hash_data_type::CertificateHashDataType;
use crate::traits::OcppEntity;

/// Revocation status of certificate
/// Used by: GetCertificateChainStatusResponse
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateStatusType {
    /// Required. Source of status: OCSP, CRL
    pub source: CertificateStatusSourceEnumType,
    /// Required. Status of certificate: good, revoked or unknown.
    pub status: CertificateStatusEnumType,
    /// Required.
    pub next_update: DateTime<Utc>,
    /// Required. Hash data of the certificate.
    pub certificate_hash_data: CertificateHashDataType
}

impl OcppEntity for CertificateStatusType {
    fn validate(self: &Self) -> Result<(), OcppError> {
        self.certificate_hash_data.validate()
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_serialization_deserialization() {
        let cert_status = CertificateStatusType {
            source: CertificateStatusSourceEnumType::Ocsp, // Placeholder
            status: CertificateStatusEnumType::Good, // Placeholder
            next_update: Utc.with_ymd_and_hms(2025, 8, 1, 0, 0, 0).unwrap(),
            certificate_hash_data: CertificateHashDataType::default(), // Placeholder
        };

        let serialized = serde_json::to_string(&cert_status).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: CertificateStatusType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cert_status, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let cert_status = CertificateStatusType {
            source: CertificateStatusSourceEnumType::Crl,
            status: CertificateStatusEnumType::Revoked,
            next_update: Utc.with_ymd_and_hms(2025, 9, 1,0, 0, 0).unwrap(),
            certificate_hash_data: CertificateHashDataType::default(),
        };
        assert!(cert_status.validate().is_ok());
    }
}
