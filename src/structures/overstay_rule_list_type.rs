use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::overstay_rule_type::OverstayRuleType;
use crate::structures::rational_number_type::RationalNumberType;
use crate::traits::OcppEntity;

/// Part of ISO 15118-20 price schedule.
/// Used by: Common:AbsolutePriceScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OverstayRuleListType {
    /// Optional. Time till overstay is applied in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overstay_time_threshold: Option<i32>,
    /// Optional. Power threshold in W at which the overstay applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overstay_power_threshold: Option<RationalNumberType>,
    /// Required. Overstay rules that will be applied.
    pub overstay_rule: Vec<OverstayRuleType>, // TODO: Implement OverstayRuleType
}

impl OcppEntity for OverstayRuleListType {
    /// Validates the fields of OverstayRuleListType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(threshold) = &self.overstay_power_threshold {
            e.check_member("overstay_power_threshold", threshold);
        }

        e.check_cardinality("overstay_rule", 1, 5, &self.overstay_rule.iter());
        e.check_iter_member("overstay_rule", self.overstay_rule.iter());

        e.build("OverstayRuleListType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::overstay_rule_type::OverstayRuleType;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let rule_list = OverstayRuleListType {
            overstay_time_threshold: Some(3600),
            overstay_power_threshold: Some(RationalNumberType::default()),
            overstay_rule: vec![OverstayRuleType::default(), OverstayRuleType::default()],
        };
        assert!(rule_list.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let rule_list = OverstayRuleListType {
            overstay_time_threshold: None,
            overstay_power_threshold: None,
            overstay_rule: vec![OverstayRuleType::default()],
        };
       
        if let Err(e) = rule_list.validate() {
            println!("{:#?}", e);
        }
        
        assert!(rule_list.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_overstay_rule_too_few() {
        let rule_list = OverstayRuleListType {
            overstay_time_threshold: None,
            overstay_power_threshold: None,
            overstay_rule: vec![], // Invalid: length 0, min 1
        };
        let result = rule_list.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "overstay_rule");
            } else {
                panic!("Expected FieldValidationError for 'overstay_rule'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_overstay_rule_too_many() {
        let rule_list = OverstayRuleListType {
            overstay_time_threshold: None,
            overstay_power_threshold: None,
            overstay_rule: vec![OverstayRuleType::default(); 6], // Invalid: length 6, max 5
        };
        let result = rule_list.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "overstay_rule");
            } else {
                panic!("Expected FieldValidationError for 'overstay_rule'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = OverstayRuleListType {
            overstay_time_threshold: Some(1800),
            overstay_power_threshold: Some(RationalNumberType::default()),
            overstay_rule: vec![OverstayRuleType::default()],
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: OverstayRuleListType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
