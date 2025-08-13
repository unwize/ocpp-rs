use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::rational_number_type::RationalNumberType;
use crate::traits::OcppEntity;

/// Part of ISO 15118-20 price schedule.
/// Used by: Common:OverstayRuleListType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OverstayRuleType {
    /// Optional. Human-readable string to identify the overstay rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overstay_rule_description: Option<String>,
    /// Required. Time in seconds after trigger of the parent Overstay Rules for this particular fee to apply.
    pub start_time: i32,
    /// Required. Time till overstay will be reapplied.
    pub overstay_fee_period: i32,
    /// Required. Fee that applies to this overstay.
    pub overstay_fee: RationalNumberType,
}

impl OcppEntity for OverstayRuleType {
    /// Validates the fields of OverstayRuleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(desc) = &self.overstay_rule_description {
            e.check_cardinality("overstay_rule_description", 0, 32, &desc.chars());
        }

        e.check_bounds("start_time", 0, i32::MAX, self.start_time);
        e.check_bounds("overstay_fee_period", 1, i32::MAX, self.overstay_fee_period);
        e.check_member("overstay_fee", &self.overstay_fee);

        e.build("OverstayRuleType")
    }
}

impl Default for OverstayRuleType {
    fn default() -> OverstayRuleType {
        Self {
            overstay_rule_description: None,
            start_time: 0,
            overstay_fee_period: 0,
            overstay_fee: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let overstay_rule = OverstayRuleType {
            overstay_rule_description: Some("overstay_rule_1".to_string()),
            start_time: 300,
            overstay_fee_period: 60,
            overstay_fee: RationalNumberType::default(),
        };
        assert!(overstay_rule.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let overstay_rule = OverstayRuleType {
            overstay_rule_description: None,
            start_time: 0,
            overstay_fee_period: 1,
            overstay_fee: RationalNumberType::default(),
        };
        assert!(overstay_rule.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_description_length() {
        let overstay_rule = OverstayRuleType {
            overstay_rule_description: Some("a".repeat(33)), // Invalid length
            start_time: 300,
            overstay_fee_period: 60,
            overstay_fee: RationalNumberType::default(),
        };
        let result = overstay_rule.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "overstay_rule_description");
            } else {
                panic!("Expected FieldValidationError for 'overstay_rule_description'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_start_time_bounds() {
        let overstay_rule = OverstayRuleType {
            overstay_rule_description: None,
            start_time: -1, // Invalid
            overstay_fee_period: 60,
            overstay_fee: RationalNumberType::default(),
        };
        let result = overstay_rule.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "start_time");
            } else {
                panic!("Expected FieldValidationError for 'start_time'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_overstay_fee_period_bounds() {
        let overstay_rule = OverstayRuleType {
            overstay_rule_description: None,
            start_time: 300,
            overstay_fee_period: 0, // Invalid
            overstay_fee: RationalNumberType::default(),
        };
        let result = overstay_rule.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "overstay_fee_period");
            } else {
                panic!("Expected FieldValidationError for 'overstay_fee_period'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = OverstayRuleType {
            overstay_rule_description: Some("example_rule".to_string()),
            start_time: 300,
            overstay_fee_period: 60,
            overstay_fee: RationalNumberType::default(),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: OverstayRuleType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
