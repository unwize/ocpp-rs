use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Defines the stream parameters for a variable.
/// Used by: Common:ConstantStreamDataType, AdjustPeriodicEventStreamRequest, SetVariableMonitoringRequest, SetMonitoringDataRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeriodicEventStreamParamsType {
    /// Optional. Time in seconds after which stream data is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    /// Optional. Number of items to be sent together in stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<i32>,
}

impl Default for PeriodicEventStreamParamsType {
    fn default() -> Self {
        Self {
            interval: None,
            values: None,
        }
    }
}

impl OcppEntity for PeriodicEventStreamParamsType {
    /// Validates the fields of PeriodicEventStreamParamsType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(interval) = self.interval {
            e.check_bounds("interval", 0, i32::MAX, interval);
        }

        if let Some(values) = self.values {
            e.check_bounds("values", 0, i32::MAX, values);
        }

        e.build("PeriodicEventStreamParamsType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let params = PeriodicEventStreamParamsType {
            interval: Some(60),
            values: Some(10),
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let params = PeriodicEventStreamParamsType {
            interval: None,
            values: None,
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_validate_success_zero_values() {
        let params = PeriodicEventStreamParamsType {
            interval: Some(0),
            values: Some(0),
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_interval_negative() {
        let params = PeriodicEventStreamParamsType {
            interval: Some(-1), // Invalid value
            values: Some(10),
        };
        let result = params.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "interval");
            } else {
                panic!("Expected FieldValidationError for 'interval'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validate_failure_values_negative() {
        let params = PeriodicEventStreamParamsType {
            interval: Some(60),
            values: Some(-1), // Invalid value
        };
        let result = params.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "values");
            } else {
                panic!("Expected FieldValidationError for 'values'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = PeriodicEventStreamParamsType {
            interval: Some(30),
            values: Some(5),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: PeriodicEventStreamParamsType =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);

        let original_struct_minimal = PeriodicEventStreamParamsType {
            interval: None,
            values: None,
        };
        let serialized_minimal = serde_json::to_string(&original_struct_minimal).unwrap();
        let deserialized_minimal: PeriodicEventStreamParamsType =
            serde_json::from_str(&serialized_minimal).unwrap();
        assert_eq!(original_struct_minimal, deserialized_minimal);
    }
}
