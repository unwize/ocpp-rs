use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::charging_period_type::ChargingPeriodType;
use crate::structures::total_cost_type::TotalCostType;
use crate::structures::total_usage_type::TotalUsageType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// CostDetailsType contains the cost as calculated by Charging Station based on provided TariffType.
/// Used by: TransactionEventRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CostDetailsType {
    /// Optional. If set to true, then Charging Station has failed to calculate the cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_to_calculate: Option<bool>,
    /// Optional. Optional human-readable reason text in case of failure to calculate.
    /// String length: 0..500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Optional. List of Charging Periods that make up this charging session.
    /// A finished session has of 1 or more periods, where each period has a different list of
    /// dimensions that determined the price. When sent as a running cost update during a transaction
    /// chargingPeriods are omitted.
    /// Cardinality 0..*
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_periods: Option<Vec<ChargingPeriodType>>,
    /// Required. Total sum of all the costs of this transaction in the specified currency.
    pub total_cost: TotalCostType,
    /// Required. Total usage of energy and time.
    pub total_usage: TotalUsageType,
}

impl OcppEntity for CostDetailsType {
    /// Validates the fields of CostDetailsType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        // Validate failure_reason length
        if let Some(failure_reason) = &self.failure_reason {
            e.check_cardinality("failure_reason", 0, 500, &failure_reason.chars());
        }

        if let Some(charging_periods) = &self.charging_periods {
            e.check_iter_member("charging_periods", charging_periods.iter());
        }

        e.check_member("total_cost", &self.total_cost);
        e.check_member("total_usage", &self.total_usage);

        e.build("CostDetailsType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};

    #[test]
    fn test_serialization_deserialization() {
        let cost_details = CostDetailsType {
            failure_to_calculate: Some(false),
            failure_reason: Some("No issues".to_string()),
            charging_periods: Some(vec![Default::default()]),
            total_cost: Default::default(),
            total_usage: Default::default(),
        };

        let serialized = serde_json::to_string(&cost_details).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: CostDetailsType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cost_details, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let cost_details_minimal = CostDetailsType {
            failure_to_calculate: None,
            failure_reason: None,
            charging_periods: None,
            total_cost: Default::default(),
            total_usage: Default::default(),
        };
        assert!(cost_details_minimal.validate().is_ok());

        let cost_details_full_lengths = CostDetailsType {
            failure_to_calculate: Some(true),
            failure_reason: Some("a".repeat(500)), // Valid length
            charging_periods: Some(vec![Default::default(), Default::default()]),
            total_cost: Default::default(),
            total_usage: Default::default(),
        };
        assert!(cost_details_full_lengths.validate().is_ok());
    }

    #[test]
    fn test_validation_failure_reason_too_long() {
        let cost_details = CostDetailsType {
            failure_to_calculate: Some(true),
            failure_reason: Some("a".repeat(501)), // Invalid
            charging_periods: None,
            total_cost: Default::default(),
            total_usage: Default::default(),
        };
        let err = cost_details.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["failure_reason"]);
    }
}
