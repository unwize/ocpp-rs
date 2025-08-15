use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::gradient_type::GradientType;
use crate::traits::OcppEntity;

/// Used by: ReportDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GradientGetType {
    /// Required. Id of setting
    pub id: String,
    /// Required. Gradient setting
    pub gradient: GradientType,
}

impl OcppEntity for GradientGetType {
    /// Validates the fields of GradientGetType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("id", 0, 36, &self.id.chars());

        e.check_member("gradient", &self.gradient);

        e.build("GradientGetType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::gradient_type::GradientType;
    use serde_json;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};

    #[test]
    fn test_validate_success() {
        let gradient_get_type = GradientGetType {
            id: "setting1".to_string(),
            gradient: GradientType::default(),
        };
        assert!(gradient_get_type.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_id_length() {
        let gradient_get_type = GradientGetType {
            id: "a".repeat(37), // Invalid length
            gradient: GradientType::default(),
        };
        let err = gradient_get_type.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["id"]);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = GradientGetType {
            id: "setting1".to_string(),
            gradient: GradientType::default(),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: GradientGetType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
