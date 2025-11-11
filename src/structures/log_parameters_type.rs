use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Generic class for the configuration of logging entries.
/// Used by: GetLogRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType {
    /// Required. The URL of the location at the remote system where the log should be stored.
    pub remote_location: String,
    /// Optional. The date and time of the oldest logging information to include in the diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_timestamp: Option<DateTime<Utc>>,
    /// Optional. The date and time of the latest logging information to include in the diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_timestamp: Option<DateTime<Utc>>,
}

impl Default for LogParametersType {
    fn default() -> LogParametersType {
        Self {
            remote_location: "".to_string(),
            oldest_timestamp: None,
            latest_timestamp: None,
        }
    }
}

impl OcppEntity for LogParametersType {
    /// Validates the fields of LogParametersType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("remote_location", 0, 2000, &self.remote_location.chars());

        if let Some(oldest_timestamp) = &self.oldest_timestamp {
            if let Some(newest_timestamp) = &self.latest_timestamp {
                if newest_timestamp < oldest_timestamp {
                    e.push_relation_error(
                        "oldest_timestamp",
                        "latest_timestamp",
                        "latest_timestamp must be greater than oldest_timestamp!",
                    );
                }
            }
        }

        e.build("LogParametersType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let log_parameters = LogParametersType {
            remote_location: "http://example.com/logs".to_string(),
            oldest_timestamp: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            latest_timestamp: Some(Utc.timestamp_opt(1672534800, 0).unwrap()),
        };
        assert!(log_parameters.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let log_parameters = LogParametersType {
            remote_location: "http://example.com/logs".to_string(),
            oldest_timestamp: None,
            latest_timestamp: None,
        };
        assert!(log_parameters.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_remote_location_length() {
        let log_parameters = LogParametersType {
            remote_location: "a".repeat(2001), // Invalid length
            oldest_timestamp: None,
            latest_timestamp: None,
        };
        let result = log_parameters.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "remote_location");
            } else {
                panic!("Expected FieldValidationError for 'remote_location'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = LogParametersType {
            remote_location: "http://example.com/logs".to_string(),
            oldest_timestamp: Some(Utc.timestamp_opt(1672531200, 0).unwrap()),
            latest_timestamp: Some(Utc.timestamp_opt(1672534800, 0).unwrap()),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: LogParametersType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }
}
