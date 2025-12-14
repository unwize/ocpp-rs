use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Used by: Common::GradientGetType, SetDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GradientType {
    /// Required. Priority of setting (0=highest)
    pub priority: i32,
    /// Required. Default ramp rate in seconds (0 if not applicable)
    pub gradient: f64,
    /// Required. Soft-start ramp rate in seconds (0 if not applicable)
    pub soft_gradient: f64,
}

impl Default for GradientType {
    fn default() -> GradientType {
        Self {
            priority: 0,
            gradient: 0.0,
            soft_gradient: 0.0,
        }
    }
}

impl Display for GradientType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[typetag::serde]
impl OcppEntity for GradientType {
    /// Validates the fields of GradientType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("priority", 0, i32::MAX, self.priority);
        e.check_bounds("gradient", 0.0, f64::MAX, self.gradient);
        e.check_bounds("soft_gradient", 0.0, f64::MAX, self.soft_gradient);

        e.build("GradientType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use serde_json;

    #[test]
    fn test_validate_success() {
        let gradient_type = GradientType {
            priority: 0,
            gradient: 10.0,
            soft_gradient: 5.0,
        };
        assert!(gradient_type.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_priority() {
        let gradient_type = GradientType {
            priority: -1, // Invalid
            gradient: 10.0,
            soft_gradient: 5.0,
        };
        let result = gradient_type.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_invalid_fields(&err, &["priority"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_gradient() {
        let gradient_type = GradientType {
            priority: 0,
            gradient: -1.0, // Invalid
            soft_gradient: 5.0,
        };
        let err = gradient_type.validate().unwrap_err();
        assert_invalid_fields(&err, &["gradient"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_soft_gradient() {
        let gradient_type = GradientType {
            priority: 0,
            gradient: 10.0,
            soft_gradient: -1.0, // Invalid
        };
        let err = gradient_type.validate().unwrap_err();
        assert_invalid_fields(&err, &["soft_gradient"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = GradientType {
            priority: 1,
            gradient: 20.0,
            soft_gradient: 15.0,
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: GradientType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
