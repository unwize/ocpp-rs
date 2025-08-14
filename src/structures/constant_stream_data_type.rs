use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::periodic_event_stream_params_type::PeriodicEventStreamParamsType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// ConstantStreamDataType is used by: OpenPeriodicEventStreamRequest, GetPeriodicEventStreamResponse
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConstantStreamDataType {
    /// Required. Uniquely identifies the stream
    /// Constraints: 0 <= val
    pub id: i32,
    /// Required. Id of monitor used to report his event. It can be a preconfigured or hardwired monitor.
    /// Constraints: 0 <= val
    pub variable_monitoring_id: i32,
    /// Required. Max time and items parameters
    pub params: PeriodicEventStreamParamsType,
}

impl OcppEntity for ConstantStreamDataType {
    /// Validates the fields of ConstantStreamDataType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();
        e.check_bounds("id", 0, i32::MAX, self.id);
        e.check_bounds(
            "variable_monitoring_id",
            0,
            i32::MAX,
            self.variable_monitoring_id,
        );
        e.check_member("params", &self.params);
        e.build("ConstantStreamDataType")
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::assert_invalid_fields;

    #[test]
    fn test_serialization_deserialization() {
        let stream_data = ConstantStreamDataType {
            id: 1,
            variable_monitoring_id: 101,
            params: Default::default(),
        };

        let serialized = serde_json::to_string(&stream_data).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ConstantStreamDataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stream_data, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let stream_data = ConstantStreamDataType {
            id: 0,                     // Valid
            variable_monitoring_id: 0, // Valid
            params: Default::default(),
        };
        assert!(stream_data.validate().is_ok());

        let stream_data_positive = ConstantStreamDataType {
            id: 123,
            variable_monitoring_id: 456,
            params: Default::default(),
        };
        assert!(stream_data_positive.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_id() {
        let stream_data = ConstantStreamDataType {
            id: -1, // Invalid
            variable_monitoring_id: 101,
            params: Default::default(),
        };
        let err = stream_data.validate().unwrap_err();
        if let OcppError::StructureValidationError {
            related: source, ..
        } = err
        {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "id");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_invalid_variable_monitoring_id() {
        let stream_data = ConstantStreamDataType {
            id: 1,
            variable_monitoring_id: -101, // Invalid
            params: Default::default(),
        };
        let err = stream_data.validate().unwrap_err();
        if let OcppError::StructureValidationError {
            related: source, ..
        } = err
        {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "variable_monitoring_id");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let stream_data = ConstantStreamDataType {
            id: -1,                       // Invalid 1
            variable_monitoring_id: -101, // Invalid 2
            params: Default::default(),
        };
        let err = stream_data.validate().unwrap_err();
        assert_invalid_fields(
            err,
            &["id", "variable_monitoring_id"],
        );
    }
}
