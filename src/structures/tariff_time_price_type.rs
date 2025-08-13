use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::tariff_conditions_type::TariffConditionsType;
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TariffTimePriceType {
    /// Required. Price per minute (excl. tax) for this element.
    pub price_minute: f64,
    /// Optional. Conditions when this tariff element price is applicable. When absent, it is always applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<TariffConditionsType>,
}

impl Default for TariffTimePriceType {
    fn default() -> TariffTimePriceType {
        Self {
            price_minute: 0.0,
            conditions: None,
        }
    }
}

impl OcppEntity for TariffTimePriceType {
    /// Validates the fields of TariffTimePriceType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Assuming price_minute should be non-negative, as is standard for price fields.
        e.check_bounds("price_minute", 0.0, f64::MAX, self.price_minute);

        if let Some(conditions) = &self.conditions {
            e.check_member("conditions", conditions);
        }

        e.build("TariffTimePriceType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> TariffTimePriceType {
        TariffTimePriceType {
            price_minute: 0.50,
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
        let data = TariffTimePriceType {
            price_minute: 0.75,
            conditions: Some(TariffConditionsType::default()),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_negative_price() {
        let mut data = create_test_instance();
        data.price_minute = -0.1;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TariffTimePriceType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}