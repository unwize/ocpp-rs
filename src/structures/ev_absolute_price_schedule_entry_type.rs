use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::ev_price_rule_type::EVPriceRuleType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// An entry in price schedule over time for which EV is willing to discharge.
/// Used by: Common::EVAbsolutePriceScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EVAbsolutePriceScheduleEntryType {
    /// Required. The amount of seconds of this entry.
    pub duration: i32, // integer
    /// Required. A set of pricing rules for energy costs.
    /// Cardinality 1..8
    pub ev_price_rule: Vec<EVPriceRuleType>,
}

impl Default for EVAbsolutePriceScheduleEntryType {
    fn default() -> EVAbsolutePriceScheduleEntryType {
        Self {
            duration: 0,
            ev_price_rule: vec![Default::default()],
        }
    }
}

impl OcppEntity for EVAbsolutePriceScheduleEntryType {
    /// Validates the fields of EVAbsolutePriceScheduleEntryType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("ev_price_rule", 1, 8, &self.ev_price_rule.iter());
        e.check_iter_member("ev_price_rule", self.ev_price_rule.iter());
        e.build("EVAbsolutePriceScheduleEntryType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};

    #[test]
    fn test_serialization_deserialization() {
        let entry = EVAbsolutePriceScheduleEntryType {
            duration: 3600,
            ev_price_rule: vec![Default::default(); 2], // Placeholder
        };

        let serialized = serde_json::to_string(&entry).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: EVAbsolutePriceScheduleEntryType =
            serde_json::from_str(&serialized).unwrap();
        assert_eq!(entry, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let entry_single_rule = EVAbsolutePriceScheduleEntryType {
            duration: 1800,
            ev_price_rule: vec![Default::default()],
        };
        assert!(entry_single_rule.validate().is_ok());

        let entry_max_rules = EVAbsolutePriceScheduleEntryType {
            duration: 7200,
            ev_price_rule: vec![Default::default(); 8], // Max cardinality
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
        assert_invalid_fields(&err, &["ev_price_rule"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validation_ev_price_rule_too_many() {
        let entry = EVAbsolutePriceScheduleEntryType {
            duration: 3600,
            ev_price_rule: vec![Default::default(); 9], // Invalid cardinality
        };
        let err = entry.validate().unwrap_err();
        assert_invalid_fields(&err, &["ev_price_rule"]);
        assert_num_field_errors(&err, 1);
    }
}
