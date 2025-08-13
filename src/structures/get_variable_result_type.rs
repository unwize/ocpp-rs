use crate::enums::attribute_enum_type::AttributeEnumType;
use crate::enums::get_variable_status_enum_type::GetVariableStatusEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::structures::status_info_type::StatusInfoType;
use crate::structures::variable_type::VariableType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Class to hold results of GetVariables request.
/// Used by: GetVariablesResponse
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GetVariableResultType {
    /// Required.
    pub attribute_status: GetVariableStatusEnumType,
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    /// Optional. Value of requested attribute type of component-variable. This field can only be empty when the given status is NOT accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    /// Required. Component for which the Variable is requested.
    pub component: ComponentType,
    /// Required. Variable for which the attribute value is requested.
    pub variable: VariableType,
    /// Optional. Detailed attribute status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>, // TODO: Implement StatusInfoType
}

impl OcppEntity for GetVariableResultType {
    /// Validates the fields of GetVariableResultType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        if let Some(attribute_value) = &self.attribute_value {
            e.check_cardinality("attribute_value", 0, 2500, &attribute_value.chars());
        }

        e.check_member("component", &self.component);
        e.check_member("variable", &self.variable);

        if let Some(attribute_status_info) = &self.attribute_status_info {
            e.check_member("attribute_status_info", attribute_status_info);
        }

        e.build("GetVariableResultType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::attribute_enum_type::AttributeEnumType;
    use crate::enums::get_variable_status_enum_type::GetVariableStatusEnumType;
    use crate::structures::component_type::ComponentType;
    use crate::structures::variable_type::VariableType;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let result_type = GetVariableResultType {
            attribute_status: GetVariableStatusEnumType::Accepted,
            attribute_type: Some(AttributeEnumType::Actual),
            attribute_value: Some("some_value".to_string()),
            component: ComponentType::default(),
            variable: VariableType::default(),
            attribute_status_info: Some(StatusInfoType::default()),
        };
        assert!(result_type.validate().is_ok());

        let result_type_minimal = GetVariableResultType {
            attribute_status: GetVariableStatusEnumType::Rejected,
            attribute_type: None,
            attribute_value: None,
            component: ComponentType::default(),
            variable: VariableType::default(),
            attribute_status_info: None,
        };
        assert!(result_type_minimal.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_value_length() {
        let result_type = GetVariableResultType {
            attribute_status: GetVariableStatusEnumType::Accepted,
            attribute_type: None,
            attribute_value: Some("a".repeat(2501)), // Invalid length
            component: ComponentType::default(),
            variable: VariableType::default(),
            attribute_status_info: None,
        };
        let result = result_type.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "attribute_value");
            } else {
                panic!("Expected FieldValidationError for 'attribute_value'");
            }
        } else {
            panic!("Expected StructureValidationError");
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = GetVariableResultType {
            attribute_status: GetVariableStatusEnumType::Accepted,
            attribute_type: Some(AttributeEnumType::Actual),
            attribute_value: Some("some_value".to_string()),
            component: ComponentType::default(),
            variable: VariableType::default(),
            attribute_status_info: Some(StatusInfoType::default()),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: GetVariableResultType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}
