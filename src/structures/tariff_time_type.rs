use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::tariff_time_price_type::TariffTimePriceType;
use crate::structures::tax_rate_type::TaxRateType;
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TariffTimeType {
    /// Required. Element tariff price and conditions.
    pub prices: Vec<TariffTimePriceType>,
    /// Optional. Applicable tax percentages for this tariff dimension. If omitted, no tax is applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRateType>>,
}

impl Default for TariffTimeType {
    fn default() -> TariffTimeType {
        Self {
            prices: vec![],
            tax_rates: None,
        }
    }
}

impl OcppEntity for TariffTimeType {
    /// Validates the fields of TariffTimeType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("prices", 1, usize::MAX, &self.prices.iter());
        e.check_iter_member("prices", self.prices.iter());

        if let Some(tax_rates) = &self.tax_rates {
            e.check_cardinality("tax_rates", 0, 5, &tax_rates.iter());
            e.check_iter_member("tax_rates", tax_rates.iter());
        }

        e.build("TariffTimeType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::tariff_time_price_type::TariffTimePriceType;
    use serde_json;

    fn create_test_instance() -> TariffTimeType {
        TariffTimeType {
            prices: vec![TariffTimePriceType::default()],
            tax_rates: Some(vec![TaxRateType::default()]),
        }
    }

    #[test]
    fn test_validate_success() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_no_tax_rates() {
        let mut data = create_test_instance();
        data.tax_rates = None;
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_prices_cardinality_fail_min() {
        let mut data = create_test_instance();
        data.prices = vec![];
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_tax_rates_cardinality_fail_max() {
        let mut data = create_test_instance();
        data.tax_rates = Some(vec![TaxRateType::default(); 6]);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TariffTimeType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
