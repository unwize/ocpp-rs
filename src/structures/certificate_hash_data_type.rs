use serde::{Deserialize, Serialize};

/// Represents information to identify a certificate.
/// Used by: Common::CertificateHashDataChainType, Common::CertificateStatusRequestInfoType,
/// Common::CertificateStatusType, SignCertificateRequest, DeleteCertificateRequest, CustomerInformationRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateHashDataType {
    /// Required. Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType, // TODO: Implement HashAlgorithmEnumType
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

impl CertificateHashDataType {
    /// Validates the fields of CertificateHashDataType based on specified string length constraints.
    /// Returns `true` if all values are valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // No specific validation for hash_algorithm without its enum definition.

        // Validate issuer_name_hash length
        if self.issuer_name_hash.len() > 128 {
            // println!("Validation failed: issuer_name_hash length exceeds 128.");
            return false;
        }

        // Validate issuer_key_hash length
        if self.issuer_key_hash.len() > 128 {
            // println!("Validation failed: issuer_key_hash length exceeds 128.");
            return false;
        }

        // Validate serial_number length
        if self.serial_number.len() > 40 {
            // println!("Validation failed: serial_number length exceeds 40.");
            return false;
        }

        true
    }
}