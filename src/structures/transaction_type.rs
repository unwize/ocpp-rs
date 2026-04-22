use crate::enums::charging_state_enum_type::ChargingStateEnumType;
use crate::enums::operation_mode_enum_type::OperationModeEnumType;
use crate::enums::reason_enum_type::ReasonEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::transaction_limit_type::TransactionLimitType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {
    /// Required. This contains the Id of the transaction.
    pub transaction_id: String,
    /// Optional. Current charging state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_state: Option<ChargingStateEnumType>,
    /// Optional. The total time energy flowed from EVSE to EV during the transaction, in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spent_charging: Option<i32>,
    /// Optional. The reason/event that initiated the process of stopping the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<ReasonEnumType>,
    /// Optional. The ID given to a remote start request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_start_id: Option<i32>,
    /// Optional. The operation mode that is currently in effect for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_mode: Option<OperationModeEnumType>,
    /// Optional. ID of the tariff in use for the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff_id: Option<String>,
    /// Optional. Maximum cost/energy/time allowed for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_limit: Option<TransactionLimitType>,
}

impl OcppEntity for TransactionType {
    /// Validates the fields of TransactionType based on specified constraints.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("transaction_id", 0, 36, &self.transaction_id.chars());

        if let Some(time_spent_charging) = self.time_spent_charging {
            e.check_bounds("time_spent_charging", 0, i32::MAX, time_spent_charging);
        }

        if let Some(tariff_id) = &self.tariff_id {
            e.check_cardinality("tariff_id", 0, 60, &tariff_id.chars());
        }

        if let Some(transaction_limit) = &self.transaction_limit {
            e.check_member("transaction_limit", transaction_limit);
        }

        e.build("TransactionType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::transaction_limit_type::TransactionLimitType;
    use serde_json;

    fn create_test_instance() -> TransactionType {
        TransactionType {
            transaction_id: "tx123".to_string(),
            charging_state: Some(ChargingStateEnumType::EVConnected),
            time_spent_charging: Some(3600),
            stopped_reason: Some(ReasonEnumType::Local),
            remote_start_id: Some(1),
            operation_mode: Some(OperationModeEnumType::LocalFrequency),
            tariff_id: Some("tariff-001".to_string()),
            transaction_limit: Some(TransactionLimitType::default()),
        }
    }

    #[test]
    fn test_validate_success_all_fields() {
        let data = create_test_instance();
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal_fields() {
        let data = TransactionType {
            transaction_id: "tx123".to_string(),
            charging_state: None,
            time_spent_charging: None,
            stopped_reason: None,
            remote_start_id: None,
            operation_mode: None,
            tariff_id: None,
            transaction_limit: None,
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_transaction_id_too_long() {
        let mut data = create_test_instance();
        data.transaction_id = "a".repeat(37);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_time_spent_charging_negative() {
        let mut data = create_test_instance();
        data.time_spent_charging = Some(-10);
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_validate_failure_tariff_id_too_long() {
        let mut data = create_test_instance();
        data.tariff_id = Some("a".repeat(61));
        assert!(data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = create_test_instance();
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: TransactionType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
