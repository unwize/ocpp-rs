use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

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
    pub params: PeriodicEventStreamParamsType, // TODO: Implement PeriodicEventStreamParamsType
}

impl ConstantStreamDataType {
    /// Validates the fields of ConstantStreamDataType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate id
        if self.id < 0 {
            errors.push(OcppError::FieldValidationError {
                field: "id".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.id.to_string(),
                    lower: "0".to_string(),
                    upper: "MAX_INT".to_string(), // No upper bound specified
                }],
            });
        }

        // Validate variable_monitoring_id
        if self.variable_monitoring_id < 0 {
            errors.push(OcppError::FieldValidationError {
                field: "variable_monitoring_id".to_string(),
                source: vec![OcppError::FieldBoundsError {
                    value: self.variable_monitoring_id.to_string(),
                    lower: "0".to_string(),
                    upper: "MAX_INT".to_string(), // No upper bound specified
                }],
            });
        }

        // TODO: No validation for 'params' without its type definition.

        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "ConstantStreamDataType".to_string(),
                source: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use crate::errors::assert_invalid_fields;
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let stream_data = ConstantStreamDataType {
            id: 1,
            variable_monitoring_id: 101,
            params: "params_placeholder".to_string(), // Placeholder
        };

        let serialized = serde_json::to_string(&stream_data).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ConstantStreamDataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(stream_data, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let stream_data = ConstantStreamDataType {
            id: 0, // Valid
            variable_monitoring_id: 0, // Valid
            params: "some_params".to_string(),
        };
        assert!(stream_data.validate().is_ok());

        let stream_data_positive = ConstantStreamDataType {
            id: 123,
            variable_monitoring_id: 456,
            params: "other_params".to_string(),
        };
        assert!(stream_data_positive.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_id() {
        let stream_data = ConstantStreamDataType {
            id: -1, // Invalid
            variable_monitoring_id: 101,
            params: "params".to_string(),
        };
        let err = stream_data.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
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
            params: "params".to_string(),
        };
        let err = stream_data.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
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
            id: -1, // Invalid 1
            variable_monitoring_id: -101, // Invalid 2
            params: "params".to_string(),
        };
        let err = stream_data.validate().unwrap_err();
        assert_invalid_fields(err, vec![
            "id".to_string(),
            "variable_monitoring_id".to_string()
        ]);
    }
}
