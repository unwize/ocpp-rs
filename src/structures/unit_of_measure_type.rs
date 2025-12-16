use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnitOfMeasureType {
    /// Optional. Unit of the value. Default is "Wh" if the measurand is an "Energy" type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// Optional. Multiplier, representing the exponent to base 10. Default is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i32>,
}
#[typetag::serde]
impl OcppEntity for UnitOfMeasureType {
    /// Validates the fields of UnitOfMeasureType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(unit) = &self.unit {
            e.check_cardinality("unit", 0, 20, &unit.chars());
        }

        e.build("UnitOfMeasureType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> UnitOfMeasureType {
        UnitOfMeasureType {
            unit: Some("Wh".to_string()),
            multiplier: Some(3), // 10^3
        }
    }

    #[test]
    fn test_validate_success_all_fields_present() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_no_fields_present() {
        let data = UnitOfMeasureType {
            unit: None,
            multiplier: None,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_unit_too_long() {
        let mut data = create_test_instance();
        data.unit = Some("a".repeat(21));
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: UnitOfMeasureType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
