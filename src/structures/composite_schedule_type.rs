use crate::enums::charging_rate_unit_enum_type::ChargingRateUnitEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::charging_schedule_period_type::ChargingSchedulePeriodType;
use crate::traits::OcppEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// CompositeScheduleType is used by: GetCompositeScheduleResponse
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompositeScheduleType {
    /// Required.
    /// Constraints: 0 <= val
    pub evse_id: i32,
    /// Required.
    pub duration: i32,
    /// Required.
    pub schedule_start: DateTime<Utc>,
    /// Required.
    pub charging_rate_unit: ChargingRateUnitEnumType,
    /// Required. List of ChargingSchedulePeriod elements defining maximum power or current over time.
    /// Cardinality 1..*
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
}

impl OcppEntity for CompositeScheduleType {
    /// Validates the fields of CompositeScheduleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("evse_id", 0, i32::MAX, self.evse_id);
        e.check_cardinality(
            "charging_schedule_period",
            1,
            usize::MAX,
            &self.charging_schedule_period.iter(),
        );
        e.check_iter_member(
            "charging_schedule_period",
            self.charging_schedule_period.iter(),
        );
        e.build("CompositeScheduletype")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};

    #[test]
    fn test_serialization_deserialization() {
        let schedule = CompositeScheduleType {
            evse_id: 1,
            duration: 3600,
            schedule_start: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            charging_rate_unit: ChargingRateUnitEnumType::A, // Placeholder
            charging_schedule_period: vec![Default::default(), Default::default()], // Placeholder
        };

        let serialized = serde_json::to_string(&schedule).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: CompositeScheduleType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schedule, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let schedule = CompositeScheduleType {
            evse_id: 0, // Valid
            duration: 1800,
            schedule_start: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            charging_rate_unit: ChargingRateUnitEnumType::W,
            charging_schedule_period: vec![Default::default()], // Valid cardinality
        };
        assert!(schedule.validate().is_ok());

        let schedule_multiple_periods = CompositeScheduleType {
            evse_id: 5,
            duration: 7200,
            schedule_start: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            charging_rate_unit: ChargingRateUnitEnumType::A,
            charging_schedule_period: vec![
                Default::default(),
                Default::default(),
                Default::default(),
            ],
        };
        assert!(schedule_multiple_periods.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_evse_id() {
        let schedule = CompositeScheduleType {
            evse_id: -1, // Invalid
            duration: 1800,
            schedule_start: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            charging_rate_unit: ChargingRateUnitEnumType::W,
            charging_schedule_period: vec![Default::default()],
        };
        let err = schedule.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["evse_id"]);
    }

    #[test]
    fn test_validation_charging_schedule_period_empty() {
        let schedule = CompositeScheduleType {
            evse_id: 1,
            duration: 1800,
            schedule_start: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            charging_rate_unit: ChargingRateUnitEnumType::W,
            charging_schedule_period: vec![], // Invalid cardinality
        };
        let err = schedule.validate().unwrap_err();
        assert_num_field_errors(&err, 1);
        assert_invalid_fields(&err, &["charging_schedule_period"]);
    }
}
