use crate::enums::attribute_enum_type::AttributeEnumType;
use crate::errors::{OcppError, StructureValidationBuilder};
use crate::structures::component_type::ComponentType;
use crate::structures::variable_type::VariableType;
use crate::traits::OcppEntity;
use serde::{Deserialize, Serialize};

/// Class to hold parameters for GetVariables request.
/// Used by: GetVariablesRequest
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GetVariableDataType {
    /// Optional. Attribute type for which value is requested. When absent, default Actual is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,
    /// Required. Component for which the Variable is requested.
    pub component: ComponentType,
    /// Required. Variable for which the attribute value is requested.
    pub variable: VariableType,
}

impl OcppEntity for GetVariableDataType {
    /// Validates the fields of GetVariableDataType based on specified constraints.
    /// Returns `Ok(())` if all values are valid, or `Err(OcppError::StructureValidationError)` if validation fails.
    fn validate(&self) -> Result<(), OcppError> {
        let mut e = StructureValidationBuilder::new();

        e.check_member("component", &self.component);

        e.check_member("variable", &self.variable);

        e.build("GetVariableDataType")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::attribute_enum_type::AttributeEnumType;
    use crate::structures::component_type::ComponentType;
    use serde_json;

    #[test]
    fn test_validate_success() {
        let get_variable_data_type = GetVariableDataType {
            attribute_type: Some(AttributeEnumType::Actual),
            component: ComponentType::default(),
            variable: VariableType::default(),
        };
        assert!(get_variable_data_type.validate().is_ok());

        let get_variable_data_type_none = GetVariableDataType {
            attribute_type: None,
            component: ComponentType::default(),
            variable: VariableType::default(),
        };
        assert!(get_variable_data_type_none.validate().is_ok());
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_struct = GetVariableDataType {
            attribute_type: Some(AttributeEnumType::Actual),
            component: ComponentType::default(),
            variable: VariableType::default(),
        };

        let serialized = serde_json::to_string(&original_struct).unwrap();
        let deserialized: GetVariableDataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_struct, deserialized);

        let original_struct_none = GetVariableDataType {
            attribute_type: None,
            component: ComponentType::default(),
            variable: VariableType::default(),
        };

        let serialized_none = serde_json::to_string(&original_struct_none).unwrap();
        let deserialized_none: GetVariableDataType =
            serde_json::from_str(&serialized_none).unwrap();
        assert_eq!(original_struct_none, deserialized_none);
    }
}
