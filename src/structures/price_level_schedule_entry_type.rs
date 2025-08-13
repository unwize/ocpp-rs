use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Part of ISO 15118-20 price schedule.
/// Used by: Common:PriceLevelScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceLevelScheduleEntryType {
    /// Required. The amount of seconds that define the duration of this given PriceLevelScheduleEntry.
    pub duration: i32,
    /// Required. Defines the price level of this PriceLevelScheduleEntry.
    pub price_level: i32,
}

impl Default for PriceLevelScheduleEntryType {
    fn default() -> PriceLevelScheduleEntryType {
        Self {
            duration: 0,
            price_level: 0,
        }
    }
}

impl OcppEntity for PriceLevelScheduleEntryType {
    /// Validates the fields of PriceLevelScheduleEntryType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("duration", 0, i32::MAX, self.duration);
        e.check_bounds("price_level", 0, i32::MAX, self.price_level);

        e.build("PriceLevelScheduleEntryType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let entry = PriceLevelScheduleEntryType {
            duration: 3600,
            price_level: 5,
        };
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn test_validate_success_zero_bounds() {
        let entry = PriceLevelScheduleEntryType {
            duration: 0,
            price_level: 0,
        };
        assert!(entry.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_duration_negative() {
        let entry = PriceLevelScheduleEntryType {
            duration: -1, // Invalid
            price_level: 5,
        };
        let result = entry.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "duration");
            } else {
                panic!("Expected FieldValidationError for 'duration'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_price_level_negative() {
        let entry = PriceLevelScheduleEntryType {
            duration: 3600,
            price_level: -1, // Invalid
        };
        let result = entry.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "price_level");
            } else {
                panic!("Expected FieldValidationError for 'price_level'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = PriceLevelScheduleEntryType {
            duration: 300,
            price_level: 10,
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: PriceLevelScheduleEntryType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
