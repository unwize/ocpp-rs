use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::price_rule_type::PriceRuleType;
use crate::traits::OcppEntity;

/// Part of ISO 15118-20 price schedule.
/// Used by: Common:AbsolutePriceScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceRuleStackType {
    /// Required. Duration of the stack of price rules. The amount of seconds that define the duration of the given PriceRule(s).
    pub duration: i32,
    /// Required. Contains the price rules.
    pub price_rule: Vec<PriceRuleType>,
}
#[typetag::serde]
impl OcppEntity for PriceRuleStackType {
    /// Validates the fields of PriceRuleStackType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("duration", 0, i32::MAX, self.duration);

        e.check_cardinality("price_rule", 1, 8, &self.price_rule.iter());
        e.check_iter_member("price_rule", self.price_rule.iter());

        e.build("PriceRuleStackType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use serde_json;

    #[test]
    fn test_validate_success() {
        let price_stack = PriceRuleStackType {
            duration: 3600,
            price_rule: vec![Default::default(), Default::default()],
        };
        assert!(price_stack.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_duration_negative() {
        let price_stack = PriceRuleStackType {
            duration: -1,
            price_rule: vec![Default::default()],
        };
        let err = price_stack.validate().unwrap_err();
        assert_invalid_fields(&err, &["duration"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_price_rule_too_few() {
        let price_stack = PriceRuleStackType {
            duration: 3600,
            price_rule: vec![],
        };
        let err = price_stack.validate().unwrap_err();
        assert_invalid_fields(&err, &["price_rule"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_price_rule_too_many() {
        let price_stack = PriceRuleStackType {
            duration: 3600,
            price_rule: vec![Default::default(); 9],
        };
        let err = price_stack.validate().unwrap_err();
        assert_invalid_fields(&err, &["price_rule"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = PriceRuleStackType {
            duration: 300,
            price_rule: vec![Default::default()],
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: PriceRuleStackType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
