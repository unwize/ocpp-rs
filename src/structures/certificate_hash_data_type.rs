use crate::enums::hash_algorithm_enum_type::HashAlgorithmEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Represents information to identify a certificate.
/// Used by: Common::CertificateHashDataChainType, Common::CertificateStatusRequestInfoType,
/// Common::CertificateStatusType, SignCertificateRequest, DeleteCertificateRequest, CustomerInformationRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CertificateHashDataType {
    /// Required. Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType,
    /// Required. The hash of the issuer's distinguished name (DN), that must be calculated
    /// over the DER encoding of the issuer's name field in the certificate being checked.
    /// String length: 0..128
    pub issuer_name_hash: String,
    /// Required. The hash of the DER encoded public key: the value (excluding tag and length)
    /// of the subject public key field in the issuer's certificate.
    /// String length: 0..128
    pub issuer_key_hash: String,
    /// Required. The string representation of the hexadecimal value of the serial number
    /// without the prefix "0x" and without leading zeroes.
    /// String length: 0..40
    pub serial_number: String,
}

impl Default for CertificateHashDataType {
    fn default() -> CertificateHashDataType {
        Self {
            hash_algorithm: HashAlgorithmEnumType::SHA256,
            issuer_name_hash: "SomeOCPPAuthHash".to_string(),
            issuer_key_hash: "0".to_string(),
            serial_number: "0".to_string(),
        }
    }
}
#[typetag::serde]
impl OcppEntity for CertificateHashDataType {
    /// Validates the fields of CertificateHashDataType based on specified string length constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    fn validate(self: &Self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("issuer_name_hash", 0, 128, &self.issuer_name_hash.chars());
        e.check_cardinality("issuer_key_hash", 0, 128, &self.issuer_key_hash.chars());
        e.check_cardinality("serial_number", 0, 40, &self.serial_number.chars());

        e.build("CertificateHashDataType")
    }
}
