use serde::{Deserialize, Serialize};
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::charging_period_type::ChargingPeriodType;
use crate::traits::OcppEntity;

/// CostDetailsType contains the cost as calculated by Charging Station based on provided TariffType.
/// Used by: TransactionEventRequest
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
    pub total_cost: TotalCostType, // TODO: Implement TotalCostType
    /// Required. Total usage of energy and time.
    pub total_usage: TotalUsageType, // TODO: Implement TotalUsageType
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
            e.push_iter_member("charging_periods", charging_periods.iter());
        }

        e.push_member("total_cost", &self.total_cost).push_member("total_usage", &self.total_usage);

        e.build("CostDetailsType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let cost_details = CostDetailsType {
            failure_to_calculate: Some(false),
            failure_reason: Some("No issues".to_string()),
            charging_periods: Some(vec![]), // TODO: Placeholder
            total_cost: "total_cost_placeholder".to_string(), // TODO: Placeholder
            total_usage: "total_usage_placeholder".to_string(), // TODO: Placeholder
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
            total_cost: "10.50".to_string(),
            total_usage: "5.0".to_string(),
        };
        assert!(cost_details_minimal.validate().is_ok());

        let cost_details_full_lengths = CostDetailsType {
            failure_to_calculate: Some(true),
            failure_reason: Some("a".repeat(500)), // Valid length
            charging_periods: Some(vec![]), // TODO: Add two test instances
            total_cost: "25.75".to_string(),
            total_usage: "10.2".to_string(),
        };
        assert!(cost_details_full_lengths.validate().is_ok());
    }

    #[test]
    fn test_validation_failure_reason_too_long() {
        let cost_details = CostDetailsType {
            failure_to_calculate: Some(true),
            failure_reason: Some("a".repeat(501)), // Invalid
            charging_periods: None,
            total_cost: "0.0".to_string(),
            total_usage: "0.0".to_string(),
        };
        let err = cost_details.validate().unwrap_err();
        if let OcppError::StructureValidationError { related: source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "failure_reason");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }
}
