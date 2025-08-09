use serde::{Deserialize, Serialize};
use crate::enums::attribute_enum_type::AttributeEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::traits::OcppEntity;

/// Class to set components, variables and variable attributes and characteristics.
/// Used by: SetVariablesRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableDataType {
    /// Optional. The type of attribute: Actual, Target, MinSet, MaxSet. Default is Actual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    /// Required. Value to be assigned to the attribute of the variable.
    pub attribute_value: String,
    /// Required. The component for which the variable data is set.
    pub component: ComponentType,
    /// Required. The variable that needs to be set.
    pub variable: VariableType,
}

impl Default for SetVariableDataType {
    fn default() -> SetVariableDataType {
        Self {
            attribute_type: Some(AttributeEnumType::Actual),
            attribute_value: "".to_string(),
            component: Default::default(),
            variable: Default::default(),
        }
    }
}

impl OcppEntity for SetVariableDataType {
    /// Validates the fields of SetVariableDataType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_cardinality("attribute_value", 0, 2500, &self.attribute_value.chars());
        e.check_member("component", &self.component);
        e.check_member("variable", &self.variable);

        e.build("SetVariableDataType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let data = SetVariableDataType {
            attribute_value: "test".to_string(),
            ..Default::default()
        };
        assert!(data.validate().is_ok());
    }

    #[test]
    fn test_validate_failure_attribute_value_too_long() {
        let mut data = SetVariableDataType::default();
        data.attribute_value = "a".repeat(2501);
        let result = data.validate();
        assert!(result.is_err());
        if let OcppError::StructureValidationError { related, .. } = result.unwrap_err() {
            assert_eq!(related.len(), 1);
            if let OcppError::FieldValidationError { field, .. } = &related[0] {
                assert_eq!(field, "attribute_value");
            }
        }
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = SetVariableDataType {
            attribute_value: "test".to_string(),
            ..Default::default()
        };
        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: SetVariableDataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);
    }
}