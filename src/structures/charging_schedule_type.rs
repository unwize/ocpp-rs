use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::enums::charging_rate_unit_enum_type::ChargingRateUnitEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::absolute_price_schedule_type::AbsolutePriceScheduleType;
use crate::structures::charging_schedule_period_type::ChargingSchedulePeriodType;
use crate::structures::limit_at_soc_type::LimitAtSOCType;
use crate::traits::OcppEntity;

/// Represents a charging schedule.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ChargingScheduleType {
    /// Required.
    pub id: i32,
    /// Optional. Starting point of an absolute schedule or recurring schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<DateTime<Utc>>,
    /// Optional. Duration of the charging schedule in seconds.
    /// If the duration is left empty, the last period will continue indefinitely
    /// or until the end of the transaction in case startSchedule is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Required. The unit of measure in which limits and setpoints are expressed.
    pub charging_rate_unit: ChargingRateUnitEnumType,
    /// Optional. Minimum charging rate supported by the EV.
    /// The unit of measure is defined by the ChargingRateUnit.
    /// This parameter is intended to be used by a local smart charging algorithm to optimize the power allocation
    /// for in the case a charging process is inefficient at lower charging rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_rate: Option<f64>,
    /// Optional. Power tolerance when following EVPowerProfile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_tolerance: Option<f64>,
    /// Optional. Id of this element for referencing in a signature.
    /// Constraints: 0 <= val
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_id: Option<i32>,
    /// Optional. Base64 encoded hash (SHA256 for ISO 15118-2; SHA512 for ISO 15118-20) of the EXI price schedule elements used in signature.
    /// String length: 0..88
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_value: Option<String>,
    /// Optional. Defaults to false. When true, disregard time zone offset in dateTime fields of ChargingScheduleType
    /// and use unqualified local time at Charging Station instead. This allows the same absolute or recurring
    /// charging profile to be used in both summer and winter time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_local_time: Option<bool>,
    /// Optional. Defaults to 0. When randomizedDelay not equals zero, then the start of each
    /// ChargingSchedulePeriodType is delayed by a randomly chosen number of seconds between 0 and randomizedDelay.
    /// Only allowed for TxProfile and TxDefaultProfile.
    /// Constraints: 0 <= val
    #[serde(skip_serializing_if = "Option::is_none")]
    pub randomized_delay: Option<i32>,

    /// Optional. Sales tariff for charging associated with this schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff: Option<SalesTarrifType>, // TODO: Implement SalesTariffType

    /// Required. List of ChargingSchedulePeriod elements defining maximum power or current usage over time.
    /// The maximum number of periods, that is supported by the Charging Station, if less than 1024, is set by device model variable SmartCharging.MaxPeriodsPerSchedule.
    /// This equals 131 bytes, which can be encoded as base64 in 176 bytes.
    /// Cardinality 1..1024
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    /// Optional. The ISO 15118-20 absolute price schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_price_schedule: Option<AbsolutePriceScheduleType>,
    /// Optional. The ISO 15118-20 price level schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_level_schedule: Option<String>,
    /// Optional. When present and SoC of EV is greater than or equal to soc, then charging limit or setpoint will be capped to the value of limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_at_soc: Option<LimitAtSOCType>, // TODO: Implement LimitAtSOCType
}

impl OcppEntity for ChargingScheduleType {
    /// Validates the fields of ChargingScheduleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(signature_id) = self.signature_id {
            e.check_bounds("signature_id", 0, i32::MAX, signature_id);
        }

        if let Some(digest_value) = &self.digest_value {
            e.check_cardinality("digest_value", 0, 88, &digest_value.chars());
        }

        if let Some(randomized_delay) = self.randomized_delay {
            e.check_bounds("randomized_delay", 0, i32::MAX, randomized_delay);
        }

        if let Some(sales_tariff) = &self.sales_tariff {
            e.check_member("sales_tariff", sales_tariff);
        }

        if let Some(absolute_price_schedule) = &self.absolute_price_schedule {
            e.check_member("absolute_price_schedule", absolute_price_schedule);
        }

        if let Some(limit_at_soc) = &self.limit_at_soc {
            e.check_member("limit_at_soc", limit_at_soc);
        }

        e.check_cardinality("charging_schedule_period", 1, 1024, &self.charging_schedule_period.iter());
        e.check_iter_member("charging_schedule_period", self.charging_schedule_period.iter());

        e.build("ChargingScheduleType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use crate::errors::assert_invalid_fields;

    #[test]
    fn test_serialization_deserialization() {
        let schedule = ChargingScheduleType {
            id: 1,
            start_schedule: Some(Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap()),
            duration: Some(3600),
            charging_rate_unit: ChargingRateUnitEnumType::A, // Placeholder
            min_charging_rate: Some(6.0),
            power_tolerance: Some(0.05),
            signature_id: Some(123),
            digest_value: Some("some_digest_value".to_string()),
            use_local_time: Some(true),
            randomized_delay: Some(30),
            sales_tariff: Some("tariff_id_example".to_string()), // Placeholder
            charging_schedule_period: vec![], // TODO: Placeholder
            absolute_price_schedule: None, // TODO: Placeholder
            price_level_schedule: Some("price_level_schedule_placeholder".to_string()), // Placeholder
            limit_at_soc: Some(Default::default()), // Placeholder
        };

        let serialized = serde_json::to_string(&schedule).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ChargingScheduleType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schedule, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let schedule = ChargingScheduleType {
            id: 1,
            start_schedule: None,
            duration: None,
            charging_rate_unit: ChargingRateUnitEnumType::W,
            min_charging_rate: None,
            power_tolerance: None,
            signature_id: Some(0), // Valid
            digest_value: Some("a".repeat(88)), // Valid length
            use_local_time: None,
            randomized_delay: Some(100), // Valid
            sales_tariff: None,
            charging_schedule_period: vec![], // TODO: Valid cardinality
            absolute_price_schedule: None,
            price_level_schedule: None,
            limit_at_soc: None,
        };
        assert!(schedule.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_signature_id() {
        let schedule = ChargingScheduleType {
            id: 1,
            start_schedule: None,
            duration: None,
            charging_rate_unit: ChargingRateUnitEnumType::W,
            min_charging_rate: None,
            power_tolerance: None,
            signature_id: Some(-5), // Invalid
            digest_value: None,
            use_local_time: None,
            randomized_delay: None,
            sales_tariff: None,
            charging_schedule_period: vec![],
            absolute_price_schedule: None,
            price_level_schedule: None,
            limit_at_soc: None,
        };
        let err = schedule.validate().unwrap_err();
        if let OcppError::StructureValidationError { related: source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "signature_id");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_digest_value_too_long() {
        let schedule = ChargingScheduleType {
            id: 1,
            start_schedule: None,
            duration: None,
            charging_rate_unit: ChargingRateUnitEnumType::W,
            min_charging_rate: None,
            power_tolerance: None,
            signature_id: None,
            digest_value: Some("a".repeat(89)), // Invalid length
            use_local_time: None,
            randomized_delay: None,
            sales_tariff: None,
            charging_schedule_period: vec![],
            absolute_price_schedule: None,
            price_level_schedule: None,
            limit_at_soc: None,
        };
        let err = schedule.validate().unwrap_err();
        if let OcppError::StructureValidationError { related: source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "digest_value");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_randomized_delay_negative() {
        let schedule = ChargingScheduleType {
            id: 1,
            start_schedule: None,
            duration: None,
            charging_rate_unit: ChargingRateUnitEnumType::W,
            min_charging_rate: None,
            power_tolerance: None,
            signature_id: None,
            digest_value: None,
            use_local_time: None,
            randomized_delay: Some(-1), // Invalid
            sales_tariff: None,
            charging_schedule_period: vec![],
            absolute_price_schedule: None,
            price_level_schedule: None,
            limit_at_soc: None,
        };
        let err = schedule.validate().unwrap_err();
        if let OcppError::StructureValidationError { related: source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "randomized_delay");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_charging_schedule_period_cardinality() {
        // Too few
        let schedule_too_few = ChargingScheduleType {
            id: 1,
            start_schedule: None,
            duration: None,
            charging_rate_unit: ChargingRateUnitEnumType::W,
            min_charging_rate: None,
            power_tolerance: None,
            signature_id: None,
            digest_value: None,
            use_local_time: None,
            randomized_delay: None,
            sales_tariff: None,
            charging_schedule_period: vec![], // Invalid cardinality
            absolute_price_schedule: None,
            price_level_schedule: None,
            limit_at_soc: None,
        };
        let err_too_few = schedule_too_few.validate().unwrap_err();
        if let OcppError::StructureValidationError { related: source, .. } = err_too_few {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "charging_schedule_period");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }

        // Too many
        let schedule_too_many = ChargingScheduleType {
            id: 1,
            start_schedule: None,
            duration: None,
            charging_rate_unit: ChargingRateUnitEnumType::W,
            min_charging_rate: None,
            power_tolerance: None,
            signature_id: None,
            digest_value: None,
            use_local_time: None,
            randomized_delay: None,
            sales_tariff: None,
            charging_schedule_period: vec![ChargingSchedulePeriodType::default(); 1025], // TODO: Invalid cardinality
            absolute_price_schedule: None,
            price_level_schedule: None,
            limit_at_soc: None,
        };
        let err_too_many = schedule_too_many.validate().unwrap_err();
        if let OcppError::StructureValidationError { related: source, .. } = err_too_many {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "charging_schedule_period");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let schedule = ChargingScheduleType {
            id: 1,
            start_schedule: None,
            duration: None,
            charging_rate_unit: ChargingRateUnitEnumType::W,
            min_charging_rate: None,
            power_tolerance: None,
            signature_id: Some(-5), // Invalid 1
            digest_value: Some("a".repeat(89)), // Invalid 2
            use_local_time: None,
            randomized_delay: Some(-1), // Invalid 3
            sales_tariff: None,
            charging_schedule_period: vec![], // Invalid 4
            absolute_price_schedule: None,
            price_level_schedule: None,
            limit_at_soc: None,
        };
        let err = schedule.validate().unwrap_err();
        assert_invalid_fields(err, vec![
            "signature_id".to_string(),
            "digest_value".to_string(),
            "randomized_delay".to_string(),
            "charging_schedule_period".to_string(),
        ]);
    }
}
