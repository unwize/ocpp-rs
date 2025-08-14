use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::tax_rate_type::TaxRateType;
use crate::traits::OcppEntity;

/// Price with and without tax. At least one of exclTax, inclTax must be present.
/// Used by: Common:TariffType, Common:TotalCostType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceType {
    /// Optional. Price/cost excluding tax. Can be absent if inclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excl_tax: Option<f64>,
    /// Optional. Price/cost including tax. Can be absent if exclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incl_tax: Option<f64>,
    /// Optional. Tax percentages that were used to calculate inclTax from exclTax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRateType>>,
}

impl Default for PriceType {
    fn default() -> PriceType {
        Self {
            excl_tax: Some(1.0),
            incl_tax: None,
            tax_rates: None,
        }
    }
}

impl OcppEntity for PriceType {
    /// Validates the fields of PriceType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // At least one of exclTax or inclTax must be present.
        if self.excl_tax.is_none() && self.incl_tax.is_none() {
            e.push_relation_error(
                "excl_tax",
                "incl_tax",
                "At least one of 'exclTax' or 'inclTax' must be present",
            );
        }

        if let Some(tax_rates) = &self.tax_rates {
            e.check_cardinality("tax_rates", 0, 5, &tax_rates.iter());
            e.check_iter_member("tax_rates", tax_rates.iter());
        }

        e.build("PriceType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::errors::assert_invalid_fields;

    #[test]
    fn test_validate_success_excl_tax_only() {
        let price = PriceType {
            excl_tax: Some(10.0),
            incl_tax: None,
            tax_rates: None,
        };
        assert!(price.validate().is_ok());
    }

    #[test]
    fn test_validate_success_incl_tax_only() {
        let price = PriceType {
            excl_tax: None,
            incl_tax: Some(12.1),
            tax_rates: None,
        };
        assert!(price.validate().is_ok());
    }

    #[test]
    fn test_validate_success_both_taxes_and_rates() {
        let price = PriceType {
            excl_tax: Some(10.0),
            incl_tax: Some(12.1),
            tax_rates: Some(vec![Default::default(), Default::default()]),
        };
        assert!(price.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_no_taxes() {
        let price = PriceType {
            excl_tax: None,
            incl_tax: None,
            tax_rates: None,
        };
        let result = price.validate();
        assert!(result.is_err());
        let error = result.err().unwrap();
        if let OcppError::StructureValidationError { related, .. } = error.clone() {
            assert_eq!(related.len(), 2);
        }
        assert_invalid_fields(error, &["excl_tax", "incl_tax"]);

    }

    #[test]
    fn test_validate_failure_tax_rates_too_many() {
        let price = PriceType {
            excl_tax: Some(10.0),
            incl_tax: None,
            tax_rates: Some(vec![Default::default(); 6]), // Invalid: max 5
        };
        let result = price.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "tax_rates");
            }
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = PriceType {
            excl_tax: Some(10.0),
            incl_tax: Some(12.1),
            tax_rates: Some(vec![Default::default()]),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: PriceType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
