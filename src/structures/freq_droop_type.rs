use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Used by: Common::FreqDroopGetType, SetDERControlRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FreqDroopType {
    /// Required. Priority of setting (0=highest)
    pub priority: i32,
    /// Required. Over-frequency start of droop
    pub over_freq: f64,
    /// Required. Under-frequency start of droop
    pub under_freq: f64,
    /// Required. Over-frequency droop per unit, oFDroop
    pub over_droop: f64,
    /// Required. Under-frequency droop per unit, uFDroop
    pub under_droop: f64,
    /// Required. Open loop response time in seconds
    pub response_time: f64,
    /// Optional. Time when this setting becomes active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    /// Optional. Duration in seconds that this setting is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}

impl Default for FreqDroopType {
    fn default() -> Self {
        Self {
            priority: 0,
            over_freq: 0.0,
            under_freq: 0.0,
            over_droop: 0.0,
            under_droop: 0.0,
            response_time: 0.0,
            start_time: None,
            duration: None,
        }
    }
}

impl OcppEntity for FreqDroopType {
    /// Validates the fields of FreqDroopType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("priority", 0, i32::MAX, self.priority);

        if let Some(duration) = self.duration {
            e.check_bounds("duration", 0.0, f64::MAX, duration);
        }

        e.build("FreqDroopType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json;

    #[test]
    fn test_validate_success() {
        let freq_droop_type = FreqDroopType {
            priority: 0,
            over_freq: 50.5,
            under_freq: 49.5,
            over_droop: 0.05,
            under_droop: 0.05,
            response_time: 1.0,
            start_time: Some(Utc::now()),
            duration: Some(3600.0),
        };
        assert!(freq_droop_type.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_priority() {
        let freq_droop_type = FreqDroopType {
            priority: -1,
            over_freq: 50.5,
            under_freq: 49.5,
            over_droop: 0.05,
            under_droop: 0.05,
            response_time: 1.0,
            start_time: None,
            duration: None,
        };
        let result = freq_droop_type.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        if let OcppError::StructureValidationError { related, .. } = err {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "priority");
            } else {
                panic!("Expected FieldValidationError for 'priority'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_duration() {
        let freq_droop_type = FreqDroopType {
            priority: 0,
            over_freq: 50.5,
            under_freq: 49.5,
            over_droop: 0.05,
            under_droop: 0.05,
            response_time: 1.0,
            start_time: None,
            duration: Some(-1.0),
        };
        let result = freq_droop_type.validate();
        assert!(result.is_err());
        let err = result.unwrap_err();
        if let OcppError::StructureValidationError { related, .. } = err {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "duration");
            } else {
                panic!("Expected FieldValidationError for 'duration'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = FreqDroopType {
            priority: 1,
            over_freq: 50.1,
            under_freq: 49.9,
            over_droop: 0.03,
            under_droop: 0.03,
            response_time: 2.0,
            start_time: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            duration: Some(1800.0),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: FreqDroopType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}