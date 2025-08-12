use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::sales_tariff_entry_type::SalesTariffEntryType;
use crate::traits::OcppEntity;

/// A SalesTariff provided by a Mobility Operator (EMSP). This data type is based on dataTypes from ISO 15118-2.
/// Used by: Common:ChargingScheduleType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffType {
    /// Required. SalesTariff identifier used to identify one sales tariff. An SAID remains a unique identifier for one schedule throughout a charging session.
    pub id: i32,
    /// Optional. A human-readable title/short description of the sales tariff e.g. for HMI display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff_description: Option<String>,
    /// Optional. Defines the overall number of distinct price levels used across all provided SalesTariff elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_e_price_levels: Option<i32>,
    /// Required. Encapsulating element describing all relevant details for one time interval of the SalesTariff. The number of SalesTariffEntry elements is limited by the parameter maxScheduleTuples.
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
}

impl Default for SalesTariffType {
    fn default() -> SalesTariffType {
        Self {
            id: 0,
            sales_tariff_description: None,
            num_e_price_levels: None,
            sales_tariff_entry: vec![Default::default()],
        }
    }
}

impl OcppEntity for SalesTariffType {
    /// Validates the fields of SalesTariffType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("id", 0, i32::MAX, self.id);
        if let Some(desc) = &self.sales_tariff_description {
            e.check_cardinality("sales_tariff_description", 0, 32, &desc.chars());
        }
        if let Some(levels) = self.num_e_price_levels {
            e.check_bounds("num_e_price_levels", 0, i32::MAX, levels);
        }
        e.check_cardinality("sales_tariff_entry", 1, 1024, &self.sales_tariff_entry.iter());
        e.check_iter_member("sales_tariff_entry", self.sales_tariff_entry.iter());

        e.build("SalesTariffType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let tariff = SalesTariffType::default();
        assert!(tariff.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let tariff = SalesTariffType {
            id: 1,
            sales_tariff_description: None,
            num_e_price_levels: None,
            sales_tariff_entry: vec![
                Default::default(),
            ],
        };
        assert!(tariff.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_id_negative() {
        let mut tariff = SalesTariffType::default();
        tariff.id = -1;
        let result = tariff.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "id");
            }
        }
    }

    #[test]
    fn test_validate_failure_description_too_long() {
        let mut tariff = SalesTariffType::default();
        tariff.sales_tariff_description = Some("a".repeat(33));
        let result = tariff.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "sales_tariff_description");
            }
        }
    }

    #[test]
    fn test_validate_failure_sales_tariff_entry_too_few() {
        let mut tariff = SalesTariffType::default();
        tariff.sales_tariff_entry = vec![];
        let result = tariff.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "sales_tariff_entry");
            }
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = SalesTariffType::default();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: SalesTariffType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}