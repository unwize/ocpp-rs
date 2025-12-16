use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::rational_number_type::RationalNumberType;
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TaxRuleType {
    /// Required. ID for the tax rule.
    pub tax_rule_id: i32,
    /// Optional. Human-readable string to identify the tax rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rule_name: Option<String>,
    /// Optional. Indicates whether the tax is included in any price or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_included_in_price: Option<bool>,
    /// Required. Indicates whether this tax applies to Energy Fees.
    pub applies_to_energy_fee: bool,
    /// Required. Indicates whether this tax applies to Parking Fees.
    pub applies_to_parking_fee: bool,
    /// Required. Indicates whether this tax applies to Overstay Fees.
    pub applies_to_overstay_fee: bool,
    /// Required. Indicates whether this tax applies to Minimum/Maximum Cost.
    pub applies_to_minimum_maximum_cost: bool,
    /// Required. Percentage of the total amount of applying fee (energy, parking, overstay, MinimumCost and/or MaximumCost).
    pub tax_rate: RationalNumberType,
}
#[typetag::serde]
impl OcppEntity for TaxRuleType {
    /// Validates the fields of TaxRuleType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("tax_rule_id", 0, i32::MAX, self.tax_rule_id);

        if let Some(name) = &self.tax_rule_name {
            e.check_cardinality("tax_rule_name", 0, 100, &name.chars());
        }

        e.check_member("tax_rate", &self.tax_rate);

        e.build("TaxRuleType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> TaxRuleType {
        TaxRuleType {
            tax_rule_id: 1,
            tax_rule_name: Some("Standard Tax".to_string()),
            tax_included_in_price: Some(false),
            applies_to_energy_fee: true,
            applies_to_parking_fee: false,
            applies_to_overstay_fee: false,
            applies_to_minimum_maximum_cost: true,
            tax_rate: RationalNumberType::default(),
        }
    }

    #[test]
    fn test_validate_success() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let data = TaxRuleType {
            tax_rule_id: 0,
            tax_rule_name: None,
            tax_included_in_price: None,
            applies_to_energy_fee: false,
            applies_to_parking_fee: false,
            applies_to_overstay_fee: false,
            applies_to_minimum_maximum_cost: false,
            tax_rate: RationalNumberType::default(),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_tax_rule_id_negative() {
        let mut data = create_test_instance();
        data.tax_rule_id = -1;
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_tax_rule_name_too_long() {
        let mut data = create_test_instance();
        data.tax_rule_name = Some("a".repeat(101));
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TaxRuleType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
