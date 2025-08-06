use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::freq_droop_get_type::tests::FreqDroopType;
use crate::traits::OcppEntity;

/// Used by: ReportDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FreqDroopGetType {
    /// Required. Id of setting
    pub id: String,
    /// Required. True if setting is a default control.
    pub is_default: bool,
    /// Required. True if this setting is superseded by a higher priority setting (i.e. lower value of priority)
    pub is_superseded: bool,
    /// Required. FreqDroop parameters
    pub freq_droop: FreqDroopType,
}

impl OcppEntity for FreqDroopGetType {
    /// Validates the fields of FreqDroopGetType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("id", 0, 36, &self.id.chars());

        e.check_member("freq_droop", &self.freq_droop);

        e.build("FreqDroopGetType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::structures::freq_droop_get_type::FreqDroopType;

    #[test]
    fn test_validate_success() {
        let freq_droop_get_type = FreqDroopGetType {
            id: "setting1".to_string(),
            is_default: true,
            is_superseded: false,
            freq_droop: FreqDroopType::default(),
        };
        assert!(freq_droop_get_type.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_id_length() {
        let freq_droop_get_type = FreqDroopGetType {
            id: "a".repeat(37), // Invalid length
            is_default: true,
            is_superseded: false,
            freq_droop: FreqDroopType::default(),
        };
        let result = freq_droop_get_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "id");
            } else {
                panic!("Expected FieldValidationError for 'id'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = FreqDroopGetType {
            id: "setting1".to_string(),
            is_default: true,
            is_superseded: false,
            freq_droop: FreqDroopType::default(),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: FreqDroopGetType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}