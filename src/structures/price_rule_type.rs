use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::rational_number_type::RationalNumberType;
use crate::traits::OcppEntity;

/// Part of ISO 15118-20 price schedule.
/// Used by: Common:PriceRuleStackType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct PriceRuleType {
    /// Optional. The duration of the parking fee period (in seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parking_fee_period: Option<i32>,
    /// Optional. Number of grams of CO2 per kWh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carbon_dioxide_emission: Option<i32>,
    /// Optional. Percentage of the power that is created by renewable resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewable_generation_percentage: Option<i32>,
    /// Required. Cost per kWh.
    pub energy_fee: RationalNumberType,
    /// Optional. Cost of parking. Mandatory whenever a parking fee applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parking_fee: Option<RationalNumberType>,
    /// Required. Power level above which this price rule applies.
    pub power_range_start: RationalNumberType,
}

#[typetag::serde]
impl OcppEntity for PriceRuleType {
    /// Validates the fields of PriceRuleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(period) = self.parking_fee_period {
            e.check_bounds("parking_fee_period", 0, i32::MAX, period);
        }

        if let Some(co2) = self.carbon_dioxide_emission {
            e.check_bounds("carbon_dioxide_emission", 0, i32::MAX, co2);
        }

        if let Some(percentage) = self.renewable_generation_percentage {
            e.check_bounds("renewable_generation_percentage", 0, 100, percentage);
        }

        e.check_member("energy_fee", &self.energy_fee);

        if self.parking_fee_period.is_some() && self.parking_fee.is_none() {
            e.push_relation_error(
                "parking_fee",
                "parking_fee_period",
                "parking_fee is mandatory if parkingFeePeriod is present",
            );
        }

        if let Some(fee) = &self.parking_fee {
            e.check_member("parking_fee", fee);
        }

        e.check_member("power_range_start", &self.power_range_start);

        e.build("PriceRuleType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let rule = PriceRuleType {
            parking_fee_period: Some(3600),
            carbon_dioxide_emission: Some(500),
            renewable_generation_percentage: Some(75),
            energy_fee: Default::default(),
            parking_fee: Some(Default::default()),
            power_range_start: Default::default(),
        };
        assert!(rule.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let rule = PriceRuleType {
            parking_fee_period: None,
            carbon_dioxide_emission: None,
            renewable_generation_percentage: None,
            energy_fee: Default::default(),
            parking_fee: None,
            power_range_start: Default::default(),
        };
        assert!(rule.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_carbon_dioxide_emission_negative() {
        let rule = PriceRuleType {
            parking_fee_period: None,
            carbon_dioxide_emission: Some(-1),
            renewable_generation_percentage: None,
            energy_fee: Default::default(),
            parking_fee: None,
            power_range_start: Default::default(),
        };
        let result = rule.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "carbon_dioxide_emission");
            }
        }
    }

    #[test]
    fn test_validate_failure_renewable_generation_percentage_out_of_bounds() {
        let rule = PriceRuleType {
            parking_fee_period: None,
            carbon_dioxide_emission: None,
            renewable_generation_percentage: Some(101),
            energy_fee: Default::default(),
            parking_fee: None,
            power_range_start: Default::default(),
        };
        let result = rule.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "renewable_generation_percentage");
            }
        }
    }

    #[test]
    fn test_validate_failure_parking_fee_conditional_missing() {
        let rule = PriceRuleType {
            parking_fee_period: Some(60),
            carbon_dioxide_emission: None,
            renewable_generation_percentage: None,
            energy_fee: Default::default(),
            parking_fee: None,
            power_range_start: Default::default(),
        };
        let err = rule.validate().unwrap_err();
        assert_num_field_errors(&err, 2);
        assert_invalid_fields(&err, &["parking_fee_period", "parking_fee"]);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = PriceRuleType {
            parking_fee_period: Some(60),
            carbon_dioxide_emission: Some(100),
            renewable_generation_percentage: Some(50),
            energy_fee: Default::default(),
            parking_fee: Some(Default::default()),
            power_range_start: Default::default(),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: PriceRuleType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
