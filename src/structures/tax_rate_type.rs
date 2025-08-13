use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaxRateType {
    /// Required. Type of this tax, e.g., "Federal", "State".
    #[serde(rename = "type")]
    pub tax_type: String,
    /// Required. Tax percentage.
    pub tax: f64,
    /// Optional. Stack level for this type of tax. Default value, when absent, is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<i32>,
}

impl Default for TaxRateType {
    fn default() -> TaxRateType {
        Self {
            tax_type: "".to_string(),
            tax: 0.0,
            stack: None,
        }
    }
}

impl OcppEntity for TaxRateType {
    /// Validates the fields of TaxRateType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("type", 0, 20, &self.tax_type.chars());

        // Assuming a tax percentage is non-negative.
        e.check_bounds("tax", 0.0, f64::MAX, self.tax);

        if let Some(stack) = &self.stack {
            e.check_bounds("stack", 0, i32::MAX, *stack);
        }

        e.build("TaxRateType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> TaxRateType {
        TaxRateType {
            tax_type: "Federal".to_string(),
            tax: 0.15,
            stack: Some(1),
        }
    }

    #[test]
    fn test_validate_success() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_no_stack() {
        let mut data = create_test_instance();
        data.stack = None;
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_type_too_long() {
        let mut data = create_test_instance();
        data.tax_type = "a".repeat(21);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_negative_tax() {
        let mut data = create_test_instance();
        data.tax = -0.05;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_negative_stack() {
        let mut data = create_test_instance();
        data.stack = Some(-1);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TaxRateType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
