use serde::{Deserialize, Serialize};
use crate::enums::get_certificate_id_use_enum_type::GetCertificateIdUseEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::certificate_hash_data_type::CertificateHashDataType;
use crate::traits::OcppEntity;

/// Represents a chain of certificate hash data.
/// Used by: GetInstalledCertificateIdsResponse
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateHashDataChainType {
    /// Required. Indicates the type of the requested certificate(s).
    pub certificate_type: GetCertificateIdUseEnumType,
    /// Required. Information to identify a certificate.
    pub certificate_hash_data: CertificateHashDataType,
    /// Optional. Information to identify the child certificate(s).
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,
}

impl OcppEntity for CertificateHashDataChainType {
    fn validate(self: &Self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.push_member("certificate_hash_data", &self.certificate_hash_data);

        if let Some(child_certificate_hash_data) = &self.child_certificate_hash_data {
            e.push_iter_member("certificate_hash_data", child_certificate_hash_data.iter());
        }

        e.build("CertificateHashDataChainType")
    }
}