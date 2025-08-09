use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Represents a signed version of the meter value.
/// Used by: Common:SampledValueType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignedMeterValueType {
    /// Required. Base64 encoded signed data from the meter.
    pub signed_meter_data: String,
    /// Optional. Method used to create the digital signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_method: Option<String>,
    /// Required. Format used by the energy meter to encode the meter data.
    pub encoding_method: String,
    /// Optional. Base64 encoded public key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

impl Default for SignedMeterValueType {
    fn default() -> SignedMeterValueType {
        Self {
            signed_meter_data: "".to_string(),
            signing_method: None,
            encoding_method: "".to_string(),
            public_key: None,
        }
    }
}

impl OcppEntity for SignedMeterValueType {
    /// Validates the fields of SignedMeterValueType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("signed_meter_data", 0, 32768, &self.signed_meter_data.chars());
        e.check_cardinality("encoding_method", 0, 50, &self.encoding_method.chars());

        if let Some(signing_method) = &self.signing_method {
            e.check_cardinality("signing_method", 0, 50, &signing_method.chars());
        }

        if let Some(public_key) = &self.public_key {
            e.check_cardinality("public_key", 0, 2500, &public_key.chars());
        }

        e.build("SignedMeterValueType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let data = SignedMeterValueType {
            signed_meter_data: "validdata".to_string(),
            encoding_method: "ocmf".to_string(),
            ..Default::default()
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_signed_meter_data_too_long() {
        let mut data = SignedMeterValueType::default();
        data.signed_meter_data = "a".repeat(32769);
        data.encoding_method = "valid".to_string();
        let result = data.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_failure_encoding_method_too_long() {
        let mut data = SignedMeterValueType::default();
        data.signed_meter_data = "validdata".to_string();
        data.encoding_method = "b".repeat(51);
        let result = data.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = SignedMeterValueType {
            signed_meter_data: "validdata".to_string(),
            encoding_method: "ocmf".to_string(),
            public_key: Some("key".to_string()),
            signing_method: Some("method".to_string()),
        };
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: SignedMeterValueType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}