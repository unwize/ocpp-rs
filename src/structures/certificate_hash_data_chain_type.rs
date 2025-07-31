use serde::{Deserialize, Serialize};
use crate::structures::certificate_hash_data_type::CertificateHashDataType;

/// Represents a chain of certificate hash data.
/// Used by: GetInstalledCertificateIdsResponse
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateHashDataChainType {
    /// Required. Indicates the type of the requested certificate(s).
    pub certificate_type: GetCertificateIdUseEnumType, // TODO: Implement GetCertificateIdUseEnumType
    /// Required. Information to identify a certificate.
    pub certificate_hash_data: CertificateHashDataType, // TODO: Implement CertificateHashDataType
    /// Optional. Information to identify the child certificate(s).
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,
}