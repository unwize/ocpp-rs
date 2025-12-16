use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLimitType {
    /// Optional. Maximum allowed cost of transaction in the currency of the tariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cost: Option<f64>,
    /// Optional. Maximum allowed energy in Wh to charge in a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_energy: Option<f64>,
    /// Optional. Maximum duration of the transaction in seconds from start to end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_time: Option<i32>,
    /// Optional. Maximum State of Charge of the EV in percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_soc: Option<i32>,
}
#[typetag::serde]
impl OcppEntity for TransactionLimitType {
    /// Validates the fields of TransactionLimitType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(max_cost) = self.max_cost {
            e.check_bounds("max_cost", 0.0, f64::MAX, max_cost);
        }
        if let Some(max_energy) = self.max_energy {
            e.check_bounds("max_energy", 0.0, f64::MAX, max_energy);
        }
        if let Some(max_time) = self.max_time {
            e.check_bounds("max_time", 0, i32::MAX, max_time);
        }
        if let Some(max_soc) = self.max_soc {
            e.check_bounds("max_soc", 0, 100, max_soc);
        }

        e.build("TransactionLimitType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_instance() -> TransactionLimitType {
        TransactionLimitType {
            max_cost: Some(50.0),
            max_energy: Some(25.0),
            max_time: Some(3600),
            max_soc: Some(80),
        }
    }

    #[test]
    fn test_validate_success_all_fields_present() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_no_fields_present() {
        let data = TransactionLimitType {
            max_cost: None,
            max_energy: None,
            max_time: None,
            max_soc: None,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_negative_max_cost() {
        let mut data = create_test_instance();
        data.max_cost = Some(-10.0);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_max_soc_too_high() {
        let mut data = create_test_instance();
        data.max_soc = Some(101);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TransactionLimitType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
