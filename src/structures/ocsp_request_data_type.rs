use crate::enums::hash_algorithm_enum_type::HashAlgorithmEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Information about a certificate for an OCSP check.
/// Used by: AuthorizeRequest, GetCertificateStatusRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    /// Required. Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType,
    /// Required. The hash of the issuer's distinguished name (DN).
    pub issuer_name_hash: String,
    /// Required. The hash of the DER encoded public key.
    pub issuer_key_hash: String,
    /// Required. The string representation of the hexadecimal value of the serial number.
    pub serial_number: String,
    /// Required. This contains the responder URL.
    pub responder_url: String,
}

impl Default for OCSPRequestDataType {
    fn default() -> OCSPRequestDataType {
        Self {
            hash_algorithm: HashAlgorithmEnumType::SHA256,
            issuer_name_hash: "".to_string(),
            issuer_key_hash: "".to_string(),
            serial_number: "".to_string(),
            responder_url: "".to_string(),
        }
    }
}

impl OcppEntity for OCSPRequestDataType {
    /// Validates the fields of OCSPRequestDataType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("issuer_name_hash", 0, 128, &self.issuer_name_hash.chars());
        e.check_cardinality("issuer_key_hash", 0, 128, &self.issuer_key_hash.chars());
        e.check_cardinality("serial_number", 0, 40, &self.serial_number.chars());
        e.check_cardinality("responder_url", 0, 2000, &self.responder_url.chars());

        e.build("OCSPRequestDataType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let ocsp_request = OCSPRequestDataType {
            hash_algorithm: HashAlgorithmEnumType::SHA256,
            issuer_name_hash: "a".repeat(128),
            issuer_key_hash: "b".repeat(128),
            serial_number: "c".repeat(40),
            responder_url: "d".repeat(2000),
        };
        assert!(ocsp_request.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_issuer_name_hash_length() {
        let ocsp_request = OCSPRequestDataType {
            hash_algorithm: HashAlgorithmEnumType::SHA256,
            issuer_name_hash: "a".repeat(129), // Invalid length
            issuer_key_hash: "b".repeat(10),
            serial_number: "c".repeat(10),
            responder_url: "d".repeat(10),
        };
        let result = ocsp_request.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "issuer_name_hash");
            } else {
                panic!("Expected FieldValidationError for 'issuer_name_hash'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_issuer_key_hash_length() {
        let ocsp_request = OCSPRequestDataType {
            hash_algorithm: HashAlgorithmEnumType::SHA256,
            issuer_name_hash: "a".repeat(10),
            issuer_key_hash: "b".repeat(129), // Invalid length
            serial_number: "c".repeat(10),
            responder_url: "d".repeat(10),
        };
        let result = ocsp_request.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "issuer_key_hash");
            } else {
                panic!("Expected FieldValidationError for 'issuer_key_hash'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_serial_number_length() {
        let ocsp_request = OCSPRequestDataType {
            hash_algorithm: HashAlgorithmEnumType::SHA256,
            issuer_name_hash: "a".repeat(10),
            issuer_key_hash: "b".repeat(10),
            serial_number: "c".repeat(41), // Invalid length
            responder_url: "d".repeat(10),
        };
        let result = ocsp_request.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "serial_number");
            } else {
                panic!("Expected FieldValidationError for 'serial_number'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_responder_url_length() {
        let ocsp_request = OCSPRequestDataType {
            hash_algorithm: HashAlgorithmEnumType::SHA256,
            issuer_name_hash: "a".repeat(10),
            issuer_key_hash: "b".repeat(10),
            serial_number: "c".repeat(10),
            responder_url: "d".repeat(2001), // Invalid length
        };
        let result = ocsp_request.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "responder_url");
            } else {
                panic!("Expected FieldValidationError for 'responder_url'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = OCSPRequestDataType {
            hash_algorithm: HashAlgorithmEnumType::SHA256,
            issuer_name_hash: "some_hash_name".to_string(),
            issuer_key_hash: "some_key_hash".to_string(),
            serial_number: "some_serial".to_string(),
            responder_url: "https://ocsp.example.com".to_string(),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: OCSPRequestDataType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
