use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Defines a relative time interval.
/// Used by: Common:SalesTariffEntryType
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct RelativeTimeIntervalType {
    /// Required. Start of the interval, in seconds from NOW.
    pub start: i32,
    /// Optional. Duration of the interval, in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

#[typetag::serde]
impl OcppEntity for RelativeTimeIntervalType {
    /// Validates the fields of RelativeTimeIntervalType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_bounds("start", 0, i32::MAX, self.start);
        if let Some(duration) = self.duration {
            e.check_bounds("duration", 0, i32::MAX, duration);
        }

        e.build("RelativeTimeIntervalType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{assert_invalid_fields, assert_num_field_errors};
    use serde_json;

    #[test]
    fn test_validate_success_full() {
        let interval = RelativeTimeIntervalType {
            start: 60,
            duration: Some(3600),
        };
        assert!(interval.validate().is_ok());
    }

    #[test]
    fn test_validate_success_minimal() {
        let interval = RelativeTimeIntervalType {
            start: 0,
            duration: None,
        };
        assert!(interval.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_start_negative() {
        let interval = RelativeTimeIntervalType {
            start: -1,
            duration: Some(3600),
        };
        let err = interval.validate().unwrap_err();
        assert_invalid_fields(&err, &["start"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_validate_failure_duration_negative() {
        let interval = RelativeTimeIntervalType {
            start: 60,
            duration: Some(-1),
        };
        let err = interval.validate().unwrap_err();
        assert_invalid_fields(&err, &["duration"]);
        assert_num_field_errors(&err, 1);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = RelativeTimeIntervalType {
            start: 120,
            duration: Some(300),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: RelativeTimeIntervalType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(original_struct, deserialized);
    }

    #[test]
    fn test_default() {
        let default_struct: RelativeTimeIntervalType = Default::default();
        assert_eq!(default_struct.start, 0);
        assert_eq!(default_struct.duration, None);
    }
}
