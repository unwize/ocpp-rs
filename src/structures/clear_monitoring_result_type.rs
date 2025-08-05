use serde::{Deserialize, Serialize};
use crate::enums::clear_monitoring_status_enum_type::ClearMonitoringStatusEnumType;
use crate::errors::OcppError;

/// Result of a clear monitoring request.
/// Used by: ClearVariableMonitoringResponse
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearMonitoringResultType {
    /// Required. Result of the clear request for this monitor, identified by its id.
    pub status: ClearMonitoringStatusEnumType,
    /// Required. Id of the monitor of which a clear was requested.
    /// Constraints: 0 <= val
    pub id: i32,
    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>, // TODO: Implement StatusInfoType
}

impl ClearMonitoringResultType {
    /// Validates the fields of ClearMonitoringResultType based on specified constraints.
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

        // TODO: No validation for 'status' or 'status_info' without their type definitions.

        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "ClearMonitoringResultType".to_string(),
                source: errors,
            })
        }
    }
}

// Example usage (optional, for demonstration)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_deserialization() {
        let result = ClearMonitoringResultType {
            status: ClearMonitoringStatusEnumType::Accepted,
            id: 123,
            status_info: Some("Monitor cleared successfully".to_string()),
        };

        let serialized = serde_json::to_string(&result).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ClearMonitoringResultType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let result_minimal = ClearMonitoringResultType {
            status: ClearMonitoringStatusEnumType::Rejected,
            id: 0, // Valid
            status_info: None,
        };
        assert!(result_minimal.validate().is_ok());

        let result_positive_id = ClearMonitoringResultType {
            status: ClearMonitoringStatusEnumType::Accepted,
            id: 456, // Valid
            status_info: Some("Some info".to_string()),
        };
        assert!(result_positive_id.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_id() {
        let result = ClearMonitoringResultType {
            status: ClearMonitoringStatusEnumType::Rejected,
            id: -1, // Invalid
            status_info: None,
        };
        let err = result.validate().unwrap_err();
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
}
