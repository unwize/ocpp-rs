use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Used by: Common::ChargingScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LimitAtSOCType {
    /// Required. The SoC value beyond which the charging rate limit should be applied.
    pub soc: i32,
    /// Required. Charging rate limit beyond the SoC value.
    pub limit: f64,
}

impl Default for LimitAtSOCType {
    fn default() -> Self {
        Self { soc: 0, limit: 0.0 }
    }
}

impl OcppEntity for LimitAtSOCType {
    /// Validates the fields of LimitAtSOCType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("soc", 0, 100, self.soc);

        e.build("LimitAtSOCType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use serde_json;

    #[test]
    fn test_validate_success() {
        let limit_at_soc_type = LimitAtSOCType {
            soc: 50,
            limit: 7.5,
        };
        assert!(limit_at_soc_type.validate().is_ok());

        let limit_at_soc_type_bounds = LimitAtSOCType {
            soc: 0,
            limit: 100.0,
        };
        assert!(limit_at_soc_type_bounds.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_soc_low() {
        let limit_at_soc_type = LimitAtSOCType {
            soc: -1, // Invalid
            limit: 7.5,
        };
        let err = limit_at_soc_type.validate().unwrap_err();
        assert_invalid_fields(&err, &["soc"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_soc_high() {
        let limit_at_soc_type = LimitAtSOCType {
            soc: 101, // Invalid
            limit: 7.5,
        };
        let err = limit_at_soc_type.validate().unwrap_err();
        assert_invalid_fields(&err, &["soc"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = LimitAtSOCType {
            soc: 50,
            limit: 7.5,
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: LimitAtSOCType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
