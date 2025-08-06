use serde::{Deserialize, Serialize};
use crate::enums::der_control_enum_type::DERControlEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::der_curve_type::DERCurveType;
use crate::traits::OcppEntity;

/// DERCurveGetType is used by: ReportDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DERCurveGetType {
    /// Required. Id of DER curve
    /// String length: 0..36
    pub id: String,
    /// Required. Type of DER curve
    pub curve_type: DERControlEnumType,
    /// Required. True if this is a default curve
    pub is_default: bool,
    /// Required. True if this setting is superseded by a higher priority setting (i.e. lower value of priority)
    pub is_superseded: bool,
    /// Required. Parameters defining the DER curve
    pub curve: DERCurveType,
}

impl OcppEntity for DERCurveGetType {
    /// Validates the fields of DERCurveGetType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.check_cardinality("id", 0, 36, &self.id.chars());
        e.push_member("curve", &self.curve);
        e.build("DERCurveGetType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let der_curve_get = DERCurveGetType {
            id: "DER_CURVE_001".to_string(),
            curve_type: DERControlEnumType::FreqWatt, // Placeholder
            is_default: false,
            is_superseded: false,
            curve: DERCurveType::default(), // Placeholder
        };

        let serialized = serde_json::to_string(&der_curve_get).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: DERCurveGetType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(der_curve_get, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let der_curve_get = DERCurveGetType {
            id: "valid_id_123".to_string(),
            curve_type: DERControlEnumType::FixedPFAbsorb,
            is_default: true,
            is_superseded: false,
            curve: DERCurveType::default(),
        };
        assert!(der_curve_get.validate().is_ok());

        let der_curve_get_max_id_len = DERCurveGetType {
            id: "a".repeat(36), // Valid length
            curve_type: DERControlEnumType::HFMayTrip,
            is_default: false,
            is_superseded: true,
            curve: DERCurveType::default(),
        };
        assert!(der_curve_get_max_id_len.validate().is_ok());
    }

    #[test]
    fn test_validation_id_too_long() {
        let der_curve_get = DERCurveGetType {
            id: "a".repeat(37), // Invalid
            curve_type: DERControlEnumType::WattPF,
            is_default: false,
            is_superseded: false,
            curve: DERCurveType::default(),
        };
        let err = der_curve_get.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "id");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }
}
