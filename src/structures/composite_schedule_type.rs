use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::errors::OcppError;
use crate::structures::charging_schedule_period_type::ChargingSchedulePeriodType;

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
    pub charging_rate_unit: ChargingRateUnitEnumType, // TODO: Implement ChargingRateUnitEnumType
    /// Required. List of ChargingSchedulePeriod elements defining maximum power or current over time.
    /// Cardinality 1..*
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
}

impl CompositeScheduleType {
    /// Validates the fields of CompositeScheduleType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate evse_id
        if self.evse_id < 0 {
            errors.push(OcppError::FieldValidationError {
                field: "evse_id".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.evse_id.to_string(),
                    lower: "0".to_string(),
                    upper: "MAX_INT".to_string(), // No upper bound specified
                }],
            });
        }

        // Validate charging_schedule_period cardinality
        if self.charging_schedule_period.is_empty() {
            errors.push(OcppError::FieldValidationError {
                field: "charging_schedule_period".to_string(),
                source: vec![OcppError::FieldCardinalityError {
                    cardinality: self.charging_schedule_period.len(),
                    lower: 1,
                    upper: usize::MAX, // Represents no upper limit
                }],
            });
        }
        // TODO: If ChargingSchedulePeriodType had its own validate method, iterate and call it here.
        // for (i, period) in self.charging_schedule_period.iter().enumerate() {
        //     if let Err(e) = period.validate() {
        //         errors.push(OcppError::FieldValidationError {
        //             field: format!("charging_schedule_period[{}]", i),
        //             source: vec![e],
        //         });
        //     }
        // }


        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "CompositeScheduleType".to_string(),
                source: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_serialization_deserialization() {
        let schedule = CompositeScheduleType {
            evse_id: 1,
            duration: 3600,
            schedule_start: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            charging_rate_unit: "A".to_string(), // Placeholder
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
            schedule_start: Utc.ymd(2025, 8, 1).and_hms(10, 0, 0),
            charging_rate_unit: "W".to_string(),
            charging_schedule_period: vec![Default::default()], // Valid cardinality
        };
        assert!(schedule.validate().is_ok());

        let schedule_multiple_periods = CompositeScheduleType {
            evse_id: 5,
            duration: 7200,
            schedule_start: Utc.with_ymd_and_hms(2025, 8, 1, 10, 0, 0).unwrap(),
            charging_rate_unit: "A".to_string(),
            charging_schedule_period: vec![Default::default(), Default::default(), Default::default()],
        };
        assert!(schedule_multiple_periods.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_evse_id() {
        let schedule = CompositeScheduleType {
            evse_id: -1, // Invalid
            duration: 1800,
            schedule_start: Utc.ymd(2025, 8, 1).and_hms(10, 0, 0),
            charging_rate_unit: "W".to_string(),
            charging_schedule_period: vec![Default::default()],
        };
        let err = schedule.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "evse_id");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_charging_schedule_period_empty() {
        let schedule = CompositeScheduleType {
            evse_id: 1,
            duration: 1800,
            schedule_start: Utc.ymd(2025, 8, 1).and_hms(10, 0, 0),
            charging_rate_unit: "W".to_string(),
            charging_schedule_period: vec![], // Invalid cardinality
        };
        let err = schedule.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
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
}
