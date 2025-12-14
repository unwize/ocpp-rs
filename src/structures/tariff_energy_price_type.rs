use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::tariff_conditions_type::TariffConditionsType;
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TariffEnergyPriceType {
    /// Required. Price per kWh (excl. tax) for this element.
    pub price_kwh: f64,
    /// Optional. Conditions when this tariff element price is applicable. When absent, it is always applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<TariffConditionsType>,
}

impl Default for TariffEnergyPriceType {
    fn default() -> TariffEnergyPriceType {
        Self {
            price_kwh: 0.0,
            conditions: None,
        }
    }
}
#[typetag::serde]
impl OcppEntity for TariffEnergyPriceType {
    /// Validates the fields of TariffEnergyPriceType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Check if price_kwh is non-negative, as is standard for price fields.
        e.check_bounds("price_kwh", 0.0, f64::MAX, self.price_kwh);

        if let Some(conditions) = &self.conditions {
            e.check_member("conditions", conditions);
        }

        e.build("TariffEnergyPriceType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // Helper function to create a test instance.
    fn create_test_instance() -> TariffEnergyPriceType {
        TariffEnergyPriceType {
            price_kwh: 0.25,
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
        let data = TariffEnergyPriceType {
            price_kwh: 0.30,
            conditions: Some(TariffConditionsType::default()),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_negative_price() {
        let mut data = create_test_instance();
        data.price_kwh = -0.1;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = TariffEnergyPriceType {
            price_kwh: 0.25,
            conditions: Some(TariffConditionsType::default()),
        };
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TariffEnergyPriceType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
