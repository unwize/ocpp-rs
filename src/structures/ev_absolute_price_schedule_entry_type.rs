use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

/// An entry in price schedule over time for which EV is willing to discharge.
/// Used by: Common::EVAbsolutePriceScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EVAbsolutePriceScheduleEntryType {
    /// Required. The amount of seconds of this entry.
    pub duration: i32, // integer
    /// Required. A set of pricing rules for energy costs.
    /// Cardinality 1..8
    pub ev_price_rule: Vec<EVPriceRuleType>, // TODO: Implement EVPriceRuleType
}

impl EVAbsolutePriceScheduleEntryType {
    /// Validates the fields of EVAbsolutePriceScheduleEntryType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // No explicit constraints for 'duration' other than its type (integer).
        // If there were, they would be added here (e.g., self.duration >= 0).

        // Validate ev_price_rule cardinality
        if self.ev_price_rule.is_empty() || self.ev_price_rule.len() > 8 {
            errors.push(OcppError::FieldValidationError {
                field: "ev_price_rule".to_string(),
                source: vec![OcppError::FieldCardinalityError {
                    cardinality: self.ev_price_rule.len(),
                    lower: 1,
                    upper: 8,
                }],
            });
        }

        for (i, rule) in self.ev_price_rule.iter().enumerate() {
             if let Err(e) = rule.validate() {
                 errors.push(OcppError::FieldValidationError {
                     field: format!("ev_price_rule[{}]", i),
                     source: vec![e],
                 });
             }
        }


        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "EVAbsolutePriceScheduleEntryType".to_string(),
                source: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let entry = EVAbsolutePriceScheduleEntryType {
            duration: 3600,
            ev_price_rule: vec!["rule1_placeholder".to_string(), "rule2_placeholder".to_string()], // Placeholder
        };

        let serialized = serde_json::to_string(&entry).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EVAbsolutePriceScheduleEntryType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let entry_single_rule = EVAbsolutePriceScheduleEntryType {
            duration: 1800,
            ev_price_rule: vec!["rule_a".to_string()],
        };
        assert!(entry_single_rule.validate().is_ok());

        let entry_max_rules = EVAbsolutePriceScheduleEntryType {
            duration: 7200,
            ev_price_rule: vec!["rule".to_string(); 8], // Max cardinality
        };
        assert!(entry_max_rules.validate().is_ok());
    }

    #[test]
    fn test_validation_ev_price_rule_empty() {
        let entry = EVAbsolutePriceScheduleEntryType {
            duration: 3600,
            ev_price_rule: vec![], // Invalid cardinality
        };
        let err = entry.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "ev_price_rule");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_ev_price_rule_too_many() {
        let entry = EVAbsolutePriceScheduleEntryType {
            duration: 3600,
            ev_price_rule: vec!["rule".to_string(); 9], // Invalid cardinality
        };
        let err = entry.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "ev_price_rule");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }
}
