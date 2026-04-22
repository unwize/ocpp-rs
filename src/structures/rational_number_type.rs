use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Represents a rational number with an exponent and a value.
/// Part of ISO 15118-20 price schedule.
/// Used by: Common:AbsolutePriceScheduleType, Common:AdditionalSelectedServicesType, Common:OverstayRuleListType, Common:OverstayRuleType, Common:PriceRuleType, Common:TaxRuleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RationalNumberType {
    /// Required. The exponent to base 10.
    pub exponent: i32,
    /// Required. Value which shall be multiplied.
    pub value: i32,
}

impl Default for RationalNumberType {
    fn default() -> Self {
        Self {
            exponent: 0,
            value: 1,
        }
    }
}

impl OcppEntity for RationalNumberType {
    /// Validates the fields of RationalNumberType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let e = StructureValidationBuilder::new();
        // The spec does not provide bounds for exponent or value, so we only perform a basic build check.
        e.build("RationalNumberType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let number = RationalNumberType {
            exponent: -1,
            value: 123,
        };
        assert!(number.validate().is_ok());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = RationalNumberType {
            exponent: -2,
            value: 550,
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: RationalNumberType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }

    #[test]
    fn test_default() {
        let default_struct: RationalNumberType = Default::default();
        assert_eq!(default_struct.exponent, 0);
        assert_eq!(default_struct.value, 1);
    }
}
