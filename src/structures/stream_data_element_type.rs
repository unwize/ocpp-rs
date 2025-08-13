use serde::{Deserialize, Serialize};

use crate::errors::{OcppError, StructureValidationBuilder};
use crate::traits::OcppEntity;

/// Class to hold a stream data element.
/// Used by: NotifyPeriodicEventStream
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamDataElementType {
    /// Required. Offset relative to basetime of this message.
    pub t: f64,
    /// Required. The recorded value.
    pub v: String,
}

impl Default for StreamDataElementType {
    fn default() -> StreamDataElementType {
        Self {
            t: 0.0,
            v: "".to_string(),
        }
    }
}

impl OcppEntity for StreamDataElementType {
    /// Validates the fields of StreamDataElementType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("v", 0, 2500, &self.v.chars());

        e.build("StreamDataElementType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let data = StreamDataElementType {
            t: 123.45,
            v: "100.0".to_string(),
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_value_too_long() {
        let mut data = StreamDataElementType::default();
        data.v = "a".repeat(2501);
        let result = data.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = StreamDataElementType {
            t: 123.45,
            v: "100.0".to_string(),
        };
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: StreamDataElementType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
