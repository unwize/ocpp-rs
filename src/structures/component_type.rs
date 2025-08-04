use serde::{Deserialize, Serialize};
use crate::errors::OcppError;

/// A physical or logical component.
/// Used by: Common::ComponentVariableType, Common::MessageInfoType,
/// GetVariablesRequest.GetVariableDataType, GetVariablesResponse.GetVariableResultType,
/// NotifyMonitoringReportRequest.MonitoringDataType, NotifyReportRequest.ReportDataType,
/// SetVariableMonitoringRequest.SetMonitoringDataType, SetVariableMonitoringResponse.SetMonitoringResultType,
/// SetVariablesRequest.SetVariableDataType, SetVariablesResponse.SetVariableResultType, NotifyEventRequest.EventDataType
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentType {
    /// Required. Name of the component. Name should be taken from the list of standardized component names
    /// whenever possible. Case Insensitive. strongly advised to use Camel Case.
    /// String length: 0..50
    pub name: String,
    /// Optional. Name of instance in case the component exists as multiple instances.
    /// Case Insensitive. strongly advised to use Camel Case.
    /// String length: 0..50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Optional. Specifies the EVSE when component is located at EVSE level, also specifies the connector when
    /// component is located at Connector level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>, // TODO: Implement EVSEType
}

impl ComponentType {
    /// Validates the fields of ComponentType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    pub fn validate(&self) -> Result<(), OcppError> {
        let mut errors: Vec<OcppError> = Vec::new();

        // Validate name length
        if self.name.len() > 50 {
            errors.push(OcppError::FieldValidationError {
                field: "name".to_string(),
                source: vec![OcppError::FieldCardinalityError {
                    cardinality: self.name.len(),
                    lower: 0,
                    upper: 50,
                }],
            });
        }

        // Validate instance length
        if let Some(instance_name) = &self.instance {
            if instance_name.len() > 50 {
                errors.push(OcppError::FieldValidationError {
                    field: "instance".to_string(),
                    source: vec![OcppError::FieldCardinalityError {
                        cardinality: instance_name.len(),
                        lower: 0,
                        upper: 50,
                    }],
                });
            }
        }

        // TODO: No validation for 'evse' without its type definition.

        // Check if any errors occurred
        if errors.is_empty() {
            Ok(())
        } else {
            Err(OcppError::StructureValidationError {
                structure: "ComponentType".to_string(),
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
        let component = ComponentType {
            name: "Meter".to_string(),
            instance: Some("Main".to_string()),
            evse: Some("evse_placeholder".to_string()), // Placeholder
        };

        let serialized = serde_json::to_string(&component).unwrap();
        println!("Serialized: {}", serialized);

        let deserialized: ComponentType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(component, deserialized);
    }

    #[test]
    fn test_validation_valid() {
        let component_minimal = ComponentType {
            name: "Core".to_string(),
            instance: None,
            evse: None,
        };
        assert!(component_minimal.validate().is_ok());

        let component_full_lengths = ComponentType {
            name: "a".repeat(50), // Valid length
            instance: Some("b".repeat(50)), // Valid length
            evse: Some("evse_placeholder".to_string()),
        };
        assert!(component_full_lengths.validate().is_ok());
    }

    #[test]
    fn test_validation_name_too_long() {
        let component = ComponentType {
            name: "a".repeat(51), // Invalid
            instance: None,
            evse: None,
        };
        let err = component.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "name");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_instance_too_long() {
        let component = ComponentType {
            name: "ValidName".to_string(),
            instance: Some("a".repeat(51)), // Invalid
            evse: None,
        };
        let err = component.validate().unwrap_err();
        if let OcppError::StructureValidationError { source, .. } = err {
            assert_eq!(source.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &source[0] {
                assert_eq!(field, "instance");
            } else {
                panic!("Expected FieldValidationError");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_validation_multiple_errors() {
        let component = ComponentType {
            name: "a".repeat(51), // Invalid 1
            instance: Some("b".repeat(51)), // Invalid 2
            evse: None,
        };
        let err = component.validate().unwrap_err();
        assert_invalid_fields(err, vec![
            "name".to_string(),
            "instance".to_string(),
        ]);
    }
}
