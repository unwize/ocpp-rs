use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::tariff_conditions_fixed_type::TariffConditionsFixedType;
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TariffFixedPriceType {
    /// Required. Fixed price for this element, e.g., a start fee.
    pub price_fixed: f64,
    /// Optional. Conditions when this tariff element price is applicable. When absent, it is always applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<TariffConditionsFixedType>,
}

impl Default for TariffFixedPriceType {
    fn default() -> TariffFixedPriceType {
        Self {
            price_fixed: 0.0,
            conditions: None,
        }
    }
}

impl OcppEntity for TariffFixedPriceType {
    /// Validates the fields of TariffFixedPriceType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Assuming price_fixed should be non-negative.
        e.check_bounds("price_fixed", 0.0, f64::MAX, self.price_fixed);

        if let Some(conditions) = &self.conditions {
            e.check_member("conditions", conditions);
        }

        e.build("TariffFixedPriceType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::structures::tariff_conditions_fixed_type::TariffConditionsFixedType;

    fn create_test_instance() -> TariffFixedPriceType {
        TariffFixedPriceType {
            price_fixed: 1.00,
            conditions: None,
        }
    }

    #[test]
    fn test_validate_success_no_conditions() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_with_conditions() {
        let data = TariffFixedPriceType {
            price_fixed: 2.50,
            conditions: Some(TariffConditionsFixedType::default()),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_negative_price() {
        let mut data = create_test_instance();
        data.price_fixed = -0.50;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TariffFixedPriceType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}